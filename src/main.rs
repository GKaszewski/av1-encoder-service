use std::env;

use actix_web::{HttpServer, App};

use crate::services::{convert, index, convert_single};

mod services;
mod encode;
mod video_utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = env::var("HOST").expect("HOST not set");
    let port = env::var("PORT").expect("PORT not set");
    let address = format!("{}:{}", host, port);
    println!("Starting server at: {}", address);
    HttpServer::new(|| App::new().service(convert)
        .service(index)
        .service(convert_single))
        .bind(address)?
        .run()
        .await
}
