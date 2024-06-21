use log;
use std::time::Duration;
use ureq::Error;
use crate::config::SentinelConfig;
use crate::address_change::IpV6Address;

pub fn update_record(config: &SentinelConfig, change: &IpV6Address) -> Result<(), Error> {
    let own_hostname = config.own_hostname.as_str();
    let domain = config.domain.as_str();
    let bearer_token = config.bearer_token.as_str();
    let own_ipv6_address = change.address.as_str();

    log::trace!("Updating hostname {own_hostname} with IPv6 address {own_ipv6_address}.");

    // TODO https://api.gandi.net/v5/livedns/domains/{fqdn}/records/{rrset_name}/{rrset_type}
    let url = format!("https://api.gandi.net/v5/livedns/domains/{domain}/records/{own_hostname}");
    let authorization_header = format!("Bearer {bearer_token}");
    let request_body = format!("{{\"items\":[{{\"rrset_type\":\"AAAA\",\"rrset_values\":[\"{own_ipv6_address}\"]}}]}}");

    // https://api.gandi.net/docs/livedns/#v5-livedns-domains-fqdn-records-rrset_name
    let response_body: String = ureq::put(url.as_str())
        .timeout(Duration::from_secs(5))
        .set("authorization", authorization_header.as_str())
        .set("content-type", "application/json")
        .send_string(request_body.as_str())?
        .into_string()?;

    log::trace!("update dyndns response succeeded with response: {response_body}.");

    Ok(())
}
