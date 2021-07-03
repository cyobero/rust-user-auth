use clap::{App, Arg};
use user_auth::db::{create_user, establish_connection};
use user_auth::models::NewUser;

fn main() {
    let matches = App::new("create-user")
        .version("1.0")
        .author("Czar Yobero (cyobero@gmail.com)")
        .arg(
            Arg::with_name("username")
                .short("u")
                .long("username")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .long("password")
                .index(2)
                .required(true),
        )
        .get_matches();

    let conn = establish_connection();
    let username = matches.value_of("username").unwrap();
    let password = matches.value_of("password").unwrap();

    println!("Creating user...\n");
    let res = create_user(&conn, &NewUser { username, password });
    match res {
        Ok(_) => println!("Successfully created new user '{}'!", username),
        Err(e) => eprintln!("Error: {}", e),
    };
}
