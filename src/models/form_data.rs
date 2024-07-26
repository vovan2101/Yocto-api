// src/models.rs

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct FormData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub relationship: String,
    pub company_name: String,
    pub description: String,
    pub industry: String,
    pub website: Option<String>,
    pub pitch_deck: String,
}

#[derive(Serialize)]
pub struct ExternalFormData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub relationship: String,
    pub company_name: String,
    pub description: String,
    pub industry: String,
    pub website: Option<String>,
    pub pitch_deck: String,
}

impl From<FormData> for ExternalFormData {
    fn from(form_data: FormData) -> Self {
        ExternalFormData {
            first_name: form_data.first_name,
            last_name: form_data.last_name,
            email: form_data.email,
            relationship: form_data.relationship,
            company_name: form_data.company_name,
            description: form_data.description,
            industry: form_data.industry,
            website: form_data.website,
            pitch_deck: form_data.pitch_deck,
        }
    }
}
