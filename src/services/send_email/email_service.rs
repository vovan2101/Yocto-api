use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::Client;
use serde::Serialize;
use std::error::Error;

#[derive(Serialize)]
struct EmailPayload {
    raw: String,
}

pub async fn send_email_via_gmail(
    to: &str,
    subject: &str,
    body: &str,
    from_email: &str,
    access_token: &str,
) -> Result<(), Box<dyn Error>> {
    // Проверка полученного токена
    println!("Access token: {:?}", access_token);

    let email = format!(
        "From: {}\r\nTo: {}\r\nSubject: {}\r\n\r\n{}",
        from_email, to, subject, body
    );
    let email_base64 = STANDARD.encode(email);

    let payload = EmailPayload { raw: email_base64 };

    let client = Client::new();
    let response = client
        .post("https://gmail.googleapis.com/gmail/v1/users/me/messages/send")
        .bearer_auth(access_token)
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Email sent successfully!");
        Ok(())
    } else {
        println!("Failed to send email: {:?}", response.text().await?);
        Err(Box::from("Failed to send email"))
    }
}
