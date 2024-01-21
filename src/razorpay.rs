use crate::api::{Api, ApiOptions};
use reqwest::header::HeaderMap;

const BASE_URL: &str = "https://api.razorpay.com";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Razorpay {
    pub api: Api,
}

pub struct RazorpayOptions {
    pub key_id: String,
    pub key_secret: String,
    pub headers: Option<HeaderMap>,
}

impl Razorpay {
    pub fn new(options: RazorpayOptions) -> Self {
        Self {
            api: Api::new(ApiOptions {
                base_url: BASE_URL.to_owned(),
                user_agent: format!("rusty-razorpay@{}", VERSION),
                key_id: options.key_id,
                key_secret: options.key_secret,
                headers: options.headers.unwrap_or_default(),
            }),
        }
    }
}
