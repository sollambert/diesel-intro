use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

use self::models::{NewPost, Post};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;
    use diesel::result::Error;
    let new_post = NewPost { title, body };

    /* Roundabout way to implement returning inserted object
    for SQLite as Diesel doesn't support last_insert_rowid() */
    let results =
    conn.transaction::<_, Error, _>(|conn: &mut SqliteConnection| {
        let inserted_count = diesel::insert_into(posts::table)
            .values(&new_post)
            .execute(conn)?;

        Ok(posts::table
            .order(posts::dsl::id.desc())
            .limit(inserted_count as i64)
            .select(Post::as_select())
            .load(conn))
    }).unwrap();

    let inserted_post = &results.unwrap()[0];
    inserted_post.to_owned()
}