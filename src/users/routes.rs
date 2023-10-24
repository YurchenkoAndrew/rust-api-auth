use actix_web::{web, middleware::{NormalizePath, TrailingSlash}};

use super::controller::{register, get_user_by_id, get_list_users, user_update};

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("")
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .service(register)
            .service(get_user_by_id)
            .service(get_list_users)
            .service(user_update)
    );
}