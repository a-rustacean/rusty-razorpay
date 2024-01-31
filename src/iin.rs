use crate::{
    api::RequestParams,
    card::{CardNetwork, CardSubType, CardTypeExtended},
    entity::InnEntity,
    error::{InternalApiResult, RazorpayResult},
    Razorpay,
};
#[cfg(not(feature = "std"))]
use alloc::{format, string::String, vec::Vec};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum InnAuthenticationType {
    #[serde(rename = "3ds")]
    ThreeDomainSecure,
    #[serde(rename = "otp")]
    OneTimePassword,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct InnAuthenticationTypeOptions {
    pub type_: InnAuthenticationType,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct InnEmiOptions {
    pub available: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct InnRecurringOptions {
    pub available: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Inn {
    pub inn: String,
    pub entity: InnEntity,
    pub network: CardNetwork,
    #[serde(rename = "type")]
    pub type_: CardTypeExtended,
    pub sub_type: CardSubType,
    pub international: bool,
    pub issuer_code: String,
    pub issuer_name: String,
    pub emi: InnEmiOptions,
    pub recurring: InnRecurringOptions,
    pub authentication_types: Vec<InnAuthenticationTypeOptions>,
}

impl Inn {
    pub async fn fetch(
        razorpay: &Razorpay,
        inn_id: &str,
    ) -> RazorpayResult<Inn> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/inns/{}", inn_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(inn) => Ok(inn),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }
}
