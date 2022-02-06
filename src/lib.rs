pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

// Connect
fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

/*--- CRUD ---*/
use self::models::{Post, NewPost};

// Create
pub fn create_the_draft(title: &str, body: &str) -> Post {
    use schema::posts;

    let connection = establish_connection();

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)  // Post-> id:i32 and published:bool is auto
        .values(&new_post)
        .get_result(&connection)
        .expect("Error saving new post")
}

pub fn create_post_publication(post: &Post) {
    use schema::posts::dsl::{posts, published};

    let connection = establish_connection();

    diesel::update(posts.find(post.id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", post.id));
}

// Read
pub fn read_posts(pattern: Option<&str>) -> Vec<Post> {
    use schema::posts::dsl::{posts, title, published};

    let connection = establish_connection();

    if let Some(pattern) = pattern {
        posts.filter(title.like(pattern))
            // .limit(limit)
            .load::<Post>(&connection)
            .expect("Error loading posts")
    } else {
        posts.filter(published.eq(true))
            .load::<Post>(&connection)
            .expect("Error loading posts")
    }
}

// Update
pub fn update_post(post: &Post, new_title: &str, new_body: &str) {
    use schema::posts::dsl::{posts, title, body, published};

    let connection = establish_connection();

    diesel::update(posts.find(post.id))
        .set((
            title.eq(new_title),
            body.eq(new_body),
            published.eq(true)
        ))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", post.id));
}

// Delete
pub fn delete_post(post: &Post) {
    use schema::posts::dsl::{posts};

    let connection = establish_connection();
    
    diesel::delete(posts.find(post.id))
        .execute(&connection)
        .expect("Error deleting posts");
}
