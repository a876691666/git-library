diesel::table! {
    git_lists (id) {
        id -> Integer,
        name -> Text,
        url -> Text,
        description -> Text,
        tags -> Text,
        is_deleted -> Integer,
        created_at -> Integer,
        updated_at -> Integer,
        info -> Text,
        info_updated_at -> Integer,
    }
}