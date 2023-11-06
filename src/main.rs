use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use serde_json::to_string;
use std::fmt;
use std::io::Result;
use std::sync::{Arc, Mutex};
use trading_system::api::res::{BalanceResponse, OrderbookResponse};
use trading_system::{
    api::req::OrderRequest,
    auth::check_user_auth,
    db::{mock_db::db_stocks, User},
    order::{Order, Orderbook, TransactionType},
};

lazy_static! {
    pub static ref GLOBAL_ORDERBOOK: Arc<Mutex<Orderbook>> =
        Arc::new(Mutex::new(Orderbook::new(db_stocks()[0].clone())));
}

impl fmt::Debug for GLOBAL_ORDERBOOK {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let orderbook = self.lock().unwrap();
        let bids = orderbook.bids.lock().unwrap();
        let asks = orderbook.asks.lock().unwrap();
        f.debug_struct("Orderbook")
            .field("stock", &orderbook.stock)
            .field("bids", &*bids)
            .field("asks", &*asks)
            .finish()
    }
}

async fn order(order_req: web::Json<OrderRequest>) -> impl Responder {
    match order_req.transaction_type {
        TransactionType::Ask => {
            let order = Order::new(
                order_req.stock_id,
                order_req.user_id,
                order_req.price,
                order_req.qty,
                TransactionType::Ask,
            );
            GLOBAL_ORDERBOOK.lock().unwrap().ask(order);
            HttpResponse::Ok().body(format!("Ask order placed"))
        }
        TransactionType::Bid => {
            let order = Order::new(
                order_req.stock_id,
                order_req.user_id,
                order_req.price,
                order_req.qty,
                TransactionType::Bid,
            );
            GLOBAL_ORDERBOOK.lock().unwrap().bid(order);
            HttpResponse::Ok().body(format!("Bid order placed"))
        }
    }
}

async fn depth() -> impl Responder {
    let orderbook = GLOBAL_ORDERBOOK.lock().unwrap();
    HttpResponse::Ok().body(
        serde_json::to_string(&OrderbookResponse::from_orderbook(&orderbook))
            .expect("Failed to get orderbook data"),
    )
}

async fn balance(req: HttpRequest, data: web::Path<(u32,)>) -> impl Responder {
    let user_id = data.0;
    if check_user_auth(req, user_id) {
        let user = User::get_user(user_id);
        match user {
            Some(user) => HttpResponse::Ok().body(
                to_string(&BalanceResponse::from_user(user)).expect("Failed to get user balance"),
            ),
            None => HttpResponse::NotFound().body("User not found"),
        }
    } else {
        HttpResponse::Unauthorized().body("You can't access this information")
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/order", web::post().to(order))
            .route("/depth", web::get().to(depth))
            .route("/balance/{user_id}", web::get().to(balance))
    })
    .bind(("127.0.0.1", 8080))
    .expect("Failed to start server")
    .run()
    .await
}
