use crate::{Amount, OrderId, Price, UserId};

#[derive(Clone, Debug)]
pub struct Order {
    // should this be address of PDA?
    pub user_id: UserId,
    pub order_id: OrderId,
    pub price: Price,
    pub unfilled_amount: Amount,
    // should add timestamps for order creation, fill, cancel
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Clone, Debug)]
pub enum Status {
    Open,
    Filled,
    Cancelled,
}

impl Order {
    pub fn new(user_id: UserId, order_id: OrderId, price: Price, unfilled_amount: Amount) -> Self {
        Self {
            user_id,
            order_id,
            price,
            unfilled_amount,
        }
    }

    pub fn is_filled(&self) -> bool {
        self.unfilled_amount.is_zero()
    }

    pub fn fill(&mut self, amount: Amount) {
        self.unfilled_amount -= amount;
    }
}
