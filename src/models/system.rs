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

#[derive(Insertable, Clone, Serialize, Deserialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Section {
  pub id: i32,
  pub name: String,
  pub target_id: i32,
  pub target_name: String,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}


#[derive(Insertable, Clone, Serialize, Deserialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSection {
  pub name: String,
  pub target_id: i32,
  pub target_name: String
}


// Validate Section Data
impl NewSection {
	pub fn validate(&self) -> Result<NewSection, String> {
		// Check if required fields are present
		if self.name.len() < 3 {
			return Err("Section name must be 3 chars or more!".to_string());
		}

		// If all checks pass, return the validated NewSection
		Ok(self.clone())
	}
}

#[derive(Insertable, Clone, Serialize, Deserialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SectionIdentity {
  pub id: i32,
  pub name: String
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Role {
  pub id: i32,
  pub section: i32,
  pub type_: RoleType,
  pub name: String,
  pub author: i32,
  pub privileges: Option<Json>,
  pub expiry: Option<NaiveDateTime>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct NewRole {
  pub section: i32,
  pub type_: RoleType,
  pub name: String,
  pub author: i32,
  pub privileges: Option<Json>,
  pub expiry: Option<i32>,
}

// Validate Section Data
impl NewRole {
	pub fn validate(&self) -> Result<NewRole, String> {
		// Check if required fields are present
		if self.name.len() < 3 {
			return Err("Role name must be 3 chars or more!".to_string());
		}

    if self.expiry {
      if self.expiry <=0 && self.expiry > 180 {
        return Err("Duration must be between 1 and 180 days!".to_string())
      }
    }

		// If all checks pass, return the validated NewSection
		Ok(self.clone())
	}
}