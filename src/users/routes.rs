use actix_web::{web, middleware::{NormalizePath, TrailingSlash}};

use super::controller::{register, get_user_by_id, get_list_users, user_update, user_delete, user_auth};

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("")
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .service(register)
            .service(get_user_by_id)
            .service(get_list_users)
            .service(user_update)
            .service(user_delete)
            .service(user_auth)
    );
}