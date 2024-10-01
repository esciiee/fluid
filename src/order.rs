use crate::types::{Price, Quantity};
pub trait Order {
    fn is_limit(&self) -> bool {
        self.price() > 0
    }

    fn is_buy(&self) -> bool;

    fn price(&self) -> Price;

    fn stop_price(&self) -> Price {
        0 // default to not a stop order
    }

    fn order_qty(&self) -> Quantity;

    fn all_or_none(&self) -> bool {
        false // default to normal
    }

    fn immediate_or_cancel(&self) -> bool {
        false // default to normal
    }
}