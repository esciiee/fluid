use rust_decimal::Decimal;

pub struct Order {
    // should this be address of PDA?
    pub user_id: u64,
    pub order_id: u64,
    pub side: Side,
    pub price: Decimal,
    pub amount: Decimal,
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