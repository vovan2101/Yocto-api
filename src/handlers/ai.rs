use reqwest::Client;
// use serde::Serialize;
use serde_json::{Value, json};
use std::error::Error;
use crate::models::investorInfo::InvestorInfo;

pub async fn generate_investor_summary_with_ai(client: &Client, investor_infos: Vec<InvestorInfo>, ai_api_key: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let investor_data_json = json!(investor_infos);
    let prompt = format!(
        "Analyze the following JSON data and provide a conversational description about the investor, including their name, company, phone number, description, city, email, and any other relevant information. The goal is to give a clear and engaging overview of the investor based on all the information from the Json. If any fields are missing or have no data, simply skip them.\n\nJSON data:\n{}",
        investor_data_json.to_string()
    );

    let request_body = json!({
        "contents": [{
            "parts": [{"text": prompt}]
        }]
    });

    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", ai_api_key);
    
    let response = client.post(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;
        
    // let status = response.status();
    let response_text = response.text().await?;
    // println!("Response Status: {}", status);
    // println!("Response Body: {}", response_text);

    let response_json: Value = serde_json::from_str(&response_text)?;
    // println!("Parsed JSON: {:?}", response_json);

    Ok(response_json["candidates"][0]["content"]["parts"][0]["text"].as_str().unwrap_or("").to_string())
}
