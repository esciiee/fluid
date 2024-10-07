use std::collections::{BTreeMap, HashMap};
use linked_hash_map::LinkedHashMap;

use crate::{order::Order, OrderId, Amount, Price};

pub struct OrderBook {
    pub asks: BTreeMap<Price, Level>,
    pub bids: BTreeMap<Price, Level>,
    pub order_id_to_price: HashMap<OrderId, Price>,
    pub price: Price,
    // store historic trades?
}

pub struct Level {
    pub orders: LinkedHashMap<OrderId, Order>,
    pub amount: Amount,
    pub price: Price,
}