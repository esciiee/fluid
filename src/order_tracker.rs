use crate::order::Order;
use crate::types::{OrderCondition, Quantity , OrderConditions};
use std::rc::Rc;

pub struct OrderTracker<T: Order> {
    order: Rc<T>,
    open_qty: Quantity,
    reserved: i64,
    conditions: OrderConditions,
}

impl<T: Order> OrderTracker<T> {
    pub fn new(order: Rc<T>, conditions: OrderConditions) -> Self {
        let open_qty = order.order_qty();
        let mut tracker = OrderTracker {
            order,
            open_qty,
            reserved: 0,
            conditions,
        };

        #[cfg(feature = "order_knows_conditions")]
        {
            if tracker.order.all_or_none() {
                tracker.conditions |= OC_ALL_OR_NONE;
            }
            if tracker.order.immediate_or_cancel() {
                tracker.conditions |= OC_IMMEDIATE_OR_CANCEL;
            }
        }

        tracker
    }

    pub fn reserve(&mut self, reserved: i64) -> Quantity {
        self.reserved += reserved;
        self.open_qty.saturating_sub(self.reserved as u64)
    }

    pub fn change_qty(&mut self, delta: i64) -> Result<(), &'static str> {
        if delta < 0 && self.open_qty < delta.unsigned_abs() as u64 {
            return Err("Replace size reduction larger than open quantity");
        }
        self.open_qty = self.open_qty.saturating_add_signed(delta);
        Ok(())
    }

    pub fn fill(&mut self, qty: Quantity) -> Result<(), &'static str> {
        if qty > self.open_qty {
            return Err("Fill size larger than open quantity");
        }
        self.open_qty -= qty;
        Ok(())
    }

    pub fn filled(&self) -> bool {
        self.open_qty == 0
    }

    pub fn filled_qty(&self) -> Quantity {
        self.order.order_qty() - self.open_qty()
    }

    pub fn open_qty(&self) -> Quantity {
        self.open_qty.saturating_sub(self.reserved as u64)
    }

    pub fn ptr(&self) -> &Rc<T> {
        &self.order
    }

    pub fn all_or_none(&self) -> bool {
        self.conditions & OrderCondition::AllOrNone as u32 != 0
    }

    pub fn immediate_or_cancel(&self) -> bool {
        self.conditions & OrderCondition::ImmediateOrCancel as u32 != 0
    }
}