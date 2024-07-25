use crate::services::send_email::email_service::send_email_via_gmail;
use crate::services::send_email::oauth_service::exchange_code_for_token;
use std::env;

pub async fn test_send_email_service() -> Result<&'static str, &'static str> {
    let client_id = env::var("CLIENT_ID").map_err(|_| "CLIENT_ID must be set")?;
    let client_secret = env::var("CLIENT_SECRET").map_err(|_| "CLIENT_SECRET must be set")?;
    let redirect_url = env::var("REDIRECT_URL").map_err(|_| "REDIRECT_URL must be set")?;

    let auth_code = {
        let auth_code = crate::services::send_email::oauth_service::AUTH_CODE.lock().unwrap();
        auth_code.clone().ok_or_else(|| {
            println!("No authorization code found. Please authorize the application first.");
            "No authorization code found. Please authorize the application first."
        })?
    };

    println!("Using authorization code: {:?}", auth_code);

    let access_token = exchange_code_for_token(&client_id, &client_secret, &redirect_url, &auth_code).await.map_err(|e| {
        println!("Failed to exchange code for token: {:?}", e);
        "Failed to exchange code for token"
    })?;

    let email = "vladeliseykin2101@gmail.com";
    let subject = "Test";
    let body = "Test";
    let from_email = "veliseykin2000@gmail.com";

    println!("Starting email send process...");

    match send_email_via_gmail(email, subject, body, from_email, &access_token).await {
        Ok(_) => {
            println!("Email sent successfully!");
            Ok("Email sent successfully!")
        },
        Err(e) => {
            println!("Failed to send email: {:?}", e);
            Err("Failed to send email")
        },
    }
}
