use diesel::prelude::*;
use crate::db::project::project::{ projects, proposals, templates, selections};
use chrono::NaiveDateTime;
use serde_json::Value as Json;
use serde::{Deserialize, Serialize};
use crate::models::custom_types::ProposalType;

#[derive(Queryable, Selectable)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Project {
  pub id: i32,
  pub author: i32,
  pub template: i32,
  pub title: String,
  pub field: String,
  pub type_: ProposalType,
  pub public: bool,
  pub active: bool,
  pub owned: bool,
  pub org: Option<i32>,
  pub description: Option<String>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = proposals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Proposal {
  pub id: i32,
  pub project: i32,
  pub summery: String,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Template {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub author: i32,
  pub layout: Json,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = selections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Selection {
  pub id: i32,
  pub org: i32,
  pub template: i32,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}

