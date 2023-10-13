use actix_web::web;

use super::controller::{register, user_details};

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(register);
    cfg.service(user_details);
}