use diesel::prelude::*;
use crate::db::project::project::{ projects, docs, templates, selections};
use chrono::NaiveDateTime;
use serde_json::Value as Json;
use serde::{Deserialize, Serialize};
use crate::models::custom_types::DocType;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Template {
  pub id: i32,
  pub author: i32,
  pub name: String,
  pub description: String,
  pub layout: Json,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewTemplate {
  pub name: String,
  pub description: String,
  pub layout: Json,
}

#[derive(Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertableTemplate {
  pub author: i32,
  pub name: String,
  pub description: String,
  pub layout: Json,
}

#[derive(Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EditTemplate {
  pub name: String,
  pub description: String,
  pub layout: Json,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
  pub id: i32,
  pub author: i32,
  pub name: String,
  pub title: String,
  pub field: String,
  pub public: bool,
  pub active: bool,
  pub owned: bool,
  pub org: Option<i32>,
  pub description: Option<String>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewProject {
  pub name: String,
  pub title: String,
  pub field: String,
  pub public: bool,
}


// Validate NewUser
impl NewProject {
	pub fn validate(&self) -> Result<NewProject, String> {
		// Check if required fields are present
		if self.name.len() < 2 || self.name.len() > 15 {
			return Err("Name must be between 2 and 15!".to_string());
		}

		// If all checks pass, return the validated NewUser
		Ok(self.clone())
	}
}

#[derive(Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertableProject {
  pub author: i32,
  pub name: String,
  pub title: String,
  pub field: String,
  pub public: bool,
  pub active: bool,
  pub owned: bool,
  pub org: Option<i32>,
}


#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = docs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Proposal {
  pub id: i32,
  pub template: i32,
  pub project: i32,
  pub kind: ProposalType,
  pub summery: String,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}


#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = selections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Selection {
  pub id: i32,
  pub org: i32,
  pub template: i32,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}

