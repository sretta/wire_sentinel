use actix::{Actor, Context, Handler, ResponseFuture};
use crate::config::SentinelConfig;
use crate::gandi_net_livedns;
use crate::messages::UpdateGandi;
use tokio::task;

pub struct GandiUpdaterActor {
    config: SentinelConfig,
}

impl GandiUpdaterActor {
    pub fn new(config: SentinelConfig) -> Self {
        Self { config }
    }
}

impl Actor for GandiUpdaterActor {
    type Context = Context<Self>;
}

impl Handler<UpdateGandi> for GandiUpdaterActor {
    type Result = ResponseFuture<Result<(), ()>>;

    fn handle(&mut self, msg: UpdateGandi, _ctx: &mut Context<Self>) -> Self::Result {
        let config = self.config.clone();
        let address = msg.0;

        Box::pin(async move {
            let result = task::spawn_blocking(move || {
                gandi_net_livedns::update_record(&config, &address)
            }).await;

            match result {
                Ok(Ok(_)) => {
                    log::info!("Gandi DNS record updated successfully.");
                    Ok(())
                }
                Ok(Err(e)) => {
                    log::error!("Failed to update Gandi DNS record: {}", e);
                    Err(())
                }
                Err(e) => {
                    log::error!("Gandi DNS update task panicked: {}", e);
                    Err(())
                }
            }
        })
    }
}
