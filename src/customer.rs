use std::fmt::Display;

use crate::{
    api::RequestParams,
    common::{Collection, Object},
    error::{InternalApiResult, RazorpayResult},
    util::{deserialize_notes, serialize_bool_as_int_option},
    Razorpay,
};
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub contact: Option<String>,
    pub email: Option<String>,
    pub gstin: Option<String>,
    #[serde(deserialize_with = "deserialize_notes")]
    pub notes: Object,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize)]
pub struct CreateCustomer {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(
        serialize_with = "serialize_bool_as_int_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fail_existing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gstin: Option<String>,
    #[serde(skip_serializing_if = "Object::is_empty")]
    pub notes: Object,
}

#[derive(Debug, Serialize)]
pub struct UpdateCustomer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct AllCustomers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<u64>,
}

impl Customer {
    pub async fn create(
        razorpay: &Razorpay,
        data: CreateCustomer,
    ) -> RazorpayResult<Customer> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: "/customers".to_owned(),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(customer) => Ok(customer),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn update<T>(
        razorpay: &Razorpay,
        customer_id: T,
        data: UpdateCustomer,
    ) -> RazorpayResult<Customer>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .put(RequestParams {
                url: format!("/customers/{}", customer_id),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(customer) => Ok(customer),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn all(
        razorpay: &Razorpay,
        data: AllCustomers,
    ) -> RazorpayResult<Collection<Customer>> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/customers".to_owned(),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(customers) => Ok(customers),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch<T>(
        razorpay: &Razorpay,
        customer_id: T,
    ) -> RazorpayResult<Customer>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/customers/{}", customer_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(customer) => Ok(customer),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }
}
