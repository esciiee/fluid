use crate::types::{Price, Quantity};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum CbType {
    Unknown,
    OrderAccept,
    OrderAcceptStop,
    OrderTriggerStop,
    OrderReject,
    OrderFill,
    OrderCancel,
    OrderCancelStop,
    OrderCancelReject,
    OrderReplace,
    OrderReplaceReject,
    BookUpdate,
}

#[derive(Debug, Clone)]
pub struct Callback<T: Clone> {
    pub cb_type: CbType,
    pub order: Option<Rc<T>>,
    pub matched_order: Option<Rc<T>>,
    pub quantity: Quantity,
    pub price: Price,
    pub flags: u8,
    pub delta: i64,
    pub reject_reason: Option<String>,
}

impl<T: Clone> Callback<T> {
    pub fn new() -> Self {
        Callback {
            cb_type: CbType::Unknown,
            order: None,
            matched_order: None,
            quantity: 0,
            price: 0,
            flags: 0,
            delta: 0,
            reject_reason: None,
        }
    }

    pub fn accept(order: Rc<T>) -> Self {
        Callback {
            cb_type: CbType::OrderAccept,
            order: Some(order),
            ..Self::new()
        }
    }

    pub fn accept_stop(order: Rc<T>) -> Self {
        Callback {
            cb_type: CbType::OrderAcceptStop,
            order: Some(order),
            ..Self::new()
        }
    }

    pub fn trigger_stop(order: Rc<T>) -> Self {
        Callback {
            cb_type: CbType::OrderTriggerStop,
            order: Some(order),
            ..Self::new()
        }
    }

    pub fn reject(order: Rc<T>, reason: &str) -> Self {
        Callback {
            cb_type: CbType::OrderReject,
            order: Some(order),
            reject_reason: Some(reason.to_string()),
            ..Self::new()
        }
    }

    pub fn fill(
        inbound_order: Rc<T>,
        matched_order: Rc<T>,
        fill_qty: Quantity,
        fill_price: Price,
        fill_flags: u8,
    ) -> Self {
        Callback {
            cb_type: CbType::OrderFill,
            order: Some(inbound_order),
            matched_order: Some(matched_order),
            quantity: fill_qty,
            price: fill_price,
            flags: fill_flags,
            ..Self::new()
        }
    }

    pub fn cancel(order: Rc<T>, open_qty: Quantity) -> Self {
        Callback {
            cb_type: CbType::OrderCancel,
            order: Some(order),
            quantity: open_qty,
            ..Self::new()
        }
    }

    pub fn cancel_stop(order: Rc<T>) -> Self {
        Callback {
            cb_type: CbType::OrderCancelStop,
            order: Some(order),
            ..Self::new()
        }
    }

    pub fn cancel_reject(order: Rc<T>, reason: &str) -> Self {
        Callback {
            cb_type: CbType::OrderCancelReject,
            order: Some(order),
            reject_reason: Some(reason.to_string()),
            ..Self::new()
        }
    }

    pub fn replace(
        order: Rc<T>,
        curr_open_qty: Quantity,
        size_delta: i64,
        new_price: Price,
    ) -> Self {
        Callback {
            cb_type: CbType::OrderReplace,
            order: Some(order),
            quantity: curr_open_qty,
            delta: size_delta,
            price: new_price,
            ..Self::new()
        }
    }

    pub fn replace_reject(order: Rc<T>, reason: &str) -> Self {
        Callback {
            cb_type: CbType::OrderReplaceReject,
            order: Some(order),
            reject_reason: Some(reason.to_string()),
            ..Self::new()
        }
    }

    pub fn book_update() -> Self {
        Callback {
            cb_type: CbType::BookUpdate,
            ..Self::new()
        }
    }
}

// pub struct FillFlags;

// impl FillFlags {
//     pub const NEITHER_FILLED: u8 = 0;
//     pub const INBOUND_FILLED: u8 = 1;
//     pub const MATCHED_FILLED: u8 = 2;
//     pub const BOTH_FILLED: u8 = 4;
// }
pub enum FillFlags {
    NeitherFilled = 0,
    InboundFilled = 1,
    MatchedFilled = 2,
    BothFilled = 4,
}