use anyhow::Result;
use crate::domain::address::entity::address_entity::{
    Address,
    FrenchAddress,
    Iso20022Address
};

pub fn french_to_internal(french: FrenchAddress) -> Result<Address> {
        let parts: Vec<&str> = french.postal_code_city.split_whitespace().collect();
        let postal_code = parts[0].to_string();
        let city = parts[1..].join(" ");

        Ok(Address {
            id: french.id,
            street: french.addr1,
            house_number: None,
            postal_code,
            city,
            country: french.country,
        })
    }

pub fn internal_to_french(address: &Address) -> FrenchAddress {
        FrenchAddress {
            id: address.id.clone(),
            addr1: address.street.clone(),
            addr2: address.house_number.clone().unwrap_or_default(),
            postal_code_city: format!("{} {}", address.postal_code, address.city),
            country: address.country.clone(),
        }
    }

pub fn iso20022_to_internal(iso: Iso20022Address) -> Result<Address> {
        Ok(Address {
            id: iso.id,
            street: iso.street_name,
            house_number: iso.building_number,
            postal_code: iso.post_code,
            city: iso.town_name,
            country: iso.country,
        })
    }

pub fn internal_to_iso20022(address: &Address) -> Iso20022Address {
        Iso20022Address {
            id: address.id.clone(),
            street_name: address.street.clone(),
            building_number: address.house_number.clone(),
            post_code: address.postal_code.clone(),
            town_name: address.city.clone(),
            country: address.country.clone(),
        }
    }
