use actix::Message;
use crate::address_change::{AddressChange, IpV6Address};

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct IpChange(pub AddressChange);

#[derive(Message, Debug, Clone)]
#[rtype(result = "Result<(), ()>")]
pub struct UpdateGandi(pub IpV6Address);

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct UpdateWireguard;
