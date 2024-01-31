# Rusty Razorpay

Rusty Razorpay is an unofficial Rust SDK for Razorpay, currently a work in progress. Please note that this library is in the early stages of development, and nothing has been tested yet. The API and features are subject to change without prior notice.

## Installation

To use Rusty Razorpay in your project, add the following line to your `Cargo.toml` file:

```toml
rusty-razorpay = "0.2.5"
```

## Progress

- [ ] Account
  - [x] Create
  - [x] Fetch
  - [x] Delete
  - [ ] Update
- [x] Addon
- [x] Address
- [ ] Card
  - [x] Fetch
  - [ ] Fingerprint
- [x] Customer
- [ ] Fund account
- [x] Dispute
- [ ] Document
  - [x] Fetch
  - [ ] Create
  - [ ] Fetch content
- [ ] IINS
- [ ] Invoice
  - [x] Create
  - [ ] Update
  - [ ] Issue
  - [ ] Delete
  - [ ] Cancel
  - [ ] Fetch
  - [ ] List
  - [ ] Send notification
- [x] Item
- [x] Line items
- [x] Offer
- [x] Order
- [x] Payment
- [x] Payment downtime
- [ ] Payment link
- [x] Plan
- [ ] Product
- [ ] QR Code
- [x] Refund
- [x] Settlement
- [x] Instant settlement
- [ ] Stakeholders
- [x] Subscription
- [ ] Token
- [ ] Transfer
- [ ] Virtual account
- [x] Webhook

## Important Notice

This library is currently under active development, and no testing has been performed yet. Therefore, it is not recommended for use in a production environment at this time. Additionally, be aware that the API may undergo significant changes as development progresses.

## License

This project is licensed under the [MIT License](LICENSE).

## Contributions

Contributions are welcome! Feel free to open issues, submit pull requests, or provide feedback to help improve Rusty Razorpay. Before contributing, please review the [Contribution Guidelines](CONTRIBUTING.md).
