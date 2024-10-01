use crate::types::{Price, Quantity};
/// Generic listener of order events. Implement to build a full order book feed.
pub trait OrderListener<Order, Quantity, Price> {
    /// Callback for an order accept
    fn on_accept(&self, order: &Order);

    /// Callback for triggered STOP order
    fn on_trigger_stop(&self, _order: &Order) {}

    /// Callback for an order reject
    fn on_reject(&self, order: &Order, reason: &str);

    /// Callback for an order fill
    fn on_fill(&self, order: &Order, matched_order: &Order, fill_qty: Quantity, fill_price: Price);

    /// Callback for an order cancellation
    fn on_cancel(&self, order: &Order);

    /// Callback for an order cancel rejection
    fn on_cancel_reject(&self, order: &Order, reason: &str);

    /// Callback for an order replace
    fn on_replace(&self, order: &Order, size_delta: i64, new_price: Price);

    /// Callback for an order replace rejection
    fn on_replace_reject(&self, order: &Order, reason: &str);
}