use crate::{
    api::RequestParams,
    common::{Collection, Currency, Filter, Object},
    entity::PlanEntity,
    error::{InternalApiResult, RazorpayResult},
    ids::PlanId,
    item::Item,
    util::deserialize_notes,
    Razorpay,
};
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    pub notes: Object,
    pub item: CreatePlanItem<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PlanPeriod {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Plan {
    pub id: PlanId,
    pub entity: PlanEntity,
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
                url: "/plans".to_string(),
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
