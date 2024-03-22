use core::fmt;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, Result, dev::Payload, FromRequest, HttpResponse, HttpMessage, ResponseError};
use std::future::{ready, Ready};
use crate::db::account::account::users::dsl::*;
use crate::models::users::User;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;
use serde::{Serialize, Deserialize};
use crate::configs::config::Config;
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, decode, DecodingKey, Validation, Header, Algorithm, EncodingKey};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserClaims {
  pub id: i32,
  pub username: String,
  pub full_name: String,
  pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String, // Subject (usually the user's ID)
  pub exp: usize,  // Expiration time (Unix timestamp)
  pub user: UserClaims,
}

impl Clone for Claims {
  fn clone(&self) -> Self {
    // Implement the cloning logic for Claims here
    Claims {
      sub: self.sub.clone(),
      exp: self.exp,
      user: self.user.clone(),
    }
  }
}


pub async fn email_or_username_exists(other_email: &str, other_username: &str, conn: &mut PgConnection) -> (bool, Option<String>) {
  match users.filter(email.eq(other_email).or(username.eq(other_username))).first::<User>(conn) {
    Ok(user) => {
      if user.email == other_email {
        return (true, Some("Similar email already exists!".to_string()));
      }
      else if user.username == other_username {
        return (true, Some("Similar username already exists!".to_string()))
      }
      else {
        return (false, None)
      }
    },
    Err(Error::NotFound) => (false, None),
    Err(_) => (false, None),
  }
}

// Function to generate the jwt
pub async fn generate_jwt(user_id: i32, other_username: &str, full_name: &str, other_email: &str) -> Result<String, jsonwebtoken::errors::Error> {
  let config = Config::init();

  // Set the expiration time for the token (e.g., 1 hour from now)
  let exp_time = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backward")
    .as_secs() + config.jwt_expires_in;

  // Create a UserClaims instance with user-specific data
  let user_claims = UserClaims {
    id: user_id,
    username: other_username.to_string(),
    full_name: full_name.to_string(),
    email: other_email.to_string()
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
pub struct ErrorResponse {
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

impl ResponseError for ErrorResponse {
  fn status_code(&self) -> StatusCode {
    StatusCode::UNAUTHORIZED
  }

  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code())
      .json(self)
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
  type Error = ErrorResponse;
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
      return ready(Err(ErrorResponse::new("You are not logged in, please provide token")));
    }

    let config = Config::init();
    let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_ref());

    let claims = match decode::<Claims>(&token.unwrap(), &decoding_key, &Validation::default(),) {
      Ok(c) => c.claims,
      Err(_) => {
        // let json_error = ErrorResponse::new("Invalid token" );
        return ready(Err(ErrorResponse::new("Invalid token")))
      }
    };


    req.extensions_mut()
      .insert::<Claims>(claims.to_owned());

      ready(Ok(JwtMiddleware { claims }))

  }
}
