use crate::domain::address::entity::address_entity::{Address};
use anyhow::Result;

pub trait AddressRepository: Send + Sync {
    fn store(&self, address: Address) -> Result<()>;
    fn fetch(&self, id: &str) -> Result<Option<Address>>;
    fn delete(&self, id: &str) -> Result<()>;
    fn fetch_all(&self) -> Result<Vec<Address>>;
}