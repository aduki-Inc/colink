use diesel::prelude::*;
use chrono::{NaiveDateTime, NaiveDate};
use serde_json::Value as Json;
use serde::{Deserialize, Serialize};
use crate::models::custom_types::RoleType;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::co_link)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Colink {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub logo: Option<String>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}

#[derive(Quarable, Selectable)]
#[diesel(table_name = crate::db::schema::sections)]
#[disel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Section {
  pub id: i32,
  pub name: String,
  pub target_id: i32,
  target_name: String
}