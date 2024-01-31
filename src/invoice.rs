use crate::{
    address::Address,
    api::RequestParams,
    common::{Currency, Object},
    entity::InvoiceEntity,
    error::{InternalApiResult, RazorpayResult},
    ids::CustomerId,
    line_item::LineItem,
    util::{deserialize_notes, serialize_bool_as_int_option},
    Collection, InvoiceId, OrderId, PaymentId, Razorpay,
};
#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, format, string::String, vec::Vec};
use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Utc,
};
#[cfg(not(feature = "std"))]
use core::fmt::{Display, Formatter, Result as FormatterResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "std")]
use std::fmt::{Display, Formatter, Result as FormatterResult};

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InvoiceType {
    Invoice,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct CustomerDetails {
    pub id: String,
    pub name: String,
    pub email: String,
    pub contact: String,
    pub billing_address: Option<Address>,
    pub shipping_address: Option<Address>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Issued,
    PartiallyPaid,
    Paid,
    Cancelled,
    Expired,
    Deleted,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InvoiceMessageStatus {
    Pending,
    Sent,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct Invoice {
    pub id: InvoiceId,
    pub entity: InvoiceEntity,
    #[serde(rename = "type")]
    pub type_: InvoiceType,
    pub invoice_number: String,
    pub customer_id: Option<CustomerId>,
    pub customer_details: Option<CustomerDetails>,
    pub order_id: OrderId,
    pub line_items: Vec<LineItem>,
    pub payment_id: PaymentId,
    pub status: InvoiceStatus,
    #[serde(with = "ts_seconds")]
    pub expire_by: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub issued_at: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub paid_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub cancelled_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub expired_at: Option<DateTime<Utc>>,
    pub sms_status: InvoiceMessageStatus,
    pub email_status: InvoiceMessageStatus,
    pub partial_payment: bool,
    pub amount: u64,
    pub amount_paid: u64,
    pub amount_due: u64,
    pub currency: Currency,
    pub description: Option<String>,
    #[serde(deserialize_with = "deserialize_notes")]
    pub notes: Object,
    pub short_url: String,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub terms: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Default, Serialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CreateOrUpdateInvoiceType {
    #[default]
    Invoice,
}

#[derive(Debug, Default, Serialize, Clone, Eq, PartialEq)]
pub struct CreateInvoiceCustomerAddress<'a> {
    pub line1: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    pub city: &'a str,
    pub zipcode: &'a str,
    pub state: &'a str,
    pub country: &'a str,
}

#[derive(Debug, Default, Serialize, Clone, Eq, PartialEq)]
pub struct CreateOrUpdateInvoiceCustomer<'a> {
    pub name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<CreateInvoiceCustomerAddress<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CreateInvoiceCustomerAddress<'a>>,
}

#[derive(Debug, Default, Serialize, Clone, Eq, PartialEq)]
pub struct CreateOrUpdateInvoiceLineItem<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}

#[derive(Debug, Default, Serialize, Clone, Eq, PartialEq)]
pub struct CreateInvoice<'a> {
    #[serde(rename = "type")]
    pub type_: CreateOrUpdateInvoiceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(
        serialize_with = "serialize_bool_as_int_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<&'a CustomerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CreateOrUpdateInvoiceCustomer<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CreateOrUpdateInvoiceLineItem<'a>>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_seconds_option"
    )]
    pub expire_by: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_bool_as_int_option"
    )]
    pub sms_notify: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_bool_as_int_option"
    )]
    pub email_notify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_payment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Object>,
}

#[derive(Debug, Default, Serialize, Clone, Eq, PartialEq)]
pub struct UpdateInvoice<'a> {
    #[serde(rename = "type")]
    pub type_: CreateOrUpdateInvoiceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(
        serialize_with = "serialize_bool_as_int_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<&'a CustomerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CreateOrUpdateInvoiceCustomer<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CreateOrUpdateInvoiceLineItem<'a>>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_seconds_option"
    )]
    pub expire_by: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_bool_as_int_option"
    )]
    pub sms_notify: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_bool_as_int_option"
    )]
    pub email_notify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_payment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Object>,
}

#[derive(Debug, Default, Serialize, Clone, Eq, PartialEq)]
pub struct ListInvoices<'a> {
    pub payment_id: Option<&'a PaymentId>,
    pub receipt: Option<&'a str>,
    pub customer_id: Option<&'a CustomerId>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InvoiceNotifyMedium {
    Sms,
    Email,
}

impl Display for InvoiceNotifyMedium {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatterResult {
        write!(
            f,
            "{}",
            match self {
                InvoiceNotifyMedium::Sms => "sms",
                InvoiceNotifyMedium::Email => "email",
            }
        )
    }
}

#[derive(Debug, Deserialize)]
struct InvoiceNotifyResult {
    success: bool,
}

impl Invoice {
    pub async fn create(
        razorpay: &Razorpay,
        params: CreateInvoice<'_>,
    ) -> RazorpayResult<Invoice> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: "/invoices".to_owned(),
                version: None,
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(invoice) => Ok(invoice),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn update(
        razorpay: &Razorpay,
        invoice_id: &InvoiceId,
        params: UpdateInvoice<'_>,
    ) -> RazorpayResult<Invoice> {
        let res = razorpay
            .api
            .patch(RequestParams {
                url: format!("/invoices/{}", invoice_id),
                version: None,
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(invoice) => Ok(invoice),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn issue(
        razorpay: &Razorpay,
        invoice_id: &InvoiceId,
    ) -> RazorpayResult<Invoice> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: format!("/invoices/{}/issue", invoice_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(invoice) => Ok(invoice),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn delete(
        razorpay: &Razorpay,
        invoice_id: &InvoiceId,
    ) -> RazorpayResult<()> {
        let res: InternalApiResult<Value> = razorpay
            .api
            .delete(RequestParams {
                url: format!("/invoices/{}", invoice_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(_) => Ok(()),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn cancel(
        razorpay: &Razorpay,
        invoice_id: &InvoiceId,
    ) -> RazorpayResult<Invoice> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: format!("/invoices/{}/cancel", invoice_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(invoice) => Ok(invoice),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch(
        razorpay: &Razorpay,
        invoice_id: &InvoiceId,
    ) -> RazorpayResult<Invoice> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/invoices/{}", invoice_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(invoice) => Ok(invoice),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn list<T>(
        razorpay: &Razorpay,
        params: T,
    ) -> RazorpayResult<Collection<Invoice>>
    where
        T: for<'a> Into<Option<ListInvoices<'a>>>,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/invoices".to_owned(),
                version: None,
                data: params.into(),
            })
            .await?;

        match res {
            InternalApiResult::Ok(invoice) => Ok(invoice),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn notify(
        razorpay: &Razorpay,
        invoice_id: &InvoiceId,
        medium: InvoiceNotifyMedium,
    ) -> RazorpayResult<bool> {
        let res: InternalApiResult<InvoiceNotifyResult> = razorpay
            .api
            .post(RequestParams {
                url: format!("/invoices/{}/notify_by/{}", invoice_id, medium),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(res) => Ok(res.success),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    // TODO: Add more invoice APIs
    //
    // there are many other APIs which needs to be implemented, all of
    // them are list in the [docs]
    //
    // [docs]: https://razorpay.com/docs/api/payments/invoices
}
