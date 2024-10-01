use std::cmp::Ordering;
use std::fmt;
use crate::types::Price;


#[derive(Debug, Clone, Copy)]
pub struct ComparablePrice {
    price : Price,
    buy_side : bool,
}

