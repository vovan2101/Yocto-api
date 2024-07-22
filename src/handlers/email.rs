use axum::{extract::Query, response::Html};
use serde::Deserialize;
use std::sync::Mutex;
use std::sync::Arc;
use lazy_static::lazy_static;
use oauth2::{AuthUrl, ClientId, RedirectUrl, CsrfToken, Scope};
use oauth2::basic::BasicClient;

use crate::handlers::send_email::send_email_oauth2;

lazy_static! {
    pub static ref AUTH_CODE: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None)); // Сделайте AUTH_CODE публичной
}

#[derive(Deserialize)]
pub struct AuthQuery { // Сделайте AuthQuery публичной
    pub code: String,
}

pub async fn oauth2_callback(Query(query): Query<AuthQuery>) -> Html<&'static str> {
    let mut auth_code = AUTH_CODE.lock().unwrap();
    *auth_code = Some(query.code.clone());
    println!("Authorization code received: {:?}", query.code); // Логирование кода авторизации
    Html("<h1>Authorization code received. You can now send email.</h1>")
}

pub async fn authorize() -> &'static str {
    let client_id = "518166397198-stt0k6d7q2c804m4j6dr402q10gko17c.apps.googleusercontent.com";
    let redirect_url = "http://localhost:3001/oauth2/callback";

    let (auth_url, _csrf_token) = generate_oauth_url(client_id, redirect_url);
    println!("Please go to this URL and authorize the application: {}", auth_url);

    "Authorization URL generated. Please check the server logs."
}

pub async fn test_send_email() -> &'static str {
    let client_id = "518166397198-stt0k6d7q2c804m4j6dr402q10gko17c.apps.googleusercontent.com";  // Замените на ваш правильный client_id
    let client_secret = "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";  // Замените на ваш правильный client_secret
    let redirect_url = "http://localhost:3001/oauth2/callback";  // URL перенаправления

    let auth_code = {
        let auth_code = AUTH_CODE.lock().unwrap();
        auth_code.clone().unwrap_or_else(|| {
            println!("No authorization code found. Please authorize the application first.");
            std::process::exit(1);
        })
    };

    println!("Using authorization code: {:?}", auth_code); // Логирование кода авторизации

    let email = "vladeliseykin2101@gmail.com";
    let subject = "Test";
    let body = "Test";
    let from_email = "veliseykin2000@gmail.com";

    println!("Starting email send process...");

    match send_email_oauth2(email, subject, body, from_email, &auth_code, client_id, client_secret, redirect_url).await {
        Ok(_) => {
            println!("Email sent successfully!");
            "Email sent successfully!"
        },
        Err(e) => {
            println!("Failed to send email: {:?}", e);
            "Failed to send email"
        },
    }
}

pub fn generate_oauth_url(client_id: &str, redirect_url: &str) -> (String, CsrfToken) {
    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        None,
        AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).unwrap(),
        None,
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url.to_string()).unwrap());

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("https://mail.google.com/".to_string())) // Правильный scope
        .url();

    (auth_url.to_string(), csrf_token)
}
