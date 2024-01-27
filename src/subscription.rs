use crate::{
    addon::Addon,
    api::RequestParams,
    common::{Collection, Currency, Filter, Object},
    entity::SubscriptionEntity,
    error::{InternalApiResult, RazorpayResult},
    ids::{CustomerId, OfferId, PlanId, SubscriptionId},
    util::{deserialize_notes, serialize_bool_as_int_option},
    Razorpay,
};
use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Utc,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct CreateSubscriptionAddonItem<'a> {
    pub name: &'a str,
    pub amount: u64,
    pub currency: Currency,
}

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct CreateSubscriptionAddon<'a> {
    pub item: CreateSubscriptionAddonItem<'a>,
}

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct CreateSubscriptionNotifyInfo<'a> {
    pub notify_email: &'a str,
    pub notify_phone: &'a str,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct CreateSubscription<'a> {
    pub plan_id: &'a PlanId,
    pub total_count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_seconds_option"
    )]
    pub start_at: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_seconds_option"
    )]
    pub expire_by: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_bool_as_int_option"
    )]
    pub customer_notify: Option<bool>,
    pub addons: &'a [CreateSubscriptionAddon<'a>],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<&'a OfferId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_info: Option<CreateSubscriptionNotifyInfo<'a>>,
}

impl<'a> CreateSubscription<'a> {
    pub fn new(plan_id: &'a PlanId) -> Self {
        Self {
            plan_id,
            total_count: 1,
            quantity: None,
            start_at: None,
            expire_by: None,
            customer_notify: None,
            addons: &[],
            offer_id: None,
            notes: None,
            notify_info: None,
        }
    }
}

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct ListSubscriptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<&'a str>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
}

#[derive(Debug, Default, Serialize, Clone, PartialEq, Eq)]
pub struct UpdateSubscription<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<u64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_seconds_option"
    )]
    pub start_at: Option<DateTime<Utc>>,
    pub schedule_change_at: SubscriptionChangeSchedule,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_notify: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionStatus {
    Created,
    Authenticated,
    Active,
    Pending,
    Halted,
    Cancelled,
    Completed,
    Expired,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionChangeSchedule {
    #[default]
    Now,
    CycleEnd,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Subscription {
    pub id: SubscriptionId,
    pub entity: SubscriptionEntity,
    pub plan_id: PlanId,
    pub customer_id: Option<CustomerId>,
    pub total_count: u8,
    pub customer_notify: bool,
    #[serde(with = "ts_seconds")]
    pub start_at: DateTime<Utc>,
    pub quantity: u64,
    #[serde(deserialize_with = "deserialize_notes")]
    pub notes: Object,
    pub addons: Vec<Addon>,
    pub status: SubscriptionStatus,
    pub paid_count: u64,
    #[serde(with = "ts_seconds")]
    pub current_start: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub current_end: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub ended_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds")]
    pub charge_at: DateTime<Utc>,
    pub auth_attempts: u64,
    #[serde(with = "ts_seconds")]
    pub expire_by: DateTime<Utc>,
    pub offer_id: Option<OfferId>,
    pub short_url: String,
    pub has_scheduled_changes: bool,
    pub schedule_change_at: Option<SubscriptionChangeSchedule>,
    pub remaining_count: u64,
}

impl Subscription {
    pub async fn create(
        razorpay: &Razorpay,
        params: CreateSubscription<'_>,
    ) -> RazorpayResult<Subscription> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: "/subscriptions".to_owned(),
                version: None,
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(subscription) => Ok(subscription),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch(
        razorpay: &Razorpay,
        subscription_id: &SubscriptionId,
    ) -> RazorpayResult<Subscription> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/subscription/{}", subscription_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(subscription) => Ok(subscription),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn list<T>(
        razorpay: &Razorpay,
        params: T,
    ) -> RazorpayResult<Collection<Subscription>>
    where
        T: for<'a> Into<Option<ListSubscriptions<'a>>>,
    {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/subscription".to_owned(),
                version: None,
                data: params.into(),
            })
            .await?;

        match res {
            InternalApiResult::Ok(subscriptions) => Ok(subscriptions),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn cancel<T>(
        razorpay: &Razorpay,
        subscription_id: &SubscriptionId,
        cancel_at_cycle_end: T,
    ) -> RazorpayResult<Subscription>
    where
        T: Into<Option<bool>>,
    {
        let cancel_at_cycle_end =
            cancel_at_cycle_end.into().unwrap_or_default();
        let cancel_at_cycle_end = if cancel_at_cycle_end { 1u8 } else { 0 };

        let res = razorpay
            .api
            .post(RequestParams {
                url: format!("/subscription/{}/cancel", subscription_id),
                version: None,
                data: Some(json!({
                    "cancel_at_cycle_end": cancel_at_cycle_end,
                })),
            })
            .await?;

        match res {
            InternalApiResult::Ok(subscription) => Ok(subscription),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn update(
        razorpay: &Razorpay,
        subscription_id: &SubscriptionId,
        params: UpdateSubscription<'_>,
    ) -> RazorpayResult<Subscription> {
        let res = razorpay
            .api
            .patch(RequestParams {
                url: format!("/subscriptions/{}", subscription_id),
                version: None,
                data: Some(params),
            })
            .await?;

        match res {
            InternalApiResult::Ok(subscription) => Ok(subscription),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch_pending_update(
        razorpay: &Razorpay,
        subscription_id: &SubscriptionId,
    ) -> RazorpayResult<Subscription> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!(
                    "/subscriptions/{}/retrieve_scheduled_changes",
                    subscription_id
                ),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(subscription) => Ok(subscription),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn cancel_scheduled_update(
        razorpay: &Razorpay,
        subscription_id: &SubscriptionId,
    ) -> RazorpayResult<Subscription> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: format!(
                    "/subscriptions/{}/cancel_scheduled_changes",
                    subscription_id
                ),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(subscription) => Ok(subscription),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn pause(
        razorpay: &Razorpay,
        subscription_id: &SubscriptionId,
    ) -> RazorpayResult<Subscription> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: format!("/subscriptions/{}/pause", subscription_id),
                version: None,
                data: Some(json!({
                    "pause_at": "now",
                })),
            })
            .await?;

        match res {
            InternalApiResult::Ok(subscription) => Ok(subscription),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn resume(
        razorpay: &Razorpay,
        subscription_id: &SubscriptionId,
    ) -> RazorpayResult<Subscription> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: format!("/subscriptions/{}/resume", subscription_id),
                version: None,
                data: Some(json!({
                    "resume_at": "now",
                })),
            })
            .await?;

        match res {
            InternalApiResult::Ok(subscription) => Ok(subscription),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }
}
