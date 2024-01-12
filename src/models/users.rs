use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::db::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
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
#[diesel(table_name = crate::db::schema::users)]
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
#[table_name = "users"]
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
}

// Validate NewUser
impl NewUser {
	pub fn validate(&self) -> Result<NewUser, String> {
		// Check if required fields are present
		if self.username.is_empty() {
			return Err("Username is required".to_string());
		}

		if self.email.is_empty() {
			return Err("Email is required".to_string());
		}

		if self.password.is_empty() {
			return Err("Password is required".to_string());
		}

		if self.name.is_empty() {
			return Err("Password is required".to_string());
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