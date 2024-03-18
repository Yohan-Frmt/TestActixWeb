extern crate core;

mod auth;
mod cards;
mod database;
mod error;
mod log;
mod user;
mod utils;

use crate::log::init_logger;
use actix_web::{middleware::Logger, web, web::Data, App, HttpServer, ResponseError};
use database::mongodb::Mongo;
use std::sync::Arc;

type Result<T> = std::result::Result<T, error::AppError>;
type WebResult<T> = std::result::Result<T, dyn ResponseError>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(e) = dotenv::dotenv() {
        print!("Not applying .env : {e}");
    }
    init_logger();
    let mongodb = Data::from(Arc::new(Mongo::init(None).await));
    let host = std::env::var("HOST").expect("HOST env may not be set");
    let port = std::env::var("PORT").expect("PORT env may not be set");
    let address = format!("{}:{}", &host, &port);
    println!("Binding server to {address} ...");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(&mongodb)
            .service(web::scope("/api/user").configure(user::controller::init_routes))
            .service(web::scope("/api/auth").configure(auth::controller::init_routes))
        // .service(web::scope("/api/card").configure(card::controller::init_routes))
    })
    .bind(&address)?
    .run()
    .await
}
