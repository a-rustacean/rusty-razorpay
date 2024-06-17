//! Update Order
//! ============
//!
//! Reference: <https://razorpay.com/docs/api/orders/update>
//!
//! This example shows how to update an order with order id.

use std::{env, str::FromStr};

use rusty_razorpay::{obj, order::Order, OrderId, Razorpay};

#[tokio::main]
async fn main() {
    let key_secret = env::var("RAZORPAY_KEY_SECRET")
        .expect("Missing RAZORPAY_KEY_SECRET in env");
    let key_id =
        env::var("RAZORPAY_KEY_ID").expect("Missing RAZORPAY_KEY_ID in env");
    let order_id = env::args().nth(1).expect("Missing order id");
    let order_id = OrderId::from_str(&order_id).expect("Invalid order id");
    let razorpay = Razorpay::new(key_id, key_secret);

    let notes = obj! {
        "updated": "true"
    };

    let order = Order::update(&razorpay, &order_id, notes).await.unwrap();

    println!("order updated: {:#?}", order);
}
