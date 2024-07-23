use axum::{Json, extract::Extension};
use hyper::StatusCode;
use mongodb::Client;
use std::sync::Arc;
use crate::models::survey::{SurveyResponse, SurveyResult};
use crate::services::send_email::survey_service::process_survey;
use crate::services::send_email::email_service::send_email;

pub async fn handle_survey(
    Json(payload): Json<SurveyResponse>,
    Extension(mongo_client): Extension<Arc<Client>>,
) -> Result<Json<SurveyResult>, StatusCode> {
    
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let redirect_url = std::env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");

    let email = "vladeliseykin2101@gmail.com";
    let subject = "Test";
    let body = "Test";
    let from_email = "veliseykin2000@gmail.com";
    let auth_code = "AUTH_CODE";

    match process_survey(payload, mongo_client).await {
        Ok(_investors) => {
            match send_email(email, subject, body, from_email, auth_code, &client_id, &client_secret, &redirect_url).await {
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
