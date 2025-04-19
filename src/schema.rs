// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        article -> Varchar,
        username -> Varchar,
        content -> Text,
        likes -> Int4,
        dislikes -> Int4,
        edited -> Bool,
        is_reply -> Bool,
        reply_to -> Nullable<Varchar>,
    }
}
