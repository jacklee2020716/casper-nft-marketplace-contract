#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

#[macro_use]
extern crate alloc;
use contract_utils::{ContractContext, OnChainContractStorage, ReentrancyGuard};
use marketplace::Marketplace;

#[derive(Default)]
struct MarketplaceContract(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for MarketplaceContract {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl Marketplace<OnChainContractStorage> for MarketplaceContract {}

#[no_mangle]
pub extern "C" fn call() {}
