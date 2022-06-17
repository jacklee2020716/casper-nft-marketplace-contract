# NFT Marketplace Documentation

Documentation around creating orders, fulfillment, and interacting with NFT Marketplace.

## Table of Contents

- [Order](#order)
- [Order Fulfillment](#order-fulfillment)
- [Sequence of events](#sequence-of-events)
- [Known limitations and Workarounds](#known-limitations-and-workarounds)

## Order

Each order contains key elements.

- The `offerer` of order supplies all offered items and must either fulfill the order personally
- The `offer` contains array of items that may be transferred from the offerer`s account, where each item consists fo the following components.
  - The `item_type` designates the type of item, with valid types being CSPR, ERC20, CEP47.
  - The `token` designates the account of the item`s token contract (with the None used for CSPR)
  - The `identifier_or_criteria` represents CEP47 token identifier or???
  - The `start_amount` represents the amount of the item in question that will be required should the order be fulfilled at the moment of the order becomes active.
  - The `end_amount` represents the amount of the item in question that will be required should the order be fulfilled at the moment the order expires. If this value differs from the item's `start_amount`, the realized amount is calculated linearly based on the time elapsed since the order becomes active.
  - The `start_time` indicates the block timestamp at which the order becomes active.
  - The `end_time` indicates the block timestamp at which the order expires. This value and the `start_time` are used in conjunction with the `start_amount` and `end_amount` of each item to derive their current amount.

## Order Fulfillment

## Sequence of events

## Known limitations and Workarounds
