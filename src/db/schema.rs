// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "institution_type"))]
    pub struct InstitutionType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "org_type"))]
    pub struct OrgType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "proposal_type"))]
    pub struct ProposalType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role_type"))]
    pub struct RoleType;
}

diesel::table! {
    approvals (id) {
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
    belongs (id) {
        id -> Int4,
        active -> Nullable<Bool>,
        author -> Int4,
        org -> Int4,
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
    use super::sql_types::OrgType;
    use super::sql_types::InstitutionType;

    orgs (id) {
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
    proposals (id) {
        id -> Int4,
        project -> Int4,
        summery -> Text,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RoleType;

    roles (id) {
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
    sections (id) {
        id -> Int4,
        #[max_length = 250]
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

diesel::table! {
    templates (id) {
        id -> Int4,
        #[max_length = 500]
        name -> Varchar,
        description -> Text,
        layout -> Nullable<Jsonb>,
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

diesel::joinable!(belongs -> orgs (org));
diesel::joinable!(belongs -> users (author));
diesel::joinable!(projects -> orgs (org));
diesel::joinable!(projects -> templates (template));
diesel::joinable!(projects -> users (author));
diesel::joinable!(proposals -> projects (project));
diesel::joinable!(roles -> sections (section));
diesel::joinable!(roles -> users (author));

diesel::allow_tables_to_appear_in_same_query!(
    approvals,
    belongs,
    co_link,
    orgs,
    projects,
    proposals,
    roles,
    sections,
    templates,
    users,
);
