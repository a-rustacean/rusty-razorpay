use crate::{
    api::RequestParams,
    common::{Collection, Currency},
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
#[serde(rename_all = "snake_case")]
pub enum DisputeStatus {
    Open,
    UnderReview,
    Won,
    Lost,
    Closed,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisputePhase {
    Fraud,
    Retrieval,
    Chargeback,
    PreArbitration,
    Arbitration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherDisputeEvidence {
    pub r#type: String,
    pub document_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DisputeEvidence {
    pub amount: u64,
    pub summary: String,
    pub shipping_proof: Option<Vec<String>>,
    pub billing_proof: Option<Vec<String>>,
    pub cancellation_proof: Option<Vec<String>>,
    pub cutomer_communication: Option<Vec<String>>,
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

#[derive(Debug, Deserialize)]
pub struct Dispute {
    pub id: String,
    pub entity: String,
    pub payment_id: String,
    pub amount: u64,
    pub currency: Currency,
    pub amount_deducted: u64,
    pub reason_code: String,
    pub reason_description: String,
    #[serde(with = "ts_seconds")]
    pub resopond_by: DateTime<Utc>,
    pub status: DisputeStatus,
    pub phase: DisputePhase,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub evidence: DisputeEvidence,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ContestDisputeAction {
    Draft,
    #[default]
    Submit,
}

#[derive(Debug, Default, Serialize)]
pub struct ContestDisputeOptions {
    pub amount: u64,
    pub summary: String,
    pub shipping_proof: Option<Vec<String>>,
    pub billing_proof: Option<Vec<String>>,
    pub cancellation_proof: Option<Vec<String>>,
    pub cutomer_communication: Option<Vec<String>>,
    pub proof_of_service: Option<Vec<String>>,
    pub explanation_letter: Option<Vec<String>>,
    pub refund_confirmation: Option<Vec<String>>,
    pub access_activity_log: Option<Vec<String>>,
    pub refund_cancellation_policy: Option<Vec<String>>,
    pub term_and_conditions: Option<Vec<String>>,
    pub others: Option<Vec<OtherDisputeEvidence>>,
    pub action: ContestDisputeAction,
}

impl Dispute {
    pub async fn all(
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

    pub async fn fetch<T>(
        razorpay: &Razorpay,
        dispute_id: T,
    ) -> RazorpayResult<Dispute>
    where
        T: Display,
    {
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

    pub async fn accept<T>(
        razorpay: &Razorpay,
        dispute_id: T,
    ) -> RazorpayResult<Dispute>
    where
        T: Display,
    {
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

    pub async fn contest<T>(
        razorpay: &Razorpay,
        dispute_id: T,
        data: ContestDisputeOptions,
    ) -> RazorpayResult<Dispute>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .patch(RequestParams {
                url: format!("/disputes/{}/contest", dispute_id),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(dispute) => Ok(dispute),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }
}
