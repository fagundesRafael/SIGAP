use mongodb::{Client, options::ClientOptions};
use std::env;

pub async fn connect() -> Client {
    // Carrega as variáveis de ambiente do arquivo .env
    dotenv::dotenv().ok(); 

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI não encontrada.");
    let client_options = ClientOptions::parse(mongo_uri).await.unwrap();
    Client::with_options(client_options).unwrap()
}