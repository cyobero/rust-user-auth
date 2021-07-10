use super::db::get_user_by_id;
use super::schema::{posts, sessions, users};
use super::DieselError;
use crate::auth::Auth;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{mysql::MysqlConnection, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
    pub published: Option<bool>,
    pub created_at: NaiveDateTime,
    pub author_id: i32,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published: bool,
    pub author_id: i32,
}

impl<'a> Default for NewPost<'a> {
    fn default() -> Self {
        NewPost {
            title: "",
            body: "",
            published: false,
            author_id: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Session {
    session_id: String,
    user_id: i32,
    login_date: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize, Serialize)]
#[table_name = "sessions"]
pub struct NewSession<'nu> {
    pub session_id: &'nu str,
    pub user_id: i32,
}

impl Session {
    pub fn new(session_id: String, user_id: i32) -> Self {
        Session {
            session_id,
            user_id,
            login_date: Utc::now(),
        }
    }
}
