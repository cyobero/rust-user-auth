use user_auth::db::establish_connection;
use user_auth::posts::get_posts;

fn main() {
    let conn = establish_connection();
    get_posts(&conn)
        .unwrap()
        .into_iter()
        .for_each(|post| println!("{:?}", post));
}
