use std::sync::mpsc::Sender;
use std::thread;
use log;
use crate::actor_messages::ActorMessage;
use crate::config::SentinelConfig;
use crate::ip_monitor::IpMonitorContext;

pub struct InputActor {
    config: SentinelConfig,
    gandi_sender: Sender<ActorMessage>,
    wireguard_sender: Sender<ActorMessage>,
}

impl InputActor {
    pub fn new(
        config: SentinelConfig,
        gandi_sender: Sender<ActorMessage>,
        wireguard_sender: Sender<ActorMessage>,
    ) -> Self {
        InputActor {
            config,
            gandi_sender,
            wireguard_sender,
        }
    }

    pub fn start(self) {
        thread::spawn(move || {
            let mut ip_monitor_context = IpMonitorContext::initialize(&self.config).unwrap();
            loop {
                match ip_monitor_context.listen_for_addr_changes() {
                    Ok(address_change) => {
                        log::debug!("InputActor: Received address change: {:?}", address_change);
                        let message = ActorMessage::AddressChanged(address_change);
                        if let Err(e) = self.gandi_sender.send(message.clone()) {
                            log::error!("InputActor: Failed to send message to Gandi actor: {}", e);
                        }
                        if let Err(e) = self.wireguard_sender.send(message) {
                            log::error!("InputActor: Failed to send message to Wireguard actor: {}", e);
                        }
                    }
                    Err(e) => {
                        log::error!("InputActor: Error listening for address changes: {}", e);
                    }
                }
            }
        });
    }
}
