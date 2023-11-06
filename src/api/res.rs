use serde::Serialize;

use crate::{
    db::{Stock, User},
    order::{Order, Orderbook},
};

#[derive(Serialize, Debug)]
pub struct BalanceResponse {
    user_id: u32,
    balance: u64,
    holdings: Vec<Stock>,
}

impl BalanceResponse {
    pub fn from_user(user: User) -> Self {
        return BalanceResponse {
            user_id: user.user_id,
            balance: user.balance,
            holdings: user.stocks,
        };
    }
}

#[derive(Serialize, Debug)]
pub struct OrderbookResponse {
    stock: String,
    asks: Vec<Order>,
    bids: Vec<Order>,
}

impl OrderbookResponse {
    pub fn from_orderbook(orderbook: &Orderbook) -> Self {
        let stock = orderbook.stock.clone();
        return OrderbookResponse {
            stock: stock.ticker,
            asks: orderbook.asks.clone(),
            bids: orderbook.bids.clone(),
        };
    }
}
