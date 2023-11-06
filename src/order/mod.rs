use std::fmt::Debug;

use crate::db::Stock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Bid,
    Ask,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Order {
    pub stock_id: u32,
    pub user_id: u32,
    pub price: u64,
    pub qty: u32,
    pub transaction_type: TransactionType,
}

impl Order {
    pub fn new(
        stock_id: u32,
        user_id: u32,
        price: u64,
        qty: u32,
        transaction_type: TransactionType,
    ) -> Self {
        Order {
            stock_id,
            user_id,
            price,
            qty,
            transaction_type,
        }
    }
}

#[derive(Debug)]
pub struct Orderbook {
    pub stock: Stock,
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
}

impl Orderbook {
    pub fn new(stock: Stock) -> Self {
        Orderbook {
            stock,
            bids: vec![],
            asks: vec![],
        }
    }

    pub fn ask(&mut self, order: Order) {
        let asks = &mut self.asks;
        match asks.binary_search_by(|a| a.price.cmp(&order.price).reverse()) {
            Ok(index) | Err(index) => asks.insert(index, order),
        };
    }

    pub fn bid(&mut self, order: Order) {
        let bids = &mut self.bids;
        match bids.binary_search_by(|a| a.price.cmp(&order.price).reverse()) {
            Ok(index) | Err(index) => bids.insert(index, order),
        };
    }
}
