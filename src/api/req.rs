use serde::Deserialize;

use crate::order::TransactionType;

#[derive(Deserialize, Debug)]
pub struct OrderRequest {
    pub stock_id: u32,
    pub user_id: u32,
    pub price: u64,
    pub qty: u32,
    pub transaction_type: TransactionType,
}
