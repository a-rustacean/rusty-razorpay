use crate::subscription::SubscriptionItem;
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Addon {
    pub id: String,
    pub item: SubscriptionItem,
    pub quantity: u64,
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    pub subscription_id: String,
    pub invoice_id: Option<String>,
}
