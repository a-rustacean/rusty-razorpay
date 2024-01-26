use crate::{
    api::RequestParams,
    common::{Collection, Currency, Filter, Object},
    entity::OrderEntity,
    error::{InternalApiResult, RazorpayResult},
    ids::OrderId,
    payment::Payment,
    util::{deserialize_notes, serialize_bool_as_int_option},
    OfferId, Razorpay,
};
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct OrderBankAccount<'a> {
    pub account_number: &'a str,
    pub name: &'a str,
    pub ifsc: &'a str,
}

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct CreateOrder<'a> {
    pub amount: u64,
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<&'a str>,
    #[serde(skip_serializing_if = "Object::is_empty")]
    pub notes: Object,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_payment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<OrderBankAccount<'a>>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum OrderExpand {
    Payments,
    PaymentsCard,
    Transfers,
    VirtualAccount,
}

#[derive(Debug, Serialize, Default, Clone, PartialEq, Eq)]
pub struct ListOrders<'a> {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_bool_as_int_option"
    )]
    pub authorized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<&'a str>,
    #[serde(rename = "expand[]")]
    pub expand: &'a [OrderExpand],
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Created,
    Attempted,
    Paid,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Order {
    pub id: OrderId,
    pub entity: OrderEntity,
    pub amount: u64,
    pub partial_payment: Option<bool>,
    pub amount_paid: u64,
    pub amount_due: u64,
    pub currency: Currency,
    pub receipt: Option<String>,
    pub offer_id: Option<OfferId>,
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
        params: CreateOrder<'_>,
    ) -> RazorpayResult<Order> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: "/orders".to_owned(),
                version: None,
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(order) => Ok(order),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn list<T>(
        razorpay: &Razorpay,
        params: T,
    ) -> RazorpayResult<Collection<Order>>
    where
        T: for<'a> Into<Option<ListOrders<'a>>>,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/orders".to_owned(),
                version: None,
                data: params.into(),
            })
            .await?;

        match res {
            InternalApiResult::Ok(order) => Ok(order),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch(
        razorpay: &Razorpay,
        order_id: &OrderId,
    ) -> RazorpayResult<Order> {
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

    pub async fn list_payments(
        razorpay: &Razorpay,
        order_id: &OrderId,
    ) -> RazorpayResult<Collection<Payment>> {
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

    pub async fn update(
        razorpay: &Razorpay,
        order_id: &OrderId,
        notes: Object,
    ) -> RazorpayResult<Order> {
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
