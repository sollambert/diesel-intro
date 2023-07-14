use self::models::Post;
use diesel::prelude::*;
use diesel_intro::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::*;
    use diesel::result::Error;

    let update_id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let conn = &mut establish_connection();

    let results =
    conn.transaction::<_, Error, _>(|conn: &mut SqliteConnection| {
        let updated_count =
        diesel::update(posts.find(update_id))
            .set(published.eq(true))
            .execute(conn)?;

        Ok(posts
            .order(id.desc())
            .limit(updated_count as i64)
            .select(Post::as_select())
            .load(conn))
    }).unwrap();

    let updated_post = &results.unwrap()[0];
    println!("Published post {}", updated_post.title);
}