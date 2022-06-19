use crate::{
    data::Orders,
    libs::{Asset, Order, OrderRouteType, OrderStatus, OrderType},
    Time,
};
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use casper_contract::contract_api::runtime;
use casper_types::{account::AccountHash, ContractHash, U256};
use contract_utils::{ContractContext, ContractStorage};
pub trait Marketplace<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self) {
        Orders::init();
    }

    fn create_order(
        &mut self,
        offerer: AccountHash,
        offer: Asset,
        start_time: Time,
        end_time: Time,
        order_route_type: OrderRouteType,
        bids: BTreeMap<AccountHash, Asset>,
    ) {
        self._check_offer_is_acceptable(offer, order_route_type);
        // TODO Check allowance
        let order: Order = Order {
            offerer,
            offers,
            bids,
            order_route_type,
            start_time,
            end_time,
            status: OrderStatus::Pending,
        };
        Orders::instance().set(offerer, start_time, order);
    }

    fn create_bid(
        &mut self,
        offerer: AccountHash,
        start_time: Time,
        bidder: AccountHash,
        bid_item: Asset,
    ) {
        // TODO Read order
        let mut order = Orders::instance().get(offerer, start_time);
        // TODO check bid is acceptable // only nft auction
        order.bids.insert(bidder, bid_item);
    }

    fn fullfill_order(&mut self, offerer: AccountHash, start_time: Time) {
        // only nft sell buyer call|| offer
    }

    fn _check_offer_is_acceptable(&self, offers: Vec<Asset>, order_route_type: OrderRouteType) {}
}
