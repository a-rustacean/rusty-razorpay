use crate::ids::OfferId;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Offer {
    pub id: OfferId,
}
