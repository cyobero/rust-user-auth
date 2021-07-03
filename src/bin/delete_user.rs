use clap::{App, Arg};
use user_auth::db::{delete_user, establish_connection};

fn main() {
    let matches = App::new("delete_user")
        .version("1.0")
        .author("Czar Yobero (cyobero@gmail.com)")
        .arg(
            Arg::with_name("id")
                .short("i")
                .long("user-id")
                .index(1)
                .required(true),
        )
        .get_matches();

    let id = matches.value_of("id").unwrap().parse::<i32>().unwrap();
    let conn = establish_connection();
    let res = delete_user(&conn, id);
    match res {
        Ok(_) => println!("User id: {} 'successfully' deleted.", id),
        Err(e) => eprintln!("Error: {}", e),
    };
}
