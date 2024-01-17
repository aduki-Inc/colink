use diesel::prelude::*;
use chrono::{NaiveDateTime, NaiveDate, Local};
use serde_json::Value as Json;
use serde::{Deserialize, Serialize};
use crate::models::custom_types::{InstitutionType, OrgType};

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
pub struct NewOrganization {
  pub short_name: String,
  pub name: String,
  pub base: OrgType,
  pub in_type: InstitutionType,
  pub active: Option<bool>,
  pub established: Option<String>
}

// Validate NewOrganization Data
impl NewOrganization {
	pub fn validate(&self) -> Result<NewOrganization, String> {
		// Check if required fields are present
		if self.short_name.len() < 2 {
			return Err("Short name(abbreviated name) must be 2 chars or more!".to_string());
		}

    if self.name.len() < 5 {
			return Err("Name must be 5 chars or more!".to_string());
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


#[derive(Queryable, Selectable, Insertable, Clone, Serialize, Deserialize)]
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


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::belongs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Belong {
  pub id: i32,
  pub author: i32,
  pub org: i32,
  pub name: String,
  pub identity: String,
  pub title: String,
  pub staff: Option<bool>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>
}
