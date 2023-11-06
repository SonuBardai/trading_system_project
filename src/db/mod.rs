pub mod mock_db;
use crate::db::mock_db::db_users;

#[derive(Clone, Debug)]
pub struct Stock {
    pub stock_id: u32,
    pub ticker: String,
    pub price: u32,
    pub amount: u32,
}

#[derive(Clone, Debug)]
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
