use actix_web::{HttpServer, App, web::Data};
use sqlx::{Pool, mysql::MySqlPoolOptions, MySql};
use dotenv::dotenv;
use std::io::Result;

mod users;
use users::routes::config as user_routes;

pub struct AppState {
    db: Pool<MySql>
}

#[actix_web::main] 
async fn main() -> Result<()> {
    dotenv().ok();
    let connecrion_string = std::env::var("DATABASE_URL").expect("Строка подлючения должна быть установлена!");
    let pool: Pool<MySql> = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&connecrion_string)
        .await.expect("Ошибка пула подключений!");
    sqlx::migrate!("./migrations").run(&pool).await.expect("Произошла ошибка миграции!");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState{db: pool.clone()}))
            .configure(user_routes)
    }).bind(("127.0.0.1", 8080))?.run().await
}
