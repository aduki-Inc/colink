use crate::db::schema::users::dsl::*;
use crate::models::users::User;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct UserClaims {
  user_id: u32,
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

//Public function to generate jwt
pub fn generate_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    // ... (other code)

    // Create a UserClaims instance with user-specific data
    let user_claims = UserClaims {
        user_id: 123, // Replace with the actual user's ID
        username: "example_user".to_string(),
        email: "user@example.com".to_string(),
    };

    let claims = Claims {
        sub: user_id.to_string(),
        exp: exp_time as usize,
        user: user_claims, // Include the UserClaims in the payload
    };

    // ... (other code)
}


// Define a secret key to sign and verify the JWT
const SECRET_KEY: &str = env::var("JWT_SECRET").expect("JWT_SECRET is not set");


// Function to generate a JWT token
fn generate_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
  let expiration = env::var("JWT_EXPIRED_IN")
        .unwrap_or_else(|_| "3600".to_string()) // Default to 1 hour (3600 seconds) if not set
        .parse::<u64>()
        .expect("Invalid JWT_EXPIRED_IN value");

    let exp_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backward")
        .as_secs() + expiration;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: exp_time as usize,
    };

    let header = Header::new(jsonwebtoken::Algorithm::HS256);
    let key = EncodingKey::from_secret(SECRET_KEY.as_ref());

    encode(&header, &claims, &key)
}