#[cfg(not(feature = "std"))]
use alloc::{
    borrow::ToOwned, format, string::String, string::ToString, vec, vec::Vec,
};

use reqwest::{header::HeaderMap, Client, ClientBuilder};
use serde::{de::DeserializeOwned, ser::Error, Serialize};
use serde_json::{to_value, Value};

use crate::error::RazorpayResult;

#[derive(Debug)]
pub struct Api {
    key_id: String,
    key_secret: String,
    base_url: String,
    client: Client,
    version: String,
}

pub struct ApiOptions {
    pub user_agent: String,
    pub base_url: String,
    pub key_id: String,
    pub key_secret: String,
}

pub struct RequestParams<T: Serialize = ()> {
    pub url: String,
    pub version: Option<String>,
    pub data: Option<T>,
}

fn make_serializable<T>(value: &T) -> serde_json::Result<Vec<(String, String)>>
where
    T: Serialize,
{
    let value = to_value(value)?;

    let map = if let Value::Object(map) = value {
        map
    } else {
        return Err(serde_json::Error::custom(
            "top level value should be a map",
        ));
    };

    let mut records = Vec::new();

    for (key, value) in map {
        let value = match value {
            Value::Bool(b) => vec![(key, b.to_string())],
            Value::String(s) => vec![(key, s)],
            Value::Number(n) => vec![(key, n.to_string())],
            Value::Array(vec) => {
                let mut res = Vec::new();
                for value in vec {
                    let value = match value {
                        Value::Bool(b) => b.to_string(),
                        Value::String(s) => s,
                        Value::Number(n) => n.to_string(),
                        _ => {
                            return Err(serde_json::Error::custom(
                                "Unsupported value in vec, cannot serialize \
                                 nested map and vec",
                            ))
                        }
                    };
                    res.push((key.clone(), value));
                }
                res
            }
            _ => {
                return Err(serde_json::Error::custom(
                    "Unsupported value in map, cannot serialize nested map",
                ));
            }
        };

        records.extend(value);
    }

    Ok(records)
}

impl Api {
    pub fn new(options: ApiOptions) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers
            .insert("User-Agent", options.user_agent.parse().unwrap());
        Self {
            key_id: options.key_id,
            key_secret: options.key_secret,
            base_url: options.base_url,
            client: ClientBuilder::new()
                .default_headers(default_headers)
                .build()
                .unwrap(),
            version: "v1".to_owned(),
        }
    }

    pub fn get_entity_url<T: Serialize>(
        &self,
        params: &RequestParams<T>,
    ) -> String {
        format!(
            "{}/{}{}",
            self.base_url,
            params.version.as_ref().unwrap_or(&self.version),
            params.url
        )
    }

    pub async fn get<T, R>(&self, params: RequestParams<T>) -> RazorpayResult<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let res = self
            .client
            .get(self.get_entity_url(&params))
            .basic_auth(&self.key_id, Some(&self.key_secret));

        let res = if let Some(data) = params.data {
            res.query(&make_serializable(&data)?)
        } else {
            res
        };

        Ok(res.send().await?.error_for_status()?.json().await?)
    }

    pub async fn post<T, R>(
        &self,
        params: RequestParams<T>,
    ) -> reqwest::Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let res = self
            .client
            .post(self.get_entity_url(&params))
            .basic_auth(&self.key_id, Some(&self.key_secret));

        let res = if let Some(data) = params.data {
            res.json(&data)
        } else {
            res
        };

        res.send().await?.error_for_status()?.json().await
    }

    pub async fn put<T, R>(
        &self,
        params: RequestParams<T>,
    ) -> reqwest::Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let res = self
            .client
            .put(self.get_entity_url(&params))
            .basic_auth(&self.key_id, Some(&self.key_secret));

        let res = if let Some(data) = params.data {
            res.json(&data)
        } else {
            res
        };

        res.send().await?.error_for_status()?.json().await
    }

    pub async fn patch<T, R>(
        &self,
        params: RequestParams<T>,
    ) -> reqwest::Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let res = self
            .client
            .patch(self.get_entity_url(&params))
            .basic_auth(&self.key_id, Some(&self.key_secret));

        let res = if let Some(data) = params.data {
            res.json(&data)
        } else {
            res
        };

        res.send().await?.error_for_status()?.json().await
    }

    pub async fn delete<T, R>(
        &self,
        params: RequestParams<T>,
    ) -> reqwest::Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.client
            .delete(self.get_entity_url(&params))
            .basic_auth(&self.key_id, Some(&self.key_secret))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    #[allow(dead_code)]
    pub async fn post_form_data<T, R>(
        &self,
        params: RequestParams<T>,
    ) -> reqwest::Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let res = self
            .client
            .post(self.get_entity_url(&params))
            .basic_auth(&self.key_id, Some(&self.key_secret));

        let res = if let Some(data) = params.data {
            res.form(&data)
        } else {
            res
        };

        res.send().await?.error_for_status()?.json().await
    }
}
