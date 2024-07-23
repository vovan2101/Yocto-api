use oauth2::{AuthUrl, ClientId, RedirectUrl, CsrfToken, Scope};
use oauth2::basic::BasicClient;
use std::sync::Mutex;
use std::sync::Arc;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref AUTH_CODE: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
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
