use crate::AddressId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum AddressType {
    #[serde(rename = "billing_address")]
    Billing,
    #[serde(rename = "shipping_address")]
    Shipping,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Address {
    pub id: AddressId,
    #[serde(rename = "type")]
    pub type_: AddressType,
    pub primary: bool,
    pub line1: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    pub city: String,
    pub zipcode: String,
    pub state: String,
    pub country: String,
}
