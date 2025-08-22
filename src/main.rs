use actix::Actor;
use log;
use std::time::SystemTime;

use crate::actors::change_router::ChangeRouterActor;
use crate::actors::gandi_updater::GandiUpdaterActor;
use crate::actors::ip_monitor::IpMonitorActor;
use crate::actors::wireguard_updater::WireguardUpdaterActor;

mod address_change;
mod config;
mod gandi_net_livedns;
mod ip_monitor;
mod parse_monitor;
mod wireguard_peer;

mod actors;
mod messages;

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

#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger()?;

    let config = config::load_config()?;

    // Start the sink actors
    let gandi_updater = GandiUpdaterActor::new(config.clone()).start();
    let wireguard_updater = WireguardUpdaterActor::new(config.clone()).start();

    // Start the router actor
    let router = ChangeRouterActor::new(gandi_updater, wireguard_updater).start();

    // Start the source actor
    let _ip_monitor = IpMonitorActor::new(config, router.recipient()).start();

    log::info!("wire-sentinel is running.");
    tokio::signal::ctrl_c().await?;
    log::info!("wire-sentinel is shutting down.");

    Ok(())
}
