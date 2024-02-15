#[cfg(not(feature = "std"))]
use alloc::{format, string::String};

use serde::Deserialize;

use crate::{
    api::RequestParams,
    error::{InternalApiResult, RazorpayResult},
    ids::CardId,
    Razorpay,
};

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum CardNetwork {
    MasterCard,
    Visa,
    RuPay,
    #[serde(rename = "American Express")]
    AmericanExpress,
    #[serde(rename = "Diners Club")]
    DinersClub,
    #[serde(rename = "Bajaj Finserv")]
    BajajFinserv,
    Maestro,
    JCB,
    #[serde(rename = "Union Pay")]
    UnionPay,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CardTypeExtended {
    Credit,
    Debit,
    Prepaid,
    Unknown,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CardType {
    Credit,
    Debit,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CardSubType {
    Customer,
    Business,
    Unknown,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "entity", rename = "card")]
pub struct Card {
    pub id: CardId,
    pub name: String,
    pub last4: String,
    pub network: CardNetwork,
    #[serde(rename = "type")]
    pub type_: CardTypeExtended,
    pub issuer: Option<String>,
    pub emi: bool,
    pub sub_type: CardSubType,
}

// INFO: I was unable to find any docs related to the cards api, but
// in the razorpay SDK for Node.js I found [something] that I have
// partially re-created
//
// [something]: https://github.com/razorpay/razorpay-node/blob/753c07b6f2bea6c784c9866ad39fe761d93ed9ad/lib/resources/cards.js

impl Card {
    pub async fn fetch(
        razorpay: &Razorpay,
        card_id: &CardId,
    ) -> RazorpayResult<Card> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/cards/{}", card_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(card) => Ok(card),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }
}
