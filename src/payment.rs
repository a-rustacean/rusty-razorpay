#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, format, string::String};

use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Utc,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    api::RequestParams,
    card::{Card, CardType},
    common::{Collection, Currency, Filter, Object},
    error::{InternalApiResult, RazorpayResult},
    ids::{CardId, DowntimeId, OrderId, PaymentId, RefundId},
    offer::Offer,
    refund::{CreateRefund, Refund},
    util::deserialize_notes,
    Razorpay,
};

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    Created,
    Authorized,
    Captured,
    Refunded,
    Failed,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentMethod {
    Card,
    NetBanking,
    Wallet,
    Emi,
    Upi,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentRefundStatus {
    Partial,
    Full,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct PaymentAcquirerData {
    pub rrn: String,
    pub authentication_reference_number: Option<String>,
    pub bank_transaction_id: Option<String>,
    pub auth_code: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentAccountType {
    BankAccount,
    CreditCard,
    Wallet,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentUpiFlow {
    InApp,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct PaymentUpiInfo {
    pub payer_account_type: PaymentAccountType,
    pub vpa: String,
    pub flow: Option<PaymentUpiFlow>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct PaymentEmiInfo {
    pub issuer: String,
    pub rate: u16,
    pub duration: u64,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(tag = "entity", rename = "payment")]
pub struct Payment {
    pub id: PaymentId,
    pub amount: u64,
    pub currency: Currency,
    pub status: PaymentStatus,
    pub method: PaymentMethod,
    pub order_id: OrderId,
    pub description: Option<String>,
    pub international: bool,
    pub refund_status: Option<PaymentRefundStatus>,
    pub amount_refunded: u64,
    pub captured: bool,
    pub email: String,
    pub contact: Option<String>,
    pub fee: u64,
    pub tax: u64,
    #[serde(deserialize_with = "deserialize_notes")]
    pub notes: Object,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub card_id: Option<CardId>,
    pub card: Option<Card>,
    pub wallet: Option<String>,
    pub acquirer_data: Option<PaymentAcquirerData>,
    pub bank: Option<String>,
    pub upi: Option<PaymentUpiInfo>,
    pub vpa: Option<String>,
    pub emi: Option<PaymentEmiInfo>,
    pub offers: Option<Collection<Offer>>,
    pub error_code: Option<String>,
    pub error_description: Option<String>,
    pub error_source: Option<String>,
    pub error_reason: Option<String>,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct CapturePayment {
    pub amount: u64,
    pub currency: Currency,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentExpand {
    Card,
    Emi,
    Offers,
    Upi,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListPaymentsExpand {
    Card,
    Emi,
}

#[derive(Debug, Default, Serialize, Clone, Eq, PartialEq)]
pub struct ListPayments<'a> {
    #[serde(rename = "expand[]")]
    pub expand: &'a [ListPaymentsExpand],
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DowntimeMethod {
    Card,
    Upi,
    Netbanking,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DowntimeStatus {
    Scheduled,
    Started,
    Resolved,
    Cancelled,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DowntimeSeverity {
    High,
    Medium,
    Low,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub enum DowntimeInstrumentBank {
    HDFC,
    ICIC,
    SBIN,
    KKBK,
    UTIB,
    PUNB,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub enum DowntimeInstrumentNetwork {
    AMEX,
    DICL,
    MC,
    RUPAY,
    VISA,
    ALL,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub enum DowntimeInstrumentIssuer {
    SBIN,
    HDFC,
    ICIC,
    UTIB,
    CITI,
    PUNB,
    KKBK,
    CNRB,
    BKID,
    BARB,
    JAKA,
    UBIN,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DowntimeInstrumentPsp {
    GooglePay,
    Phonepe,
    Paytm,
    Bhim,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct DowntimeInstruments {
    pub bank: Option<DowntimeInstrumentBank>,
    pub network: Option<DowntimeInstrumentNetwork>,
    pub issuer: Option<DowntimeInstrumentIssuer>,
    pub psp: Option<DowntimeInstrumentPsp>,
    pub vpa_handle: Option<String>,
    pub card_type: Option<CardType>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DowntimeFlow {
    Collect,
    Intent,
    InApp,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(tag = "entity", rename = "payment.downtime")]
pub struct Downtime {
    pub id: DowntimeId,
    pub method: DowntimeMethod,
    #[serde(with = "ts_seconds")]
    pub begin: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub end: Option<DateTime<Utc>>,
    pub status: DowntimeStatus,
    pub scheduled: bool,
    pub severity: DowntimeSeverity,
    pub instrument: DowntimeInstruments,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
    pub flow: DowntimeFlow,
}

impl Payment {
    pub async fn capture(
        razorpay: &Razorpay,
        payment_id: &PaymentId,
        params: CapturePayment,
    ) -> RazorpayResult<Payment> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: format!("/payments/{}/capture", payment_id),
                version: None,
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(payment) => Ok(payment),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch(
        razorpay: &Razorpay,
        payment_id: &PaymentId,
        expand: &[PaymentExpand],
    ) -> RazorpayResult<Payment> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/payments/{}", payment_id),
                version: None,
                data: Some(json!({
                    "expand[]": expand,
                })),
            })
            .await?;

        match res {
            InternalApiResult::Ok(payment) => Ok(payment),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn list<T>(
        razorpay: &Razorpay,
        params: T,
    ) -> RazorpayResult<Collection<Payment>>
    where
        T: for<'a> Into<Option<ListPayments<'a>>>,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/payments".to_owned(),
                version: None,
                data: params.into(),
            })
            .await?;

        match res {
            InternalApiResult::Ok(payments) => Ok(payments),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch_card(
        razorpay: &Razorpay,
        payment_id: &PaymentId,
    ) -> RazorpayResult<Card> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/payments/{}/card", payment_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(card) => Ok(card),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn update(
        razorpay: &Razorpay,
        payment_id: &PaymentId,
        notes: Object,
    ) -> RazorpayResult<Payment> {
        let res = razorpay
            .api
            .patch(RequestParams {
                url: format!("/payments/{}", payment_id),
                version: None,
                data: Some(json!({
                    "notes": notes,
                })),
            })
            .await?;

        match res {
            InternalApiResult::Ok(payment) => Ok(payment),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn refund(
        razorpay: &Razorpay,
        payment_id: &PaymentId,
        params: CreateRefund<'_>,
    ) -> RazorpayResult<Refund> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: format!("/payments/{}/refund", payment_id),
                version: None,
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(refund) => Ok(refund),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn list_refunds<T>(
        razorpay: &Razorpay,
        payment_id: &PaymentId,
        params: T,
    ) -> RazorpayResult<Collection<Refund>>
    where
        T: Into<Option<Filter>>,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/payments/{}/refunds", payment_id),
                version: None,
                data: params.into(),
            })
            .await?;

        match res {
            InternalApiResult::Ok(refunds) => Ok(refunds),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch_refund(
        razorpay: &Razorpay,
        payment_id: &PaymentId,
        refund_id: &RefundId,
    ) -> RazorpayResult<Refund> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/payments/{}/refunds{}", payment_id, refund_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(refund) => Ok(refund),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }
}

impl Downtime {
    pub async fn list(
        razorpay: &Razorpay,
    ) -> RazorpayResult<Collection<Downtime>> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/payments/downtimes".to_owned(),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(downtimes) => Ok(downtimes),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch(
        razorpay: &Razorpay,
        downtime_id: &DowntimeId,
    ) -> RazorpayResult<Downtime> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/payments/downtimes/{}", downtime_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(downtime) => Ok(downtime),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }
}
