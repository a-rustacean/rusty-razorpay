#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, format, string::String};

use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Utc,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    api::RequestParams,
    common::{Collection, Currency, Filter},
    error::{InternalApiResult, RazorpayResult},
    ids::ItemId,
    Razorpay,
};

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Plan,
    Addon,
    Invoice,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub active: bool,
    pub amount: u64,
    pub unit_amount: u64,
    pub currency: Currency,
    pub description: Option<String>,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    pub type_: ItemType,
    pub unit: Option<u64>,
    pub tax_inclusive: Option<bool>,
    pub hsn_code: Option<u32>,
    pub sac_code: Option<u32>,
    pub tax_rate: Option<String>,
    pub tax_id: Option<String>,
    pub tax_group_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct CreateItem<'a> {
    pub name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    pub amount: u64,
    pub currency: Currency,
}

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct UpdateItem<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct ListItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub filter: Option<Filter>,
}

impl Item {
    pub async fn create(
        razorpay: &Razorpay,
        params: CreateItem<'_>,
    ) -> RazorpayResult<Item> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: "/items".to_owned(),
                version: None,
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(item) => Ok(item),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch(
        razorpay: &Razorpay,
        item_id: &ItemId,
    ) -> RazorpayResult<Item> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/items/{}", item_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(item) => Ok(item),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn list<T>(
        razorpay: &Razorpay,
        params: T,
    ) -> RazorpayResult<Collection<Item>>
    where
        T: Into<Option<ListItems>>,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/items".to_owned(),
                version: None,
                data: params.into(),
            })
            .await?;

        match res {
            InternalApiResult::Ok(items) => Ok(items),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn update(
        razorpay: &Razorpay,
        item_id: &ItemId,
        params: UpdateItem<'_>,
    ) -> RazorpayResult<Item> {
        let res = razorpay
            .api
            .patch(RequestParams {
                url: format!("/items/{}", item_id),
                version: None,
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(res) => Ok(res),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn delete(
        razorpay: &Razorpay,
        item_id: &ItemId,
    ) -> RazorpayResult<()> {
        let res: InternalApiResult<Value> = razorpay
            .api
            .delete(RequestParams {
                url: format!("/items/{}", item_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(_) => Ok(()),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }
}
