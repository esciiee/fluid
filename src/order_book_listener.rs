use crate::order::Order;
use crate::order_book::OrderBook;

pub trait OrderBookListener<O: Order + Clone> {
    /// Callback for change anywhere in order book
    /// 
    /// # Parameters
    /// * `book`: The order book that has changed
    fn on_order_book_change(&self, book: &OrderBook<O>);
}