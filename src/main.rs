use crate::config::SentinelConfig;
use log;
use std::time::SystemTime;
use tokio::sync::broadcast;
use crate::actors::{GandiUpdaterActor, IpMonitorActor, WireguardUpdaterActor};

mod address_change;
mod config;
mod gandi_net_livedns;
mod ip_monitor;
mod parse_monitor;
mod wireguard_peer;
mod actors;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger()?;

    let config = config::load_config()?;
    start_actors(config).await?;

    Ok(())
}

async fn start_actors(config: SentinelConfig) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx1) = broadcast::channel(16);
    let rx2 = tx.subscribe();

    let ip_monitor_actor = IpMonitorActor::new(config.clone(), tx);
    let gandi_updater_actor = GandiUpdaterActor::new(config.clone(), rx1);
    let wireguard_updater_actor = WireguardUpdaterActor::new(config.clone(), rx2);

    let ip_monitor_handle = tokio::spawn(async move {
        let mut actor = ip_monitor_actor;
        actor.run().await;
    });

    let gandi_updater_handle = tokio::spawn(async move {
        let mut actor = gandi_updater_actor;
        actor.run().await;
    });

    let wireguard_updater_handle = tokio::spawn(async move {
        let mut actor = wireguard_updater_actor;
        actor.run().await;
    });

    tokio::try_join!(
        ip_monitor_handle,
        gandi_updater_handle,
        wireguard_updater_handle
    )?;

    Ok(())
}
