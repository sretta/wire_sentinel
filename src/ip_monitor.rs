use log;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{ChildStdout, Command, Stdio};
use crate::config::SentinelConfig;
use crate::parse_monitor::parse_line;
use crate::address_change::AddressChange;

pub struct IpMonitorContext {
    pub reader: Box<BufReader<ChildStdout>>,
}

impl IpMonitorContext {
    pub fn listen_for_addr_changes(&mut self) -> Result<AddressChange, Error> {
        loop {
            log::trace!("IpMonitorContext::listen_for_addr_changes(): Entering main loop.");

            let reader = self.reader.as_mut();

            let mut buf = String::new();
            let status = reader.read_line(&mut buf)?;

            if status != 0 {
                match parse_line(buf.as_str())? {
                    Some(x) => return Ok(x),
                    None => {}
                }
            }
        }
    }

    pub fn initialize(config: &SentinelConfig) -> Result<IpMonitorContext, Error> {
        log::trace!("IpMonitorContext::initialize(): starting ip monitor command.");

        let stdout = Command::new("ip")
            .arg("monitor")
            .arg("address")
            .arg("label")
            .arg("dev")
            .arg(config.internal_interface.as_str())
            .stdout(Stdio::piped())
            .spawn()?
            .stdout
            .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

        let reader: BufReader<ChildStdout> = BufReader::new(stdout);

        Ok(IpMonitorContext { reader: Box::new(reader) })
    }
}
