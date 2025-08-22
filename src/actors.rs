use tokio::sync::broadcast;
use crate::address_change::{AddressChange, AddressScope, IpV6Address};
use crate::config::SentinelConfig;
use tokio::process::Command;
use std::process::Stdio;
use tokio::io::{BufReader, AsyncBufReadExt};
use crate::parse_monitor::parse_line;
use log;
use serde::Serialize;

pub struct IpMonitorActor {
    config: SentinelConfig,
    sender: broadcast::Sender<AddressChange>,
}

impl IpMonitorActor {
    pub fn new(config: SentinelConfig, sender: broadcast::Sender<AddressChange>) -> Self {
        Self { config, sender }
    }

    pub async fn run(&mut self) {
        log::info!("IpMonitorActor: starting");

        let mut cmd = Command::new("ip")
            .arg("monitor")
            .arg("address")
            .arg("label")
            .arg("dev")
            .arg(self.config.internal_interface.as_str())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to spawn ip monitor");

        let stdout = cmd.stdout.take().expect("child did not have a handle to stdout");
        let mut reader = BufReader::new(stdout).lines();

        loop {
            match reader.next_line().await {
                Ok(Some(line)) => {
                    match parse_line(&line) {
                        Ok(Some(change)) => {
                            if self.is_relevant(&change) {
                                log::info!("IpMonitorActor: detected relevant address change: {:?}", change);
                                if let Err(e) = self.sender.send(change) {
                                    log::error!("IpMonitorActor: failed to send address change: {}", e);
                                }
                            }
                        }
                        Ok(None) => (),
                        Err(e) => log::error!("IpMonitorActor: failed to parse line: {}", e),
                    }
                }
                Ok(None) => {
                    log::info!("IpMonitorActor: ip monitor stdout closed");
                    break;
                }
                Err(e) => {
                    log::error!("IpMonitorActor: failed to read line from ip monitor: {}", e);
                    break;
                }
            }
        }
    }

    fn is_relevant(&self, change: &AddressChange) -> bool {
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
}

pub struct GandiUpdaterActor {
    config: SentinelConfig,
    receiver: broadcast::Receiver<AddressChange>,
}

#[derive(Serialize)]
struct GandiApiRequest<'a> {
    rrset_values: [&'a str; 1],
    rrset_ttl: u32,
}

impl GandiUpdaterActor {
    pub fn new(config: SentinelConfig, receiver: broadcast::Receiver<AddressChange>) -> Self {
        Self { config, receiver }
    }

    pub async fn run(&mut self) {
        log::info!("GandiUpdaterActor: starting");
        loop {
            match self.receiver.recv().await {
                Ok(AddressChange::AdditionV6(address)) => {
                    if let Err(e) = self.update_gandi_record(&address).await {
                        log::error!("GandiUpdaterActor: failed to update gandi record: {}", e);
                    }
                }
                Ok(_) => {
                    // Not interested in other address changes
                }
                Err(e) => {
                    log::error!("GandiUpdaterActor: error receiving message: {}", e);
                    break;
                }
            }
        }
    }

    async fn update_gandi_record(&self, change: &IpV6Address) -> Result<(), reqwest::Error> {
        let own_hostname = self.config.own_hostname.as_str();
        let domain = self.config.domain.as_str();
        let bearer_token = self.config.bearer_token.as_str();
        let own_ipv6_address = change.address.as_str();

        log::info!("GandiUpdaterActor: Updating hostname {own_hostname} with IPv6 address {own_ipv6_address}.");

        let url = format!("https://api.gandi.net/v5/livedns/domains/{domain}/records/{own_hostname}/AAAA");
        let authorization_header = format!("Bearer {bearer_token}");

        let request_body = GandiApiRequest {
            rrset_values: [own_ipv6_address],
            rrset_ttl: 300,
        };

        let client = reqwest::Client::new();
        let response = client.put(&url)
            .header("authorization", &authorization_header)
            .header("content-type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        log::info!("GandiUpdaterActor: update dyndns response succeeded with response: {:?}", response.text().await?);

        Ok(())
    }
}

pub struct WireguardUpdaterActor {
    config: SentinelConfig,
    receiver: broadcast::Receiver<AddressChange>,
}

impl WireguardUpdaterActor {
    pub fn new(config: SentinelConfig, receiver: broadcast::Receiver<AddressChange>) -> Self {
        Self { config, receiver }
    }

    pub async fn run(&mut self) {
        log::info!("WireguardUpdaterActor: starting");
        loop {
            match self.receiver.recv().await {
                Ok(AddressChange::AdditionV6(_)) => {
                    if let Err(e) = self.update_wireguard_peer().await {
                        log::error!("WireguardUpdaterActor: failed to update wireguard peer: {}", e);
                    }
                }
                Ok(_) => {
                    // Not interested in other address changes
                }
                Err(e) => {
                    log::error!("WireguardUpdaterActor: error receiving message: {}", e);
                    break;
                }
            }
        }
    }

    async fn update_wireguard_peer(&self) -> Result<(), std::io::Error> {
        let wg_interface = self.config.wg_interface.as_str();
        let peer_pubkey = self.config.peer_pubkey.as_str();
        let peer_hostname = self.config.peer_hostname.as_str();
        let peer_endpoint = format!("{peer_hostname}:51820");

        log::info!("WireguardUpdaterActor: Updating wg endpoint for if {wg_interface} to {peer_pubkey} with endpoint {peer_hostname}");

        let mut cmd = Command::new("wg")
            .arg("set")
            .arg(wg_interface)
            .arg("peer")
            .arg(peer_pubkey)
            .arg("endpoint")
            .arg(peer_endpoint)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        let status = cmd.wait().await?;

        if status.success() {
            log::info!("WireguardUpdaterActor: wg endpoint update succeeded for {wg_interface}");
        } else {
            log::error!("WireguardUpdaterActor: wg endpoint update failed for {wg_interface}");
        }

        Ok(())
    }
}
