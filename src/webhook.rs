use crate::{
    account::Account, dispute::Dispute, entity::WebhookEntity,
    invoice::Invoice, order::Order, payment::Payment, refund::Refund,
    subscription::Subscription, util::generate_webhook_signature,
};
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};
use chrono::{serde::ts_seconds, DateTime, Utc};
#[cfg(not(feature = "std"))]
use core::fmt::{Display, Formatter, Result as FormatterResult};
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
use serde::Deserialize;
use serde_json::Value;
#[cfg(feature = "std")]
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FormatterResult},
};

#[derive(Debug)]
pub enum WebhookError {
    ParseError(serde_json::error::Error),
    BadSignature,
}

impl Display for WebhookError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatterResult {
        match self {
            WebhookError::ParseError(error) => {
                write!(f, "Parsing error: {}", error)
            }
            WebhookError::BadSignature => write!(f, "Bad signature"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WebhookError {}

impl From<serde_json::error::Error> for WebhookError {
    fn from(error: serde_json::error::Error) -> Self {
        Self::ParseError(error)
    }
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub enum WebhookEvent {
    // Payment events
    #[serde(rename = "payment.authorized")]
    PaymentAuthorized,
    #[serde(rename = "payment.failed")]
    PaymentFailed,
    #[serde(rename = "payment.captured")]
    PaymentCaptured,
    #[serde(rename = "payment.dispute.created")]
    PaymentDisputeCreated,
    #[serde(rename = "payment.dispute.won")]
    PaymentDisputeWon,
    #[serde(rename = "payment.dispute.lost")]
    PaymentDisputeLost,
    #[serde(rename = "payment.dispute.closed")]
    PaymentDisputeClosed,
    #[serde(rename = "payment.dispute.under_review")]
    PaymentDisputeUnderReview,
    #[serde(rename = "payment.dispute.action_required")]
    PaymentDisputeActionRequired,
    #[serde(rename = "payment.downtime.started")]
    PaymentDowntimeStarted,
    #[serde(rename = "payment.downtime.updated")]
    PaymentDowntimeUpdated,
    #[serde(rename = "payment.downtime.resolved")]
    PaymentDowntimeResolved,

    // Order events
    #[serde(rename = "order.paid")]
    OrderPaid,

    // Invoice events
    #[serde(rename = "invoice.paid")]
    InvoicePaid,
    #[serde(rename = "invoice.partially_paid")]
    InvoicePartiallyPaid,
    #[serde(rename = "invoice.expired")]
    InvoiceExpired,

    // Subscription events
    #[serde(rename = "subscription.authenticated")]
    SubscriptionAuthenticated,
    #[serde(rename = "subscription.paused")]
    SubscriptionPaused,
    #[serde(rename = "subscription.resumed")]
    SubscriptionResumed,
    #[serde(rename = "subscription.activated")]
    SubscriptionActivated,
    #[serde(rename = "subscription.pending")]
    SubscriptionPending,
    #[serde(rename = "subscription.halted")]
    SubscriptionHalted,
    #[serde(rename = "subscription.charged")]
    SubscriptionCharged,
    #[serde(rename = "subscription.cancelled")]
    SubscriptionCancelled,
    #[serde(rename = "subscription.completed")]
    SubscriptionCompleted,
    #[serde(rename = "subscription.updated")]
    SubscriptionUpdated,

    // Settlement events
    #[serde(rename = "settlement.processed")]
    SettlementProcessed,

    // Virtual account events
    #[serde(rename = "virtual_account.credited")]
    VirtualAccountCredited,
    #[serde(rename = "virtual_account.created")]
    VirtualAccountCreated,
    #[serde(rename = "virtual_account.closed")]
    VirtualAccountClosed,

    // Fund account events
    #[serde(rename = "fund_account.validation.completed")]
    FundAccountValidationCompleted,
    #[serde(rename = "fund_account.validation.failed")]
    FundAccountValidationFailed,

    // Payout events
    #[serde(rename = "payout.processed")]
    PayoutProcessed,
    #[serde(rename = "payout.reversed")]
    PayoutReversed,
    #[serde(rename = "payout.initiated")]
    PayoutInitiated,
    #[serde(rename = "payout.updated")]
    PayoutUpdated,
    #[serde(rename = "payout.rejected")]
    PayoutRejected,
    #[serde(rename = "payout.pending")]
    PayoutPending,
    #[serde(rename = "payout.queued")]
    PayoutQueued,
    #[serde(rename = "payout.failed")]
    PayoutFailed,
    #[serde(rename = "payout.downtime.started")]
    PayoutDowntimeStarted,
    #[serde(rename = "payout.downtime.resolved")]
    PayoutDowntimeResolved,

    // Refund events
    #[serde(rename = "refund.speed_changed")]
    RefundSpeedChanged,
    #[serde(rename = "refund.processed")]
    RefundProcessed,
    #[serde(rename = "refund.failed")]
    RefundFailed,
    #[serde(rename = "refund.created")]
    RefundCreated,

    // Transfer events
    #[serde(rename = "transfer.processed")]
    TransferProcessed,
    #[serde(rename = "transfer.failed")]
    TransferFailed,

    // Account events
    #[serde(rename = "account.under_review")]
    AccountUnderReview,
    #[serde(rename = "account.needs_clarification")]
    AccountNeedsClarification,
    #[serde(rename = "account.activated")]
    AccountActivated,
    #[serde(rename = "account.rejected")]
    AccountRejected,
    #[serde(rename = "account.updated")]
    AccountUpdated,
    #[serde(rename = "account.suspended")]
    AccountSuspended,
    #[serde(rename = "account.funds_hold")]
    AccountFundsHold,
    #[serde(rename = "account.funds_unhold")]
    AccountFundsUnhold,
    #[serde(rename = "account.instantly_activated")]
    AccountInstantlyActivated,
    #[serde(rename = "account.payments_enabled")]
    AccountPaymentsEnabled,

    // Payment link events
    PaymentLinkPending,
    #[serde(rename = "payment_link.paid")]
    PaymentLinkPaid,
    #[serde(rename = "payment_link.partially_paid")]
    PaymentLinkPartiallyPaid,
    #[serde(rename = "payment_link.expired")]
    PaymentLinkExpired,
    #[serde(rename = "payment_link.cancelled")]
    PaymentLinkCancelled,

    // Product events
    #[serde(rename = "product.route.activated")]
    ProductRouteActivated,
    #[serde(rename = "product.route.under_review")]
    ProductRouteUnderReview,
    #[serde(rename = "product.route.needs_clarification")]
    ProductRouteNeedsClarification,
    #[serde(rename = "product.route.rejected")]
    ProductRouteRejected,
    #[serde(rename = "product.payment_gateway.activated")]
    ProductPaymentGatewayActivated,
    #[serde(rename = "product.payment_gateway.under_review")]
    ProductPaymentGatewayUnderReview,
    #[serde(rename = "product.payment_gateway.needs_clarification")]
    ProductPaymentGatewayNeedsClarification,
    #[serde(rename = "product.payment_gateway.rejected")]
    ProductPaymentGatewayRejected,
    #[serde(rename = "product.payment_gateway.activated_kyc_pending")]
    ProductPaymentGatewayActivatedKYCPending,

    // Oauth partner events
    #[serde(rename = "account.app.authorization_revoked")]
    AccountAppAuthorizationRevoked,

    // Payout link events
    #[serde(rename = "payout_link.pending")]
    PayoutLinkPending,
    #[serde(rename = "payout_link.issued")]
    PayoutLinkIssued,
    #[serde(rename = "payout_link.processing")]
    PayoutLinkProcessing,
    #[serde(rename = "payout_link.processed")]
    PayoutLinkProcessed,
    #[serde(rename = "payout_link.attempted")]
    PayoutLinkAttempted,
    #[serde(rename = "payout_link.cancelled")]
    PayoutLinkCancelled,
    #[serde(rename = "payout_link.rejected")]
    PayoutLinkRejected,
    #[serde(rename = "payout_link.expired")]
    PayoutLinkExpired,

    // Transaction events
    #[serde(rename = "transaction.created")]
    TransactionCreated,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum WebhookPayloadItemName {
    Order,
    Payment,
    Refund,
    Dispute,
    Invoice,
    Subscription,
    Transfer,
    VirtualAccount,
    PaymentLink,
    #[serde(rename = "fund_account.validation")]
    FundAccountValidation,
    Payout,
    PayoutLink,
    MerchantProduct,
    Account,
    #[serde(rename = "payout.downtime")]
    PayoutDowntime,
    Transaction,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum WebhookPayloadItem {
    Order(Order),
    Payment(Payment),
    Refund(Refund),
    Dispute(Dispute),
    Invoice(Invoice),
    Subscription(Subscription),
    // TODO: Add missing webhook payload item
    //
    // the following items need to be implemented, the workaround for now is
    // the `Other` variant
    //        |
    //         "--------------------------------------------.
    //                                                       |
    // Transfer(Transfer),                                   |
    // VirtualAccount(VirtualAccount),                       |
    // PaymentLink(PaymentLink),                             |
    // FundAccountValidation(FundAccountValidation),         |
    // Payout(Payout),                                       |
    // PayoutLink(PayoutLink),                               |
    // MerchantProduct(MerchantProduct),                     |
    Account(Account),
    // PayoutDowntime(PayoutDowntime),                       |
    // Transaction(Transaction),                             |
    Other(Value),
    //^^^^^^^^^^                                             |
    //    |                                                  |
    //     "------------------------------------------------"
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct WebhookPayload {
    pub entity: WebhookPayloadItem,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct Webhook {
    #[serde(default)]
    pub entity: WebhookEntity,
    pub account_id: String,
    pub event: WebhookEvent,
    pub contains: Vec<WebhookPayloadItemName>,
    #[serde(default)]
    pub payload: HashMap<WebhookPayloadItemName, WebhookPayload>,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Webhook {
    pub fn construct_event(
        payload: &str,
        sig: &str,
        secret: &str,
    ) -> Result<Webhook, WebhookError> {
        let expected_sig = generate_webhook_signature(payload, secret);
        if sig != expected_sig {
            return Err(WebhookError::BadSignature);
        }

        Ok(serde_json::from_str(payload)?)
    }
}
