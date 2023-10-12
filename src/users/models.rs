use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub active: bool,
}
#[derive(Serialize, Deserialize, FromRow)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
    pub password: String,
}