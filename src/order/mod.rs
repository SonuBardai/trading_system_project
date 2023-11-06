use std::{fmt::Debug, sync::Mutex};

use crate::db::{Stock, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Bid,
    Ask,
}

#[derive(Debug, Clone, Serialize)]
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
    pub bids: Mutex<Vec<Order>>,
    pub asks: Mutex<Vec<Order>>,
}

impl Orderbook {
    pub fn new(stock: Stock) -> Self {
        Orderbook {
            stock,
            bids: Mutex::new(vec![]),
            asks: Mutex::new(vec![]),
        }
    }

    pub fn bid(&mut self, order: Order) {
        self.bids.lock().unwrap().push(order);
    }

    pub fn ask(&mut self, order: Order) {
        self.asks.lock().unwrap().push(order);
    }
}
