use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

use crate::{
    users::{
        repository,
        requests::{UserCreateOuery, UserUpdateRequest},
        responses::TokenResponse,
        service,
    },
    AppState,
};

use super::requests::AuthQuery;

#[post("/users/register")]
async fn register(state: Data<AppState>, new_user: Json<UserCreateOuery>) -> impl Responder {
    match repository::create_user(state, new_user).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            if let Some(db_error) = e.as_database_error() {
                match db_error.is_unique_violation() {
                    _ => {
                        // Обработка ошибки уникальности email
                        return HttpResponse::Conflict().json("Email уже зарегистрирован");
                    }
                }
            } else {
                // Обработка других ошибок
                eprintln!("Error: {:?}", e);
                return HttpResponse::InternalServerError().json("Internal Server Error");
            }
        }
    }
}

#[post("/users/login")]
async fn user_auth(state: Data<AppState>, auth_query: Json<AuthQuery>) -> impl Responder {
    let password_query = auth_query.password.clone();
    match repository::user_auth(&state, auth_query).await {
        Ok(auth_response) => {
            match service::verify_password(&password_query, &auth_response.password) {
                true => match service::create_token(&auth_response) {
                    Ok(token) => {
                        let expires_at = service::create_token_exp_time();
                        let token_response: Json<TokenResponse> = Json(TokenResponse {
                            token,
                            expires_at: expires_at,
                            user_id: auth_response.user_id,
                            created_at: None,
                            updated_at: None,
                        });
                        match repository::save_token(state, token_response).await {
                            Ok(token_response) => HttpResponse::Ok().json(token_response),
                            Err(error) => {
                                eprintln!("Error: {:?}", error);
                                HttpResponse::InternalServerError()
                                    .json("Внутренняя ошибка сервера")
                            }
                        }
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        HttpResponse::InternalServerError().json("Internal Server Error")
                    }
                },
                false => HttpResponse::NotAcceptable().json("Неверный пароль!"),
            }
        }
        Err(error) => {
            match error {
                sqlx::Error::RowNotFound => {
                    HttpResponse::NotFound().json("Нет такой учетной записи!")
                }
                _ => {
                    // Обработка других ошибок
                    eprintln!("Error: {:?}", error);
                    HttpResponse::InternalServerError().json("Внутренняя ошибка сервера")
                }
            }
        }
    }
}

#[get("/users")]
async fn get_list_users(state: Data<AppState>) -> impl Responder {
    match repository::user_list(state).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return HttpResponse::InternalServerError().json("Internal Server Error");
        }
    }
}

#[get("/users/{id}")]
async fn get_user_by_id(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let user_id: i64 = path.into_inner();
    match repository::user_details(state, user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => {
            eprintln!("Error: {:?}", error);
            return HttpResponse::NotFound().json("Такаой пользователь не найден!");
        }
    }
}

#[patch("/users/{id}")]
async fn user_update(
    state: Data<AppState>,
    path: Path<i64>,
    update_user: Json<UserUpdateRequest>,
) -> impl Responder {
    let id = path.into_inner();
    match repository::user_update(state, id, update_user).await {
        Ok(user) => HttpResponse::Created().json(user),

        Err(error) => {
            eprintln!("Error: {:?}", error);
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }
}

#[delete("/users/{id}")]
async fn user_delete(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match repository::user_delete(state, id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }
}
