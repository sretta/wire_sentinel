#[derive(Debug, Clone, PartialEq)]
pub enum AddressScope {
    Global,
    Link,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IpV4Address {
    pub address: String,
    pub scope: AddressScope,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IpV6Address {
    pub address: String,
    pub scope: AddressScope,
}


#[derive(Debug, Clone)]
pub enum AddressChange {
    AdditionV4(IpV4Address),
    AdditionV6(IpV6Address),
    DeletionV4(IpV4Address),
    DeletionV6(IpV6Address),
}
