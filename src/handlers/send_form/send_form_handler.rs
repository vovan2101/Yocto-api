use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use crate::models::form_data::FormData;
use crate::services::send_form::send_form_service::send_form;

pub async fn handle_send_form(Json(payload): Json<FormData>) -> impl IntoResponse {
    println!("Received form data: {:?}", payload);
    match send_form(payload.into()).await {
        Ok(_) => {
            println!("Form sent successfully");
            StatusCode::OK
        },
        Err(err) => {
            println!("Failed to send form: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        },
    }
}
