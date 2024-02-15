use serde::Deserialize;

use crate::ids::OfferId;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Offer {
    pub id: OfferId,
}
