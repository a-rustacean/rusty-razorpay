//! Fetch All Order
//! ============
//!
//! Reference: <https://razorpay.com/docs/api/orders/fetch-all>
//!
//! This example shows how to fetch all orders.

use std::env;

use rusty_razorpay::{
    order::{ListOrders, Order, OrderExpand},
    Filter, Razorpay,
};

#[tokio::main]
async fn main() {
    let key_secret = env::var("RAZORPAY_KEY_SECRET")
        .expect("Missing RAZORPAY_KEY_SECRET in env");
    let key_id =
        env::var("RAZORPAY_KEY_ID").expect("Missing RAZORPAY_KEY_ID in env");
    let razorpay = Razorpay::new(key_id, key_secret);

    let orders = Order::list(&razorpay, None).await.unwrap();

    println!("{} orders found!", orders.count);
    for order in orders.items {
        println!("order: {:#?}", order);
    }

    let orders = Order::list(
        &razorpay,
        Some(ListOrders {
            expand: &[OrderExpand::Payments, OrderExpand::PaymentsCard],
            filter: Some(Filter {
                count: Some(10),
                ..Default::default()
            }),
            authorized: Some(true),
            ..Default::default()
        }),
    )
    .await
    .unwrap();

    println!("{} orders found!", orders.count);
    for order in orders.items {
        println!("order: {:#?}", order);
    }
}
