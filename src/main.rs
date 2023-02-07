pub mod db;
pub mod modules;
use crate::modules::*;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(git::init))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
