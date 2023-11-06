// Mock db

use crate::db::User;

use super::Stock;

pub fn db_stocks() -> Vec<Stock> {
    vec![Stock {
        stock_id: 1,
        ticker: "GOOGL".to_string(),
        price: 130,
        amount: 1,
    }]
}

pub fn db_users() -> Vec<User> {
    vec![
        User {
            user_id: 1,
            balance: 50_000,
            stocks: vec![db_stocks()[0].clone()],
        },
        User {
            user_id: 2,
            balance: 50_000,
            stocks: vec![],
        },
    ]
}
