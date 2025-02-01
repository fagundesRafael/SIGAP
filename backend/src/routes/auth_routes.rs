use actix_web::{get, post, HttpResponse, Responder, web};
use crate::models::user::User;
use crate::controllers::user_controller::{create_user, login}; // Não precisa importar login aqui
use crate::services::user_service::UserService;

#[get("/entrar")]
async fn login_page() -> impl Responder {
    HttpResponse::Ok().body("Página de login")
}

#[post("/entrar")]
async fn login_route(
    service: web::Data<UserService>,
    user: web::Json<User>
) -> impl Responder {
    login(service, user).await // Chama a função login do controlador
}

#[get("/criar")]
async fn create_user_page() -> impl Responder {
    HttpResponse::Ok().body("Página de criação de usuário")
}

#[post("/criar")]
async fn create_user_route(
    service: web::Data<UserService>,
    user: web::Json<User>
) -> impl Responder {
    create_user(service, user).await // Chama a função do controlador
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login_page);
    cfg.service(
        web::resource("/entrar")
            .route(web::post().to(|data: web::Data<UserService>, user: web::Json<User>| async {
                login(data, user).await
            }))
    );
    cfg.service(create_user_page);
    cfg.service(create_user_route);
}