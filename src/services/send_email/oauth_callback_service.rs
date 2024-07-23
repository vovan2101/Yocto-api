use axum::response::Html;
use crate::models::auth_query::AuthQuery;
use crate::services::send_email::oauth_service::AUTH_CODE;

pub async fn oauth2_callback_service(query: AuthQuery) -> Html<&'static str> {
    let mut auth_code = AUTH_CODE.lock().unwrap();
    *auth_code = Some(query.code.clone());
    println!("Authorization code received: {:?}", query.code);
    Html("<h1>Authorization code received. You can now send email.</h1>")
}
