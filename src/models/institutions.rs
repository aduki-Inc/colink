#[macro_use] extern crate diesel;
use diesel_enum::DbEnum;

use diesel::prelude::*;
use chrono::{NaiveDateTime, NaiveDate};
use serde_json::Value as Json;
use serde::{Deserialize, Serialize};
// use crate::diesel_derive_enum::DbEnum;


#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, DbEnum)]
#[sql_type = "VarChar"]
pub enum Status {
    /// Will be represented as `"reddy"`.
    #[val = "reddy"]
    Ready,
    /// Will be represented as `"pending"`.
    Pending,
}

#[derive(Debug, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::institution_type"]
pub enum InstitutionType {
  Vocational,
  HighSchool,
  College,
  University,
  Technical,
  Other,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::institutions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Institution {
  pub id: i32,
  pub short_name: String,
  pub name: String,
  pub logo: Option<String>,
  pub contact: Option<Json>,
  pub in_type: Option<InstitutionType>,
  pub active: Option<bool>,
  pub location: Option<String>,
  pub about: Option<String>,
  pub established: Option<NaiveDate>,
  pub picture: Option<String>,
  pub created_at: Option<NaiveDateTime>,
}
