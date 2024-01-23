use crate::{
    api::RequestParams,
    card::{Card, CardType},
    common::{Collection, Currency, Filter, Object},
    error::{InternalApiResult, RazorpayResult},
    offer::Offer,
    refund::{CreateRefund, Refund},
    util::deserialize_notes,
    Razorpay,
};
use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Utc,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    Created,
    Authorized,
    Captured,
    Refunded,
    Failed,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentMethod {
    Card,
    NetBanking,
    Wallet,
    Emi,
    Upi,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentRefundStatus {
    Partial,
    Full,
}

#[derive(Debug, Deserialize)]
pub struct PaymentAcquirerData {
    pub rrn: String,
    pub authentication_reference_number: Option<String>,
    pub bank_transaction_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentAccountType {
    BankAccount,
    CreditCard,
    Wallet,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentUpiFlow {
    InApp,
}

#[derive(Debug, Deserialize)]
pub struct PaymentUpiInfo {
    pub payer_account_type: PaymentAccountType,
    pub vpa: String,
    pub flow: Option<PaymentUpiFlow>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentEmiInfo {
    pub issuer: String,
    pub rate: u16,
    pub duration: u64,
}

#[derive(Debug, Deserialize)]
pub struct Payment {
    pub id: String,
    pub entity: String,
    pub amount: u64,
    pub currency: Currency,
    pub status: PaymentStatus,
    pub method: PaymentMethod,
    pub order_id: String,
    pub description: Option<String>,
    pub international: bool,
    pub refund_status: Option<PaymentRefundStatus>,
    pub amount_refunded: u64,
    pub captured: bool,
    pub email: String,
    pub contact: String,
    pub fee: u64,
    pub tax: u64,
    #[serde(deserialize_with = "deserialize_notes")]
    pub notes: Object,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub card_id: Option<String>,
    pub card: Option<Card>,
    pub wallet: Option<String>,
    pub acquirer_data: Option<PaymentAcquirerData>,
    pub bank: Option<String>,
    pub upi: Option<PaymentUpiInfo>,
    pub vpa: String,
    pub emi: Option<PaymentEmiInfo>,
    pub offers: Option<Collection<Offer>>,
    pub error_code: Option<String>,
    pub error_description: Option<String>,
    pub error_source: Option<String>,
    pub error_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CapturePayment {
    pub amount: u64,
    pub currency: Currency,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentExpand {
    Card,
    Emi,
    Offers,
    Upi,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AllPaymentsExpand {
    Card,
    Emi,
}

#[derive(Debug, Default, Serialize)]
pub struct AllPayments {
    #[serde(rename = "expand[]")]
    pub expand: Vec<AllPaymentsExpand>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DowntimeMethod {
    Card,
    Upi,
    Netbanking,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DowntimeStatus {
    Scheduled,
    Started,
    Resolved,
    Cancelled,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DowntimeSeverity {
    High,
    Medium,
    Low,
}

#[derive(Debug, Deserialize)]
pub enum DowntimeInstrumentBank {
    HDFC,
    ICIC,
    SBIN,
    KKBK,
    UTIB,
    PUNB,
}

#[derive(Debug, Deserialize)]
pub enum DowntimeInstrumentNetwork {
    AMEX,
    DICL,
    MC,
    RUPAY,
    VISA,
    ALL,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowntimeInstrumentPsp {
    GooglePay,
    Phonepe,
    Paytm,
    Bhim,
}

#[derive(Debug, Deserialize)]
pub struct DowntimeInstruments {
    pub bank: Option<DowntimeInstrumentBank>,
    pub network: Option<DowntimeInstrumentNetwork>,
    pub issuer: Option<DowntimeInstrumentIssuer>,
    pub psp: Option<DowntimeInstrumentPsp>,
    pub vpa_handle: Option<String>,
    pub card_type: Option<CardType>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowntimeFlow {
    Collect,
    Intent,
    InApp,
}

#[derive(Debug, Deserialize)]
pub struct Downtime {
    pub id: String,
    pub entity: String,
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
    pub async fn capture<T>(
        razorpay: &Razorpay,
        payment_id: T,
        data: CapturePayment,
    ) -> RazorpayResult<Payment>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .post(RequestParams {
                url: format!("/payments/{}/capture", payment_id),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(payment) => Ok(payment),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch<T>(
        razorpay: &Razorpay,
        payment_id: T,
        expand: Vec<PaymentExpand>,
    ) -> RazorpayResult<Payment>
    where
        T: Display,
    {
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

    pub async fn all(
        razorpay: &Razorpay,
        data: AllPayments,
    ) -> RazorpayResult<Collection<Payment>> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/payments".to_owned(),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(payments) => Ok(payments),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch_card<T>(
        razorpay: &Razorpay,
        payment_id: T,
    ) -> RazorpayResult<Card>
    where
        T: Display,
    {
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

    pub async fn update<T>(
        razorpay: &Razorpay,
        payment_id: T,
        data: Object,
    ) -> RazorpayResult<Payment>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .patch(RequestParams {
                url: format!("/payments/{}", payment_id),
                version: None,
                data: Some(json!({
                    "notes": data,
                })),
            })
            .await?;

        match res {
            InternalApiResult::Ok(payment) => Ok(payment),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn all_downtimes(
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

    pub async fn fetch_downtime<T>(
        razorpay: &Razorpay,
        downtime_id: T,
    ) -> RazorpayResult<Downtime>
    where
        T: Display,
    {
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

    pub async fn refund<T>(
        razorpay: &Razorpay,
        payment_id: T,
        data: CreateRefund,
    ) -> RazorpayResult<Refund>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .post(RequestParams {
                url: format!("/payments/{}/refund", payment_id),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(refund) => Ok(refund),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn all_refunds<T>(
        razorpay: &Razorpay,
        payment_id: T,
        data: Filter,
    ) -> RazorpayResult<Collection<Refund>>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/payments/{}/refunds", payment_id),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(refunds) => Ok(refunds),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch_refund<T, U>(
        razorpay: &Razorpay,
        payment_id: T,
        refund_id: U,
    ) -> RazorpayResult<Refund>
    where
        T: Display,
        U: Display,
    {
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
