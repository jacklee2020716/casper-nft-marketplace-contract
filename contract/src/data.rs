use casper_types::U256;
use contract_utils::{key_and_value_to_str, Dict};

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

    pub fn get(&self, account: &Key, schedule_time: Time) -> Option<bool> {
        self.dict
            .get(&key_and_value_to_str(account, &schedule_time))
    }

    pub fn set(&self, account: &Key, schedule_time: Time, claimed: bool) {
        self.dict
            .set(&key_and_value_to_str(account, &schedule_time), claimed);
    }

    pub fn remove(&self, account: &Key, schedule_time: Time) {
        self.dict
            .remove::<U256>(&key_and_value_to_str(account, &schedule_time));
    }
}
