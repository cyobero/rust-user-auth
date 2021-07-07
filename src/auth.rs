use super::db::get_user_by_username;
use super::models;
use actix_session::Session;
use actix_web::{web, HttpRequest};
use bcrypt::{verify, BcryptError};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

/// Authenticates a user by a) verifying username exists and b) verifying if password matches.
pub fn authenticate(
    conn: &MysqlConnection,
    username: &str,
    password: &str,
) -> Result<models::User, String> {
    let user = get_user_by_username(&conn, username.to_string()).expect("User not found.");
    if let Ok(true) = verify(password, &user.password) {
        Ok(user)
    } else {
        Err("Invalid Credentials. PLease try again.".to_string())
    }
}

pub fn login<U: User>(conn: &MysqlConnection, user: &U) -> Result<(), Error> {
    unimplemented!()
}

pub trait User {
    fn is_user() -> bool {
        true
    }
}

pub struct AnonymousUser {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl AnonymousUser {
    pub fn new() -> Self {
        AnonymousUser::default()
    }
}

impl Default for AnonymousUser {
    fn default() -> Self {
        AnonymousUser {
            username: None,
            password: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::authenticate;
    use crate::db::establish_connection;
    #[test]
    fn user_authenticated() {
        let username = "testuser2";
        let password = "password123";
        let conn = establish_connection();
        let user = authenticate(&conn, username, password).unwrap();
        assert_eq!(user.username, "testuser2");
    }
}
