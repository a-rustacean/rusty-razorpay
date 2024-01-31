# Rusty Razorpay

Rusty Razorpay is an unofficial Rust SDK for Razorpay, currently a work in progress. Please note that this library is in the early stages of development, and nothing has been tested yet. The API and features are subject to change without prior notice.

## Installation

To use Rusty Razorpay in your project, add the following line to your `Cargo.toml` file:

```toml
rusty-razorpay = "0.2.5"
```

## Progress

- [x] Account
  - [x] Types
  - [x] APIs
    - [x] Create
    - [x] Fetch
    - [x] Update
    - [x] Delete
- [x] Addon
  - [x] Types
  - [x] APIs
    - [x] Create
    - [x] Fetch
    - [x] Delete
    - [x] List
- [x] Address
  - [x] Types
- [ ] Card
  - [x] Types
  - [ ] APIs
    - [x] Fetch
    - [ ] Fingerprint
- [x] Customer
  - [x] Types
  - [x] APIs
    - [x] Create
    - [x] Fetch
    - [x] Update
    - [x] List
- [ ] Fund account
  - [ ] Types
  - [ ] APIs
    - [ ] Create
    - [ ] Fetch
- [x] Dispute
  - [x] Types
  - [x] APIs
    - [x] Fetch
    - [x] Accept
    - [x] Contest
    - [x] List
- [ ] Document
  - [x] Types
  - [ ] APIs
    - [x] Fetch
    - [ ] Create
    - [ ] Fetch content
- [x] IIN
  - [x] Types
  - [x] APIs
    - [x] Fetch
- [x] Invoice
  - [x] Types
  - [x] APIs
    - [x] Create
    - [x] Fetch
    - [x] Update
    - [x] Issue
    - [x] Delete
    - [x] Cancel
    - [x] List
    - [x] Notify
- [x] Item
  - [x] Types
  - [x] APIs
    - [x] Create
    - [x] Fetch
    - [x] Update
    - [x] Delete
    - [x] List
- [x] Line items
  - [x] Types
- [x] Offer
  - [x] Types
- [x] Order
  - [x] Types
  - [x] APIs
    - [x] Create
    - [x] Fetch
    - [x] Update
    - [x] List
    - [x] List payments
- [x] Payment
  - [x] Types
  - [x] APIs
    - [x] Capture
    - [x] Fetch
    - [x] Fetch card
    - [x] Fetch refund
    - [x] Update
    - [x] Refund
    - [x] List
    - [x] List refunds
- [x] Payment downtime
  - [x] Types
  - [x] APIs
    - [x] Fetch
    - [x] List
- [ ] Payment link
  - [ ] Types
  - [ ] APIs
    - [ ] Create
    - [ ] Fetch
    - [ ] Update
    - [ ] Notify
    - [ ] Cancel
    - [ ] List
- [x] Plan
  - [x] Types
  - [x] APIs
    - [x] Create
    - [x] Fetch
    - [x] List
- [ ] Product
  - [ ] Types
  - [ ] APIs
    - [ ] Create
    - [ ] Fetch
    - [ ] Fetch T & C
    - [ ] Update
- [ ] QR Code
  - [ ] Types
  - [ ] APIs
    - [ ] Create
    - [ ] Fetch
    - [ ] Close
    - [ ] List
    - [ ] List payments
- [x] Refund
  - [x] Types
  - [x] APIs
    - [x] Fetch
    - [x] Update
    - [x] List
- [x] Settlement
  - [x] Types
  - [x] APIs
    - [x] Fetch
    - [x] Fetch recon
    - [x] List
- [x] Instant settlement
  - [x] Types
  - [x] APIs
    - [x] Create
    - [x] Fetch
    - [x] List
- [ ] Stakeholders
  - [ ] Types
  - [ ] APIs
    - [ ] Create
    - [ ] Fetch
    - [ ] Fetch documents
    - [ ] Update
    - [ ] Update documents
    - [ ] List
- [x] Subscription
  - [x] Types
  - [x] APIs
    - [x] Create
    - [x] Fetch
    - [x] Fetch pending update
    - [x] Update
    - [x] Cancel
    - [x] Cancel scheduled update
    - [x] Pause
    - [x] Resume
    - [x] List
- [ ] Token
  - [ ] Types
  - [ ] APIs
    - [ ] Create
    - [ ] Fetch
    - [ ] Delete
    - [ ] Process payment on alternate PA or PG
- [ ] Transfer
  - [ ] Types
  - [ ] APIs
    - [ ] Create
    - [ ] Fetch
    - [ ] Fetch settlement
    - [ ] Update
    - [ ] Reverse
    - [ ] List
- [ ] Virtual account
  - [ ] Types
  - [ ] APIs
    - [ ] Create
    - [ ] Fetch
    - [ ] Fetch payments
    - [ ] Add receiver
    - [ ] Add allowed payer
    - [ ] Delete allowed payer
    - [ ] Close
    - [ ] List
- [x] Webhook
  - [x] Types
  - [ ] APIs
    - [ ] Create
    - [ ] Fetch
    - [ ] Update
    - [ ] Delete
    - [ ] List
  - [x] Utility methods
    - [x] Construct event

## Important Notice

This library is currently under active development, and no testing has been performed yet. Therefore, it is not recommended for use in a production environment at this time. Additionally, be aware that the API may undergo significant changes as development progresses.

## License

This project is licensed under the [MIT License](LICENSE).

## Contributions

Contributions are welcome! Feel free to open issues, submit pull requests, or provide feedback to help improve Rusty Razorpay. Before contributing, please review the [Contribution Guidelines](CONTRIBUTING.md).
