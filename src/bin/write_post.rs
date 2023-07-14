use diesel_intro::*;
use std::io::{stdin, Read};

fn main() {
    let conn = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("Please provide a title for the post:");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!("\nOK! Now write a body for {title} (Press {EOF} when finished)\n");
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(conn, title, &body);
    println!("\nSaved draft {title} with id {}", post.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";