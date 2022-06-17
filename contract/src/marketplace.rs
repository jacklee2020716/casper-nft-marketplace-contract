use crate::{
    libs::{Asset, Order, OrderRouteType, OrderStatus, OrderType},
    Time,
};
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use casper_contract::contract_api::runtime;
use casper_types::{account::AccountHash, ContractHash, U256};
use contract_utils::{ContractContext, ContractStorage};
pub trait Marketplace<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self) {}

    fn create_order(
        &mut self,
        offerer: AccountHash,
        offers: Vec<Asset>,
        start_time: Time,
        order_route_type: OrderRouteType,
    ) {
        let order: Order = Order {
            offerer,
            offers,
            considerations: BTreeMap::new(),
            order_route_type,
            start_time,
            end_time: 0u64,
            status: OrderStatus::Pending,
        };
    }

    fn fill_order(&mut self, order_key: String) {}

    fn fullfill_order(&mut self) {}
}
