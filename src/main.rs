mod handler;
mod errors;
mod models;
mod posting;
mod config;
mod db;
mod utils;

use crate::config::Config;
use crate::handler::*;
use crate::models::AppState;

use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use std::io;
use std::env;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");
    let token = env::var("TOKEN").expect("Token not set");
    let path = env::var("MONGODB_URI").expect("Mongodb url not set");
    let log = Config::configure_log();

    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .data(AppState {log: log.clone(), token: token.clone(), path: path.clone()})
        .route("/", web::post().to(process))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
