use super::db::{establish_connection, get_user_by_username};
use super::models::User;
use super::DieselError;
use actix_session::Session;
use actix_web::{web, HttpRequest};
use bcrypt::{verify, BcryptError};
use diesel::connection::Connection;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

//pub fn authenticate(
//conn: &MysqlConnection,
//username: &str,
//password: &str,
//) -> Result<models::User, String> {
//let user = get_user_by_username(&conn, username.to_string()).expect("User not found.");
//if let Ok(true) = verify(password, &user.password) {
//Ok(user)
//} else {
//Err("Invalid Credentials. PLease try again.".to_string())
//}
//}

pub trait Auth<C = MysqlConnection, E = DieselError> {
    type Output;
    fn authenticate(&self, conn: &C) -> Result<Self::Output, E>;
}

impl Auth for User {
    type Output = User;
    fn authenticate(&self, conn: &MysqlConnection) -> Result<User, DieselError> {
        let username = self.username.to_string();
        get_user_by_username(conn, username)
    }
}

//pub trait User {}
//pub trait Auth<U: User, C: Connection, E> {
//type Username;
//type Password;
//fn authenticate(&self, conn: &C) -> Result<U, E> {
//use super::schema::users;
//users::table.filter(users::username.eq(Self::Username)).get_results(conn);
//};
//}

//pub struct AnonymousUser {
//pub username: Option<String>,
//pub password: Option<String>,
//}

//impl AnonymousUser {
//pub fn new() -> Self {
//AnonymousUser::default()
//}
//}

//impl Default for AnonymousUser {
//fn default() -> Self {
//AnonymousUser {
//username: None,
//password: None,
//}
//}
//}

//#[derive(Debug, Serialize, Deserialize)]
//pub struct BaseUser {
//username: Option<String>,
//password: Option<String>,
//}

//#[cfg(test)]
//mod tests {
//use crate::db::establish_connection;
//use crate::models::User;
//#[test]
//fn user_authenticated() {
//let usr = User {
//username: "testuser2".to_owned(),
//password: "password123".to_owned(),
//};
//let conn = establish_connection();
//assert_eq!(user.username, "testuser2");
//}
//}
