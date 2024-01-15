// use diesel::backend::Backend;
// use diesel::expression::AsExpression;
// use diesel::deserialize::{self, FromSql};
// use diesel::pg::Pg;
// use std::io::Write;
// use diesel::serialize::{self, Output, ToSql};
// use diesel::sql_types::Text;
// use diesel::FromSqlRow;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[derive(Debug, DbEnum)]
#[ExistingTypePath = "crate::db::schema::sql_types::InstitutionType"]
pub enum InstitutionType {
  Vocational,
  High,
  College,
  University,
  Technical,
  Other,
}

#[derive(Serialize, Deserialize, Clone)]
#[derive(Debug, DbEnum)]
#[ExistingTypePath = "crate::db::schema::sql_types::ProposalType"]
pub enum ProposalType {
  Proposal,
  Revised,
  Supplemental,
  Continuation,
  Notice,
  Solicited,
  Other
}

#[derive(Clone, Serialize, Deserialize, Debug, DbEnum)]
#[ExistingTypePath = "crate::db::schema::sql_types::RoleType"]
pub enum RoleType {
  Owner,
  Admin,
  Staff,
  Period
}
