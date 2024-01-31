use diesel::prelude::*;
use chrono::{NaiveDateTime, NaiveDate, Local};
use serde_json::Value as Json;
use serde::{Deserialize, Serialize};
use crate::models::custom_types::{InstitutionType, OrgType};


#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct OrgPermission {
  pub title: String,
  pub name: String
}

#[derive(Queryable, Selectable, Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::orgs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Organization {
  pub id: i32,
  pub short_name: String,
  pub name: String,
  pub logo: Option<String>,
  pub contact: Option<Json>,
  pub base: OrgType,
  pub in_type: InstitutionType,
  pub active: Option<bool>,
  pub location: Option<String>,
  pub about: Option<String>,
  pub established: Option<NaiveDate>,
  pub picture: Option<String>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrganizationInfo {
  pub name: String,
  pub location: Option<String>,
  pub about: Option<String>,
  pub established: Option<NaiveDate>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrganizationContact {
  pub contact: Option<Json>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewOrganization {
  pub short_name: String,
  pub name: String,
  pub base: OrgType,
  pub in_type: InstitutionType,
  pub active: Option<bool>,
  pub established: Option<String>
}

// Validate NewOrganization
impl NewOrganization {
	pub fn validate(&self) -> Result<NewOrganization, String> {
		// Check if required fields are present
		if self.short_name.len() < 2 || self.short_name.len() > 100 {
			return Err("Short name(abbreviated name) must be between 2 and 100 chars!".to_string());
		}

    if self.name.len() < 5 || self.name.len() > 500 {
			return Err("Name must be between 5 and 500 chars!".to_string());
		}

    if self.established.is_some() {
      let parse_date = NaiveDate::parse_from_str(&self.established.clone().unwrap(), "%Y-%m-%d");

      if parse_date.is_err() {
        return  Err("Error converting the date!".to_string());
      } else {
        let today_date = Local::now().naive_utc().date();
        if today_date < parse_date.unwrap() || today_date == parse_date.unwrap() {
          return Err("Establishment date cannot be today or in the future!".to_string());
        }
      }
  
    }
		// If all checks pass, return the validated NewSection
		Ok(self.clone())
	}
}


#[derive(Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::orgs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertableOrganization{
  pub short_name: String,
  pub name: String,
  pub base: OrgType,
  pub in_type: InstitutionType,
  pub active: Option<bool>,
  pub established: Option<NaiveDate>
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::belongs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Belong {
  pub id: i32,
  pub active: Option<bool>,
  pub author: i32,
  pub org: i32,
  pub section: i32,
  pub name: String,
  pub identity: String,
  pub title: String,
  pub staff: Option<bool>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}


#[derive(Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::belongs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertableBelong {
  pub author: i32,
  pub org: i32,
  pub section: i32,
  pub name: String,
  pub identity: String,
  pub title: String,
  pub staff: Option<bool>
}


#[derive(Clone, Serialize, Deserialize)]
pub struct BelongIntermediate {
  pub user: i32,
  pub section: i32,
  pub date: Option<String>
}


#[derive(Clone, Serialize, Deserialize)]
pub struct NewBelong {
  pub author: i32,
  pub name: String,
  pub identity: String,
  pub title: String,
  pub date: Option<String>,
  pub staff: Option<bool>
}


// Validate NewBelong
impl NewBelong {
	pub fn validate(&self) -> Result<NewBelong, String> {
		// Check if required fields are present
		if self.name.len() < 5 || self.name.len() > 500 {
			return Err("Member name must be between 5 and 500 chars!".to_string());
		}

    if self.identity.len() < 2 || self.identity.len() > 100 {
			return Err("Identity/Reg. Number must be between 5 and 100 chars!".to_string());
		}

    if self.date.is_some() {
      let parse_date = NaiveDate::parse_from_str(&self.date.clone().unwrap(), "%Y-%m-%d");

      if parse_date.is_err() {
        return  Err("Error converting the date!".to_string());
      } else {
        let today_date = Local::now().naive_utc().date();
        if today_date > parse_date.unwrap() || today_date == parse_date.unwrap() {
          return Err("Member Expiry date cannot be today or in the past!".to_string());
        }
      }
  
    }
		Ok(self.clone())
	}
}

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct EditBelong {
  pub id: i32,
  pub name: String,
  pub identity: String,
  pub title: String,
}

// Validate EditBelong
impl EditBelong {
	pub fn validate(&self) -> Result<EditBelong, String> {
		// Check if required fields are present
		if self.name.len() < 5 || self.name.len() > 500 {
			return Err("Member name must be between 5 and 500 chars!".to_string());
		}

    if self.identity.len() < 2 || self.identity.len() > 100 {
			return Err("Identity/Reg. Number must be between 5 and 100 chars!".to_string());
		}

		Ok(self.clone())
	}
}


#[derive(Debug,Copy, Serialize, Deserialize, Clone)]
pub struct BelongStaff {
  pub id: i32,
  pub staff: bool
}

#[derive(Debug, Copy, Serialize, Deserialize, Clone)]
pub struct BelongIdentity {
  pub id: i32,
  pub author: i32
}

