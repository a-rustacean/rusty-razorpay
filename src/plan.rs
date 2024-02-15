#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, format};

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    api::RequestParams,
    common::{Collection, Currency, Filter, Object},
    error::{InternalApiResult, RazorpayResult},
    ids::PlanId,
    item::Item,
    util::deserialize_notes,
    Razorpay,
};

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct CreatePlanItem<'a> {
    pub name: &'a str,
    pub amount: u64,
    pub currency: Currency,
    pub description: Option<&'a str>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct CreatePlan<'a> {
    pub interval: u8,
    pub period: PlanPeriod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Object>,
    pub item: CreatePlanItem<'a>,
}

impl<'a> Default for CreatePlan<'a> {
    fn default() -> Self {
        Self {
            interval: 1,
            period: Default::default(),
            notes: None,
            item: Default::default(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PlanPeriod {
    Daily,
    Weekly,
    #[default]
    Monthly,
    Yearly,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "entity", rename = "plan")]
pub struct Plan {
    pub id: PlanId,
    pub interval: u8,
    pub period: PlanPeriod,
    pub item: Item,
    #[serde(deserialize_with = "deserialize_notes")]
    pub notes: Object,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Plan {
    pub async fn create(
        razorpay: &Razorpay,
        params: CreatePlan<'_>,
    ) -> RazorpayResult<Plan> {
        let res = razorpay
            .api
            .post(crate::api::RequestParams {
                url: "/plans".to_owned(),
                version: None,
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(plan) => Ok(plan),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn list<T>(
        razorpay: &Razorpay,
        params: T,
    ) -> RazorpayResult<Collection<Plan>>
    where
        T: Into<Option<Filter>>,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/plans".to_owned(),
                version: None,
                data: params.into(),
            })
            .await?;

        match res {
            InternalApiResult::Ok(plans) => Ok(plans),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch(
        razorpay: &Razorpay,
        plan_id: &PlanId,
    ) -> RazorpayResult<Plan> {
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
