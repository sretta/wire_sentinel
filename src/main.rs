use log;
use std::sync::mpsc;
use std::thread;
use std::time::SystemTime;
use crate::gandi_actor::GandiActor;
use crate::input_actor::InputActor;
use crate::wireguard_actor::WireguardActor;

mod address_change;
mod config;
mod gandi_net_livedns;
mod ip_monitor;
mod parse_monitor;
mod wireguard_peer;
mod actor_messages;
mod input_actor;
mod gandi_actor;
mod wireguard_actor;

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

    // Create channels
    let (gandi_sender, gandi_receiver) = mpsc::channel();
    let (wireguard_sender, wireguard_receiver) = mpsc::channel();

    // Create actors
    let input_actor = InputActor::new(config.clone(), gandi_sender, wireguard_sender);
    let gandi_actor = GandiActor::new(config.clone(), gandi_receiver);
    let wireguard_actor = WireguardActor::new(config.clone(), wireguard_receiver);

    // Start actors
    input_actor.start();
    gandi_actor.start();
    wireguard_actor.start();

    // Wait forever
    thread::park();

    Ok(())
}
