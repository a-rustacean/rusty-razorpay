use crate::{
    api::RequestParams,
    common::{Collection, Currency, Filter, Object},
    entity::RefundEntity,
    error::{InternalApiResult, RazorpayResult},
    ids::RefundId,
    util::deserialize_notes,
    BatchId, PaymentId, Razorpay,
};
#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, format, string::String};
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RefundSpeed {
    #[default]
    Normal,
    Optimum,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RefundStatus {
    Pending,
    Processed,
    Failed,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Refund {
    pub id: RefundId,
    pub entity: RefundEntity,
    pub amount: u64,
    pub currency: Currency,
    pub payment_id: PaymentId,
    pub speed: RefundSpeed,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub batch_id: Option<BatchId>,
    #[serde(deserialize_with = "deserialize_notes")]
    pub notes: Object,
    pub receipt: Option<String>,
    pub acquirer_data: Value,
    //  ^^^^^^^^^^^^^
    //        |
    //         "----------------------------------------.
    //                                                   "
    // TODO: look at it later                            |
    //                                                   |
    // The [docs] aren't clear, about the schema of `acquirer_data`, so
    // more testing is needed before coming up with a robust type
    //
    // [docs]: https://razorpay.com/docs/api/refunds/
    pub status: RefundStatus,
    pub speed_requested: Option<RefundSpeed>,
    pub speed_processed: Option<RefundSpeed>,
}

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct CreateRefund<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<RefundSpeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<&'a str>,
}

impl Refund {
    pub async fn list<T>(
        razorpay: &Razorpay,
        params: T,
    ) -> RazorpayResult<Collection<Refund>>
    where
        T: Into<Option<Filter>>,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/refunds".to_owned(),
                version: None,
                data: params.into(),
            })
            .await?;

        match res {
            InternalApiResult::Ok(refunds) => Ok(refunds),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch(
        razorpay: &Razorpay,
        refund_id: &RefundId,
    ) -> RazorpayResult<Refund> {
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

    pub async fn update(
        razorpay: &Razorpay,
        refund_id: &RefundId,
        notes: Object,
    ) -> RazorpayResult<Refund> {
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
