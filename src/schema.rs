// @generated automatically by Diesel CLI.

diesel::table! {
    invitations (id) {
        id -> Uuid,
        email -> Varchar,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        email -> Varchar,
        hash -> Varchar,
        created_at -> Timestamp,
        last_login -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    invitations,
    users,
);
