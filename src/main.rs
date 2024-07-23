use dotenv::dotenv;
use hyper::Method;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use reqwest::Client;
use mongodb::{options::ClientOptions, Client as MongoClient};
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};

mod models;
mod routes;
mod handlers;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let hubspot_access_token = env::var("HUBSPOT_ACCESS_TOKEN").expect("HUBSPOT_ACCESS_TOKEN must be set");
    let ai_api_key = env::var("AI_ACCESS_TOKEN").expect("AI_ACCESS_TOKEN must be set");

    let client = Arc::new(Client::new());

    let mongo_client_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let mongo_client_options = ClientOptions::parse(&mongo_client_uri).await.unwrap();
    let mongo_client = Arc::new(MongoClient::with_options(mongo_client_options).unwrap());
    
    let app = routes::create_router(client, hubspot_access_token, ai_api_key, mongo_client)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::GET, Method::POST])
                .allow_headers(Any),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
