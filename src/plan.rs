use std::fmt::Display;

use crate::{
    api::RequestParams,
    common::{Collection, Currency, FilterOptions, Object},
    error::{InternalApiResult, RazorpayResult},
    subscription::SubscriptionItem,
    Razorpay,
};
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize)]
pub struct CreatePlanItemOptions {
    pub name: String,
    pub amount: u64,
    pub currency: Currency,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreatePlanOptions {
    pub interval: u8,
    pub period: PlanPeriod,
    pub notes: Object,
    pub item: CreatePlanItemOptions,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlanPeriod {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Deserialize)]
pub struct Plan {
    pub id: String,
    pub entity: String,
    pub interval: u8,
    pub period: PlanPeriod,
    pub item: SubscriptionItem,
    pub notes: Object,
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
}

impl Plan {
    pub async fn create(
        razorpay: &Razorpay,
        data: CreatePlanOptions,
    ) -> RazorpayResult<Plan> {
        let res = razorpay
            .api
            .post(crate::api::RequestParams {
                url: "/plans".to_string(),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(plan) => Ok(plan),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn all<T>(
        razorpay: &Razorpay,
        filter: T,
    ) -> RazorpayResult<Collection<Plan>>
    where
        T: Into<Option<FilterOptions>>,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/plans".to_owned(),
                version: None,
                data: filter.into(),
            })
            .await?;

        match res {
            InternalApiResult::Ok(plans) => Ok(plans),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch<T>(
        razorpay: &Razorpay,
        plan_id: T,
    ) -> RazorpayResult<Plan>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/plans/{}", plan_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(plan) => Ok(plan),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }
}
