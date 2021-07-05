#[macro_use]
extern crate diesel;
extern crate serde;

pub mod handlers;
pub mod models;
pub mod posts;
pub mod schema;

pub mod db {
    use super::models::*;

    use bcrypt::{hash, DEFAULT_COST};
    use diesel::mysql::MysqlConnection;
    use diesel::prelude::*;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
    use std::env;

    type QueryError = diesel::result::Error;

    /// Establishes connection to db.
    pub fn establish_connection() -> MysqlConnection {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set.");
        MysqlConnection::establish(&database_url).expect("Connection failed.")
    }

    /// Retreive all users.
    pub fn get_users(conn: &MysqlConnection) -> Result<Vec<User>, QueryError> {
        use super::schema::users;
        users::table.get_results(conn)
    }

    /// Get user by user id.
    pub fn get_user_by_id(conn: &MysqlConnection, user_id: i32) -> Result<User, QueryError> {
        use crate::schema::users::dsl::*;
        users.filter(id.eq(user_id)).get_result(conn)
    }

    /// Get user by username.
    pub fn get_user_by_username(
        conn: &MysqlConnection,
        _username: String,
    ) -> Result<User, QueryError> {
        use crate::schema::users::dsl::*;
        users.filter(username.eq(_username)).get_result(conn)
    }

    /// Create new user.
    pub fn create_user(conn: &MysqlConnection, new_user: &NewUser) -> Result<usize, QueryError> {
        use super::schema::users;

        // Hash password.
        let hashed = hash(new_user.password, DEFAULT_COST);
        let user = NewUser {
            username: new_user.username,
            password: &hashed.unwrap(),
        };

        diesel::insert_into(users::table).values(user).execute(conn)
    }

    /// Delete user by id.
    pub fn delete_user(conn: &MysqlConnection, user_id: i32) -> Result<usize, QueryError> {
        use super::schema::users::dsl::*;
        diesel::delete(users.filter(id.eq(user_id))).execute(conn)
    }

    #[cfg(test)]
    mod tests {
        use super::establish_connection;
        use crate::models::{NewUser, User};
        use crate::schema::users;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

        #[test]
        fn password_hashed() {
            use super::{create_user, establish_connection, get_user_by_username};
            use bcrypt::{hash, verify, DEFAULT_COST};
            let new_user = NewUser {
                username: "testuser1",
                password: "password123",
            };
            let hashed = hash(new_user.password, DEFAULT_COST);
            let conn = establish_connection();
            let _ = create_user(&conn, &new_user);
            let usr = get_user_by_username(&conn, "testuser1".to_string()).unwrap();
            let should_succeed = verify(usr.password, &hashed.unwrap());
            assert!(should_succeed.is_ok());
        }

        #[test]
        fn user_retrieved_by_username() {
            use super::{establish_connection, get_user_by_username};
            let conn = establish_connection();
            let usr = get_user_by_username(&conn, "thrillho".to_string()).unwrap();
            assert_eq!(usr.username, "thrillho");
        }

        #[test]
        fn user_deleted() {
            use super::{create_user, delete_user, establish_connection, get_user_by_username};
            let conn = establish_connection();
            let tmp = create_user(
                &conn,
                &NewUser {
                    username: "tempuser",
                    password: "bar",
                },
            );
            let usr = get_user_by_username(&conn, "tempuser".to_string()).unwrap();
            let res = delete_user(&conn, usr.id);
            assert_eq!(res, Ok(1));
        }

        #[test]
        fn user_retrieved_by_id() {
            use super::get_user_by_id;
            let conn = establish_connection();
            let user = get_user_by_id(&conn, 34).unwrap();
            let cyobero = get_user_by_id(&conn, 35).unwrap();

            assert_eq!(user.id, 34);
            assert_eq!(cyobero.id, 35);
        }
        #[test]
        fn user_created() {
            use super::create_user;
            let conn = establish_connection();
            let new_user = NewUser {
                username: "thrillho",
                password: "password123",
            };

            let res = create_user(&conn, &new_user);
            let user: User = users::table
                .filter(users::username.eq(&new_user.username))
                .get_result(&conn)
                .unwrap();

            assert_eq!(user.username, new_user.username);
        }

        #[test]
        fn users_retrieved() {
            use super::{establish_connection, get_users};
            use crate::{models::User, schema::users};
            use diesel::RunQueryDsl;
            let conn = establish_connection();
            let res: Vec<User> = users::table.get_results(&conn).unwrap();

            assert!(res.len() > 0);
        }

        #[test]
        fn connection_established() {
            use super::establish_connection;
            let conn = establish_connection();
        }
    }
}
