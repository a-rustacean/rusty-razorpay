use crate::{
    api::RequestParams,
    error::{InternalApiResult, RazorpayResult},
    ids::DocumentId,
    Razorpay,
};
#[cfg(not(feature = "std"))]
use alloc::{format, string::String};
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DocumentPurpose {
    DisputeEvidence,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum DocumentMimeType {
    #[serde(rename = "image/jpg")]
    ImageJpg,
    #[serde(rename = "image/jpeg")]
    ImageJpeg,
    #[serde(rename = "image/png")]
    ImagePng,
    #[serde(rename = "application/pdf")]
    ApplicationPdf,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(tag = "entity", rename = "document")]
pub struct Document {
    pub id: DocumentId,
    pub purpose: DocumentPurpose,
    pub name: String,
    pub size: u64,
    pub mime_type: DocumentMimeType,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Document {
    // TODO: add api for creating document
    //
    // This time it's not the [docs]' fault it's just that I hate the multipart
    // thing so left it for the end
    //
    // [docs]: https://razorpay.com/docs/api/documents/create

    pub async fn fetch(
        razorpay: &Razorpay,
        document_id: &DocumentId,
    ) -> RazorpayResult<Document> {
        let res = razorpay
            .api
            .get(RequestParams {
                url: format!("/documents/{}", document_id),
                version: None,
                data: None::<()>,
            })
            .await?;

        match res {
            InternalApiResult::Ok(document) => Ok(document),
            InternalApiResult::Err { error } => Err(error.into()),
        }
    }

    // TODO: add api for fetching contents of a document
    //
    // It will take more time as the contents of the document would be
    // some kinda binary and we may have to update the current api to
    // add a method to allow us to get the raw body
    //
    // [docs]: https://razorpay.com/docs/api/documents/fetch-content
}
