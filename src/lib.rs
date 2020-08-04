#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

pub mod models;
pub mod schema;

use models::{Post, NewPost};

pub fn get_server_port() -> u16 {
  env::var("PORT")
  .ok()
  .and_then(|p| p.parse().ok())
  .unwrap_or(8181)
}

pub fn get_server_host() -> String {
    env::var("HOST")
    .ok()
    .and_then(|p| p.parse().ok())
    .unwrap_or("0.0.0.0".to_string())
  }

pub fn get_db_url() -> String {
  env::var("DATABASE_URL")
  .ok()
  .and_then(|p| p.parse().ok())
  .unwrap_or("postgres://root:password@chronicler_db:5432/chronicler_db".to_string())
}

pub fn establish_connection() -> PgConnection {
    let database_url = get_db_url();
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let connection = establish_connection();
    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&connection)
        .expect("Error saving new post")
}

pub fn show_posts() -> Vec<models::Post>{
  use schema::posts::dsl::*;

  let connection = establish_connection();
  let results = posts
      // .filter(published.eq(true))
      // .limit(5)
      .load::<Post>(&connection)
      .expect("Error loading posts");

  println!("Displaying {} posts", results.len());
  results
}

pub fn delete_post(post_id: i32) -> Option<i32> {
    use schema::posts::dsl::*;
    let connection = establish_connection();
    let post_deleted = diesel::delete(posts.filter(id.eq(post_id)))
        .execute(&connection);

    match post_deleted {
        Ok(num_deleted) => {
            if num_deleted > 0 {
                Some(post_id)
            } else {
                None
            }
        },
        Err(_) => {
            None
        }
    }
}

pub fn toggle_post(post_id: i32, published_status: bool) -> Option<i32> {
    use schema::posts::dsl::*;
    let connection = establish_connection();
    let post_toggled = diesel::update(posts.filter(id.eq(post_id)))
        .set(published.eq(published_status))
        .execute(&connection);

    match post_toggled {
        Ok(_) => {
            Some(post_id)
        },
        Err(_) => {
            None
        }
    }
}