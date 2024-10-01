pub type Price = u64;
pub type Quantity = u64;
pub type Cost = u64;
pub type FillId = u32;
pub type ChangeId = u32;
pub type OrderConditions = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderCondition {
    NoConditions = 0,
    AllOrNone = 1,
    ImmediateOrCancel = 1 << 1,
    FillOrKill = Self::AllOrNone as isize | Self::ImmediateOrCancel as isize,
    Stop = 1 << 2,
}

pub const MARKET_ORDER_PRICE: Price = 0;
pub const PRICE_UNCHANGED: Price = 0;
pub const QUANTITY_MAX: Quantity = u64::MAX;
pub const SIZE_UNCHANGED: i64 = 0;