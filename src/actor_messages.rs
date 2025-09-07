use crate::address_change::AddressChange;

#[derive(Debug, Clone)]
pub enum ActorMessage {
    AddressChanged(AddressChange),
}
