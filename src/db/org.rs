// @generated automatically by Diesel CLI.

pub mod org {
    pub mod sql_types {
        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "institution_type"))]
        pub struct InstitutionType;

        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "org_type"))]
        pub struct OrgType;
    }

    diesel::table! {
        org.belongs (id) {
            id -> Int4,
            active -> Nullable<Bool>,
            author -> Int4,
            org -> Int4,
            section -> Int4,
            #[max_length = 500]
            name -> Varchar,
            #[max_length = 500]
            identity -> Varchar,
            #[max_length = 500]
            title -> Varchar,
            staff -> Nullable<Bool>,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::OrgType;
        use super::sql_types::InstitutionType;

        org.orgs (id) {
            id -> Int4,
            #[max_length = 250]
            short_name -> Varchar,
            #[max_length = 500]
            name -> Varchar,
            #[max_length = 500]
            logo -> Nullable<Varchar>,
            contact -> Nullable<Jsonb>,
            base -> OrgType,
            in_type -> InstitutionType,
            active -> Nullable<Bool>,
            #[max_length = 500]
            location -> Nullable<Varchar>,
            about -> Nullable<Text>,
            established -> Nullable<Date>,
            #[max_length = 500]
            picture -> Nullable<Varchar>,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
        }
    }

    diesel::joinable!(belongs -> orgs (org));

    diesel::allow_tables_to_appear_in_same_query!(
        belongs,
        orgs,
    );
}
