use actix::{Actor, Context, Handler};
use crate::config::SentinelConfig;
use crate::wireguard_peer;
use crate::messages::UpdateWireguard;
use tokio::task;

pub struct WireguardUpdaterActor {
    config: SentinelConfig,
}

impl WireguardUpdaterActor {
    pub fn new(config: SentinelConfig) -> Self {
        Self { config }
    }
}

impl Actor for WireguardUpdaterActor {
    type Context = Context<Self>;
}

impl Handler<UpdateWireguard> for WireguardUpdaterActor {
    type Result = ();

    fn handle(&mut self, _msg: UpdateWireguard, _ctx: &mut Context<Self>) {
        let config = self.config.clone();

        task::spawn_blocking(move || {
            match wireguard_peer::update_peer(&config) {
                Ok(_) => {
                    log::info!("WireGuard peer updated successfully.");
                }
                Err(e) => {
                    log::error!("Failed to update WireGuard peer: {}", e);
                }
            }
        });
    }
}
