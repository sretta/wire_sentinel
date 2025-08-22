use actix::{Actor, Context, Handler, Addr, ResponseFuture};
use crate::messages::{IpChange, UpdateGandi, UpdateWireguard};
use crate::address_change::{AddressChange, AddressScope};
use crate::actors::gandi_updater::GandiUpdaterActor;
use crate::actors::wireguard_updater::WireguardUpdaterActor;

pub struct ChangeRouterActor {
    gandi_updater: Addr<GandiUpdaterActor>,
    wireguard_updater: Addr<WireguardUpdaterActor>,
}

impl ChangeRouterActor {
    pub fn new(gandi_updater: Addr<GandiUpdaterActor>, wireguard_updater: Addr<WireguardUpdaterActor>) -> Self {
        Self { gandi_updater, wireguard_updater }
    }
}

impl Actor for ChangeRouterActor {
    type Context = Context<Self>;
}

impl Handler<IpChange> for ChangeRouterActor {
    type Result = ResponseFuture<()>;

    fn handle(&mut self, msg: IpChange, _ctx: &mut Context<Self>) -> Self::Result {
        let gandi_updater = self.gandi_updater.clone();
        let wireguard_updater = self.wireguard_updater.clone();

        Box::pin(async move {
            let change = msg.0;
            if is_relevant(&change) {
                if let AddressChange::AdditionV6(address) = change {
                    match gandi_updater.send(UpdateGandi(address)).await {
                        Ok(Ok(_)) => {
                            wireguard_updater.do_send(UpdateWireguard);
                        }
                        _ => {
                            log::error!("Gandi update failed, not updating wireguard.");
                        }
                    }
                }
            }
        })
    }
}

fn is_relevant(change: &AddressChange) -> bool {
    match change {
        AddressChange::AdditionV4(_) => false,
        AddressChange::AdditionV6(v6) => match v6.scope {
            AddressScope::Link => false,
            AddressScope::Global => {
                (!v6.address.starts_with("fd")) && (!v6.address.starts_with("fec"))
            }
        },
        AddressChange::DeletionV4(_) => false,
        AddressChange::DeletionV6(_) => false,
    }
}
