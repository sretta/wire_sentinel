use std::sync::mpsc::Receiver;
use std::thread;
use log;
use crate::actor_messages::ActorMessage;
use crate::address_change::{AddressChange, AddressScope, IpV6Address};
use crate::config::SentinelConfig;
use crate::wireguard_peer;

pub struct WireguardActor {
    config: SentinelConfig,
    receiver: Receiver<ActorMessage>,
    last_ipv6_address: Option<IpV6Address>,
}

impl WireguardActor {
    pub fn new(config: SentinelConfig, receiver: Receiver<ActorMessage>) -> Self {
        WireguardActor {
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
                            log::debug!("WireguardActor: Received new IPv6 address: {:?}", ipv6_address);
                            match wireguard_peer::update_peer(&self.config) {
                                Ok(_) => {
                                    log::info!("WireguardActor: Successfully updated WireGuard peer.");
                                    self.last_ipv6_address = Some(ipv6_address);
                                }
                                Err(e) => {
                                    log::error!("WireguardActor: Failed to update WireGuard peer: {}", e);
                                }
                            }
                        } else {
                            log::debug!("WireguardActor: Received duplicate IPv6 address. Ignoring.");
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
