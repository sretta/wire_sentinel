use log;
use std::io::Error;
use pest::Parser;
use pest_derive::Parser;
use crate::address_change::{AddressChange, AddressScope, IpV4Address, IpV6Address};
use crate::address_change::AddressChange::{AdditionV4, AdditionV6, DeletionV4, DeletionV6};
use crate::address_change::AddressScope::{Global, Link};

#[derive(Parser)]
#[grammar = "ip_monitor.pest"]
pub struct IpMonitorParser;

pub fn parse_line(line: &str) -> Result<Option<AddressChange>, Error> {
    log::trace!("parse_line(): parsing: {}", line.trim());

    let parse_result = IpMonitorParser::parse(Rule::line, line).unwrap();

    let mut is_address_change = false;
    let mut is_addition = true;

    let mut ipv4_addr: Option<String> = None;
    let mut ipv6_addr: Option<String> = None;

    let mut scope: Option<AddressScope> = None;

    for token in parse_result.clone().into_iter() {
        let inner_rules = token.into_inner();

        for inner in inner_rules {
            match inner.as_rule() {
                Rule::deletion => {
                    is_addition = false;
                }
                Rule::ipv4_address => {
                    is_address_change = true;
                    ipv4_addr = Some(inner.as_str().to_string());
                }
                Rule::ipv6_address => {
                    is_address_change = true;
                    ipv6_addr = Some(inner.as_str().to_string());
                }
                Rule::global_scope => {
                    scope = Some(Global)
                }
                Rule::link_scope => {
                    scope = Some(Link)
                }
                _ => {
                    log::error!("ERROR: Unknown Token: {} -> {}", inner.to_string(), inner.as_str());

                    for subtoken in inner.into_inner() {
                        log::error!("ERROR: Unknown SubToken: {} -> {}", subtoken.to_string(), subtoken.as_str());
                    }
                }
            }
        }
    }

    // TODO add a site scope and differentiate by address range

    log::trace!("parse_line(): finished parsing: {} / {:?} / {:?} / {:?}", is_addition, ipv4_addr, ipv6_addr, scope);

    if !is_address_change {
        Ok(None)
    } else {
        Ok(match (is_addition, ipv6_addr, ipv4_addr, scope) {
            (true, Some(address), None, Some(scope)) => {Some(AdditionV6(IpV6Address{address, scope}))}
            (false, Some(address), None, Some(scope)) => {Some(DeletionV6(IpV6Address{address, scope}))}
            (true, None, Some(address), Some(scope)) => {Some(AdditionV4(IpV4Address{address, scope}))}
            (false, None, Some(address), Some(scope)) => {Some(DeletionV4(IpV4Address{address, scope}))}
            _ => {None}
        })
    }
}

#[cfg(test)]
mod tests_ip_monitor_parse_liner {
    use crate::address_change::IpV6Address;
    use crate::address_change::AddressChange::{AdditionV4, AdditionV6, DeletionV4, DeletionV6};
    use super::*;

    #[test]
    fn test_parse_line_lifetime_forever() {
        assert!(matches!(parse_line("   valid_lft forever preferred_lft forever"), Ok(None)));
    }

    #[test]
    fn test_parse_line_lifetime_seconds() {
        assert!(matches!(parse_line("   valid_lft 7201sec preferred_lft 1799sec"), Ok(None)));
    }

    #[test]
    fn test_parse_line_ipv4_addition() {
        let actual = parse_line("[ADDR]3: eno2    inet 192.168.1.2/24 brd 192.168.1.255 scope global noprefixroute eno2");

        assert!(matches!(
            actual,
            Ok(Some(AdditionV4(IpV4Address {
                address: _,
                scope: Global,
            })))
        ));

        match actual.unwrap().unwrap() {
            AdditionV4(address) => {assert_eq!(address.address, "192.168.1.2")}
            AdditionV6(_) => {assert_eq!(true, false)}
            DeletionV4(_) => {assert_eq!(true, false)}
            DeletionV6(_) => {assert_eq!(true, false)}
        };
    }

    #[test]
    fn test_parse_line_ipv4_addition_no_brd() {
        let actual = parse_line("[ADDR]3: eno2    inet 192.168.1.2/24 scope global eno2");

        assert!(matches!(
            actual,
            Ok(Some(AdditionV4(IpV4Address {
                address: _,
                scope: Global,
            })))
        ));

        match actual.unwrap().unwrap() {
            AdditionV4(address) => {assert_eq!(address.address, "192.168.1.2")}
            AdditionV6(_) => {assert_eq!(true, false)}
            DeletionV4(_) => {assert_eq!(true, false)}
            DeletionV6(_) => {assert_eq!(true, false)}
        };
    }

    #[test]
    fn test_parse_line_ipv4_deletion() {
        let actual = parse_line("[ADDR][ADDR]Deleted 3: eno2    inet 192.168.1.2/24 brd 192.168.1.255 scope global noprefixroute eno2");

        assert!(matches!(
            actual,
            Ok(Some(DeletionV4(IpV4Address {
                address: _,
                scope: Global,
            })))
        ));

        match actual.unwrap().unwrap() {
            AdditionV4(_) => {assert_eq!(true, false)}
            AdditionV6(_) => {assert_eq!(true, false)}
            DeletionV4(address) => {assert_eq!(address.address, "192.168.1.2")}
            DeletionV6(_) => {assert_eq!(true, false)}
        };
    }

    #[test]
    fn test_parse_line_ipv6_addition_link() {
        let actual = parse_line("[ADDR]3: eno2    inet6 fe80::1234:5678:9876:5432/64 scope link tentative noprefixroute");
        assert!(matches!(
            actual,
            Ok(Some(AdditionV6(IpV6Address {
                address: _,
                scope: Link,
            })))
        ));

        match actual.unwrap().unwrap() {
            AdditionV4(_) => {assert_eq!(true, false)}
            AdditionV6(address) => {assert_eq!(address.address, "fe80::1234:5678:9876:5432")}
            DeletionV4(_) => {assert_eq!(true, false)}
            DeletionV6(_) => {assert_eq!(true, false)}
        };
    }

    #[test]
    fn test_parse_line_ipv6_deletion_global() {
        let actual = parse_line("[ADDR]Deleted 3: eno2    inet6 2222:ab:ab11:1111:2222:3333:4444:5555/64 scope global dynamic noprefixroute");

        assert!(matches!(
            actual,
            Ok(Some(DeletionV6(IpV6Address {
                address: _,
                scope: Global,
            })))
        ));

        match actual.unwrap().unwrap() {
            AdditionV4(_) => { assert_eq!(true, false) }
            AdditionV6(_) => { assert_eq!(true, false) }
            DeletionV6(address) => { assert_eq!(address.address, "2222:ab:ab11:1111:2222:3333:4444:5555") }
            DeletionV4(_) => { assert_eq!(true, false) }
        }
    }

    #[test]
    fn test_parse_line_ipv6_addition_fd_range() {
        let actual = parse_line("[ADDR]3: eno2    inet6 fd00::abcd:ef12:3456:7890/64 scope global dynamic noprefixroute");

        assert!(matches!(
            actual,
            Ok(Some(AdditionV6(IpV6Address {
                address: _,
                scope: Global,
            })))
        ));

        match actual.unwrap().unwrap() {
            AdditionV4(_) => { assert_eq!(true, false) }
            AdditionV6(address) => { assert_eq!(address.address, "fd00::abcd:ef12:3456:7890") }
            DeletionV4(_) => { assert_eq!(true, false) }
            DeletionV6(_) => { assert_eq!(true, false) }
        }
    }
}
