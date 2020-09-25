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
use tokio_postgres::NoTls;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    let host = config.server.host;
    let p = match env::var("PORT") {
        Ok(pr) => i32(p),
        Err(e) => 8779
    };
    let port = p;
    let token = config.server.token;
    let path = config.server.mongo_uri;
    let log = Config::configure_log();

    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .data(AppState {log: log.clone(), token: token.clone(), path: path.clone(), pool: pool.clone()})
        .route("/", web::post().to(process))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
