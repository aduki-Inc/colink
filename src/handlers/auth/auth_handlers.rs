use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use diesel::prelude::*;
use crate::db::schema::users::dsl::*;
use diesel::result::Error;
use bcrypt::{hash, verify};
use crate::db::connection::establish_connection;
use crate::db::schema::users;
use crate::models::users::{User, LoggedUser, NewUser, LoginData, Username};
use crate::configs::state::AppState;
use serde_json::json;
use crate::middlewares::auth::auth_middleware::{email_exists, username_exists, generate_jwt, JwtMiddleware, Claims};


// Define handler for user registration with JSON data.
pub async fn register_user(app_data: web::Data<AppState>, data: web::Json<NewUser>) -> impl Responder {
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
							"message": "Failed to register user: Internal Error occurred!"
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
					"message": "Failed to register user: Internal Error occurred!"
				})
			)
		}
	}


}


// Define handler for user login
pub async fn login_user(app_data: web::Data<AppState>, data: web::Json<LoginData>) -> impl Responder {
	// let mut conn = establish_connection().await;
	// let mut counter = app_data.counter.lock().unwrap();
	// *counter += 1; // <- Access counters inside MutexGuard

	let mut conn = establish_connection(&app_data.config.database_url).await;

	let login_data = data.into_inner();

	// Check if the user exists based on email or username
	if !email_exists(&login_data.user_key, &mut conn) && !username_exists(&login_data.user_key, &mut conn) {
		return HttpResponse::Unauthorized().json(json!({
			"success": false,
			"message": "User not found"
		}));
	}

	// Retrieve the user from the database based on the user_key (email or username)
	let user = match users::table
		.filter(users::columns::email.eq(&login_data.user_key).or(users::columns::username.eq(&login_data.user_key)))
		.select((users::columns::id, users::columns::username, users::columns::password, users::columns::email, users::columns::name))
		.first::<LoggedUser>(&mut conn) {
				Ok(user) => user,
				Err(_) => {
					return HttpResponse::InternalServerError().json(json!({
						"success": false,
						"message": "Error, User not found!"
					}));
				}
		};

		// Compare the provided password with the stored hashed password
		match verify(&login_data.password, &user.password) {
			Ok(true) => {
				// Password is correct
				// Generate a JWT for the user
				let jwt_result = generate_jwt(user.id, &user.username, &user.email);
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
					Err(_) => {
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
	
	match users.filter(username.eq(username_data.username)).first::<User>(&mut conn) {
    Ok(_) => {
			return HttpResponse::Ok().json(json!({
				"success": true,
				"user_exists": true,
				"message": "User with that username already exists"
			}));
		},
    Err(Error::NotFound) => {
			return HttpResponse::Ok().json(json!({
				"success": true,
				"user_exists": false,
				"message": "User with that username does not exists"
			}));
		},
    Err(_) => {
			return HttpResponse::InternalServerError().json(json!({
				"success": false,
				"message": "Internal server error occurred!"
			}));
		},
  }
}

pub async fn check_user(req: HttpRequest, _: JwtMiddleware) -> impl Responder {
	let ext = req.extensions();

	// Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user_info = &claims.user;

		return HttpResponse::Ok().json(json!({
			"success": true,
			"user": user_info
		}));

	}
	else {
		
		return HttpResponse::Ok().json(json!({
			"success": false,
			"user": "An error has occurred"
		}));
	}
}