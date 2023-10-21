use core::fmt;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, Result, dev::Payload, FromRequest};
use std::future::{ready, Ready};
use crate::db::schema::users::dsl::*;
use crate::models::users::User;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;
use serde::{Serialize, Deserialize};
use crate::configs::config::Config;
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, decode, DecodingKey, Validation, Header, Algorithm, EncodingKey};
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error as ActixWebError, HttpResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
  pub user_id: i32,
  pub username: String,
  pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String, // Subject (usually the user's ID)
  pub exp: usize,  // Expiration time (Unix timestamp)
  pub user: UserClaims,
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

// Function to generate the jwt
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


#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
  success: bool,
  message: String,
}

impl ErrorResponse {
  fn new(message: &str) -> Self {
    ErrorResponse {
      success: false,
      message: message.to_string(),
    }
  }
}


impl fmt::Display for ErrorResponse {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", serde_json::to_string(&self).unwrap())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtMiddleware {
  pub claims: Claims,
}

impl FromRequest for JwtMiddleware {
  type Error = ActixWebError;
  type Future = Ready<Result<Self, Self::Error>>;

  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    let token = req
      .cookie("x-access-token")
      .map(|c| c.value().to_string())
      .or_else(|| {
        req.headers()
          .get("x-access-token")
          .and_then(|h| h.to_str().ok())
          .map(|s| s.to_string())
      });


    if token.is_none() {
      let json_error = ErrorResponse {
        success: false,
        message: "You are not logged in, please provide token".to_string(),
      };
      return ready(Err(ErrorUnauthorized(json_error)));
    }

    let config = Config::init();
    let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_ref());
  
    if let Some(token) = token {

      match decode::<Claims>(&token,  &decoding_key, &Validation::default()) {
        Ok(c) => {
          ready(Ok(JwtMiddleware {claims: c.claims}))
        }
        Err(_) => {
          println!("Invalid token");
          // In case of an error, create and return the error response
          // let error_response = create_error_response("Invalid token");
          let json_error = ErrorResponse::new ("Invalid token");
          return ready(Err(ErrorUnauthorized(json_error)));
        }
      }
    }
    else {
      let json_error = ErrorResponse::new("You are not logged in, please provide a token");
      ready(Err(ErrorUnauthorized(json_error)))
    }
  }
}
