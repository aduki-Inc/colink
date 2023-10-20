use diesel::prelude::*;
use chrono::{NaiveDateTime, NaiveDate};
use serde_json::Value as Json;
use serde::{Deserialize, Serialize};
use crate::db::schema::sql_types::InstitutionType;

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
  pub in_type: InstitutionType,
  pub active: Option<bool>,
  pub location: Option<String>,
  pub about: Option<String>,
  pub established: Option<NaiveDate>,
  pub picture: Option<String>,
  pub created_at: Option<NaiveDateTime>,
}
