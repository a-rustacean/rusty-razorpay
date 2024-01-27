use crate::{
    api::RequestParams,
    common::{Collection, Object},
    error::{InternalApiResult, RazorpayResult},
    ids::CustomerId,
    util::{deserialize_notes, serialize_bool_as_int_option},
    Razorpay,
};
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct Customer {
    pub id: CustomerId,
    pub name: String,
    pub contact: Option<String>,
    pub email: Option<String>,
    pub gstin: Option<String>,
    #[serde(deserialize_with = "deserialize_notes")]
    pub notes: Object,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Clone, Eq, PartialEq)]
pub struct CreateCustomer<'a> {
    pub name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(
        serialize_with = "serialize_bool_as_int_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fail_existing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gstin: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Object>,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct UpdateCustomer<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<&'a str>,
}

#[derive(Debug, Default, Serialize, Clone, Eq, PartialEq)]
pub struct ListCustomers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<u64>,
}

impl Customer {
    pub async fn create(
        razorpay: &Razorpay,
        params: CreateCustomer<'_>,
    ) -> RazorpayResult<Customer> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: "/customers".to_owned(),
                version: None,
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(customer) => Ok(customer),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn update(
        razorpay: &Razorpay,
        customer_id: &CustomerId,
        params: UpdateCustomer<'_>,
    ) -> RazorpayResult<Customer> {
        let res = razorpay
            .api
            .put(RequestParams {
                url: format!("/customers/{}", customer_id),
                version: None,
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(customer) => Ok(customer),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn list<T>(
        razorpay: &Razorpay,
        params: T,
    ) -> RazorpayResult<Collection<Customer>>
    where
        T: Into<Option<ListCustomers>>,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/customers".to_owned(),
                version: None,
                data: params.into(),
            })
            .await?;

        match res {
            InternalApiResult::Ok(customers) => Ok(customers),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch(
        razorpay: &Razorpay,
        customer_id: &CustomerId,
    ) -> RazorpayResult<Customer> {
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
