use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use diesel::{ prelude::*, result::Error };
use bcrypt::{hash, verify};
use crate::db::{
	connection::establish_connection,
	account::account::{
		users, users::dsl::*
	}
};
use crate::configs::state::AppState;
use serde_json::json;
use crate::middlewares::{
	log::log::create_log,
	auth::auth::{
		email_or_username_exists, generate_jwt, JwtMiddleware, Claims
	}
};

use crate::models::{
	custom::{ActionType, LogType},
	users::{User, LoggedUser, NewUser, LoginData, Username},
	platform::InsertableLog
};

// Define handler for user registration with JSON data.
pub async fn register_user(app_data: web::Data<AppState>, data: web::Json<NewUser>) -> impl Responder {
	// let mut counter = app_data.counter.lock().unwrap();
	// *counter += 1; // <- Access counters inside MutexGuard
	let mut conn = establish_connection(&app_data.config.database_url).await;

	// Collect Registration data from the body
	return match data.validate() {
		Ok(registration_data) => {
			
			// Check if email or username exists
			let (exists, msg) = email_or_username_exists(
				&registration_data.email,
				&registration_data.username, &mut conn
			).await;
			
			if exists {
				return HttpResponse::Conflict().json(
					json!({
						"success": false,
						"message": msg
					})
				);
			}
			
			
			// Hash the user's password securely using bcrypt.
			let hashed_password = match hash(&registration_data.password, 12) {
				Ok(h) => h,
				Err(_) => return HttpResponse::InternalServerError().json(
					json!({
						"success": false,
						"message": "Password hashing failed"
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
				created_at: None,
				updated_at: None
			};
			
			match diesel::insert_into(users::table)
				.values(&new_user)
				.get_result::<User>(&mut conn)
			{
				Ok(_) => {
					HttpResponse::Ok().json(
						json!({
							"success": true,
							"message": "User registered successfully"
						})
					)
				}
				Err(err) => {
					// Create new log & save it to db
					let new_log = InsertableLog {
						audit: LogType::Error,
						author: 0,
						target: 0,
						name: "database".to_owned(),
						action: ActionType::Create,
						verb: format!("User registration failed: {}", err.to_string())
					};
					create_log(&new_log, &mut conn).await;
					
					// Handle the database error and return an error response
					HttpResponse::InternalServerError().json(
						json!({
							"success": false,
							"message": "Failed to register user: Internal Error occurred!"
						})
					)
				}
			}
		}
		Err(err) => {
			// Directly return the HttpResponse
			HttpResponse::ExpectationFailed().json(
				json!({
					"success": false,
					"message": err.to_string()
				})
			)
		}
	};
}


// Define handler for user login
pub async fn login_user(app_data: web::Data<AppState>, data: web::Json<LoginData>) -> impl Responder {
	// let mut conn = establish_connection().await;
	// let mut counter = app_data.counter.lock().unwrap();
	// *counter += 1; // <- Access counters inside MutexGuard
	let mut conn = establish_connection(&app_data.config.database_url).await;
	let login_data = data.into_inner();

	// Retrieve the user from the database based on the user_key (email or username)
	let user = match users::table
		.filter(users::columns::email.eq(&login_data.user_key).or(users::columns::username.eq(&login_data.user_key)))
		.select((users::columns::id, users::columns::username, users::columns::password, users::columns::email, users::columns::name))
		.first::<LoggedUser>(&mut conn) {
				Ok(user) => user,
				Err(Error::NotFound) => {
					return HttpResponse::NotFound().json(json!({
						"success": false,
						"message": "User not found"
					}));
				}
				Err(err) => {
					// Create new log & save it to db
					let new_log = InsertableLog {
						audit: 	LogType::Error,
						author: 0,
						target: 0,
						name: "database".to_owned(),
						action: ActionType::Read,
						verb: format!("User login failed: {}", err.to_string()),
					};
					create_log(&new_log, &mut conn).await;

					return HttpResponse::InternalServerError().json(json!({
						"success": false,
						"message": "An error occurred while trying to log you in, try again later!"
					}));
				}
		};

		// Compare the provided password with the stored hashed password
		match verify(&login_data.password, &user.password) {
			Ok(true) => {
				// Password is correct
				// Generate a JWT for the user
				let jwt_result = generate_jwt(user.id, &user.username, &user.name, &user.email).await;
				match jwt_result {
					Ok(jwt) => {
						// Respond with a successful login message, user info, and the generated JWT
						HttpResponse::Ok().json(json!({
							"success": true,
							"message": "Login successful",
							"user": {
								"id": &user.id,
								"username": &user.username,
								"email": &user.email,
								"name": &user.name
							}, // Include user data
							"token": jwt, // Include the JWT
						}))
					}
					Err(err) => {
						// Create new log & save it to db
						let new_log = InsertableLog {
							audit: 	LogType::Error,
							author: 0,
							target: 0,
							name: "system".to_owned(),
							action: ActionType::Read,
							verb: format!("Failed to generate jwt: {}", err.to_string()),
						};
						create_log(&new_log, &mut conn).await;

						HttpResponse::Unauthorized().json(json!({
							"success": false,
							"message": "Failed, error occurred while generating auth token"
						}))
					}
				}
			}
			Ok(false) => {
				// Incorrect password
				return HttpResponse::Unauthorized().json(json!({
					"success": false,
					"message": "Invalid password"
				}));
			}
			Err(_) => {
				// Handle password verification error
				return HttpResponse::InternalServerError().json(json!({
					"success": false,
					"message": "Password verification failed"
				}));
			}
		}
}


// Check if username already exists
pub async fn check_username(app_data: web::Data<AppState>, data: web::Json<Username>) -> impl Responder {
	let mut conn = establish_connection(&app_data.config.database_url).await;
	let username_data = data.into_inner();
	return match users.filter(username.eq(&username_data.username)).first::<User>(&mut conn) {
		Ok(_) => {
			HttpResponse::Ok().json(json!({
				"success": true,
				"user_exists": true,
				"message": "User with that username already exists"
			}))
		},
		Err(Error::NotFound) => {
			HttpResponse::Ok().json(json!({
				"success": true,
				"user_exists": false,
				"message": "User with that username does not exists"
			}))
		},
		Err(err) => {
			// Create new log & save it to db
			let new_log = InsertableLog {
				audit: LogType::Error,
				author: 0,
				target: 0,
				name: "database".to_owned(),
				action: ActionType::Read,
				verb: format!("Check username -({})- failed: {}", username_data.username, err.to_string()),
			};
			create_log(&new_log, &mut conn).await;
			
			HttpResponse::InternalServerError().json(json!({
				"success": false,
				"message": "Internal server error occurred!"
			}))
		},
	}
}

pub async fn check_user(req: HttpRequest, _: JwtMiddleware) -> impl Responder {
	let ext = req.extensions();
	
	// Use the 'get' method to retrieve the 'Claims' value from extensions
	match ext.get::<Claims>() {
		Some(claims) => {
			// Access 'user' from 'Claims'
			let user_info = &claims.user;
			
			HttpResponse::Ok().json(json!({
				"success": true,
				"user": user_info
			}))
		}
		None => HttpResponse::Unauthorized().json(json!({
			"success": false,
			"user": "Authorization failed"
		})),
	}
}