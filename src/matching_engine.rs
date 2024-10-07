use crate::{
    order::{Order, Side},
    orderbook::{Level, OrderBook},
    Amount, Price, UserId,
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum State {
    Placed,
    Canceled,
    Filled,
    PartiallyFilled,
    ConditionallyCanceled,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Role {
    Taker,
    Maker,
}

impl Into<u32> for Role {
    fn into(self) -> u32 {
        match self {
            Role::Maker => 0,
            Role::Taker => 1,
        }
    }
}

impl Into<u32> for State {
    fn into(self) -> u32 {
        match self {
            State::Placed => 0,
            State::Canceled => 1,
            State::Filled => 2,
            State::PartiallyFilled => 3,
            State::ConditionallyCanceled => 4,
        }
    }
}

impl Into<u8> for State {
    fn into(self) -> u8 {
        match self {
            State::Placed => 0,
            State::Canceled => 1,
            State::Filled => 2,
            State::PartiallyFilled => 3,
            State::ConditionallyCanceled => 4,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Taker {
    pub user_id: UserId,
    pub order_id: u64,
    pub price: Price,
    pub unfilled: Amount,
    pub ask_or_bid: Side,
    pub state: State,
}

impl Taker {
    pub fn taker(order: Order, ask_or_bid: Side, state: State) -> Self {
        Self {
            user_id: order.user_id,
            order_id: order.order_id,
            price: order.price,
            unfilled: order.unfilled_amount,
            ask_or_bid,
            state,
        }
    }

    pub fn taker_filled(user_id: UserId, order_id: u64, price: Price, ask_or_bid: Side) -> Self {
        Self {
            user_id,
            order_id,
            price,
            unfilled: Amount::ZERO,
            ask_or_bid,
            state: State::Filled,
        }
    }

    pub const fn taker_placed(
        user_id: UserId,
        order_id: u64,
        price: Price,
        unfilled: Amount,
        ask_or_bid: Side,
    ) -> Self {
        Self {
            user_id,
            order_id,
            price,
            unfilled,
            ask_or_bid,
            state: State::PartiallyFilled,
        }
    }

    pub const fn cancel(
        user_id: UserId,
        order_id: u64,
        price: Price,
        unfilled: Amount,
        ask_or_bid: Side,
    ) -> Self {
        Self {
            user_id,
            order_id,
            price,
            unfilled,
            ask_or_bid,
            state: State::Canceled,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Maker {
    pub user_id: UserId,
    pub order_id: u64,
    pub price: Price,
    pub filled: Amount,
    pub state: State,
}

impl Maker {
    pub const fn maker_filled(
        user_id: UserId,
        order_id: u64,
        price: Price,
        filled: Amount,
    ) -> Self {
        Self {
            user_id,
            order_id,
            price,
            filled,
            state: State::Filled,
        }
    }

    pub const fn maker_so_far(
        user_id: UserId,
        order_id: u64,
        price: Price,
        filled: Amount,
    ) -> Self {
        Self {
            user_id,
            order_id,
            price,
            filled,
            state: State::PartiallyFilled,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Match {
    pub maker: Vec<Maker>,
    pub taker: Taker,
    pub page_delta: std::collections::BTreeMap<Price, (Amount, Amount)>,
}

pub fn execute_limit(
    book: &mut OrderBook,
    user_id: UserId,
    price: Price,
    amount: Amount,
    ask_or_bid: Side,
) -> Match {
    use rust_decimal::prelude::Zero;
    let order_id = book.incr_then_fetch_order_id();
    // TODO move to config
    let mut max_makers = 20u32;
    let mut page_delta = std::collections::BTreeMap::<Price, (Amount, Amount)>::new();
    let mut makers = Vec::<Maker>::new();
    let mut order = Order::new(user_id, order_id, price, amount);
    loop {
        if order.is_filled() {
            return Match {
                maker: makers,
                taker: Taker::taker(order, ask_or_bid, State::Filled),
                page_delta,
            };
        }
        if let Some(mut best) = book.get_best_if_match(ask_or_bid, &order.price) {
            let page = best.get_mut();
            let (mut traded, interrupted) = take(page, &mut order, &mut max_makers);
            let taking_at_page = traded.iter().map(|o| o.filled).sum::<Amount>();
            if !taking_at_page.is_zero() {
                page_delta.insert(page.price, (taking_at_page + page.amount, page.amount));
            }
            if page.is_empty() {
                best.remove();
            }
            traded
                .iter()
                .filter(|m| m.state == State::Filled)
                .for_each(|m| {
                    book.order_id_to_price.remove(&m.order_id);
                });
            makers.append(&mut traded);
            if interrupted {
                return Match {
                    taker: Taker::taker(order, ask_or_bid, State::ConditionallyCanceled),
                    maker: makers,
                    page_delta,
                };
            }
        } else {
            let size_before = book.get_page_size(&order.price).unwrap_or(Amount::zero());
            page_delta
                .entry(order.price)
                .and_modify(|v| v.1 += order.unfilled_amount)
                .or_insert((size_before, size_before + order.unfilled_amount));
            book.place_order(order.clone(), ask_or_bid);
            return Match {
                taker: match makers.is_empty() {
                    true => Taker::taker(order, ask_or_bid, State::Placed),
                    false => Taker::taker(order, ask_or_bid, State::PartiallyFilled),
                },
                maker: makers,
                page_delta,
            };
        }
    }
}

fn take(page: &mut Level, taker: &mut Order, limit: &mut u32) -> (Vec<Maker>, bool) {
    let mut matches = Vec::<Maker>::new();
    while !taker.is_filled() && !page.is_empty() {
        if *limit == 0u32 {
            return (matches, true);
        }
        let mut oldest = page.orders.entries().next().unwrap();
        if oldest.get().user_id == taker.user_id {
            return (matches, true);
        }
        let m = if taker.unfilled_amount >= oldest.get().unfilled_amount {
            let maker = oldest.get().clone();
            oldest.remove();
            Maker::maker_filled(
                maker.user_id,
                maker.user_id,
                maker.price,
                maker.unfilled_amount,
            )
        } else {
            let maker = oldest.get_mut();
            maker.fill(taker.unfilled_amount);
            Maker::maker_so_far(
                maker.user_id,
                maker.user_id,
                maker.price,
                taker.unfilled_amount,
            )
        };
        taker.fill(m.filled);
        page.decr_size(&m.filled);
        *limit -= 1;
        matches.push(m);
    }
    (matches, false)
}
