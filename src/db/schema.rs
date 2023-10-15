// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 250]
        username -> Varchar,
        #[max_length = 500]
        password -> Varchar,
        #[max_length = 250]
        email -> Varchar,
        #[max_length = 250]
        name -> Varchar,
        active -> Nullable<Bool>,
        bio -> Nullable<Text>,
        dob -> Nullable<Timestamptz>,
        #[max_length = 500]
        picture -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
    }
}
