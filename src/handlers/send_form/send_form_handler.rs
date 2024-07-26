use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use crate::models::form_data::FormData;
use crate::services::send_form::send_form_service::send_form;

pub async fn handle_send_form(Json(payload): Json<FormData>) -> impl IntoResponse {
    match send_form(payload.into()).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
