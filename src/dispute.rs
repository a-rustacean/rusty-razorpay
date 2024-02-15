#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, format, string::String, vec::Vec};

use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Utc,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::RequestParams,
    common::{Collection, Currency},
    error::{InternalApiResult, RazorpayResult},
    ids::DisputeId,
    PaymentId, Razorpay,
};

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DisputeStatus {
    Open,
    UnderReview,
    Won,
    Lost,
    Closed,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DisputePhase {
    Fraud,
    Retrieval,
    Chargeback,
    PreArbitration,
    Arbitration,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct OtherDisputeEvidence {
    #[serde(rename = "type")]
    pub type_: String,
    pub document_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct DisputeEvidence {
    pub amount: u64,
    pub summary: String,
    pub shipping_proof: Option<Vec<String>>,
    pub billing_proof: Option<Vec<String>>,
    pub cancellation_proof: Option<Vec<String>>,
    pub customer_communication: Option<Vec<String>>,
    pub proof_of_service: Option<Vec<String>>,
    pub explanation_letter: Option<Vec<String>>,
    pub refund_confirmation: Option<Vec<String>>,
    pub access_activity_log: Option<Vec<String>>,
    pub refund_cancellation_policy: Option<Vec<String>>,
    pub term_and_conditions: Option<Vec<String>>,
    pub others: Option<Vec<OtherDisputeEvidence>>,
    #[serde(with = "ts_seconds_option")]
    pub submitted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "entity", rename = "dispute")]
pub struct Dispute {
    pub id: DisputeId,
    pub payment_id: PaymentId,
    pub amount: u64,
    pub currency: Currency,
    pub amount_deducted: u64,
    pub reason_code: String,
    pub reason_description: String,
    #[serde(with = "ts_seconds")]
    pub respond_by: DateTime<Utc>,
    pub status: DisputeStatus,
    pub phase: DisputePhase,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub evidence: DisputeEvidence,
}

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ContestDisputeAction {
    Draft,
    #[default]
    Submit,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct ContestDisputeOtherEvidence<'a> {
    #[serde(rename = "type")]
    pub type_: &'a str,
    pub document_ids: &'a [&'a str],
}

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct ContestDispute<'a> {
    pub amount: u64,
    pub summary: &'a str,
    pub shipping_proof: Option<&'a [&'a str]>,
    pub billing_proof: Option<&'a [&'a str]>,
    pub cancellation_proof: Option<&'a [&'a str]>,
    pub customer_communication: Option<&'a [&'a str]>,
    pub proof_of_service: Option<&'a [&'a str]>,
    pub explanation_letter: Option<&'a [&'a str]>,
    pub refund_confirmation: Option<&'a [&'a str]>,
    pub access_activity_log: Option<&'a [&'a str]>,
    pub refund_cancellation_policy: Option<&'a [&'a str]>,
    pub term_and_conditions: Option<&'a [&'a str]>,
    pub others: Option<&'a [ContestDisputeOtherEvidence<'a>]>,
    pub action: ContestDisputeAction,
}

impl Dispute {
    pub async fn list(
        razorpay: &Razorpay,
    ) -> RazorpayResult<Collection<Dispute>> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/disputes".to_owned(),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(disputes) => Ok(disputes),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch(
        razorpay: &Razorpay,
        dispute_id: &DisputeId,
    ) -> RazorpayResult<Dispute> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/disputes/{}", dispute_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(dispute) => Ok(dispute),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn accept(
        razorpay: &Razorpay,
        dispute_id: &DisputeId,
    ) -> RazorpayResult<Dispute> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: format!("/disputes/{}/accept", dispute_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(dispute) => Ok(dispute),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn contest(
        razorpay: &Razorpay,
        dispute_id: &DisputeId,
        params: ContestDispute<'_>,
    ) -> RazorpayResult<Dispute> {
        let res = razorpay
            .api
            .patch(RequestParams {
                url: format!("/disputes/{}/contest", dispute_id),
                version: None,
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(dispute) => Ok(dispute),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }
}
