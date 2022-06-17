use alloc::{collections::BTreeMap, vec::Vec};
use casper_types::{account::AccountHash, ContractHash, U256};

use crate::Time;

use super::consideration_enums::{Asset, OrderRouteType, OrderStatus, OrderType};

/// An order contains seven components: an `offerer`, a `zone` (or account that
/// can cancel the order or restrict who can fulfill the order depending on
/// the type), the `order_type` (specifying partial fill support as well as
/// restricted order status), the `start and end time`, a hash that will be
/// provided to the zone when validating restricted orders, a salt, a key
/// corresponding to a given conduit, a counter, and an arbitrary number of
/// offer items that can be spent along with consideration items that must
/// be received by their respective recipient.
pub struct Order {
    pub offerer: AccountHash,
    pub offers: Vec<Asset>,
    pub considerations: BTreeMap<AccountHash, Asset>,
    pub order_route_type: OrderRouteType,
    pub start_time: Time,
    pub end_time: Time,
    pub status: OrderStatus,
}
