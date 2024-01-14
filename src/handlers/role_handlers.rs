use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use diesel::prelude::*;
use crate::db::schema::users::dsl::*;
use diesel::result::Error;
use crate::db::connection::establish_connection;
use crate::db::schema::{users, roles, section};
use crate::models::{system::{Colink, Section, Role},users::{User, LoggedUser, NewUser, LoginData, Username}};
use crate::configs::state::AppState;
use serde_json::json;
use crate::middlewares::auth_middleware::{JwtMiddleware, Claims};


// Define handler for user registration with JSON data.
pub async fn create_section(app_data: web::Data<AppState>, data: web::Json<NewUser>) -> impl Responder {
	// let mut counter = app_data.counter.lock().unwrap();
	// *counter += 1; // <- Access counters inside MutexGuard

	let mut conn = establish_connection(&app_data.config.database_url).await;

	// Collect Registration data from the body
	match data.validate() {
		Ok(registration_data) => {
			// Check if the email already exists
			if email_exists(&registration_data.email, &mut conn) {
				return HttpResponse::Conflict().json(
					json!({
						"success": false,
						"message": "Email already exists"
					})
				);
			}

			// Check if the username already exists
			if username_exists(&registration_data.username, &mut conn) {
				return HttpResponse::Conflict().json(
					json!({
						"success": false,
						"message": "Username already exists"
					})
				);
			}


			// Hash the user's password securely using bcrypt.
			let hashed_password = match hash(&registration_data.password, 12) {
				Ok(h) => h,
				Err(_) => return HttpResponse::InternalServerError().json(
					json!({
						"success": false,
						"error": "Password hashing failed"
					})
				),
			};

			let new_user = NewUser {
				username: registration_data.username,
				email: registration_data.email,
				password: hashed_password,
				name: registration_data.name,
				active: None,
				bio: None,
				dob: None,
				picture: None,
				created_at: None
			};

			match diesel::insert_into(users::table)
			.values(&new_user)
			.execute(&mut conn)
			{
				Ok(_) => return HttpResponse::Ok().json(
					json!({
						"success": true,
						"message": "User registered successfully"
					})
				),
				Err(err) => {
					// Handle the database error and return an error response
					return	HttpResponse::InternalServerError().json(
						json!({
							"success": false,
							"error": format!("Failed to register user: {}", err.to_string())
						})
					)
				}
			}
		}
		Err(err) => {
			// Directly return the HttpResponse
			return HttpResponse::BadRequest().json(
				json!({
					"success": false,
					"error": err.to_string()
				})
			)
		}
	}


}