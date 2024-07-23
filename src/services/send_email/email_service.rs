use lettre::message::header::ContentType;
use lettre::message::{Message, SinglePart};
use lettre::{SmtpTransport, Transport};
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse, TokenUrl};

pub async fn send_email(
    to: &str,
    subject: &str,
    body: &str,
    from_email: &str,
    auth_code: &str,
    client_id: &str,
    client_secret: &str,
    redirect_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Настройка OAuth2 клиента
    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(client_secret.to_string())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())?,
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?)
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url.to_string())?);

    // Обмен кода авторизации на токен доступа
    println!("Exchanging authorization code for access token...");
    let token_result = client.exchange_code(AuthorizationCode::new(auth_code.to_string()))
        .request_async(async_http_client)
        .await?;

    let access_token = token_result.access_token().secret();
    println!("Access token received: {:?}", access_token);

    // Создание сообщения
    let email = Message::builder()
        .from(from_email.parse()?)
        .reply_to(from_email.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .singlepart(
            SinglePart::builder()
                .header(ContentType::TEXT_PLAIN)
                .body(body.to_string()),
        )?;

    // Настройка транспортного средства для отправки email
    let creds = Credentials::new(from_email.to_string(), access_token.to_string());

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .authentication(vec![Mechanism::Plain])
        .build();

    // Отправка email
    match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent successfully!");
            Ok(())
        }
        Err(e) => {
            println!("Could not send email: {:?}", e);
            Err(Box::new(e))
        }
    }
}
