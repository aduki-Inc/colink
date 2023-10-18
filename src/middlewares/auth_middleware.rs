use crate::db::schema::users::dsl::*;
use crate::models::users::User;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
use serde::{Serialize, Deserialize};
use crate::configs::config::Config;
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};

#[derive(Debug, Serialize, Deserialize)]
struct UserClaims {
  user_id: i32,
  username: String,
  email: String,
}

#[derive(Debug, Serialize)]
struct Claims {
  sub: String, // Subject (usually the user's ID)
  exp: usize,  // Expiration time (Unix timestamp)
  user: UserClaims,
}

pub fn email_exists(other_email: &str, conn: &mut PgConnection) -> bool {
  match users.filter(email.eq(other_email)).first::<User>(conn) {
    Ok(_) => true,
    Err(Error::NotFound) => false,
    Err(_) => false,
  }
}

pub fn username_exists(other_username: &str, conn: &mut PgConnection) -> bool {
  match users.filter(username.eq(other_username)).first::<User>(conn) {
    Ok(_) => true,
    Err(Error::NotFound) => false,
    Err(_) => false,
  }
}

pub fn generate_jwt(user_id: i32, other_username: &str, other_email: &str) -> Result<String, jsonwebtoken::errors::Error> {
  let config = Config::init();
    
  // Set the expiration time for the token (e.g., 1 hour from now)
  let exp_time = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backward")
    .as_secs() + config.jwt_expires_in;

  // Create a UserClaims instance with user-specific data
  let user_claims = UserClaims {
    user_id,
    username: other_username.to_string(),
    email: other_email.to_string(),
  };

  // Create the claims for the JWT
  let claims = Claims {
    sub: user_id.to_string(),
    exp: exp_time as usize,
    user: user_claims, // Include the UserClaims in the payload
  };

  
  // Encode the JWT using the secret key
  let header = Header::new(Algorithm::HS256);
  let token = encode(&header, &claims, &EncodingKey::from_secret(config.jwt_secret.as_ref()))?;

  Ok(token)
}