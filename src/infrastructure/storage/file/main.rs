use crate::domain::address::entity::address_entity::{Address};
use crate::domain::address::repository::address_repository::AddressRepository;
use serde_json;
use std::fs::File;
use std::path::Path;
use anyhow::{Result, Context};
use std::collections::HashMap;

pub struct FileAddressRepository {
    file_path: String,
}

impl FileAddressRepository {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }

    fn load_addresses(&self) -> Result<HashMap<String, Address>> {
        if Path::new(&self.file_path).exists() {
            let file = File::open(&self.file_path)?;
            serde_json::from_reader(file).context("Failed to parse JSON file")
        } else {
            Ok(HashMap::new())
        }
    }

    fn save_addresses(&self, addresses: &HashMap<String, Address>) -> Result<()> {
        let file = File::create(&self.file_path)?;
        serde_json::to_writer_pretty(file, addresses)?;
        Ok(())
    }
}

impl AddressRepository for FileAddressRepository {
    fn store(&self, address: Address) -> Result<()> {
        let mut addresses = self.load_addresses()?;
        addresses.insert(address.id.clone(), address);
        self.save_addresses(&addresses)
    }

    fn fetch(&self, id: &str) -> Result<Option<Address>> {
        let addresses = self.load_addresses()?;
        Ok(addresses.get(id).cloned())
    }

    fn delete(&self, id: &str) -> Result<()> {
        let mut addresses = self.load_addresses()?;
        addresses.remove(id);
        self.save_addresses(&addresses)
    }

     fn fetch_all(&self) -> Result<Vec<Address>> {
        let addresses = self.load_addresses()?;
        Ok(addresses.into_values().collect())
    }
}
