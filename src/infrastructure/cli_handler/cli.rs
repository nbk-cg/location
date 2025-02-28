use clap::{Parser, Subcommand};
use crate::domain::address::service::address_service::{AddressService};
use crate::domain::address::entity::address_entity::{FrenchAddress, Iso20022Address};
use std::sync::Arc;
use anyhow::Result;
use crate::infrastructure::storage::file::main::FileAddressRepository;

#[derive(Parser)]
#[command(name = "address")]
#[command(about = "Convert postal address formats", author = "Sileymane Djimera")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

const DB_FILE: &str = "addresses.json";
#[derive(Subcommand)]
enum Commands {
    /// To add a French Address Format #### example: cargo run -- store 1 "30 Rue Des Mesanges" "" "78300 Poissy" "France"
    /// example: cargo run -- <command> <ID> "<addr1>" "<addr2>" "<zipcode city>" "<country>"
    Store {
        id: String,
        addr1: String,
        addr2: String,
        postal_code_city: String,
        country: String,
    },

    /// To add an Iso Address Format example: cargo run -- store-iso 1 "30 Rue Des Mesanges" "" "78300 Poissy" "France"
    StoreIso {
        id: String,
        street_name: String,
        building_number: Option<String>,
        post_code: String,
        town_name: String,
        country: String,
    },

    /// Fetch French Address Format by ID ####  example: cargo run -- fetch 1
    /// Example: cargo run -- <COMMAND> <ID> => cargo run -- fetch 1
    Fetch { id: String },

    /// Fetch Iso 20022 Address Format by ID #### example: cargo run -- fetch-iso 2
    FetchIso { id: String },

    /// Delete any Address Format by ID #### example: cargo run -- delete 2
    Delete { id: String },

    /// Fetch all Addresses ####  example: cargo run -- fetch-all
    FetchAll {},
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let repository = Arc::new(FileAddressRepository::new(DB_FILE));
        let service = AddressService::new(repository);

        match &self.command {
            Commands::Store {
                id,
                addr1,
                addr2,
                postal_code_city,
                country
            } => {
                let french = FrenchAddress {
                    id: id.clone(),
                    addr1: addr1.clone(),
                    addr2: addr2.clone(),
                    postal_code_city: postal_code_city.clone(),
                    country: country.clone(),
                };
                service.address_from_french(french)?;
                println!("French address saved successfully");
            }
            Commands::StoreIso {
                id,
                street_name,
                building_number,
                post_code,
                town_name,
                country
            } => {
                let iso = Iso20022Address {
                    id: id.clone(),
                    street_name: street_name.clone(),
                    building_number: building_number.clone(),
                    post_code: post_code.clone(),
                    town_name: town_name.clone(),
                    country: country.clone(),
                };
                service.address_from_iso20022(iso)?;
                println!("ISO address saved successfully");
            }
            Commands::Fetch { id } => {
                if let Some(french) = service.to_french(id)? {
                    println!("French format: {:?}", french);
                } else {
                    println!("Address not found");
                }
            }
            Commands::FetchIso { id } => {
                if let Some(iso) = service.to_iso20022(id)? {
                    println!("ISO format: {:?}", iso);
                } else {
                    println!("Address not found");
                }
            }
            Commands::Delete { id } => {
                service.delete(id)?;
                println!("Address deleted successfully");
            }
            Commands::FetchAll {} => {
                println!("Fetch all addresses");
                let addresses = service.fetch_all()?;
                println!("All addresses: {:?}", addresses);
            }
        }
        Ok(())
    }
}