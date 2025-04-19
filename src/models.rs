use diesel::prelude::*;
use crate::schema::comments;
use serde::Deserialize;

#[derive(Queryable, Selectable)]
#[diesel(table_name = comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: i32,
    pub article: String,
    pub username: String,
    pub content: String,
    pub likes: i32,
    pub dislikes: i32,
    pub edited: bool,
    pub is_reply: bool,
    pub reply_to: Option<String>,
}

#[derive(Deserialize)]
#[derive(Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub article: String,
    pub username: String,
    pub content: String,
    pub likes: i32,
    pub dislikes: i32,
    pub is_reply: bool,
    pub reply_to: Option<String>,
}

