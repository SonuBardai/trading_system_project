pub mod auth;
pub mod db;
pub mod order;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use auth::check_user_auth;
use db::mock_db::db_users;
use db::{mock_db::db_stocks, User};
use lazy_static::lazy_static;
use order::{Order, Orderbook};
use std::fmt;
use std::io::Result;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref GLOBAL_ORDERBOOK: Arc<Mutex<Orderbook>> =
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

async fn order() -> impl Responder {
    let order = Order::new(
        db_stocks()[0].clone(),
        db_users()[0].clone(),
        order::TransactionType::Ask,
    );
    GLOBAL_ORDERBOOK.lock().unwrap().bid(order);
    HttpResponse::Ok().body(format!("Ask order placed"))
}

async fn depth() -> impl Responder {
    let orderbook = GLOBAL_ORDERBOOK.lock().unwrap();
    HttpResponse::Ok().body(format!("{:?}", orderbook))
}

async fn balance(req: HttpRequest, data: web::Path<(u32,)>) -> impl Responder {
    let user_id = data.0;
    if check_user_auth(req, user_id) {
        let user = User::get_user(user_id);
        match user {
            Some(user) => HttpResponse::Ok().body(format!(
                "{{\"balance\": {}, \"stocks\": {:?}}}", // use serde to define response structs and serialize them
                user.balance, user.stocks
            )),
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
