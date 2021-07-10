#[macro_use]
extern crate diesel;
extern crate serde;

pub mod auth;
pub mod db;
pub mod handlers;
pub mod models;
pub mod posts;
pub mod schema;

use chrono::NaiveDateTime;
use models::*;
use serde::{Deserialize, Serialize};

type ActixError = actix_web::Error;
type DieselError = diesel::result::Error;

#[derive(Serialize)]
pub struct UserResponse {
    id: i32,
    username: String,
    created_at: NaiveDateTime,
}

impl UserResponse {
    /// Creates a new `UserResponse` from a `User` struct.
    pub fn from_user(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            created_at: user.created_at,
        }
    }
}

#[derive(Deserialize)]
pub struct NewUserInput {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct UsersResponse {
    user_resp: Vec<UserResponse>,
}

#[derive(Serialize)]
pub struct GetPostsResponse {
    posts: Vec<Post>,
}

#[derive(Deserialize)]
pub struct NewPostInput {
    title: String,
    body: String,
    author_id: i32,
}
