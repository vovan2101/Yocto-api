use axum::{extract::Query, response::Html};
use serde::Deserialize;
use std::sync::Mutex;
use std::sync::Arc;
use lazy_static::lazy_static;
use oauth2::{AuthUrl, ClientId, RedirectUrl, CsrfToken, Scope};
use oauth2::basic::BasicClient;
use std::env;

use crate::handlers::send_email::send_email_oauth2;

lazy_static! {
    pub static ref AUTH_CODE: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
}

#[derive(Deserialize)]
pub struct AuthQuery {
    pub code: String,
}

pub async fn oauth2_callback(Query(query): Query<AuthQuery>) -> Html<&'static str> {
    let mut auth_code = AUTH_CODE.lock().unwrap();
    *auth_code = Some(query.code.clone());
    println!("Authorization code received: {:?}", query.code);
    Html("<h1>Authorization code received. You can now send email.</h1>")
}

pub async fn authorize() -> &'static str {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let redirect_url = env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");

    let (auth_url, _csrf_token) = generate_oauth_url(&client_id, &redirect_url);
    println!("Please go to this URL and authorize the application: {}", auth_url);

    "Authorization URL generated. Please check the server logs."
}

pub async fn test_send_email() -> &'static str {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let redirect_url = env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");

    let auth_code = {
        let auth_code = AUTH_CODE.lock().unwrap();
        auth_code.clone().unwrap_or_else(|| {
            println!("No authorization code found. Please authorize the application first.");
            std::process::exit(1);
        })
    };

    println!("Using authorization code: {:?}", auth_code);

    let email = "vladeliseykin2101@gmail.com";
    let subject = "Test";
    let body = "Test";
    let from_email = "veliseykin2000@gmail.com";

    println!("Starting email send process...");

    match send_email_oauth2(email, subject, body, from_email, &auth_code, &client_id, &client_secret, &redirect_url).await {
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
        .add_scope(Scope::new("https://mail.google.com/".to_string()))
        .url();

    (auth_url.to_string(), csrf_token)
}
