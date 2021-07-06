use actix_web::{
    delete, get, http::StatusCode, post, web, web::Form, Error, HttpResponse, ResponseError,
};
use chrono::NaiveDateTime;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use serde::{Deserialize, Serialize};

use super::db::*;
use super::models::*;
use super::posts;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Serialize)]
pub struct UserResponse {
    id: i32,
    username: String,
    created_at: NaiveDateTime,
}

impl UserResponse {
    /// Creates a new `UserResponse` from a `User` struct.
    pub fn from_user(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            created_at: user.created_at,
        }
    }
}

#[derive(Deserialize)]
pub struct NewUserInput {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct UsersResponse {
    user_resp: Vec<UserResponse>,
}

#[derive(Serialize)]
pub struct GetPostsResponse {
    posts: Vec<Post>,
}

#[derive(Deserialize)]
pub struct NewPostInput {
    title: String,
    body: String,
    author_id: i32,
}

pub trait Validate {
    type Form;
    fn validate(&mut self) -> Result<Self::Form, Error>;
}

/// Handler for GET /posts/{id}
#[get("/posts/{id: i32}")]
pub async fn get_posts_id(
    pool: web::Data<DbPool>,
    post_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    use super::posts::get_post_by_id;
    let conn = pool.get().expect("Could not establish db pool connection.");
    let post = web::block(move || get_post_by_id(&conn, post_id.into_inner()))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().body("Could not retrieve post.")
        })?;

    Ok(HttpResponse::Ok().json(post))
}

/// Handler for GET /users/{id}
#[get("/users/{id: i32}")]
pub async fn get_users_id(
    pool: web::Data<DbPool>,
    _id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = _id.into_inner();
    let conn = pool.get().expect("Could not establish db pool connection.");
    let usr = web::block(move || get_user_by_id(&conn, user_id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("text/html; charset=utf-8")
                .body(format!("{}", e))
        })?;

    Ok(HttpResponse::Ok().json(UserResponse {
        id: usr.id,
        username: usr.username,
        created_at: usr.created_at,
    }))
}

/// Retreives all users.
#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Could not establish db pool connection.");
    use super::db::get_users;
    let users: Vec<UserResponse> = web::block(move || get_users(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().body(format!("{}", e))
        })
        .unwrap()
        .into_iter()
        .map(|usr| UserResponse::from_user(usr))
        .collect();

    Ok(HttpResponse::Ok().json(users))
}

/// Handler for POST /users
#[post("/users")]
pub async fn post_users<'a>(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUserInput>,
) -> Result<HttpResponse, Error> {
    let username = new_user.username.to_string();
    let password = new_user.password.to_string();

    let conn = pool.get().expect("Could not establish db pool connection.");
    let res = web::block(move || {
        create_user(
            &conn,
            &NewUser {
                username: &username,
                password: &password,
            },
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().body(format!("{}", e))
    })?;

    match res {
        1 => Ok(HttpResponse::Ok().body("New user created!")),
        _ => Ok(HttpResponse::InternalServerError().body("Failed to create user.")),
    }
}

/// Handler for DELETE /users/{id}
#[delete("/users/{id}")]
pub async fn delete_users_id(
    pool: web::Data<DbPool>,
    _id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let id = _id.into_inner().clone();

    let conn = pool.get().expect("Could not establish db poolc onnection.");
    let _ = web::block(move || delete_user(&conn, id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().body(format!("{}", e))
        })?;

    Ok(HttpResponse::Ok().body(format!("User ID {} deleted.", &id)))
}

/// Handler request that just serves up an index page.
#[get("/")]
pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html")))
}

/// Handler for signup page.
#[get("/signup")]
pub async fn signup() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/signup.html")))
}

#[post("/signup")]
pub async fn signup_form(
    pool: web::Data<DbPool>,
    form: web::Form<NewUserInput>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Could not establish db pool connection.");

    let _ = web::block(move || {
        create_user(
            &conn,
            &NewUser {
                username: &form.username,
                password: &form.password,
            },
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().body(format!("{}", e))
    })?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/login_success.html")))
}

#[post("/login")]
pub async fn login_form(
    pool: web::Data<DbPool>,
    _req: web::HttpRequest,
    form: web::Form<NewUserInput>,
) -> Result<HttpResponse, Error> {
    let conn = pool
        .get()
        .expect("Could not establish connection to db pool.");

    let password = form.password.clone();
    let username = form.username.clone();
    // See if username exists in db.
    let user = web::block(move || get_user_by_username(&conn, username))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<h2>Invalid credentials.</h2>")
        });

    match user {
        // If user found
        Ok(u) => {
            if password == u.password {
                // If passwords match.
                Ok(HttpResponse::Ok().body("successfully logged in!"))
            } else {
                Ok(HttpResponse::Ok().body("Passwords did not match."))
            }
        }
        // Username does not exist.
        Err(_) => Ok(HttpResponse::InternalServerError().body("No such user exists.")),
    }
}

/// Handler for login page.
#[get("/")]
pub async fn login(_req: web::HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/login.html")))
}

/// Handler for create post page.
#[get("/posts/new")]
pub async fn get_posts_new() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/new_post.html")))
}

/// Handler for `POST /posts/new`
#[post("/posts/new")]
pub async fn post_posts_new(
    pool: web::Data<DbPool>,
    form: web::Form<NewPostInput>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to establish db pool connection.");

    let form = form.into_inner();

    let new_post = web::block(move || {
        posts::create_post(
            &conn,
            NewPost {
                title: &form.title,
                body: &form.body,
                published: true,
                author_id: form.author_id,
            },
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().body("whoops")
    });

    match new_post {
        Ok(_) => Ok(HttpResponse::Ok().body("New post created!")),
        Err(e) => Ok(HttpResponse::InternalServerError()
            .body(format!("whoops. something went wrong: {:?}", e))),
    }
}

#[get("/posts")]
pub async fn get_posts(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Could not establish db connection.");
    let posts = web::block(move || posts::get_posts(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().body("whoops. something went wrong.")
        });

    match posts {
        Ok(p) => Ok(HttpResponse::Ok().json(GetPostsResponse { posts: p })),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Whoops. Something went wrong.")),
    }
}
