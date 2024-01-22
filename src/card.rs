use std::fmt::Display;

use serde::Deserialize;

use crate::{
    api::RequestParams,
    error::{InternalApiResult, RazorpayResult},
    Razorpay,
};

#[derive(Debug, Deserialize)]
pub enum CardNetwork {
    MasterCard,
    Visa,
    RuPay,
    #[serde(rename = "American Express")]
    AmericanExpress,
    #[serde(rename = "Dirners Club")]
    DinersClub,
    Maestro,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardTypeExtended {
    Credit,
    Debit,
    Prepaid,
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardType {
    Credit,
    Debit,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardSubType {
    Customer,
    Business,
}

#[derive(Debug, Deserialize)]
pub struct Card {
    pub id: String,
    pub entity: String,
    pub name: String,
    pub last4: String,
    pub network: CardNetwork,
    pub r#type: CardTypeExtended,
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
    pub async fn fetch<T>(
        razorpay: &Razorpay,
        card_id: T,
    ) -> RazorpayResult<Card>
    where
        T: Display,
    {
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
