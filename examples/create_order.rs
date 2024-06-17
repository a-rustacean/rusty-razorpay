//! Create Order
//! ============
//!
//! Reference: <https://razorpay.com/docs/api/orders/create>
//!
//! This example shows how to create an order.

use std::env;

use rusty_razorpay::{
    obj,
    order::{CreateOrder, Order},
    Currency, Razorpay,
};

#[tokio::main]
async fn main() {
    let key_secret = env::var("RAZORPAY_KEY_SECRET")
        .expect("Missing RAZORPAY_KEY_SECRET in env");
    let key_id =
        env::var("RAZORPAY_KEY_ID").expect("Missing RAZORPAY_KEY_ID in env");
    let razorpay = Razorpay::new(key_id, key_secret);

    let notes = obj! {
        "name": "John Doe",
        "username": "johndoe",
    };

    let order = Order::create(
        &razorpay,
        CreateOrder {
            amount: 199, // ₹ 1.99
            currency: Currency::INR,
            receipt: Some("receipt#10002"),
            notes: Some(notes),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    println!("created a order: {:#?}", order);
}
