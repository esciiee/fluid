pub mod order;
pub mod orderbook;

use rust_decimal::Decimal;

pub type UserId = u64;
pub type OrderId = u64;
pub type Price = Decimal;
pub type Amount = Decimal;
