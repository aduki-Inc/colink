// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "institution_type"))]
    pub struct InstitutionType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "proposal_type"))]
    pub struct ProposalType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "section_type"))]
    pub struct SectionType;
}

diesel::table! {
    co_link (id) {
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
    use super::sql_types::InstitutionType;

    institutions (id) {
        id -> Int4,
        #[max_length = 250]
        short_name -> Varchar,
        #[max_length = 500]
        name -> Varchar,
        #[max_length = 500]
        logo -> Nullable<Varchar>,
        contact -> Nullable<Json>,
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

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProposalType;

    projects (id) {
        id -> Int4,
        author -> Int4,
        template -> Int4,
        #[max_length = 500]
        title -> Varchar,
        #[max_length = 500]
        field -> Varchar,
        #[sql_name = "type"]
        type_ -> ProposalType,
        public -> Nullable<Bool>,
        active -> Nullable<Bool>,
        owned -> Bool,
        institution -> Nullable<Int4>,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    proposals (id) {
        id -> Int4,
        project -> Int4,
        summery -> Text,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        section -> Nullable<Int4>,
        author -> Int4,
        #[max_length = 500]
        name -> Nullable<Varchar>,
        privileges -> Nullable<Json>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SectionType;

    sections (id) {
        id -> Int4,
        #[max_length = 500]
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> SectionType,
        target_id -> Int4,
        #[max_length = 500]
        target_name -> Varchar,
    }
}

diesel::table! {
    templates (id) {
        id -> Int4,
        #[max_length = 500]
        name -> Varchar,
        description -> Text,
        layout -> Nullable<Json>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

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
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(projects -> institutions (institution));
diesel::joinable!(projects -> templates (template));
diesel::joinable!(projects -> users (author));
diesel::joinable!(proposals -> projects (project));
diesel::joinable!(roles -> sections (section));
diesel::joinable!(roles -> users (author));

diesel::allow_tables_to_appear_in_same_query!(
    co_link,
    institutions,
    projects,
    proposals,
    roles,
    sections,
    templates,
    users,
);
