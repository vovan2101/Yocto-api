use axum::{extract::Query, response::Html};
use std::env;
use crate::models::auth_query::AuthQuery;
use crate::services::send_email::oauth_service::generate_oauth_url;
use crate::services::send_email::oauth_callback_service::oauth2_callback_service;
use crate::services::send_email::email_test_service::test_send_email_service;

pub async fn oauth2_callback(Query(query): Query<AuthQuery>) -> Html<&'static str> {
    oauth2_callback_service(query).await
}

pub async fn authorize() -> &'static str {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let redirect_url = env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");

    let (auth_url, _csrf_token) = generate_oauth_url(&client_id, &redirect_url);
    println!("Please go to this URL and authorize the application: {}", auth_url);

    "Authorization URL generated. Please check the server logs."
}

pub async fn test_send_email() -> &'static str {
    match test_send_email_service().await {
        Ok(message) => message,
        Err(error_message) => error_message,
    }
}