use actix_web::{Responder, web::{Json, Data}, post, HttpResponse};


use crate::{AppState, users::{models::UserCreate, repository}};

#[post("/users/register")]
async fn register(state: Data<AppState>, new_user: Json<UserCreate>) -> impl Responder {
    match repository::create_user(state, new_user).await {
        Ok(()) => HttpResponse::Ok().json("User created!"),
        Err(e) => {
            if let Some(db_error) = e.as_database_error() {
                match db_error.is_unique_violation() {
                    _ => {
                        // Обработка ошибки уникальности email
                        return HttpResponse::Conflict().json("Email уже зарегистрирован");
                    },
                }
            } else {
                // Обработка других ошибок
                eprintln!("Error: {:?}", e);
                return HttpResponse::InternalServerError().json("Internal Server Error");
            }
        }
    }
}
