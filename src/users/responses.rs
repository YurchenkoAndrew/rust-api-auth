use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct AuthResponse {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, FromRow)]
pub struct TokenResponse {
    pub token: String,
    pub token_exp: DateTime<Utc>
}

#[derive(Serialize, FromRow)]
pub struct UserDetailsResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub active: bool,
}

#[derive(Serialize, FromRow)]
pub struct UserListResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
}