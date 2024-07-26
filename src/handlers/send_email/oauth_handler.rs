use axum::{extract::Query, response::{IntoResponse, Redirect}};
use std::env;
use crate::models::auth_query::AuthQuery;
use crate::services::send_email::oauth_callback_service::oauth2_callback_service;
use crate::services::send_email::oauth_service::generate_oauth_url;
use crate::services::send_email::email_test_service::test_send_email_service;

pub async fn oauth2_callback(Query(query): Query<AuthQuery>) -> impl IntoResponse {
    let _ = oauth2_callback_service(query).await;
    Redirect::temporary("http://localhost:8080/survey")
}

pub async fn authorize() -> impl IntoResponse {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let redirect_url = env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");

    let (auth_url, _csrf_token) = generate_oauth_url(&client_id, &redirect_url);
    println!("Redirecting to: {}", auth_url);

    Redirect::temporary(&auth_url)
}

pub async fn test_send_email() -> &'static str {
    match test_send_email_service().await {
        Ok(message) => message,
        Err(error_message) => error_message,
    }
}
