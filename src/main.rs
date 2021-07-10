#[macro_use]
extern crate diesel;
extern crate serde_json;

use actix_cors::Cors;
use actix_session::CookieSession;
use actix_web::{get, web, App, HttpServer};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use handlebars::Handlebars;

use std::env;

use user_auth::handlers;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("index", include_str!("../templates/index.html"))
        .unwrap();
    handlebars
        .register_template_string("login", include_str!("../templates/login.html"))
        .unwrap();
    handlebars
        .register_template_string("signup", include_str!("../templates/signup.html"))
        .unwrap();

    let hb_ref = web::Data::new(handlebars);

    // Start HTTP server.
    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .data(pool.clone())
            .app_data(hb_ref.clone())
            .service(handlers::get_users)
            .service(handlers::get_users_id)
            .service(handlers::post_users)
            .service(handlers::delete_users_id)
            .service(handlers::index)
            .service(handlers::signup)
            .service(handlers::signup_form)
            .service(handlers::login)
            .service(handlers::login_form)
            .service(handlers::get_posts)
            .service(handlers::get_posts_id)
            .service(handlers::get_posts_new)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
