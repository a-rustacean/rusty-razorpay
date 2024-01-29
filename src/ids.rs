// Stuff related to Ids of entities, macros and some other items are
// inspired from [async-stripe]
//
// [async-stripe]: https://docs.rs/async-stripe/0.31.0/src/stripe/ids.rs.html#3-465

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use core::{
    cmp::{Ordering, PartialOrd},
    fmt::{Display, Formatter, Result as FormatterResult},
    ops::Deref,
    str::FromStr,
};
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
#[cfg(feature = "std")]
use std::{
    cmp::{Ordering, PartialOrd},
    fmt::{Display, Formatter, Result as FormatterResult},
    ops::Deref,
    str::FromStr,
};

macro_rules! def_id_serde_impls {
    ($struct_name:ident) => {
        impl Serialize for $struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                self.as_str().serialize(serializer)
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
    };
}

macro_rules! def_id {
    ($struct_name:ident, $prefix:literal) => {
        /// An id for the corresponding object type.
        ///
        /// This type _typically_ will not allocate and
        /// therefore is usually cheaply clonable.
        #[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
        pub struct $struct_name(smol_str::SmolStr);

        impl $struct_name {
            /// The prefix of the id type (e.g. `cust_` for a `CustomerId`).
            #[inline(always)]
            pub fn prefix() -> &'static str {
                $prefix
            }

            /// Extracts a string slice containing the entire id.
            #[inline(always)]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl PartialEq<str> for $struct_name {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }

        impl PartialEq<&str> for $struct_name {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        impl PartialEq<String> for $struct_name {
            fn eq(&self, other: &String) -> bool {
                self.as_str() == other
            }
        }

        impl PartialOrd for $struct_name {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $struct_name {
            fn cmp(&self, other: &Self) -> Ordering {
                self.as_str().cmp(other.as_str())
            }
        }

        impl AsRef<str> for $struct_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl Deref for $struct_name {
            type Target = str;

            fn deref(&self) -> &str {
                self.as_str()
            }
        }

        impl Display for $struct_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FormatterResult {
                self.0.fmt(f)
            }
        }

        impl FromStr for $struct_name {
            type Err = ParseIdError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if !s.starts_with($prefix) {
                    Err(ParseIdError {
                        typename: stringify!($struct_name),
                        expected: stringify!(id to start with $prefix),
                    })
                } else {
                    Ok($struct_name(s.into()))
                }
            }
        }

        def_id_serde_impls!($struct_name);
    };
}

#[derive(Debug, Clone)]
pub struct ParseIdError {
    typename: &'static str,
    expected: &'static str,
}

impl Display for ParseIdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatterResult {
        write!(f, "invalid `{}`, expected {}", self.typename, self.expected)
    }
}

def_id!(CardId, "card_");
def_id!(ItemId, "item_");
def_id!(PlanId, "plan_");
def_id!(AddonId, "ao_");
def_id!(OrderId, "order_");
def_id!(OfferId, "offer_");
def_id!(BatchId, "batch_");
def_id!(RefundId, "rfnd_");
def_id!(AccountId, "acc_");
def_id!(AddressId, "addr_");
def_id!(DisputeId, "disp_");
def_id!(InvoiceId, "inv_");
def_id!(PaymentId, "pay_");
def_id!(CustomerId, "cust_");
def_id!(DowntimeId, "down_");
def_id!(DocumentId, "doc_");
def_id!(TransferId, "trf_");
def_id!(LineItemId, "li_");
def_id!(AdjustmentId, "adj_");
def_id!(SettlementId, "setl_");
def_id!(SubscriptionId, "sub_");
def_id!(InstantSettlementId, "setlod_");
def_id!(InstantSettlementPayoutId, "setlodp_");
