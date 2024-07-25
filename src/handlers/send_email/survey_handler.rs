use axum::extract::{Json, Extension};
use mongodb::Client;
use std::sync::Arc;
use crate::models::survey::SurveyResponse;
use crate::models::survey::SurveyResult;
use crate::services::send_email::oauth_service::exchange_code_for_token;
use crate::services::send_email::survey_service::process_survey;
use axum::http::StatusCode;
use crate::services::send_email::email_service::send_email_via_gmail;

pub async fn handle_survey(
    Json(payload): Json<SurveyResponse>,
    Extension(mongo_client): Extension<Arc<Client>>,
) -> Result<Json<SurveyResult>, StatusCode> {
    
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let redirect_url = std::env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");

    let auth_code = {
        let auth_code = crate::services::send_email::oauth_service::AUTH_CODE.lock().unwrap();
        auth_code.clone().ok_or_else(|| {
            println!("No authorization code found. Please authorize the application first.");
            StatusCode::UNAUTHORIZED
        })?
    };

    let access_token = exchange_code_for_token(&client_id, &client_secret, &redirect_url, &auth_code).await.map_err(|e| {
        println!("Failed to exchange code for token: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let email = "vladeliseykin2101@gmail.com";
    let subject = "Test";
    let body = "Test";
    let from_email = "veliseykin2000@gmail.com";

    match process_survey(payload, mongo_client).await {
        Ok(_investors) => {
            match send_email_via_gmail(email, subject, body, from_email, &access_token).await {
                Ok(_) => {
                    println!("Email sent to {}", email);
                    Ok(Json(SurveyResult {
                        message: "Survey submitted and email sent successfully".to_string(),
                    }))
                },
                Err(e) => {
                    println!("Failed to send email to {}: {:?}", email, e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        },
        Err(status) => Err(status),
    }
}
