table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Nullable<Text>,
        published -> Nullable<Bool>,
        created_at -> Timestamp,
        author_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(posts -> users (author_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
