use axum::{Extension, Json};
use axum::{extract::Query, routing::get, routing::post, Router};
use std::sync::Arc;
use reqwest::Client;
use crate::handlers::investors::investors;
use crate::models::searchParams::SearchParams;
use crate::handlers::survey::handle_survey;
use crate::models::survey::SurveyResponse;
use crate::handlers::email::{test_send_email, authorize, oauth2_callback}; // Импортируйте oauth2_callback

pub fn create_router(client: Arc<Client>, hubspot_access_token: String, ai_api_key: String, mongo_client: Arc<mongodb::Client>) -> Router {
    let client_clone1 = Arc::clone(&client);
    let hubspot_access_token_clone = hubspot_access_token.clone();
    let ai_api_key_clone1 = ai_api_key.clone();

    Router::new()
        .route("/investors", get(move |Query(params): Query<SearchParams>| {
            let client = Arc::clone(&client_clone1);
            let hubspot_access_token = hubspot_access_token_clone.clone();
            let ai_api_key = ai_api_key_clone1.clone();
            async move {
                println!("Handling /investors request");
                investors(client, hubspot_access_token, ai_api_key, Query(params)).await
            }
        }))
        .route(
            "/survey",
            post({
                let mongo_client = Arc::clone(&mongo_client);
                move |Json(payload): Json<SurveyResponse>| {
                    println!("Received request at /survey");
                    let mongo_client = Arc::clone(&mongo_client);
                    async move {
                        println!("Processing survey...");
                        handle_survey(Json(payload), Extension(mongo_client)).await
                    }
                }
            }),
        )
        .route("/test_send_email", get(test_send_email))
        .route("/authorize", get(authorize)) // Новый маршрут для авторизации
        .route("/oauth2/callback", get(oauth2_callback)) // Новый маршрут для обработки перенаправления
}
