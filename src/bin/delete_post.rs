use diesel::prelude::*;
use diesel_intro::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{target}%");

    let conn = &mut establish_connection();
    let num_deleted =
        diesel::delete(posts.filter(title.like(pattern)))
            .execute(conn)
            .expect("Error deleting posts");

    println!("Deleted {num_deleted} posts");
}