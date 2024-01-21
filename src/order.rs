use crate::{
    api::RequestParams,
    common::{Collection, Currency, FilterOptions, Object},
    error::{InternalApiResult, RazorpayResult},
    payment::Payment,
    util::{deserialize_notes, serialize_bool_as_int_option},
    Razorpay,
};
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;

#[derive(Debug, Serialize)]
pub struct OrderBankAccount {
    pub account_number: String,
    pub name: String,
    pub ifsc: String,
}

#[derive(Debug, Default, Serialize)]
pub struct CreateOrderOptions {
    pub amount: u64,
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<String>,
    #[serde(skip_serializing_if = "Object::is_empty")]
    pub notes: Object,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_payment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<OrderBankAccount>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum OrderExpand {
    Payments,
    PaymentsCard,
    Transfers,
    VirtualAccount,
}

#[derive(Debug, Serialize, Default)]
pub struct AllOrdersOptions {
    #[serde(flatten)]
    pub filter: FilterOptions,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_bool_as_int_option"
    )]
    pub authorized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<String>,
    #[serde(rename = "expand[]")]
    pub expand: Vec<OrderExpand>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Created,
    Attempted,
    Paid,
}

#[derive(Debug, Deserialize)]
pub struct Order {
    pub id: String,
    pub entity: String,
    pub amount: u64,
    pub partial_payment: Option<bool>,
    pub amount_paid: u64,
    pub amount_due: u64,
    pub currency: Currency,
    pub receipt: Option<String>,
    pub offer_id: Option<String>,
    pub payments: Option<Collection<Payment>>,
    pub status: OrderStatus,
    #[serde(deserialize_with = "deserialize_notes")]
    pub notes: Object,
    pub attempts: u32,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Order {
    pub async fn create(
        razorpay: &Razorpay,
        data: CreateOrderOptions,
    ) -> RazorpayResult<Order> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: "/orders".to_owned(),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(order) => Ok(order),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn all(
        razorpay: &Razorpay,
        data: AllOrdersOptions,
    ) -> RazorpayResult<Collection<Order>> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/orders".to_owned(),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(order) => Ok(order),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch<T>(
        razorpay: &Razorpay,
        order_id: T,
    ) -> RazorpayResult<Order>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/orders/{}", order_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(order) => Ok(order),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch_payments<T>(
        razorpay: &Razorpay,
        order_id: T,
    ) -> RazorpayResult<Collection<Payment>>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/orders/{}/payments", order_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(payments) => Ok(payments),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn update<T>(
        razorpay: &Razorpay,
        order_id: T,
        notes: Object,
    ) -> RazorpayResult<Order>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .patch(RequestParams {
                url: format!("/orders/{}", order_id),
                version: None,
                data: Some(json!({ "notes": notes })),
            })
            .await?;

        match res {
            InternalApiResult::Ok(order) => Ok(order),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }
}
