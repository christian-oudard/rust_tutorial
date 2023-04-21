// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        text -> Text,
        completed -> Bool,
    }
}
