use axum::{
    extract::Query,
    response::IntoResponse,
};
use reqwest::Client;
// use serde_json::Value;
use std::sync::Arc;
use std::convert::Infallible;
use axum::http::StatusCode;
use crate::handlers::hubspot::fetch_contacts_from_hubspot;
use crate::handlers::ai::generate_investor_summary_with_ai;
use crate::models::searchParams::SearchParams;

pub async fn investors(client: Arc<Client>, hubspot_access_token: String, ai_api_key: String, Query(params): Query<SearchParams>) -> Result<impl IntoResponse, Infallible> {
    match fetch_contacts_from_hubspot(&client, &hubspot_access_token, &params.name).await {
        Ok(investor_infos) => {
            match generate_investor_summary_with_ai(&client, investor_infos, &ai_api_key).await {
                Ok(info) => Ok(axum::Json(info).into_response()),
                Err(e) => {
                    let error_message = format!("Failed to fetch investor info: {}", e);
                    Ok((StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response())
                }
            }
        }
        Err(e) => {
            let error_message = format!("Failed to fetch contacts: {}", e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response())
        }
    }
}
