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
use sqlx::PgPool;
use std::error::Error;
use std::result::Result;

use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use std::io;
use std::env;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = Config::from_env().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DB_URL is not set in .env file"); 
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    let host = config.server.host;
    let p = match env::var("PORT") {
        Ok(pr) => pr.parse::<i32>().unwrap(),
        Err(e) => 8779
    };
    let port = p;
    let tkn: String = match env::var("TOKEN") {
        Ok(t) => t,
        Err(e) => "".to_string(),
    };
    let token = tkn;
    let path = "".to_string();
    let log = Config::configure_log();

    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .data(AppState {log: log.clone(), token: token.clone(), path: path.clone(), pool: db_pool.clone()})
        .route("/", web::post().to(process))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}


