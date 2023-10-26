use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserCreateOuery {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AuthQuery {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserUpdateRequest {
    pub username: String,
    pub email: String,
    pub active: bool,
}