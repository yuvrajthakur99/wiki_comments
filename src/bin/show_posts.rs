use diesel::prelude::*;
use self::models::*;
use wiki_comments::*;

fn main() {
    use self::schema::comments::dsl::*;

    let connection = &mut establish_connection();
    let results = comments
        .filter(is_reply.eq(false))
        .select(Comment::as_select())
        .load(connection)
        .expect("Error loading comments");
    for comment in results {
        println!("{}", comment.username);
        println!("-----------------------------");
        println!("{}", comment.content);
    }
}
