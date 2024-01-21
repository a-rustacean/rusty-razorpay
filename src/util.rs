use crate::common::Object;
use data_encoding::HEXLOWER;
use ring::hmac;
use serde::{Deserialize, Deserializer, Serializer};
use std::fmt::{Debug, Display};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ObjectOrEmptyArray {
    Array([u8; 0]),
    Object(Object),
}

pub(crate) fn serialize_bool_as_int_option<S>(
    val: &Option<bool>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match val {
        Some(val) => serializer.serialize_u8(if *val { 1 } else { 0 }),
        None => serializer.serialize_none(),
    }
}

pub(crate) fn deserialize_notes<'a, D>(
    deserializer: D,
) -> Result<Object, D::Error>
where
    D: Deserializer<'a>,
{
    let val: ObjectOrEmptyArray = Deserialize::deserialize(deserializer)?;

    Ok(match val {
        ObjectOrEmptyArray::Array(_) => Object::new(),
        ObjectOrEmptyArray::Object(obj) => obj,
    })
}

pub(crate) fn display_option<T>(option: Option<&T>) -> String
where
    T: Display,
{
    match option {
        Some(value) => format!("{}", value),
        None => "none".to_owned(),
    }
}

pub(crate) fn debug_option<T>(option: Option<&T>) -> String
where
    T: Debug,
{
    match option {
        Some(value) => format!("{:#?}", value),
        None => "none".to_owned(),
    }
}

pub fn generate_webhook_signature<T, U>(body: T, secret: U) -> String
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    let body = body.as_ref();
    let secret = secret.as_ref();

    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let expected_signature = hmac::sign(&key, body.as_bytes());
    HEXLOWER.encode(expected_signature.as_ref())
}
