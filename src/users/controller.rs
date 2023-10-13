use actix_web::{Responder, web::{Json, Data, Path}, post, HttpResponse, get};


use crate::{AppState, users::{models::UserCreate, repository}};

#[post("/users/register")]
async fn register(state: Data<AppState>, new_user: Json<UserCreate>) -> impl Responder {
    match repository::create_user(state, new_user).await {
        Ok(user) => HttpResponse::Ok().json(user),
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


#[get("/users/{id}")]
async fn user_details(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let user_id: i64 = path.into_inner();
    match repository::user_details(state, user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => {
            eprintln!("Error: {:?}", error);
            return HttpResponse::NotFound().json("Такаой пользователь не найден!");
        }
    }
}