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
    sessions (session_id, user_id) {
        session_id -> Varchar,
        user_id -> Integer,
        login_time -> Timestamp,
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
joinable!(sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    sessions,
    users,
);
