use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
use std::{error::Error, fmt::Display, ops::Deref, str::FromStr};

macro_rules! def_entity {
    ($struct_name:ident, $value:literal) => {
        #[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
        pub struct $struct_name([u8; 0]);

        impl Serialize for $struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                $value.serialize(serializer)
            }
        }

        impl<'de> Deserialize<'de> for $struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s: String = Deserialize::deserialize(deserializer)?;
                s.parse::<Self>().map_err(::serde::de::Error::custom)
            }
        }

        impl Deref for $struct_name {
            type Target = str;

            fn deref(&self) -> &str {
                $value
            }
        }

        impl Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $value)
            }
        }

        impl AsRef<str> for $struct_name {
            fn as_ref(&self) -> &str {
                $value
            }
        }

        impl FromStr for $struct_name {
            type Err = EntityMismatch;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s != $value {
                    return Err(EntityMismatch {
                        typename: stringify!($struct_name),
                        expected: stringify!(entity to be $val),
                    });
                }
                Ok(Self([]))
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct EntityMismatch {
    typename: &'static str,
    expected: &'static str,
}

impl Display for EntityMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid `{}`, expected {}", self.typename, self.expected)
    }
}

impl Error for EntityMismatch {
    fn description(&self) -> &str {
        "entity type mismatch"
    }
}

def_entity!(CardEntity, "card");
def_entity!(PlanEntity, "plan");
def_entity!(OrderEntity, "order");
def_entity!(AddonEntity, "addon");
def_entity!(RefundEntity, "refund");
def_entity!(WebhookEntity, "event");
def_entity!(DisputeEntity, "dispute");
def_entity!(InvoiceEntity, "invoice");
def_entity!(PaymentEntity, "payment");
def_entity!(DocumentEntity, "document");
def_entity!(DowntimeEntity, "payment.downtime");
def_entity!(CollectionEntity, "collection");
def_entity!(SettlementEntity, "settlement");
def_entity!(SubscriptionEntity, "subscription");
def_entity!(InstantSettlementEntity, "settlement.ondemand");
def_entity!(InstantSettlementPayoutEntity, "settlement.ondemand_payout");
