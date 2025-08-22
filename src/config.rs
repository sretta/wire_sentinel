use std::fs;
use std::io::Error;
use serde_derive::Deserialize;
use log;

#[derive(Deserialize, Clone)]
struct Data {
    config: SentinelConfig,
}

#[derive(Deserialize, Clone)]
pub struct SentinelConfig {
    pub internal_interface: String,
    // TODO listen to three interfaces
    // pub public_interface: String,
    pub wg_interface: String,
    pub bearer_token: String,
    pub domain: String,
    pub peer_pubkey: String,
    pub peer_hostname: String,
    pub own_hostname: String,
}

pub fn load_config() -> Result<SentinelConfig, Error> {
    let filename = "wire-sentinel.toml";
    log::trace!("load_config(): Loading config from {filename}.");

    let contents = fs::read_to_string(filename).unwrap();
    let data: Data = toml::from_str(&contents).unwrap();

    Ok(data.config)
}
