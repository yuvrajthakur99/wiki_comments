use actix_web::{web::{Json, Path}, App, HttpServer, Responder, HttpResponse, post};
use wiki_comments::models::{NewComment, Comment};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Feedback {
    like: bool,
    dislike: bool,
}

#[post("/comments/add")]
async fn adding_comment(new_comment: Json<NewComment>) -> impl Responder {
    let connection = &mut wiki_comments::establish_connection();
    let comment = wiki_comments::create_comment(connection, new_comment.into_inner());
    let response = format!("{} your comment got added", comment.username);
    HttpResponse::Ok().body(response)
}

#[post("/comments/update/feedback/{id}")]
async fn update_feedback(id: Path<i32>, feedback: Json<Feedback>) -> impl Responder {
    use wiki_comments::schema::comments::dsl::{comments, likes, dislikes};
    let connection = &mut wiki_comments::establish_connection();
    let id = &id.into_inner();
    let feedback = &feedback.into_inner();
    match feedback.like {
        true => {
            diesel::update(comments.find(id))
                .set(likes.eq(likes + 1))
                .execute(connection);
        }
        false => {
            diesel::update(comments.find(id))
                .set(likes.eq(likes - 1))
                .execute(connection);
        }
    };

    match feedback.dislike {
        true => {
            diesel::update(comments.find(id))
                .set(dislikes.eq(dislikes + 1))
                .execute(connection);
        }
        false => {
            diesel::update(comments.find(id))
                .set(dislikes.eq(dislikes - 1))
                .execute(connection);
        }
    };
    let comment = comments
        .find(id)
        .select(Comment::as_select())
        .first(connection)
        .expect("Error loading comment");

    HttpResponse::Ok().body(format!("{comment:?}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(adding_comment)
        .service(update_feedback)
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
