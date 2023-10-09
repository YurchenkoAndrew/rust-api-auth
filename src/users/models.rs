use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
    pub active: bool,
}
#[derive(Serialize, Deserialize, FromRow)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
    pub password: String,
}