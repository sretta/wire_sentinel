use log;
use std::time::Duration;
use ureq::Error;
use ureq::Agent;
use ureq::config::Config;
use ureq::tls::{TlsConfig, TlsProvider};
use crate::config::SentinelConfig;
use crate::address_change::IpV6Address;

pub fn update_record(config: &SentinelConfig, change: &IpV6Address) -> Result<(), Error> {
    let own_hostname = config.own_hostname.as_str();
    let domain = config.domain.as_str();
    let bearer_token = config.bearer_token.as_str();
    let own_ipv6_address = change.address.as_str();

    log::trace!("Updating hostname {own_hostname} with IPv6 address {own_ipv6_address}.");

    let url = format!("https://api.gandi.net/v5/livedns/domains/{domain}/records/{own_hostname}/AAAA");
    let authorization_header = format!("Bearer {bearer_token}");
    let request_body = format!("{{\"rrset_values\":[\"{own_ipv6_address}\"],\"rrset_ttl\":300}}");

    let mut config = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(5)))
        .tls_config(
            TlsConfig::builder()
                .provider(TlsProvider::NativeTls)
                .build()
        )
        .build();

    let agent: Agent = config.into();

    let response_body: String = agent.put(url.as_str())
        .header("authorization", authorization_header.as_str())
        .header("content-type", "application/json")
        .send(request_body.as_str())?
        .body_mut()
        .read_to_string()?;

    log::trace!("update dyndns response succeeded with response: {response_body}.");

    Ok(())
}
