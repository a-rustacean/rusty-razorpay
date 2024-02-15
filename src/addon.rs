#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, format, string::String};

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    api::RequestParams,
    common::{Collection, Filter},
    error::{InternalApiResult, RazorpayResult},
    ids::{AddonId, SubscriptionId},
    item::{CreateItem, Item},
    InvoiceId, Razorpay,
};

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "entity", rename = "addon")]
pub struct Addon {
    pub id: AddonId,
    pub item: Item,
    pub quantity: u64,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub subscription_id: String,
    pub invoice_id: Option<InvoiceId>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct CreateAddon<'a> {
    pub item: CreateItem<'a>,
    pub quantity: u64,
}

impl<'a> Default for CreateAddon<'a> {
    fn default() -> Self {
        Self {
            item: Default::default(),
            quantity: 1,
        }
    }
}

impl Addon {
    pub async fn create(
        razorpay: &Razorpay,
        subscription_id: &SubscriptionId,
        params: CreateAddon<'_>,
    ) -> RazorpayResult<Addon> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: format!("/subscriptions/{}/addons", subscription_id),
                version: None,
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(addon) => Ok(addon),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn list<T>(
        razorpay: &Razorpay,
        params: T,
    ) -> RazorpayResult<Collection<Addon>>
    where
        T: Into<Option<Filter>>,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/addons".to_owned(),
                version: None,
                data: params.into(),
            })
            .await?;

        match res {
            InternalApiResult::Ok(addons) => Ok(addons),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch(
        razorpay: &Razorpay,
        addon_id: &AddonId,
    ) -> RazorpayResult<Addon> {
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

    pub async fn delete(
        razorpay: &Razorpay,
        addon_id: &AddonId,
    ) -> RazorpayResult<()> {
        let res: InternalApiResult<Value> = razorpay
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
