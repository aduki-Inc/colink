use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde_json::Value as Json;
use serde::{Deserialize, Serialize};
use crate::models::custom::{RoleType, SectionType, LogType, ActionType};
use crate::db::platform::platform;



// - Colink
#[derive(Queryable, Selectable)]
#[diesel(table_name = platform::co_link)]
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

// - Section
#[derive(Debug, Queryable, Selectable, Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = platform::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Section {
  pub id: i32,
  pub kind: SectionType,
  pub identity: String,
  pub target: i32,
  pub name: String,
  pub description: Option<String>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}


#[derive(Insertable, Clone, Serialize, Deserialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = platform::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSection {
  pub identity: String,
  pub target: i32,
  pub name: String,
  pub description: Option<String>,
}

// Validate Section Data
impl NewSection {
	pub fn validate(&self) -> Result<NewSection, String> {
		// Check if required fields are present
		if self.identity.len() < 2 || self.identity.len() > 250 {
			return Err("Section name must between 2 and 250 chars!".to_string());
		}

    if self.description.is_some() {
      if self.description.clone().unwrap().len() < 2 || self.description.clone().unwrap().len() > 250 {
        return Err("Section name must be between 2 and 500 chars!".to_string());
      }
    }

		// If all checks pass, return the validated NewSection
		Ok(self.clone())
	}
}

#[derive(Insertable, Clone, Serialize, Deserialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = platform::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SectionIdentity {
  pub id: i32,
  pub identity: String
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



// - Roles
#[derive(Debug, Queryable, Selectable, Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = platform::roles)]
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
		if self.name.len() < 3 || self.name.len() > 500 {
			return Err("Role name must be between 3 and 500 chars!".to_string());
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


#[derive(Debug, Queryable, Selectable, Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = platform::roles)]
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


// - Approvals
#[derive(Queryable, Selectable, Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = platform::approvals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Approval {
  pub id: i32,
  pub target: i32,
  pub name: String,
  pub approved: Option<bool>,
  pub description: Option<String>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}

#[derive(Queryable, Selectable, Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = platform::approvals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertableApproval {
  pub target: i32,
  pub name: String,
  pub approved: Option<bool>,
  pub description: Option<String>,
}


// - Logs
#[derive(Debug, Queryable, Selectable, Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = platform::logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Log {
  pub id: i32,
  pub audit: LogType,
  pub author: i32,
  pub target: i32,
  pub name: String,
  pub action: ActionType,
  pub verb: String,
  pub created_at: Option<NaiveDateTime>
}


// - Logs
#[derive(Debug, Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = platform::logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertableLog {
  pub audit: LogType,
  pub author: i32,
  pub target: i32,
  pub name: String,
  pub action: ActionType,
  pub verb: String,
}
