#[derive(Debug)]
pub enum AddressScope {
    Link,
    Global,
}

pub struct IpV4Address {
    pub address: String,
    pub scope: AddressScope,
}

pub struct IpV6Address {
    pub address: String,
    pub scope: AddressScope,
}


pub enum AddressChange {
    AdditionV4(IpV4Address),
    AdditionV6(IpV6Address),
    DeletionV4(IpV4Address),
    DeletionV6(IpV6Address),
}
