use actix::{Actor, Context, Recipient};
use crate::config::SentinelConfig;
use crate::ip_monitor::IpMonitorContext;
use crate::messages::IpChange;
use tokio::task;

pub struct IpMonitorActor {
    config: SentinelConfig,
    router: Recipient<IpChange>,
}

impl IpMonitorActor {
    pub fn new(config: SentinelConfig, router: Recipient<IpChange>) -> Self {
        Self { config, router }
    }
}

impl Actor for IpMonitorActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("IpMonitorActor started");

        let config = self.config.clone();
        let router = self.router.clone();

        task::spawn_blocking(move || {
            let mut ip_monitor_context = IpMonitorContext::initialize(&config).unwrap();

            loop {
                let change_result = ip_monitor_context.listen_for_addr_changes();
                match change_result {
                    Ok(change) => {
                        router.do_send(IpChange(change));
                    }
                    Err(e) => {
                        log::error!("Error listening for address changes: {}", e);
                    }
                }
            }
        });
    }
}
