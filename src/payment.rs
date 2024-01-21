use crate::common::{Currency, Object};
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum PaymentCardNetwork {
    MasterCard,
    Visa,
    RuPay,
    #[serde(rename = "American Express")]
    AmericanExpress,
    #[serde(rename = "Dirners Club")]
    DinersClub,
    Maestro,
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentCardType {
    Credit,
    Debit,
    Prepaid,
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentCardSubType {
    Customer,
    Business,
}

#[derive(Debug, Deserialize)]
pub struct PaymentCard {
    pub id: String,
    pub entity: String,
    pub name: String,
    pub last4: String,
    pub network: PaymentCardNetwork,
    pub r#type: PaymentCardType,
    pub issuer: Option<String>,
    pub emi: bool,
    pub sub_type: PaymentCardSubType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    Created,
    Authorized,
    Captured,
    Refunded,
    Failed,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentMethod {
    Card,
    NetBanking,
    Wallet,
    Emi,
    Upi,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentRefundStatus {
    Partial,
    Full,
}

#[derive(Debug, Deserialize)]
pub struct PaymentAcquirerData {
    pub rrn: String,
    pub authentication_reference_number: Option<String>,
    pub bank_transaction_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentAccountType {
    BankAccount,
    CreditCard,
    Wallet,
}

#[derive(Debug, Deserialize)]
pub struct PaymentUpiInfo {
    pub payer_account_type: PaymentAccountType,
    pub vpa: String,
}

#[derive(Debug, Deserialize)]
pub struct Payment {
    pub id: String,
    pub entity: String,
    pub amount: u64,
    pub currency: Currency,
    pub status: PaymentStatus,
    pub method: PaymentMethod,
    pub order_id: String,
    pub description: Option<String>,
    pub international: bool,
    pub refund_status: Option<PaymentRefundStatus>,
    pub amount_refunded: u64,
    pub captured: bool,
    pub email: String,
    pub contact: String,
    pub fee: u64,
    pub tax: u64,
    pub notes: Object,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub card_id: Option<String>,
    pub card: Option<PaymentCard>,
    pub wallet: Option<String>,
    pub acquirer_data: Option<PaymentAcquirerData>,
    pub bank: Option<String>,
    pub upi: Option<PaymentUpiInfo>,
    pub vpa: String,
}
