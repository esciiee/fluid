use crate::{Amount, OrderId, Price, UserId};

#[derive(Clone, Debug)]
pub struct Order {
    // should this be address of PDA?
    pub user_id: UserId,
    pub order_id: OrderId,
    pub side: Side,
    pub price: Price,
    pub unfilled_amount: Amount,
    pub status: Status,
    // should add timestamps for order creation, fill, cancel
}

#[derive(Clone, Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Debug)]
pub enum Status {
    Open,
    Filled,
    Cancelled,
}
