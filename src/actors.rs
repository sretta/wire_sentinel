use crate::address_change::{AddressChange, AddressScope};
use crate::config::SentinelConfig;
use crate::gandi_net_livedns;
use crate::ip_monitor::IpMonitorContext;
use crate::wireguard_peer;
use async_channel::{Receiver, Sender};
use log;
use std::thread;

pub struct IpMonitor;

impl IpMonitor {
    pub fn run(config: SentinelConfig, sinks: Vec<Sender<AddressChange>>) {
        thread::spawn(move || {
            let mut ip_monitor_context = IpMonitorContext::initialize(&config).unwrap();
            loop {
                match ip_monitor_context.listen_for_addr_changes() {
                    Ok(change) => {
                        for sink in &sinks {
                            if let Err(e) = smol::block_on(sink.send(change.clone())) {
                                log::error!("Error sending to sink: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Error listening for address changes: {}", e);
                    }
                }
            }
        });
    }
}

pub struct GandiUpdater;

impl GandiUpdater {
    pub async fn run(config: SentinelConfig, receiver: Receiver<AddressChange>) {
        while let Ok(change) = receiver.recv().await {
            if is_relevant(&change) {
                if let AddressChange::AdditionV6(address) = change {
                    if let Err(e) = gandi_net_livedns::update_record(&config, &address) {
                        log::error!("Gandi update failed: {:?}", e);
                    } else {
                        log::info!("Gandi update successful");
                    }
                }
            }
        }
    }
}

pub struct WireGuardUpdater;

impl WireGuardUpdater {
    pub async fn run(config: SentinelConfig, receiver: Receiver<AddressChange>) {
        while let Ok(change) = receiver.recv().await {
            if is_relevant(&change) {
                if let Err(e) = wireguard_peer::update_peer(&config) {
                    log::error!("WireGuard update failed: {:?}", e);
                } else {
                    log::info!("WireGuard update successful");
                }
            }
        }
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
