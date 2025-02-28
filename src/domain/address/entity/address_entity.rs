use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub id: String,
    pub street: String,
    pub house_number: Option<String>,
    pub postal_code: String,
    pub city: String,
    pub country: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FrenchAddress {
    pub id: String,
    pub addr1: String,
    pub addr2: String,
    pub postal_code_city: String,
    pub country: String,
}

#[derive(Debug, Clone)]
pub struct Iso20022Address {
    pub id: String,
    pub street_name: String,
    pub building_number: Option<String>,
    pub post_code: String,
    pub town_name: String,
    pub country: String,
}
