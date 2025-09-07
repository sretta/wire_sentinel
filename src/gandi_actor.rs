use std::sync::mpsc::Receiver;
use std::thread;
use log;
use crate::actor_messages::ActorMessage;
use crate::config::SentinelConfig;
use crate::gandi_net_livedns;
use crate::address_change::{AddressChange, AddressScope, IpV6Address};

pub struct GandiActor {
    config: SentinelConfig,
    receiver: Receiver<ActorMessage>,
    last_ipv6_address: Option<IpV6Address>,
}

impl GandiActor {
    pub fn new(config: SentinelConfig, receiver: Receiver<ActorMessage>) -> Self {
        GandiActor {
            config,
            receiver,
            last_ipv6_address: None,
        }
    }

    pub fn start(mut self) {
        thread::spawn(move || {
            for msg in self.receiver.iter() {
                if let ActorMessage::AddressChanged(AddressChange::AdditionV6(ipv6_address)) = msg {
                    if self.is_relevant(&ipv6_address) {
                        if Some(ipv6_address.clone()) != self.last_ipv6_address {
                            log::debug!("GandiActor: Received new IPv6 address: {:?}", ipv6_address);
                            match gandi_net_livedns::update_record(&self.config, &ipv6_address) {
                                Ok(_) => {
                                    log::info!("GandiActor: Successfully updated Gandi DNS record.");
                                    self.last_ipv6_address = Some(ipv6_address);
                                }
                                Err(e) => {
                                    log::error!("GandiActor: Failed to update Gandi DNS record: {}", e);
                                }
                            }
                        } else {
                            log::debug!("GandiActor: Received duplicate IPv6 address. Ignoring.");
                        }
                    }
                }
            }
        });
    }

    fn is_relevant(&self, address: &IpV6Address) -> bool {
        match address.scope {
            AddressScope::Link => false,
            AddressScope::Global => {
                (!address.address.starts_with("fd")) && (!address.address.starts_with("fec"))
            }
        }
    }
}
