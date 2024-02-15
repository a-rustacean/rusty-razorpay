use crate::{
    api::RequestParams,
    card::{CardNetwork, CardType},
    common::{Collection, Currency, Filter, Object},
    error::{InternalApiResult, RazorpayResult},
    ids::{InstantSettlementId, InstantSettlementPayoutId, SettlementId},
    payment::PaymentMethod,
    util::deserialize_notes,
    AdjustmentId, DisputeId, OrderId, PaymentId, Razorpay, RefundId,
    TransferId,
};
#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, format, string::String};
use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Utc,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SettlementStatus {
    Created,
    Processed,
    Failed,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "entity", rename = "settlement")]
pub struct Settlement {
    pub id: SettlementId,
    pub amount: u64,
    pub status: SettlementStatus,
    pub fees: u64,
    pub tax: u64,
    pub utr: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstantSettlementStatus {
    Created,
    Initiated,
    PartiallyProcessed,
    Processed,
    Reversed,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InstantSettlementPayoutStatus {
    Created,
    Initiated,
    Processed,
    Reversed,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "entity", rename = "settlement.ondemand_payout")]
pub struct InstantSettlementPayout {
    pub id: InstantSettlementPayoutId,
    #[serde(with = "ts_seconds")]
    pub initiated_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub processed_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub reversed_at: Option<DateTime<Utc>>,
    pub amount: u64,
    pub amount_settled: u64,
    pub fees: u64,
    pub tax: u64,
    pub utr: String,
    pub status: InstantSettlementPayoutStatus,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "entity", rename = "settlement.ondemand")]
pub struct InstantSettlement {
    pub id: InstantSettlementId,
    pub amount_requested: u64,
    pub amount_settled: u64,
    pub amount_pending: u64,
    pub amount_reversed: u64,
    pub fees: u64,
    pub tax: u64,
    pub currency: Currency,
    pub settle_full_balance: bool,
    pub status: InstantSettlementStatus,
    pub description: Option<String>,
    #[serde(deserialize_with = "deserialize_notes")]
    pub notes: Object,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub ondemand_payouts: Option<Collection<InstantSettlementPayout>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SettlementType {
    Payment,
    Refund,
    Transfer,
    Adjustment,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum SettlementReconEntityId {
    Payment(PaymentId),
    Refund(RefundId),
    Transfer(TransferId),
    Adjustment(AdjustmentId),
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct SettlementRecon {
    pub entity_id: SettlementReconEntityId,
    #[serde(rename = "type")]
    pub type_: SettlementType,
    pub debit: u64,
    pub credit: u64,
    pub amount: u64,
    pub currency: Currency,
    pub fee: u64,
    pub tax: u64,
    pub on_hold: bool,
    pub settled: bool,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub settled_at: Option<DateTime<Utc>>,
    pub settlement_id: Option<SettlementId>,
    pub description: Option<String>,
    #[serde(deserialize_with = "deserialize_notes")]
    pub notes: Object,
    //  ^^^^^
    //    |
    //     "--------------------------------------------------.
    //                                                         |
    // TODO: look at it later                                  |
    //                                                         |
    // I don't know TF is wrong with the [docs], but it says   |
    // the type of `notes` should be `object`, that's okay,    |
    //                |                                        |
    //                 "--------------------------------------"
    // but every single example payload have a string in
    // the place instead, haven't tested it yet
    //
    // [docs]: https://razorpay.com/docs/api/settlements/fetch-recon/
    pub payment_id: Option<PaymentId>,
    pub settlement_utr: Option<String>,
    pub order_id: Option<OrderId>,
    pub order_receipt: Option<String>,
    pub method: Option<PaymentMethod>,
    pub card_network: Option<CardNetwork>,
    pub card_issuer: Option<String>,
    pub card_type: Option<CardType>,
    pub dispute_id: Option<DisputeId>,
}

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct FetchRecon {
    pub year: u16,
    pub month: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<u64>,
}

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct CreateInstantSettlement<'a> {
    pub amount: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_full_balance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Object>,
}

impl Settlement {
    pub async fn list<T>(
        razorpay: &Razorpay,
        params: T,
    ) -> RazorpayResult<Collection<Settlement>>
    where
        T: Into<Option<Filter>>,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/settlements".to_owned(),
                version: None,
                data: params.into(),
            })
            .await?;

        match res {
            InternalApiResult::Ok(settlements) => Ok(settlements),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch(
        razorpay: &Razorpay,
        settlement_id: &SettlementId,
    ) -> RazorpayResult<Settlement> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/settlements/{}", settlement_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(settlement) => Ok(settlement),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }
}

impl SettlementRecon {
    pub async fn fetch(
        razorpay: &Razorpay,
        params: FetchRecon,
    ) -> RazorpayResult<SettlementRecon> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/settlements/recon/combined".to_owned(),
                version: None,
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(settlement) => Ok(settlement),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }
}

impl InstantSettlement {
    pub async fn create(
        razorpay: &Razorpay,
        params: CreateInstantSettlement<'_>,
    ) -> RazorpayResult<InstantSettlement> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: "/settlements/ondemand".to_owned(),
                version: None,
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(settlement) => Ok(settlement),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn list(
        razorpay: &Razorpay,
        expand_payout: bool,
    ) -> RazorpayResult<Collection<InstantSettlement>> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/settlements/ondemand".to_owned(),
                version: None,
                data: expand_payout.then_some(json!({
                    "expand[]": "ondemand_payouts",
                })),
            })
            .await?;

        match res {
            InternalApiResult::Ok(settlements) => Ok(settlements),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch(
        razorpay: &Razorpay,
        instant_settlement_id: &InstantSettlementId,
        expand_payout: bool,
    ) -> RazorpayResult<InstantSettlement> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/settlements/ondemand/{}", instant_settlement_id),
                version: None,
                data: expand_payout.then_some(json!({
                    "expand[]": "ondemand_payouts",
                })),
            })
            .await?;

        match res {
            InternalApiResult::Ok(settlement) => Ok(settlement),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }
}
