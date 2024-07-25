use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use std::sync::{Mutex, Arc};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref AUTH_CODE: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    pub static ref PKCE_CODE_VERIFIER: Arc<Mutex<Option<PkceCodeVerifier>>> = Arc::new(Mutex::new(None));
}

pub fn generate_oauth_url(client_id: &str, redirect_url: &str) -> (String, CsrfToken) {
    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        None,
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        None,
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url.to_string()).unwrap());

    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    *PKCE_CODE_VERIFIER.lock().unwrap() = Some(pkce_code_verifier);

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("https://mail.google.com/".to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    (auth_url.to_string(), csrf_token)
}

pub async fn exchange_code_for_token(client_id: &str, client_secret: &str, redirect_url: &str, auth_code: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(client_secret.to_string())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?,
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?)
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url.to_string())?);

    let pkce_code_verifier = PKCE_CODE_VERIFIER.lock().unwrap().take().ok_or("Missing PKCE code verifier")?;

    let token_result = client.exchange_code(AuthorizationCode::new(auth_code.to_string()))
        .set_pkce_verifier(pkce_code_verifier)
        .request_async(async_http_client)
        .await?;

    let access_token = token_result.access_token().secret().clone();
    println!("Access token: {}", access_token); // Выводим токен для отладки

    Ok(access_token)
}
