use std::time::SystemTime;
use crate::config::SentinelConfig;
use crate::ip_monitor::{IpMonitorContext};
use log;
use crate::address_change::{AddressChange, AddressScope};

mod config;
mod ip_monitor;
mod parse_monitor;
mod gandi_net_livedns;
mod wireguard_peer;
mod address_change;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger()?;

    let config = config::load_config()?;
    start_main_loop(&config)?;

    Ok(())
}

fn start_main_loop(config: &SentinelConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut ip_monitor_context = IpMonitorContext::initialize(config)?;

    let mut applicable_change = Box::new(None);
    let mut gandi_needs_update = false;
    let mut wireguard_needs_update = false;

    loop {
        log::trace!("Entering main loop.");

        let change = ip_monitor_context.listen_for_addr_changes()?;

        if is_relevant(&change) {
            applicable_change = Box::new(Some(change));
            gandi_needs_update = true;
            wireguard_needs_update = true;
        }

        if gandi_needs_update {
            match applicable_change.as_ref() {
                Some(AddressChange::AdditionV6(address)) => {
                    let gandi_result = gandi_net_livedns::update_record(&config, address);
                    match gandi_result {
                        Ok(_) => {
                            gandi_needs_update = false;

                            if wireguard_needs_update {
                                let wireguard_result = wireguard_peer::update_peer(&config);
                                match wireguard_result {
                                    Ok(_) => {
                                        wireguard_needs_update = false
                                    }
                                    Err(failure) => { log::error!("wireguard update failed: {:?}", failure) }
                                }
                            }
                        }
                        Err(failure) => { log::error!("gandi update failed: {:?}", failure) }
                    }
                }
                Some(AddressChange::DeletionV6(_)) => {}
                Some(AddressChange::AdditionV4(_)) => {}
                Some(AddressChange::DeletionV4(_)) => {}
                None => {}
            }
        }
    }
}

fn is_relevant(change: &AddressChange) -> bool {
    match change {
        AddressChange::AdditionV4(_) => { false }
        AddressChange::AdditionV6(v6) => {
            match v6.scope {
                AddressScope::Link => { false }
                AddressScope::Global => {
                    (!v6.address.starts_with("fd"))
                        && (!v6.address.starts_with("fec"))
                }
            }
        }
        AddressChange::DeletionV4(_) => { false }
        AddressChange::DeletionV6(_) => { false }
    }
}
