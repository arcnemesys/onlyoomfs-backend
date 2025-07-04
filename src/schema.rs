// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password_hash -> Text,
        latitude -> Nullable<Float>,
        longitude -> Nullable<Float>,
        real_name -> Nullable<Text>,
        bio -> Nullable<Text>,
    }
}
