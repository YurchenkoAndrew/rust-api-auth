use actix_web::web::{Data, Json};
use sqlx::Error;
use super::{models::{UserCreate, User}, service};
use crate::AppState;

pub async fn create_user(state: Data<AppState>, new_user: Json<UserCreate>) -> Result<(), Error> {
    sqlx::query("INSERT INTO users (username, email, password, active) VALUES (?, ?, ?, ?)")
        .bind(new_user.username.to_string())
        .bind(new_user.email.to_string())
        .bind(service::password_hashing(&new_user.password))
        .bind(true)
        .execute(&state.db).await?;
    Ok(())
}

pub async fn user_details(state: Data<AppState>, id: u32) -> Result<User, Error> {
    let user: Result<User, Error> = sqlx::query_as::<_, User>("SELECT id, username, email, created_at, updated_at, active FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await;
    user
}