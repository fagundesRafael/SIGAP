use actix_web::{web, HttpResponse, Responder};
use crate::services::user_service::UserService;
use crate::models::user::User;

pub async fn create_user(
    service: web::Data<UserService>,
    user: web::Json<User>
) -> impl Responder {
    match service.create_user(user.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            eprintln!("Erro ao criar usuário: {}", e); // Imprime o erro no console
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn list_users(service: web::Data<UserService>) -> impl Responder {
    match service.list_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Erro ao listar usuários: {}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn login(
    service: web::Data<UserService>,
    user: web::Json<User>
) -> impl Responder {
    match service.login(user.email.clone(), user.password.clone()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::Unauthorized().finish(),
        Err(e) => {
            eprintln!("Erro ao fazer login: {}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
}