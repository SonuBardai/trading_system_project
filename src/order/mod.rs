use std::fmt::Debug;

use crate::db::{Stock, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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

    pub fn from(stock: Stock, orders: Vec<Order>) -> Self {
        let asks = orders
            .clone()
            .into_iter()
            .filter(|order| order.transaction_type == TransactionType::Ask)
            .collect();
        let bids = orders
            .into_iter()
            .filter(|order| order.transaction_type == TransactionType::Bid)
            .collect();
        Orderbook { stock, bids, asks }
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

    pub fn fill_order(&mut self, order: Order) -> u32 {
        let mut remaining_qty = order.qty;
        let stock = Stock::get_stock(order.stock_id)
            .expect(&format!("Stock with stock_id {} not found", order.stock_id));
        match order.transaction_type {
            TransactionType::Ask => {
                for bid in self.bids.clone() {
                    if order.price > bid.price {
                        break;
                    } else {
                        let mut buyer = User::get_user(bid.user_id)
                            .expect(&format!("Can't find user with user_id {}", bid.user_id));
                        let mut seller = User::get_user(order.user_id)
                            .expect(&format!("Can't find user with user_id {}", order.user_id));
                        if order.qty <= bid.qty {
                            self.flip_balances(
                                &mut buyer,
                                &mut seller,
                                stock.clone(),
                                bid.price,
                                remaining_qty,
                            );
                            remaining_qty = 0;
                            break;
                        } else {
                            remaining_qty -= bid.qty;
                            self.flip_balances(
                                &mut buyer,
                                &mut seller,
                                stock.clone(),
                                bid.price,
                                bid.qty,
                            );
                        }
                    }
                }
            }
            TransactionType::Bid => {
                for ask in self.asks.clone().into_iter().rev() {
                    if order.price < ask.price {
                        break;
                    } else {
                        let mut buyer = User::get_user(ask.user_id)
                            .expect(&format!("Can't find user with user_id {}", ask.user_id));
                        let mut seller = User::get_user(order.user_id)
                            .expect(&format!("Can't find user with user_id {}", order.user_id));
                        if order.qty <= ask.qty {
                            self.flip_balances(
                                &mut buyer,
                                &mut seller,
                                stock.clone(),
                                ask.price,
                                order.qty,
                            );
                            remaining_qty = 0;
                            break;
                        } else {
                            remaining_qty -= ask.qty;
                            self.flip_balances(
                                &mut buyer,
                                &mut seller,
                                stock.clone(),
                                order.price,
                                ask.qty,
                            );
                        }
                    }
                }
            }
        }
        remaining_qty
    }

    pub fn flip_balances(
        &mut self,
        buyer: &mut User,
        seller: &mut User,
        stock: Stock,
        price: u64,
        qty: u32,
    ) {
        let total_amount = price * qty as u64;
        buyer.balance -= total_amount;
        buyer.stocks.push(stock.clone());
        if let Some(seller_stock) = seller
            .stocks
            .iter_mut()
            .find(|s| s.stock_id == stock.stock_id)
        {
            seller_stock.qty -= qty;
        } else {
            panic!("Seller does not have the stock");
        }
    }

    pub fn place_order(&mut self, order: Order) -> &mut Self {
        match order.transaction_type {
            TransactionType::Ask => {
                self.ask(order);
            }
            TransactionType::Bid => {
                self.bid(order);
            }
        }
        self
    }
}
