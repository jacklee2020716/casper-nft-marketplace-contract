#![no_std]
#[macro_use]
extern crate alloc;

mod data;
mod event;
mod interfaces;
mod libs;
mod marketplace;

pub type Time = u64;
pub use marketplace::Marketplace;
