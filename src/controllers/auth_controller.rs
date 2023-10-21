use actix_web::{web, HttpResponse, Responder};
use crate::db::schema::users::dsl::*;
use diesel::prelude::*;
use bcrypt::{hash, verify};
use crate::db::connection::establish_connection;
use crate::db::schema::users;
use crate::models::users::{LoggedUser, NewUser, LoginData};
use serde_json::json;
use crate::middlewares::auth_middleware::{email_exists, username_exists};
use crate::middlewares::auth_middleware::generate_jwt;


// Define an Actix route for user registration with JSON data.
pub async fn register_user(data: web::Json<NewUser>) -> impl Responder {

  let mut conn = establish_connection().await;

  let registration_data = data.into_inner();

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
    Ok(_) => HttpResponse::Ok().json(
      json!({
        "success": true,
        "message": "User registered successfully"
      })
    ),
    Err(err) => {
      // Handle the database error and return an error response
      HttpResponse::InternalServerError().json(
        json!({
          "success": false,
          "error": format!("Failed to register user: {}", err.to_string())
          })
      )
    }
  }
}


// Define an Actix route for user login
pub async fn login_user(data: web::Json<LoginData>) -> impl Responder {
  let mut conn = establish_connection().await;

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
    .select((users::columns::id, users::columns::username, users::columns::password, users::columns::email))
    .first::<LoggedUser>(&mut conn) {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Error retrieving user data"
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
              "user": user, // Include user data
              "token": jwt, // Include the JWT
            }))
          }
          Err(_) => {
            HttpResponse::Unauthorized().json(json!({
              "success": false,
              "message": "JWT generation failed"
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