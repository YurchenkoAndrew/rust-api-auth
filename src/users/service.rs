use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc, Duration};
use jsonwebtoken::{Header, errors::Error, encode, EncodingKey};
use serde::{Serialize, Deserialize};

use super::responses::AuthResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: i64,
}

const PERIOD: i64 = 1_296_000;

pub fn password_hashing(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("Failed to hash password")
}


pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    verify(password, hashed_password).expect("Failed to compare password hash with password!")
}

pub fn create_token(user: &AuthResponse) -> Result<String, Error> {
    let claims = Claims {
        sub: user.email.to_string(),
        exp: PERIOD,
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
    Ok(token)
}

pub fn create_token_exp_time() -> DateTime<Utc> {
    let current_time = Utc::now();
    let duration = Duration::seconds(PERIOD);
    current_time.checked_add_signed(duration).expect("Failed to add duration to current time")
}
