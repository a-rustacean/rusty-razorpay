use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum AddressType {
    #[serde(rename = "billing_address")]
    Billing,
    #[serde(rename = "shipping_address")]
    Shipping,
}

#[derive(Debug, Deserialize)]
pub struct Address {
    pub id: String,
    pub r#type: AddressType,
    pub primary: bool,
    pub line1: String,
    pub line2: String,
    pub city: String,
    pub zipcode: String,
    pub state: String,
    pub country: String,
}
