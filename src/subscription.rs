use crate::{
    addon::Addon,
    api::RequestParams,
    common::{Collection, Currency, Filter, Object},
    error::{InternalApiResult, RazorpayResult},
    util::{deserialize_notes, serialize_bool_as_int_option},
    Razorpay,
};
use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Utc,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;

#[derive(Debug, Default, Serialize)]
pub struct CreateSubscriptionAddonItem {
    pub name: String,
    pub amount: u64,
    pub currency: Currency,
}

#[derive(Debug, Default, Serialize)]
pub struct CreateSubscriptionAddon {
    pub item: CreateSubscriptionAddonItem,
}

#[derive(Debug, Default, Serialize)]
pub struct CreateSubscriptionNotifyInfo {
    pub notify_email: String,
    pub notify_phone: String,
}

#[derive(Debug, Default, Serialize)]
pub struct CreateSubscription {
    pub plan_id: String,
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
    pub addons: Vec<CreateSubscriptionAddon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Object::is_empty")]
    pub notes: Object,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_info: Option<CreateSubscriptionNotifyInfo>,
}

#[derive(Debug, Default, Serialize)]
pub struct AllSubscriptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(flatten)]
    pub filter: Filter,
}

#[derive(Debug, Serialize)]
pub struct UpdateSubscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionChangeSchedule {
    #[default]
    Now,
    CycleEnd,
}

#[derive(Debug, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub entity: String,
    pub plan_id: String,
    pub customer_id: Option<String>,
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
    pub offer_id: Option<String>,
    pub short_url: String,
    pub has_scheduled_changes: bool,
    pub schedule_change_at: Option<SubscriptionChangeSchedule>,
    pub remaining_count: u64,
}

impl Subscription {
    pub async fn create(
        razorpay: &Razorpay,
        data: CreateSubscription,
    ) -> RazorpayResult<Subscription> {
        let res = razorpay
            .api
            .post(RequestParams {
                url: "/subscriptions".to_owned(),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(subscription) => Ok(subscription),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch<T>(
        razorpay: &Razorpay,
        subscription_id: T,
    ) -> RazorpayResult<Subscription>
    where
        T: Display,
    {
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

    pub async fn all(
        razorpay: &Razorpay,
        data: AllSubscriptions,
    ) -> RazorpayResult<Collection<Subscription>> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: "/subscription".to_owned(),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(subscriptions) => Ok(subscriptions),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn cancel<T, U>(
        razorpay: &Razorpay,
        subscription_id: T,
        cancel_at_cycle_end: U,
    ) -> RazorpayResult<Subscription>
    where
        T: Display,
        U: Into<Option<bool>>,
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

    pub async fn update<T>(
        razorpay: &Razorpay,
        subscription_id: T,
        data: UpdateSubscription,
    ) -> RazorpayResult<Subscription>
    where
        T: Display,
    {
        let res = razorpay
            .api
            .patch(RequestParams {
                url: format!("/subscriptions/{}", subscription_id),
                version: None,
                data: Some(data),
            })
            .await?;

        match res {
            InternalApiResult::Ok(subscription) => Ok(subscription),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    pub async fn fetch_pending_update<T>(
        razorpay: &Razorpay,
        subscription_id: T,
    ) -> RazorpayResult<Subscription>
    where
        T: Display,
    {
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

    pub async fn cancel_scheduled_update<T>(
        razorpay: &Razorpay,
        subscription_id: T,
    ) -> RazorpayResult<Subscription>
    where
        T: Display,
    {
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

    pub async fn pause<T>(
        razorpay: &Razorpay,
        subscription_id: T,
    ) -> RazorpayResult<Subscription>
    where
        T: Display,
    {
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

    pub async fn resume<T>(
        razorpay: &Razorpay,
        subscription_id: T,
    ) -> RazorpayResult<Subscription>
    where
        T: Display,
    {
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
