// @generated automatically by Diesel CLI.

diesel::table! {
    git_lists (id) {
        id -> Nullable<Integer>,
        name -> Text,
        url -> Text,
        description -> Nullable<Text>,
        tags -> Nullable<Text>,
        is_deleted -> Nullable<Integer>,
        created_at -> Nullable<Integer>,
        updated_at -> Nullable<Integer>,
        info -> Nullable<Text>,
        info_updated_at -> Nullable<Integer>,
    }
}
