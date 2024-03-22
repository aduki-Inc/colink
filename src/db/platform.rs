// @generated automatically by Diesel CLI.

pub mod platform {
    pub mod sql_types {
        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "action_type"))]
        pub struct ActionType;

        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "log_type"))]
        pub struct LogType;

        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "role_type"))]
        pub struct RoleType;

        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "section_type"))]
        pub struct SectionType;
    }

    diesel::table! {
        platform.approvals (id) {
            id -> Int4,
            target -> Int4,
            #[max_length = 250]
            name -> Varchar,
            approved -> Nullable<Bool>,
            description -> Nullable<Text>,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        platform.co_link (id) {
            id -> Int4,
            #[max_length = 500]
            name -> Varchar,
            description -> Text,
            #[max_length = 500]
            logo -> Nullable<Varchar>,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::LogType;
        use super::sql_types::ActionType;

        platform.logs (id) {
            id -> Int4,
            audit -> LogType,
            author -> Int4,
            target -> Int4,
            #[max_length = 250]
            name -> Varchar,
            action -> ActionType,
            #[max_length = 500]
            verb -> Varchar,
            created_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::RoleType;

        platform.roles (id) {
            id -> Int4,
            section -> Int4,
            base -> RoleType,
            author -> Int4,
            #[max_length = 500]
            name -> Varchar,
            privileges -> Nullable<Jsonb>,
            expiry -> Nullable<Timestamptz>,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::SectionType;

        platform.sections (id) {
            id -> Int4,
            kind -> SectionType,
            #[max_length = 300]
            identity -> Varchar,
            target -> Int4,
            #[max_length = 500]
            name -> Varchar,
            #[max_length = 500]
            description -> Nullable<Varchar>,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
        }
    }

    diesel::joinable!(roles -> sections (section));

    diesel::allow_tables_to_appear_in_same_query!(
        approvals,
        co_link,
        logs,
        roles,
        sections,
    );
}
