use diesel::prelude::*;
use chrono::{NaiveDateTime, NaiveDate};
use serde_json::Value as Json;
use serde::{Deserialize, Serialize};
use crate::models::custom_types::ProposalType;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Project {
  pub id: i32,
  pub author: i32,
  pub template: i32,
  pub title: String,
  pub field: String,
  pub type_: ProposalType,
  pub public: Option<bool>,
  pub active: Option<bool>,
  pub owned: Option<bool>,
  pub institution: Option<i32>,
  pub description: Option<String>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Template {
  pub id: i32,
  pub author: i32,
  pub template: i32,
  pub title: String,
  pub field: String,
  pub type_: ProposalType,
  pub public: Option<bool>,
  pub active: Option<bool>,
  pub owned: Option<bool>,
  pub institution: Option<i32>,
  pub description: Option<String>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}
