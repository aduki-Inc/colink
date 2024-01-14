use diesel::prelude::*;
use chrono::NaiveDateTime;
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

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Section {
  pub id: i32,
  pub name: String,
  pub target_id: i32,
  pub target_name: String,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}


// Validate Section Data
impl Section {
	pub fn validate(&self) -> Result<Section, String> {
		// Check if required fields are present
		if self.name.len() < 3 {
			return Err("Username must be 3 chars or more!".to_string());
		}

		// If all checks pass, return the validated NewUser
		Ok(self.clone())
	}
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Role {
  pub id: i32,
  pub section: Option<i32>,
  pub type_: RoleType,
  pub privileges: Option<Json>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}