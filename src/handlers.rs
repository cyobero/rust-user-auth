use actix_web::{delete, get, http::StatusCode, post, web, Error, HttpResponse};
use chrono::NaiveDateTime;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use serde::{Deserialize, Serialize};

use super::db;
use super::models::*;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Serialize)]
pub struct UserResponse {
    id: i32,
    username: String,
    created_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct UsersResponse {
    users: Vec<User>,
}

#[derive(Deserialize)]
pub struct NewUserInput {
    username: String,
    password: String,
}

/// Handler for GET /users/{id}
#[get("/users/{id}")]
pub async fn get_users_id(
    pool: web::Data<DbPool>,
    _id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    use super::db::get_user_by_id;

    let user_id = _id.into_inner();
    let conn = pool.get().expect("Could not establish db pool connection.");
    let usr = web::block(move || get_user_by_id(&conn, user_id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(UserResponse {
        id: usr.id,
        username: usr.username,
        created_at: usr.created_at,
    }))
}

/// Retreives all users.
#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Could not establish db pool connection.");
    use super::db::get_users;
    let res = web::block(move || get_users(&conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(UsersResponse { users: res }))
}

/// Handler for POST /users
#[post("/users")]
pub async fn post_users<'a>(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUserInput>,
) -> Result<HttpResponse, Error> {
    use super::db::create_user;

    let username = new_user.username.to_string();
    let password = new_user.password.to_string();

    let conn = pool.get().expect("Could not establish db pool connection.");
    let res = web::block(move || {
        create_user(
            &conn,
            &NewUser {
                username: &username,
                password: &password,
            },
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().body(format!("{}", e))
    })?;

    match res {
        1 => Ok(HttpResponse::Ok().body("New user created!")),
        _ => Ok(HttpResponse::InternalServerError().body("Failed to create user.")),
    }
}

/// Handler for DELETE /users/{id}
#[delete("/users/{id}")]
pub async fn delete_users_id(
    pool: web::Data<DbPool>,
    _id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    use super::db::delete_user;

    let id = _id.into_inner().clone();

    let conn = pool.get().expect("Could not establish db poolc onnection.");
    let _ = web::block(move || delete_user(&conn, id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().body(format!("{}", e))
        })?;

    Ok(HttpResponse::Ok().body(format!("User ID {} deleted.", &id)))
}

/// Handler request that just serves up an index page.
#[get("/")]
pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html")))
}

/// Handler for signup page.
#[get("/signup")]
pub async fn signup() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/signup.html")))
}

#[post("/signup")]
pub async fn signup_form(
    pool: web::Data<DbPool>,
    form: web::Form<NewUserInput>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Could not establish db pool connection.");

    let _ = web::block(move || {
        db::create_user(
            &conn,
            &NewUser {
                username: &form.username,
                password: &form.password,
            },
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().body("whoops! something went wrong.")
    })?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/login_success.html")))
}
