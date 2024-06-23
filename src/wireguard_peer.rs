use std::io::Error;
use std::process::{Command, Stdio};
use log;

use crate::config::SentinelConfig;

pub fn update_peer(config: &SentinelConfig) -> Result<(), Error> {
    let wg_interface = config.wg_interface.as_str();
    let peer_pubkey = config.peer_pubkey.as_str();
    let peer_hostname = config.peer_hostname.as_str();
    let peer_endpoint = format!("{peer_hostname}:51820");

    log::trace!("Updating wg endpoint for if {wg_interface} to {peer_pubkey} with endpoint {peer_hostname}");

    let child = Command::new("wg")
        .arg("set")
        .arg(wg_interface)
        .arg("peer")
        .arg(peer_pubkey)
        .arg("endpoint")
        .arg(peer_endpoint)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        log::trace!("wg endpoint update succeeded for {wg_interface}");
        Ok(())
    } else {
        // charset errors are systematic and need to be fixed on system level
        let err = String::from_utf8(output.stderr).unwrap();
        log::error!("wg endpoint update failed for {wg_interface}: {err}");
        Ok(())
    }
}
