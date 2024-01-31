#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod api;
mod entity;
mod ids;
mod razorpay;

pub mod account;
pub mod addon;
pub mod address;
pub mod card;
pub mod common;
pub mod customer;
pub mod dispute;
pub mod document;
pub mod error;
pub mod iin;
pub mod invoice;
pub mod item;
pub mod line_item;
pub mod offer;
pub mod order;
pub mod payment;
pub mod plan;
pub mod refund;
pub mod settlement;
pub mod subscription;
pub mod util;
pub mod webhook;

pub use common::{Collection, Country, Currency, Filter, Object};
pub use ids::*;

pub use razorpay::{Razorpay, VERSION};
