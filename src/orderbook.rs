use anyhow::Result;
use linked_hash_map::LinkedHashMap;
use std::collections::{BTreeMap, HashMap};

use crate::{
    order::{Order, Side},
    Amount, OrderId, OrderIdToPrice, Price, Table,
};

pub struct OrderBook {
    pub asks: BTreeMap<Price, Level>,
    pub bids: BTreeMap<Price, Level>,
    pub order_id_to_price: OrderIdToPrice,
    pub price: Price,
    // store historic trades?
}

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
}

impl OrderBook {
    pub fn new(init_price: Price) -> Self {
        Self {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
            order_id_to_price: HashMap::new(),
            price: init_price,
        }
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

    pub fn place_order(&mut self, order: Order) -> Result<()> {
        match order.side {
            Side::Buy => Self::insert_into(&mut self.asks, &mut self.order_id_to_price, order),
            Side::Sell => Self::insert_into(&mut self.bids, &mut self.order_id_to_price, order),
        }
        Ok(())
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
}
