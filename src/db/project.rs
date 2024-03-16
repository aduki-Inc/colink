// @generated automatically by Diesel CLI.

pub mod project {
    pub mod sql_types {
        #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "proposal_type"))]
        pub struct ProposalType;
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::ProposalType;

        project.projects (id) {
            id -> Int4,
            author -> Int4,
            template -> Int4,
            #[max_length = 250]
            name -> Varchar,
            #[max_length = 500]
            title -> Varchar,
            #[max_length = 500]
            field -> Varchar,
            #[sql_name = "type"]
            type_ -> ProposalType,
            public -> Bool,
            active -> Bool,
            owned -> Bool,
            org -> Nullable<Int4>,
            description -> Nullable<Text>,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        project.proposals (id) {
            id -> Int4,
            project -> Int4,
            summery -> Text,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        project.selections (id) {
            id -> Int4,
            org -> Int4,
            template -> Int4,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        project.templates (id) {
            id -> Int4,
            author -> Int4,
            #[max_length = 500]
            name -> Varchar,
            description -> Text,
            layout -> Jsonb,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
        }
    }

    diesel::joinable!(projects -> templates (template));
    diesel::joinable!(proposals -> projects (project));
    diesel::joinable!(selections -> templates (template));

    diesel::allow_tables_to_appear_in_same_query!(
        projects,
        proposals,
        selections,
        templates,
    );
}
