use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InvestorInfo {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Company")]
    pub company: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "Phone Number")]
    pub phone_number: Option<String>,
    #[serde(rename = "Email")]
    pub email: Option<String>,
    #[serde(rename = "Preferred Sectors")]
    pub preferred_sectors: Option<String>,
}
