use crate::common::Currency;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LineItemType {
    Invoice,
}

#[derive(Debug, Deserialize)]
pub struct LineItem {
    pub id: String,
    pub item_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub amount: u64,
    pub currency: Currency,
    pub r#type: LineItemType,
    pub quantity: u64,
}
