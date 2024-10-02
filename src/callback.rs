use crate::types::{Price, Quantity};
use std::rc::Rc;
use crate::order_book::OrderBook;
use crate::order::Order;

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

#[derive(Debug, Clone, Copy)]
pub enum FillFlags {
    NeitherFilled = 0,
    InboundFilled = 1,
    MatchedFilled = 2,
    BothFilled = 4,
}

#[derive(Debug, Clone)]
pub struct Callback<O: Order + Clone> {
    pub cb_type: CbType,
    pub order: Option<Rc<O>>,
    pub matched_order: Option<Rc<O>>,
    pub quantity: Quantity,
    pub price: Price,
    pub flags: u8,
    pub delta: i64,
    pub reject_reason: Option<String>,
    pub book: Option<*const OrderBook<O>>,
}

impl<O: Order + Clone> Callback<O> {
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
            book: None,
        }
    }

    pub fn accept(order: Rc<O>) -> Self {
        Callback {
            cb_type: CbType::OrderAccept,
            order: Some(order),
            ..Self::new()
        }
    }

    pub fn accept_stop(order: Rc<O>) -> Self {
        Callback {
            cb_type: CbType::OrderAcceptStop,
            order: Some(order),
            ..Self::new()
        }
    }

    pub fn trigger_stop(order: Rc<O>) -> Self {
        Callback {
            cb_type: CbType::OrderTriggerStop,
            order: Some(order),
            ..Self::new()
        }
    }

    pub fn reject(order: Rc<O>, reason: &str) -> Self {
        Callback {
            cb_type: CbType::OrderReject,
            order: Some(order),
            reject_reason: Some(reason.to_string()),
            ..Self::new()
        }
    }

    pub fn fill(
        inbound_order: Rc<O>,
        matched_order: Rc<O>,
        fill_qty: Quantity,
        fill_price: Price,
        fill_flags: FillFlags,
    ) -> Self {
        Callback {
            cb_type: CbType::OrderFill,
            order: Some(inbound_order),
            matched_order: Some(matched_order),
            quantity: fill_qty,
            price: fill_price,
            flags: fill_flags as u8,
            ..Self::new()
        }
    }

    pub fn cancel(order: Rc<O>, open_qty: Quantity) -> Self {
        Callback {
            cb_type: CbType::OrderCancel,
            order: Some(order),
            quantity: open_qty,
            ..Self::new()
        }
    }

    pub fn cancel_stop(order: Rc<O>) -> Self {
        Callback {
            cb_type: CbType::OrderCancelStop,
            order: Some(order),
            ..Self::new()
        }
    }

    pub fn cancel_reject(order: Rc<O>, reason: &str) -> Self {
        Callback {
            cb_type: CbType::OrderCancelReject,
            order: Some(order),
            reject_reason: Some(reason.to_string()),
            ..Self::new()
        }
    }

    pub fn replace(
        order: Rc<O>,
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

    pub fn replace_reject(order: Rc<O>, reason: &str) -> Self {
        Callback {
            cb_type: CbType::OrderReplaceReject,
            order: Some(order),
            reject_reason: Some(reason.to_string()),
            ..Self::new()
        }
    }

    pub fn book_update(book: Option<&OrderBook<O>>) -> Self {
        Callback {
            cb_type: CbType::BookUpdate,
            book: book.map(|b| b as *const OrderBook<O>),
            ..Self::new()
        }
    }

    pub fn get_book(&self) -> Option<&OrderBook<O>> {
        self.book.map(|ptr| unsafe { &*ptr })
    }
}