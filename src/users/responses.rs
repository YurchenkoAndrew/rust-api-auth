use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct AuthResponse {
    pub email: String,
    pub password: String,
    pub user_id: i64,
}

#[derive(Serialize, FromRow)]
pub struct TokenResponse {
    pub token: String,
    pub user_id: i64,
    pub expires_at: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
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