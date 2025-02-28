#[cfg(test)]
mod tests {
    use crate::domain::address::entity::address_entity::{FrenchAddress};
    use crate::domain::address::service::address_service::{AddressService};
    use crate::infrastructure::storage::file::main::{FileAddressRepository};
    use std::sync::Arc;

    #[test]
    fn test_french_to_internal_and_back() {
        let repo = Arc::new(FileAddressRepository::new("addresses_test.json"));
        let service = AddressService::new(repo);

        let french = FrenchAddress {
            id: "3".to_string(),
            addr1: "30 Rue des Mesanges".to_string(),
            addr2: "".to_string(),
            postal_code_city: "78300 Poissy".to_string(),
            country: "France".to_string(),
        };

        println!("french address: {:?}",french.clone());

        service.address_from_french(french.clone()).unwrap();
        let result = service.to_french("3").unwrap().unwrap();

        assert_eq!(result.addr1, french.addr1);
        assert_eq!(result.postal_code_city, french.postal_code_city);
    }
}
