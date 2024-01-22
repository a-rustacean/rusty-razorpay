use crate::{
    api::RequestParams,
    common::{Collection, FilterOptions},
    error::{InternalApiResult, RazorpayResult},
    item::{CreateItemOptions, Item},
    Razorpay,
};
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Deserialize)]
pub struct Addon {
    pub id: String,
    pub item: Item,
    pub quantity: u64,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub subscription_id: String,
    pub invoice_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateAddonOptions {
    pub item: CreateItemOptions,
    pub quantity: u64,
}

impl Addon {
    pub async fn create<T>(
        razorpay: &Razorpay,
        subscription_id: T,
        data: CreateAddonOptions,
    ) -> RazorpayResult<Addon>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .post(RequestParams {
                url: format!("/subscriptions/{}/addons", subscription_id),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(addon) => Ok(addon),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn all(
        razorpay: &Razorpay,
        data: FilterOptions,
    ) -> RazorpayResult<Collection<Addon>> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/addons".to_owned(),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(addons) => Ok(addons),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch<T>(
        razorpay: &Razorpay,
        addon_id: T,
    ) -> RazorpayResult<Addon>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/addons/{}", addon_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(addon) => Ok(addon),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn delete<T>(
        razorpay: &Razorpay,
        addon_id: T,
    ) -> RazorpayResult<()>
    where
        T: Display,
    {
        let res: InternalApiResult<[u8; 0]> = razorpay
            .api
            .delete(RequestParams {
                url: format!("/addons/{}", addon_id),
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
