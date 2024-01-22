use std::fmt::Display;

use crate::{
    api::RequestParams,
    common::{Collection, Currency, FilterOptions, Object},
    error::{InternalApiResult, RazorpayResult},
    util::deserialize_notes,
    Razorpay,
};
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RefundSpeed {
    #[default]
    Normal,
    Optimum,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RefundStatus {
    Pending,
    Processed,
    Failed,
}

#[derive(Debug, Deserialize)]
pub struct Refund {
    pub id: String,
    pub entity: String,
    pub amount: u64,
    pub currency: Currency,
    pub payment_id: String,
    pub speed: RefundSpeed,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub batch_id: Option<String>,
    #[serde(deserialize_with = "deserialize_notes")]
    pub notes: Object,
    pub receipt: Option<String>,
    // TODO: look at it later
    //
    // The [docs] aren't clear, about the schema of `acquirer_data`, so
    // needs some testing is needed before coming up with a robust type
    //
    // [docs]: https://razorpay.com/docs/api/refunds/
    pub acquirer_data: Value,
    pub status: RefundStatus,
    pub speed_requested: Option<RefundSpeed>,
    pub speed_processed: Option<RefundSpeed>,
}

#[derive(Debug, Serialize, Default)]
pub struct CreateRefundOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<RefundSpeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<String>,
}

impl Refund {
    pub async fn all(
        razorpay: &Razorpay,
        data: FilterOptions,
    ) -> RazorpayResult<Collection<Refund>> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/refunds".to_owned(),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(refunds) => Ok(refunds),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch<T>(
        razorpay: &Razorpay,
        refund_id: T,
    ) -> RazorpayResult<Refund>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/refunds/{}", refund_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(refund) => Ok(refund),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn update<T>(
        razorpay: &Razorpay,
        refund_id: T,
        notes: Object,
    ) -> RazorpayResult<Refund>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .patch(RequestParams {
                url: format!("/refunds/{}", refund_id),
                version: None,
                data: Some(json!({
                    "notes": notes
                })),
            })
            .await?;

        match res {
            InternalApiResult::Ok(refund) => Ok(refund),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }
}