use crate::{Amount, OrderId, Price, UserId};

pub struct Order {
    // should this be address of PDA?
    pub user_id: UserId,
    pub order_id: OrderId,
    pub side: Side,
    pub price: Price,
    pub amount: Amount,
    pub status: Status,
    // should add timestamps for order creation, fill, cancel
}

pub enum Side {
    Buy,
    Sell,
}

pub enum Status {
    Open,
    Filled,
    Cancelled,
}
