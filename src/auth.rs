use super::db::get_user_by_username;
use super::models::User;
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
) -> Result<User, String> {
    let user = get_user_by_username(&conn, username.to_string()).expect("User not found.");
    let valid = verify(password, &user.password).unwrap();
    if valid {
        Ok(user)
    } else {
        Err("Wrong password.".to_string())
    }
}

pub fn login(req: HttpRequest, user: Option<User>, sess: Session) -> Result<(), Error> {
    unimplemented!()
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
