// @generated automatically by Diesel CLI.

diesel::table! {
  users (id) {
    id -> Int4,
    #[max_length = 250]
    username -> Varchar,
    #[max_length = 50]
    password -> Varchar,
    #[max_length = 250]
    email -> Varchar,
    #[max_length = 250]
    name -> Varchar,
    active -> Bool,
    bio -> Nullable<Text>,
    dob -> Timestamp,
    #[max_length = 500]
    picture -> Nullable<Varchar>,
    created_at -> Timestamp,
  }
}
