use self::models::*;
use diesel::prelude::*;
use diesel_intro::*;

fn main() {
    /* Imports aliases to allow us to say posts instead of
    posts::table and published instead of posts::published.
    Useful when using one table, but can muddle with multiple tables.
    Keep import inside current function to prevent polluting
    module namespace */
    use self::schema::posts::dsl::*;

    //Establish connection
    let connection = &mut establish_connection();
    
    //Load posts with limit 5, filtered to published=true
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}