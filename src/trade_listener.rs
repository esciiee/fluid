use crate::types::{Quantity, Price};
use crate::order::Order;
use crate::order_book::OrderBook;

pub trait TradeListener<O: Order + Clone> {
    /// Callback for a trade
    /// 
    /// # Parameters
    /// * `book`: The order book of the fill (not defined whether this is before or after fill)
    /// * `qty`: The quantity of this fill
    /// * `price`: The price of this fill
    fn on_trade(&self, book: &OrderBook<O>, qty: Quantity, price: Price);
}