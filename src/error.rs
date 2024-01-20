use crate::{
    common::Object,
    util::{debug_option, display_option},
};
use serde::Deserialize;
use std::{error::Error, fmt::Display};

#[derive(Debug, Deserialize, Clone)]
pub struct ApiError {
    pub code: String,
    pub description: String,
    pub source: Option<String>,
    pub step: Option<String>,
    pub reason: Option<String>,
    pub metadata: Option<Object>,
    pub field: Option<String>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Razorpay Error: {}: {}\n\nsource: {}\nstep: {}\nreason: {}\nfield: {}\nmetadata: {}",
            self.code, self.description, display_option(self.source.as_ref()),
            display_option(self.step.as_ref()), display_option(self.reason.as_ref()),
            display_option(self.field.as_ref()), debug_option(self.metadata.as_ref()))
    }
}

impl Error for ApiError {}

#[derive(Debug)]
pub enum RazorpayError {
    ApiError(ApiError),
    ReqwestError(reqwest::Error),
    SerializationError(serde_json::Error),
}

impl Display for RazorpayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RazorpayError::ApiError(error) => write!(f, "API Error: {}", error),
            RazorpayError::ReqwestError(error) => {
                write!(f, "Reqwest Error: {}", error)
            }
            RazorpayError::SerializationError(error) => {
                write!(f, "Serialization Error: {}", error)
            }
        }
    }
}

impl From<reqwest::Error> for RazorpayError {
    fn from(error: reqwest::Error) -> Self {
        RazorpayError::ReqwestError(error)
    }
}

impl From<ApiError> for RazorpayError {
    fn from(error: ApiError) -> Self {
        RazorpayError::ApiError(error)
    }
}

impl From<serde_json::Error> for RazorpayError {
    fn from(error: serde_json::Error) -> Self {
        RazorpayError::SerializationError(error)
    }
}

impl Error for RazorpayError {}

pub type RazorpayResult<T> = Result<T, RazorpayError>;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub(crate) enum InternalApiResult<T> {
    Ok(T),
    Err { error: ApiError },
}
