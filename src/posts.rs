use super::models::NewPost;
use super::schema::posts;
use diesel::prelude::*;
use diesel::{mysql::MysqlConnection, result::Error};

pub fn create_post(conn: &MysqlConnection, new_post: NewPost) -> Result<usize, Error> {
    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
}

#[cfg(test)]
mod tests {
    use crate::db::establish_connection;
    use crate::models::NewPost;

    #[test]
    fn post_created() {
        use super::create_post;
        let conn = establish_connection();
        let new_post = NewPost {
            title: "My new post",
            body: "Hello everyone! This is my very first post. Hope you enjoy.",
            published: false,
            author_id: 57,
        };
        let res = create_post(&conn, new_post);
        assert_eq!(res, Ok(1));
    }
}
