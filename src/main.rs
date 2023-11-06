pub mod auth;
pub mod db;
pub mod order;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use auth::check_user_auth;
use db::User;
use std::io::Result;

async fn order() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

async fn depth() -> impl Responder {
    HttpResponse::Ok().body("hello world")
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
