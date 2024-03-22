// @generated automatically by Diesel CLI.

pub mod account {
    diesel::table! {
        account.contacts (id) {
            id -> Int4,
            user_from -> Int4,
            user_to -> Int4,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        account.users (id) {
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
            updated_at -> Nullable<Timestamptz>,
        }
    }

    diesel::allow_tables_to_appear_in_same_query!(
        contacts,
        users,
    );
}
