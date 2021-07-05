use super::models::{NewPost, Post};
use super::schema::posts;
use diesel::{mysql::MysqlConnection, result::Error, ExpressionMethods, QueryDsl, RunQueryDsl};

/// Create new post.
pub fn create_post(conn: &MysqlConnection, new_post: NewPost) -> Result<usize, Error> {
    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
}

/// Get post by post ID.
pub fn get_post_by_id(conn: &MysqlConnection, post_id: i32) -> Result<Post, Error> {
    posts::table.filter(posts::id.eq(post_id)).get_result(conn)
}

/// Retreive all posts from user with user id.
pub fn get_posts_by_user_id(conn: &MysqlConnection, author_id: i32) -> Result<Vec<Post>, Error> {
    posts::table
        .filter(posts::author_id.eq(author_id))
        .get_results(conn)
}

#[cfg(test)]
mod tests {
    use crate::db::establish_connection;
    use crate::models::NewPost;

    #[test]
    fn post_retreived_by_id() {
        use super::get_post_by_id;
        let conn = establish_connection();
        let res = get_post_by_id(&conn, 1);
        assert_eq!(res.unwrap().id, 1);
    }

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
        match res {
            Ok(_) => assert!(false, "Should not succeed."),
            Err(e) => assert!(true, "{}", e),
        };
    }
}
