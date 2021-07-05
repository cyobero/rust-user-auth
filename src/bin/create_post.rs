use clap::{App, Arg};
use user_auth::db::establish_connection;
use user_auth::models::NewPost;
use user_auth::posts::create_post;

fn main() {
    let conn = establish_connection();
    let matches = App::new("create-post")
        .version("1.0")
        .author("Czar Yobero (cyobero@gmail.com)")
        .arg(
            Arg::with_name("title")
                .short("t")
                .long("title")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("body")
                .short("b")
                .takes_value(true)
                .long("body")
                .required(false),
        )
        .arg(
            Arg::with_name("author-id")
                .short("a")
                .long("author-id")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let title = matches.value_of("title");
    let body = matches.value_of("body");
    let id: Result<i32, _> = matches.value_of("author-id").unwrap().parse();
    let res = create_post(
        &conn,
        NewPost {
            title: title.unwrap(),
            body: body.unwrap(),
            published: false,
            author_id: id.unwrap(),
        },
    );
    match res {
        Ok(_) => println!("New post successfully created!"),
        Err(_) => println!("Ooops. Something went wrong."),
    };
}
