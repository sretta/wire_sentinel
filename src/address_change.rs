#[derive(Clone, Debug)]
pub enum AddressScope {
    Link,
    Global,
}

#[derive(Clone, Debug)]
pub struct IpV4Address {
    pub address: String,
    pub scope: AddressScope,
}

#[derive(Clone, Debug)]
pub struct IpV6Address {
    pub address: String,
    pub scope: AddressScope,
}


#[derive(Clone, Debug)]
pub enum AddressChange {
    AdditionV4(IpV4Address),
    AdditionV6(IpV6Address),
    DeletionV4(IpV4Address),
    DeletionV6(IpV6Address),
}
