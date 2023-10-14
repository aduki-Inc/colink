// @generated automatically by Diesel CLI.

diesel::table! {
  users (id) {
    id -> Integer,
    username -> Varchar,
    password -> Varchar,
    email -> Varchar,
    name -> Varchar,
    active -> Bool,
    bio -> Varchar,
    dob -> Date,
    picture -> Varchar,
    created_at -> Timestamp,
  }
}