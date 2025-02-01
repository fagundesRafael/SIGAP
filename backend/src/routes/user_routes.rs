use actix_web::web;
use crate::controllers::user_controller::{create_user, list_users};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/criar")
            .route(web::post().to(create_user))
    );
    cfg.service(
        web::resource("/users")
            .route(web::get().to(list_users))
    );
}