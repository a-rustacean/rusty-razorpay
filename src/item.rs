use crate::{
    api::RequestParams,
    common::{Collection, Currency, FilterOptions},
    error::{InternalApiResult, RazorpayResult},
    Razorpay,
};
use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Utc,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Plan,
    Addon,
    Invoice,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub id: String,
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
    pub r#type: ItemType,
    pub unit: Option<u64>,
    pub tax_inclusive: Option<bool>,
    pub hsn_code: Option<u32>,
    pub sac_code: Option<u32>,
    pub tax_rate: Option<String>,
    pub tax_id: Option<String>,
    pub tax_group_id: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct CreateItemOptions {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub amount: u64,
    pub currency: Currency,
}

#[derive(Debug, Serialize)]
pub struct UpdateItemOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AllItemsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub filter: Option<FilterOptions>,
}

impl Item {
    pub async fn create(
        razorpay: &Razorpay,
        data: CreateItemOptions,
    ) -> RazorpayResult<Item> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: "/items".to_owned(),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(item) => Ok(item),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch<T>(
        razorpay: &Razorpay,
        item_id: T,
    ) -> RazorpayResult<Item>
    where
        T: Display,
    {
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

    pub async fn all(
        razorpay: &Razorpay,
        data: AllItemsOptions,
    ) -> RazorpayResult<Collection<Item>> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/items".to_owned(),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(items) => Ok(items),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn update<T>(
        razorpay: &Razorpay,
        item_id: T,
        data: UpdateItemOptions,
    ) -> RazorpayResult<Item>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .patch(RequestParams {
                url: format!("/items/{}", item_id),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(res) => Ok(res),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn delete<T>(
        razorpay: &Razorpay,
        item_id: T,
    ) -> RazorpayResult<()>
    where
        T: Display,
    {
        let res: InternalApiResult<[u8; 0]> = razorpay
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
