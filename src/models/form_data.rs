// src/models/form_data.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
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
    pub headquartered: String,
    pub country: String,
    pub legal_structure: String,
    pub raising_round: String,
    pub raising_amount: String,
    pub pre_money_valuation: Option<String>,
}

#[derive(Debug, Serialize)]
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
    pub headquartered: String,
    pub country: String,
    pub legal_structure: String,
    pub raising_round: String,
    pub raising_amount: String,
    pub pre_money_valuation: Option<String>,
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
            headquartered: form_data.headquartered,
            country: form_data.country,
            legal_structure: form_data.legal_structure,
            raising_round: form_data.raising_round,
            raising_amount: form_data.raising_amount,
            pre_money_valuation: form_data.pre_money_valuation,
        }
    }
}
