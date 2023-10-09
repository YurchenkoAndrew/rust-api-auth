use actix_web::web;

use super::controller::register;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(register);
}