use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Seja bem vindo")
}

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(home);
}