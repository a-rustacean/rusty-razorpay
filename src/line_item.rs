use crate::{common::Currency, LineItemId};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LineItemType {
    Invoice,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct LineItem {
    pub id: LineItemId,
    pub item_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub amount: u64,
    pub currency: Currency,
    #[serde(rename = "type")]
    pub type_: LineItemType,
    pub quantity: u64,
}
