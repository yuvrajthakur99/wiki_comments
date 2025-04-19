use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewComment, Comment};

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_comment(conn: &mut PgConnection, new_comment: NewComment) -> Comment {
    use crate::schema::comments;
    diesel::insert_into(comments::table)
        .values(&new_comment)
        .returning(Comment::as_returning())
        .get_result(conn)
        .expect("Error saving new comment")
}

