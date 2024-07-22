use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Deserialize)]
pub struct SurveyResponse {
    pub answers: Vec<String>,
}

#[derive(Serialize)]
pub struct SurveyResult {
    pub message: String,
}
