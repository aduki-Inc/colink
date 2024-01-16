use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde_json::Value as Json;
use serde::{Deserialize, Serialize};
// use crate::db::schema::RoleType;
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

#[derive(Queryable, Selectable, Insertable, Clone, Serialize, Deserialize)]
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
		if self.name.len() < 2 {
			return Err("Section name must be 2 chars or more!".to_string());
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

// Validate SectionIdentity Data
impl SectionIdentity {
	pub fn validate(&self) -> Result<SectionIdentity, String> {
		// Check if required fields are present
		if self.id <= 0 {
			return Err("Section validation error: zero(0) was encountered for value(id)".to_string());
		}

		// If all checks pass, return the validated NewSection
		Ok(self.clone())
	}
}


#[derive(Queryable, Selectable, Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
  pub id: i32,
  pub section: i32,
  pub base: RoleType,
  pub author: i32,
  pub name: String,
  pub privileges: Option<Json>,
  pub expiry: Option<NaiveDateTime>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewRole {
  pub section: i32,
  pub base: RoleType,
  pub author: i32,
  pub name: String,
  pub privileges: Option<Json>,
  pub expiry: Option<i64>,
}

// Validate NewRole Data
impl NewRole {
	pub fn validate(&self) -> Result<NewRole, String> {
		// Check if required fields are present
		if self.name.len() < 3 {
			return Err("Role name must be 3 chars or more!".to_string());
		}

    if self.expiry.is_some() {
      if self.expiry <= Some(0) || self.expiry > Some(180) {
        return Err("Duration should not be less than 1 or greater than 180!".to_string())
      }
    }

		// If all checks pass, return the validated NewSection
		Ok(self.clone())
	}
}


#[derive(Queryable, Selectable, Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertableRole {
  pub section: i32,
  pub base: RoleType,
  pub author: i32,
  pub name: String,
  pub privileges: Option<Json>,
  pub expiry: Option<NaiveDateTime>,
}



#[derive(Clone, Serialize, Deserialize)]
pub struct RoleData {
  pub id: i32,
  pub section: i32,
  pub base: RoleType
}

// Validate RoleData Data
impl RoleData {
	pub fn validate(&self) -> Result<RoleData, String> {
		// Check if required fields are present
		if self.id <= 0 {
			return Err("Role validation error: zero(0) was encountered for value(id)".to_string());
		}

		// If all checks pass, return the validated NewSection
		Ok(self.clone())
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RolePrivileges {
  pub id: i32,
  pub section: i32,
  pub base: RoleType,
  pub privileges: Json
}

// Validate RolePrivileges Data
impl RolePrivileges {
	pub fn validate(&self) -> Result<RolePrivileges, String> {
		// Check if required fields are present
		if self.id <= 0 {
			return Err("Role validation error: zero(0) was encountered for value(id)".to_string());
		}

		// If all checks pass, return the validated NewSection
		Ok(self.clone())
	}
}


#[derive(Clone, Serialize, Deserialize)]
pub struct RoleExpiry {
  pub id: i32,
  pub section: i32,
  pub base: RoleType,
  pub expiry: i64
}

// Validate RoleExpiry Data
impl RoleExpiry {
	pub fn validate(&self) -> Result<RoleExpiry, String> {
		// Check if required fields are present
		if self.id <= 0 {
			return Err("Role validation error: zero(0) was encountered for value(id)".to_string());
		}

    if self.expiry <= 0 || self.expiry > 180 {
      return Err("Duration should not be less than 1 or greater than 180!".to_string())
    }

		// If all checks pass, return the validated NewSection
		Ok(self.clone())
	}
}