use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum Currency {
    #[default]
    INR,
    USD,
    EUR,
    SGD,
}

pub type Object = HashMap<String, String>;

#[derive(Debug, Deserialize)]
pub struct Collection<T> {
    pub entity: String,
    pub count: usize,
    pub items: Vec<T>,
}

#[derive(Debug, Serialize, Default)]
pub struct FilterOptions {
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_seconds_option"
    )]
    pub from: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_seconds_option"
    )]
    pub to: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<usize>,
}
