use crate::{
    address::Address,
    api::RequestParams,
    common::{Currency, Object},
    error::{InternalApiResult, RazorpayResult},
    line_item::LineItem,
    util::{deserialize_notes, serialize_bool_as_int_option},
    Razorpay,
};
use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Utc,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InvoiceType {
    Invoice,
}

#[derive(Debug, Deserialize)]
pub struct CustomerDetails {
    pub id: String,
    pub name: String,
    pub email: String,
    pub contact: String,
    pub billing_address: Option<Address>,
    pub shipping_address: Option<Address>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InvoiceMessageStatus {
    Pending,
    Sent,
}

#[derive(Debug, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub entity: String,
    pub r#type: InvoiceType,
    pub invoice_number: String,
    pub customer_id: Option<String>,
    pub customer_details: Option<CustomerDetails>,
    pub order_id: String,
    pub line_items: Vec<LineItem>,
    pub payment_id: String,
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

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateInvoiceType {
    #[default]
    Invoice,
}

#[derive(Debug, Serialize)]
pub struct CreateInvoiceCustomerAddress {
    pub line1: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    pub city: String,
    pub zipcode: String,
    pub state: String,
    pub country: String,
}

#[derive(Debug, Default, Serialize)]
pub struct CreateInvoiceCustomer {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<CreateInvoiceCustomerAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CreateInvoiceCustomerAddress>,
}

#[derive(Debug, Serialize)]
pub struct CreateInvoiceLineItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}

#[derive(Debug, Default, Serialize)]
pub struct CreateInvoice {
    pub r#type: CreateInvoiceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        serialize_with = "serialize_bool_as_int_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CreateInvoiceCustomer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CreateInvoiceLineItem>>,
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
    #[serde(skip_serializing_if = "Object::is_empty")]
    pub notes: Object,
}

impl Invoice {
    pub async fn create(
        razorpay: &Razorpay,
        data: CreateInvoice,
    ) -> RazorpayResult<Invoice> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: "/invoices".to_owned(),
                version: None,
                data: Some(data),
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
