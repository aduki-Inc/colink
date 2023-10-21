use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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