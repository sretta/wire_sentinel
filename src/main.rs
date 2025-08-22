use crate::actors::{GandiUpdater, IpMonitor, WireGuardUpdater};
use log;
use std::time::SystemTime;

mod actors;
mod address_change;
mod config;
mod gandi_net_livedns;
mod ip_monitor;
mod parse_monitor;
mod wireguard_peer;

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

    smol::block_on(async {
        let (gandi_sender, gandi_receiver) = async_channel::unbounded();
        let (wireguard_sender, wireguard_receiver) = async_channel::unbounded();

        let sinks = vec![gandi_sender, wireguard_sender];

        IpMonitor::run(config.clone(), sinks);

        let gandi_handle = smol::spawn(GandiUpdater::run(config.clone(), gandi_receiver));
        let wireguard_handle =
            smol::spawn(WireGuardUpdater::run(config.clone(), wireguard_receiver));

        log::info!("Actors started, application is running.");

        // We can wait on the handles if we want to exit when they are done.
        // Or just wait forever if they are supposed to run forever.
        gandi_handle.await;
        wireguard_handle.await;
    });

    Ok(())
}
