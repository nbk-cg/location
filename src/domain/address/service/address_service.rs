use crate::domain::address::repository::address_repository::{AddressRepository};
use crate::domain::address::service::mapper::address_mapper::{
    internal_to_french,
    internal_to_iso20022,
    iso20022_to_internal,
    french_to_internal
};
use crate::domain::address::entity::address_entity::{Address, FrenchAddress, Iso20022Address};
use anyhow::Result;
use std::sync::Arc;

pub struct AddressService {
    repository: Arc<dyn AddressRepository>,
}

impl AddressService {
    pub fn new(repository: Arc<dyn AddressRepository>) -> Self {
        Self { repository }
    }

    pub fn address_from_french(&self, french: FrenchAddress) -> Result<()> {
        let address = french_to_internal(french)?;
        self.repository.store(address)
    }

    pub fn to_french(&self, id: &str) -> Result<Option<FrenchAddress>> {
        let address = self.repository.fetch(id)?;
        Ok(address.map(|a| internal_to_french(&a)))
    }

    pub fn address_from_iso20022(&self, iso: Iso20022Address) -> Result<()> {
        let address = iso20022_to_internal(iso)?;
        self.repository.store(address)
    }

    pub fn to_iso20022(&self, id: &str) -> Result<Option<Iso20022Address>> {
        let address = self.repository.fetch(id)?;
        Ok(address.map(|a| internal_to_iso20022(&a)))
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.repository.delete(id)
    }

    pub fn fetch_all(&self) -> Result<Vec<Address>> {
        self.repository.fetch_all()
    }
}

