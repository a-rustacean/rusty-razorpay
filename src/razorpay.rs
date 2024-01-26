use crate::api::{Api, ApiOptions};

const BASE_URL: &str = "https://api.razorpay.com";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Razorpay {
    pub(crate) api: Api,
}

impl Razorpay {
    pub fn new(
        key_id: impl Into<String>,
        key_secret: impl Into<String>,
    ) -> Self {
        Self {
            api: Api::new(ApiOptions {
                base_url: BASE_URL.to_owned(),
                user_agent: format!("rusty-razorpay@{}", VERSION),
                key_id: key_id.into(),
                key_secret: key_secret.into(),
            }),
        }
    }
}
