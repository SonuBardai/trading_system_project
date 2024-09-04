pub mod mock_db;
use serde::Serialize;

use self::mock_db::db_stocks;
use self::mock_db::db_users;

#[derive(Clone, Debug, Serialize)]
pub struct Stock {
    pub stock_id: u32,
    pub ticker: String,
    pub price: u32,
    pub qty: u32,
}

impl Stock {
    pub fn get_stock(stock_id: u32) -> Option<Self> {
        match db_stocks().iter().find(|stock| stock.stock_id == stock_id) {
            Some(stock) => Some(stock.clone()),
            None => None,
        }
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct User {
    pub user_id: u32,
    pub balance: u64,
    pub stocks: Vec<Stock>,
}

impl User {
    pub fn get_user(user_id: u32) -> Option<User> {
        match db_users().iter().find(|user| user.user_id == user_id) {
            Some(user) => Some(user.clone()),
            None => None,
        }
    }
}
