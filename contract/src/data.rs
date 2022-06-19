use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{account::AccountHash, U256};
use contract_utils::{key_and_value_to_str, Dict};

use crate::libs::Order;

const ORDERS_DICT: &str = "orders";

pub struct Orders {
    dict: Dict,
}

impl Orders {
    pub fn instance() -> Orders {
        Orders {
            dict: Dict::instance(ORDERS_DICT),
        }
    }

    pub fn init() {
        Dict::init(ORDERS_DICT)
    }

    fn account_hash_and_value_to_str(&self, offerer: AccountHash, start_time: Time) -> String {
        key_and_value_to_str(&Key::from(account), &schedule_time)
    }

    pub fn get(&self, offerer: AccountHash, start_time: Time) -> Order {
        self.dict
            .get(&self.account_hash_and_value_to_str(offerer, start_time))
            .unwrap_or_revert()
    }

    pub fn set(&self, offerer: AccountHash, start_time: Time, order: Order) {
        self.dict.set(
            &self.account_hash_and_value_to_str(offerer, start_time),
            order,
        );
    }
}
