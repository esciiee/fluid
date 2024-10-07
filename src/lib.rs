pub mod order;
pub mod orderbook;

use crate::orderbook::Level;
use rust_decimal::Decimal;
use std::collections::{BTreeMap, HashMap};

pub type UserId = u64;
pub type OrderId = u64;
pub type Price = Decimal;
pub type Amount = Decimal;
pub type OrderIdToPrice = HashMap<OrderId, Price>;
pub type Table = BTreeMap<Price, Level>;
