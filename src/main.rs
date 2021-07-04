#[macro_use]
extern crate diesel;

use actix_web::{get, App, HttpServer};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

use std::env;

use user_auth::handlers;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start HTTP server.
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(handlers::get_users)
            .service(handlers::get_users_id)
            .service(handlers::post_users)
            .service(handlers::delete_users_id)
            .service(handlers::index)
            .service(handlers::signup)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
