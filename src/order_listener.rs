use crate::types::{Price, Quantity};
use crate::order::Order;

/// Generic listener of order events. Implement to build a full order book feed.
pub trait OrderListener<O: Order> {
    fn on_accept(&self, order: &O);
    fn on_trigger_stop(&self, order: &O) {}
    fn on_reject(&self, order: &O, reason: &str);
    fn on_fill(&self, order: &O, matched_order: &O, fill_qty: Quantity, fill_price: Price);
    fn on_cancel(&self, order: &O);
    fn on_cancel_reject(&self, order: &O, reason: &str);
    fn on_replace(&self, order: &O, size_delta: i64, new_price: Price);
    fn on_replace_reject(&self, order: &O, reason: &str);
}