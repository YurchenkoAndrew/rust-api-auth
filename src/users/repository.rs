use actix_web::web::{Data, Json};
use sqlx::Error;
use super::{service, requests::{AuthQuery, UserCreateOuery, UserUpdateRequest}, responses::{AuthResponse, UserDetailsResponse, UserListResponse}};
use crate::AppState;

pub async fn create_user(state: Data<AppState>, new_user: Json<UserCreateOuery>) -> Result<UserDetailsResponse, Error> {
    let sql: &str = "INSERT INTO users (username, email, password, active) VALUES ($1, $2, $3, $4) RETURNING id, username, email, password, created_at, updated_at, active";
    let user = sqlx::query_as::<_, UserDetailsResponse>(sql)
        .bind(new_user.username.to_string())
        .bind(new_user.email.to_string())
        .bind(service::password_hashing(&new_user.password))
        .bind(true)
        .fetch_one(&state.db).await;
    user
}

pub async fn user_auth(state: Data<AppState>, auth_query: Json<AuthQuery>) -> Result<AuthResponse, Error> {
    let sql = "SELECT email, password FROM users WHERE email = $1 AND active = true";
    let auth_response = sqlx::query_as::<_, AuthResponse>(sql)
    .bind(auth_query.email.to_string())
        .fetch_one(&state.db)
        .await;
    auth_response
}

pub async fn user_list(state: Data<AppState>) -> Result<Vec<UserListResponse>, Error> {
    let sql: &str = "SELECT id, username, email FROM users";
    let users = sqlx::query_as::<_, UserListResponse>(sql)
        .fetch_all(&state.db)
        .await?;
    Ok(users)
}

pub async fn user_details(state: Data<AppState>, id: i64) -> Result<UserDetailsResponse, Error> {
    let sql: &str = "SELECT id, username, email, created_at, updated_at, active FROM users WHERE id = $1";
    let user: Result<UserDetailsResponse, Error> = sqlx::query_as::<_, UserDetailsResponse>(sql)
        .bind(id)
        .fetch_one(&state.db)
        .await;
    user
}

pub async fn user_update(state: Data<AppState>, id: i64, update_user: Json<UserUpdateRequest>) -> Result<UserDetailsResponse, Error> {
    let sql: &str = "UPDATE users SET username = $1, email = $2, active = $3 WHERE id = $4 RETURNING id, username, email, password, created_at, updated_at, active";
    let user = sqlx::query_as::<_, UserDetailsResponse>(sql)
        .bind(update_user.username.to_string())
        .bind(update_user.email.to_string())
        .bind(update_user.active)
        .bind(id)
        .fetch_one(&state.db)
        .await;
    user
}

pub async fn user_delete(state: Data<AppState>, id: i64) -> Result<UserDetailsResponse, Error> {
    let sql = "DELETE FROM users WHERE id = $1 RETURNING id, username, email, password, created_at, updated_at, active";
    let user = sqlx::query_as::<_, UserDetailsResponse>(sql)
        .bind(id)
        .fetch_one(&state.db)
        .await;
    user
}