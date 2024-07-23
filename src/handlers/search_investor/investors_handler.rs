use axum::extract::Query;
use axum::response::IntoResponse;
use reqwest::Client;
use std::sync::Arc;
use std::convert::Infallible;
use axum::http::StatusCode;
use crate::services::search_investor::hubspot_service::fetch_hubspot_contacts;
use crate::services::search_investor::ai_service::ai_generate_investor_summary;
use crate::models::search_params::SearchParams;

pub async fn investors_handler(
    client: Arc<Client>, 
    hubspot_access_token: String, 
    ai_api_key: String, 
    Query(params): Query<SearchParams>
) -> Result<impl IntoResponse, Infallible> {
    match fetch_hubspot_contacts(&client, &hubspot_access_token, &params.name).await {
        Ok(investor_infos) => {
            match ai_generate_investor_summary(&client, investor_infos, &ai_api_key).await {
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
