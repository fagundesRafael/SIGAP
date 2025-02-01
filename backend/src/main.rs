use actix_web::{App, HttpServer};
mod config;
mod services;
mod routes;
mod controllers;
mod models;

use crate::config::database::connect;
use crate::services::user_service::UserService;
use crate::routes::user_routes;
use crate::routes::home_routes;
use crate::routes::auth_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = connect().await;
    let user_service = UserService::new(client);

    HttpServer::new(move || {
        {
            println!("Servidor rodando...");
            App::new()
                .app_data(actix_web::web::Data::new(user_service.clone()))
                .configure(user_routes::config)
                .configure(home_routes::config)
                .configure(auth_routes::config)
        }
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}