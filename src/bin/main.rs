use actix_web::{web, App, HttpServer, Responder, HttpResponse, post};
use wiki_comments::models::{NewComment};

#[post("/comments")]
async fn adding_comment(new_comment: web::Json<NewComment>) -> impl Responder {
    let connection = &mut wiki_comments::establish_connection();
    let comment = wiki_comments::create_comment(connection, new_comment.into_inner());
    let response = format!("{} your comment got added", comment.username);
    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(adding_comment)
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
