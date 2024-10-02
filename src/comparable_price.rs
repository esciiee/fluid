use std::cmp::Ordering;
use std::fmt;
use crate::types::{Price , MARKET_ORDER_PRICE};


#[derive(Clone, Copy, Debug)]
pub struct ComparablePrice {
    price: Price,
    buy_side: bool,
}

impl ComparablePrice {
    pub fn new(buy_side: bool, price: Price) -> Self {
        ComparablePrice { price, buy_side }
    }

    pub fn matches(&self, rhs: Price) -> bool {
        if self.price == rhs {
            return true;
        }
        if self.buy_side {
            rhs < self.price || self.price == MARKET_ORDER_PRICE
        } else {
            self.price < rhs || rhs == MARKET_ORDER_PRICE
        }
    }

    pub fn price(&self) -> Price {
        self.price
    }

    pub fn is_buy(&self) -> bool {
        self.buy_side
    }

    pub fn is_market(&self) -> bool {
        self.price == MARKET_ORDER_PRICE
    }
}

impl PartialEq<Price> for ComparablePrice {
    fn eq(&self, other: &Price) -> bool {
        self.price == *other
    }
}

impl PartialOrd<Price> for ComparablePrice {
    fn partial_cmp(&self, other: &Price) -> Option<Ordering> {
        if self.price == MARKET_ORDER_PRICE {
            if *other == MARKET_ORDER_PRICE {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Less)
            }
        } else if *other == MARKET_ORDER_PRICE {
            Some(Ordering::Greater)
        } else if self.buy_side {
            other.partial_cmp(&self.price)
        } else {
            self.price.partial_cmp(other)
        }
    }
}

impl PartialEq for ComparablePrice {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl PartialOrd for ComparablePrice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.partial_cmp(&other.price)
    }
}

impl Ord for ComparablePrice {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for ComparablePrice {}

impl fmt::Display for ComparablePrice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {}",
            if self.buy_side { "Buy" } else { "Sell" },
            if self.is_market() {
                "Market".to_string()
            } else {
                self.price.to_string()
            }
        )
    }
}
