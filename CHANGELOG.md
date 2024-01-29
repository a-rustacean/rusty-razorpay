# Changelog

## v0.2.2

- update `Subscription` struct schema to match razorpay API
- fix typos
- make entity types un-constructable without parsing

**Full Changelog**: [v0.2.1...v0.2.2](https://github.com/a-rustacean/rusty-razorpay/compare/v0.2.1...v0.2.2)

## v0.2.1

- use `Option<Object>` instead of `Object` for notes
- derive and implement `Default` trait for common traits with many fields
- add a utility `new` method on `CreateSubscription`
- lower version requirements for `smol_str`
- rename function arguments

**Full Changelog**: [v0.2.0...v0.2.1](https://github.com/a-rustacean/rusty-razorpay/compare/v0.2.0...v0.2.1)

## v0.2.0

- add typed IDs:
  `OrderId`, `CustomerId`, `SubscriptionId`, ... instead of just `String`
- add zero-sized structs for each entity type: each entity has a field called `entity` which can have only a single possible value, for example, the `Order` entity can only have `"order"` as the value of its `entity` field, so no allocation is needed, we only need to check it once when creating (deserializing).
- use borrowed values in temporary structs:
  structs like  `CreateOrder` are not meant to be stored somewhere, they are created very often like on every request, so it is reasonable to use borrowed values instead of owned values for these structs where possible
- use optional filters when listing all items
- improve method scopes and names

**Full Changelog**: [v0.1.0...v0.2.0](https://github.com/a-rustacean/rusty-razorpay/compare/v0.1.0...v0.2.0)

## v0.1.0

**Full Changelog**: [v0.1.0](https://github.com/a-rustacean/rusty-razorpay/commits/v0.1.0)
