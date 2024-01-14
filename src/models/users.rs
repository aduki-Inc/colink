use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::db::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub password: String,
  pub email: String,
  pub name: String,
  pub active: Option<bool>,
  pub bio: Option<String>,
  pub dob: Option<NaiveDateTime>,
  pub picture: Option<String>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct LoggedUser {
  pub id: i32,
  pub username: String,
  pub password: String,
  pub email: String,
  pub name: String,
}

#[derive(Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
  pub username: String,
  pub password: String,
  pub email: String,
  pub name: String,
  pub active: Option<bool>,
  pub bio: Option<String>,
  pub dob: Option<NaiveDateTime>,
  pub picture: Option<String>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>,
}



// Validate NewUser
impl NewUser {
	pub fn validate(&self) -> Result<NewUser, String> {
		// Check if required fields are present
		if self.username.len() < 5 {
			return Err("Username must be 5 chars or more!".to_string());
		}

		if self.password.len() < 6 {
			return Err("Password must be 6 chars or more!".to_string());
		}

		// If all checks pass, return the validated NewUser
		Ok(self.clone())
	}
}


#[derive(Serialize, Deserialize)]
pub struct LoginData {
  pub user_key: String,
  pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Username {
  pub username: String,
}