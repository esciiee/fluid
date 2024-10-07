use linked_hash_map::LinkedHashMap;
use std::collections::btree_map::OccupiedEntry;
use std::collections::{BTreeMap, HashMap};

use crate::{
    order::{Order, Side},
    Amount, OrderId, OrderIdToPrice, Price, Table,
};

#[derive(Debug)]
pub struct OrderBook {
    pub asks: BTreeMap<Price, Level>,
    pub bids: BTreeMap<Price, Level>,
    pub order_id_to_price: OrderIdToPrice,
    pub price: Price,
    pub id: u64,
    // store historic trades?
}

#[derive(Debug)]
pub struct Level {
    pub orders: LinkedHashMap<OrderId, Order>,
    pub amount: Amount,
    pub price: Price,
}

impl Level {
    pub fn insert_level(order: Order) -> Self {
        let mut orders = LinkedHashMap::new();
        let amount = order.unfilled_amount;
        let price = order.price;
        orders.insert(order.order_id, order);
        Self {
            orders,
            amount,
            price,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    pub fn decr_size(&mut self, amount: &Amount) {
        self.amount -= amount;
    }
}

impl OrderBook {
    pub fn new(init_price: Price) -> Self {
        Self {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
            order_id_to_price: HashMap::new(),
            price: init_price,
            id: 0,
        }
    }

    pub fn incr_then_fetch_order_id(&mut self) -> OrderId {
        self.id += 1;
        self.id
    }

    // pub fn remove_order(&mut self, order_id: OrderId) {
    //     let price = self.order_id_to_price.remove(&order_id).unwrap();
    //     let level = match self.bids.get_mut(&price) {
    //         Some(level) => level,
    //         None => self.asks.get_mut(&price).unwrap(),
    //     };

    //     let order = level.orders.remove(&order_id).unwrap();
    //     level.amount -= order.amount;
    // }

    pub fn place_order(&mut self, order: Order, side: Side) {
        match side {
            Side::Bid => Self::insert_into(&mut self.bids, &mut self.order_id_to_price, order),
            Side::Ask => Self::insert_into(&mut self.asks, &mut self.order_id_to_price, order),
        }
    }

    fn insert_into(table: &mut Table, order_id_to_price: &mut OrderIdToPrice, order: Order) {
        order_id_to_price.insert(order.order_id, order.price);
        table
            .entry(order.price)
            .and_modify(|page| {
                page.amount += order.unfilled_amount;
                page.orders.insert(order.order_id, order.clone());
            })
            .or_insert_with(|| Level::insert_level(order));
    }

    pub fn get_best_if_match(
        &mut self,
        ask_or_bid: Side,
        taker_price: &Price,
    ) -> Option<OccupiedEntry<Price, Level>> {
        match ask_or_bid {
            Side::Bid => self.asks.first_entry().filter(|v| taker_price >= v.key()),
            Side::Ask => self.bids.last_entry().filter(|v| taker_price <= v.key()),
        }
    }

    pub fn get_best_ask(&self) -> Option<Price> {
        self.asks.first_key_value().map(|(price, _)| *price)
    }

    pub fn get_best_bid(&self) -> Option<Price> {
        self.bids.last_key_value().map(|(price, _)| *price)
    }

    fn get_size_from(tape: &Table, price: &Price) -> Option<Amount> {
        tape.get(price).map(|page| page.amount)
    }

    pub fn get_page_size(&self, price: &Price) -> Option<Amount> {
        match (self.get_best_ask(), self.get_best_bid()) {
            (Some(best_ask), Some(_)) => {
                if price >= &best_ask {
                    Self::get_size_from(&self.asks, price)
                } else {
                    Self::get_size_from(&self.bids, price)
                }
            }
            (None, Some(_)) => Self::get_size_from(&self.bids, price),
            (Some(_), None) => Self::get_size_from(&self.asks, price),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{matching_engine::*, order::*, orderbook::*};
    use rust_decimal_macros::dec;

    #[test]
    pub fn test_self_trade_on_best() {
        let mut book = OrderBook::new(dec!(0.1));
        let price = dec!(0.1);
        let amount = dec!(100);
        execute_limit(&mut book, 1, price, amount, Side::Bid);
        let mr = execute_limit(&mut book, 1, price, amount, Side::Ask);
        assert_eq!(2, mr.taker.order_id);
        assert_eq!(mr.taker.state, State::ConditionallyCanceled);
        assert!(mr.maker.is_empty());
        // assert!(book.find_order(1).is_some());
        // assert!(book.find_order(2).is_none());
    }
}
