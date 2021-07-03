use user_auth::db::{establish_connection, get_users};

fn main() {
    let conn = establish_connection();
    let users = get_users(&conn).unwrap();
    println!("Retreived {} users from db...", users.len());
    users.into_iter().for_each(|user| println!("{:?}", user));
}
