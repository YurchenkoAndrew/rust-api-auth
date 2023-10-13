use actix_web::web::{Data, Json};
use sqlx::Error;
use super::{models::{UserCreate, UserDetails, UserList}, service};
use crate::AppState;

pub async fn create_user(state: Data<AppState>, new_user: Json<UserCreate>) -> Result<UserDetails, Error> {
    let sql: &str = "INSERT INTO users (username, email, password, active) VALUES ($1, $2, $3, $4) RETURNING id, username, email, password, created_at, updated_at, active";
    let user = sqlx::query_as::<_, UserDetails>(sql)
        .bind(new_user.username.to_string())
        .bind(new_user.email.to_string())
        .bind(service::password_hashing(&new_user.password))
        .bind(true)
        .fetch_one(&state.db).await;
    user
}

pub async fn user_list(state: Data<AppState>) -> Result<Vec<UserList>, Error> {
    let sql: &str = "SELECT id, username, email FROM users";
    let users = sqlx::query_as::<_, UserList>(sql)
        .fetch_all(&state.db)
        .await?;
    Ok(users)
}

pub async fn user_details(state: Data<AppState>, id: i64) -> Result<UserDetails, Error> {
    let sql: &str = "SELECT id, username, email, created_at, updated_at, active FROM users WHERE id = $1";
    let user: Result<UserDetails, Error> = sqlx::query_as::<_, UserDetails>(sql)
        .bind(id)
        .fetch_one(&state.db)
        .await;
    user
}