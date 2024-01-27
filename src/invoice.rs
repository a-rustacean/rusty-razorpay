use crate::{
    address::Address,
    api::RequestParams,
    common::{Currency, Object},
    entity::InvoiceEntity,
    error::{InternalApiResult, RazorpayResult},
    ids::CustomerId,
    line_item::LineItem,
    util::{deserialize_notes, serialize_bool_as_int_option},
    InvoiceId, OrderId, PaymentId, Razorpay,
};
use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Utc,
};
use serde::{Deserialize, Serialize};

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
pub enum CreateInvoiceType {
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
pub struct CreateInvoiceCustomer<'a> {
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
pub struct CreateInvoiceLineItem<'a> {
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
    pub type_: CreateInvoiceType,
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
    pub customer: Option<CreateInvoiceCustomer<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CreateInvoiceLineItem<'a>>>,
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

    // TODO: Add more invoice APIs
    //
    // there are many other APIs which needs to be implemented, all of
    // them are list in the [docs]
    //
    // [docs]: https://razorpay.com/docs/api/payments/invoices
}
