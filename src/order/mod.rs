use std::{fmt::Debug, sync::Mutex};

use crate::db::{Stock, User};

#[derive(Debug)]
pub enum TransactionType {
    Bid,
    Ask,
}

#[derive(Debug)]
pub struct Order {
    pub stock: Stock,
    pub user: User,
    pub transaction_type: TransactionType,
}

impl Order {
    pub fn new(stock: Stock, user: User, transaction_type: TransactionType) -> Self {
        Order {
            stock,
            user,
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
