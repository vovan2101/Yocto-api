use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use crate::models::investorInfo::InvestorInfo;

pub async fn fetch_contacts_from_hubspot(client: &Client, ai_api_key: &str, investor_name: &str) -> Result<Vec<InvestorInfo>, Box<dyn Error + Send + Sync>> {
    let url = format!("https://api.hubapi.com/contacts/v1/search/query?q={}", investor_name);
    let response = client.get(url).bearer_auth(ai_api_key).send().await?;
    let response_text = response.text().await?;
    let response_json: Value = serde_json::from_str(&response_text)?;
    // println!("{:?}", response_json);

    let mut investor_infos = Vec::new();

    if let Some(contacts) = response_json.get("contacts").and_then(|c| c.as_array()) {
        let vids: Vec<i64> = contacts.iter()
            .filter_map(|contact| contact.get("vid").and_then(|vid| vid.as_i64()))
            .collect();

        if !vids.is_empty() {
            let vids_str = vids.iter().map(|vid| vid.to_string()).collect::<Vec<String>>().join("&vid=");
            let details_url = format!("https://api.hubapi.com/contacts/v1/contact/vids/batch/?vid={}", vids_str);

            let details_response = client.get(&details_url).bearer_auth(ai_api_key).send().await?;
            let details_response_text = details_response.text().await?;
            let details_response_json: Value = serde_json::from_str(&details_response_text)?;
            // println!("Details Response: {:?}", details_response_json);

            for vid in vids {
                if let Some(contact) = details_response_json.get(&vid.to_string()) {
                    let name = contact.get("properties").and_then(|p| p.get("firstname")).and_then(|n| n.get("value")).unwrap_or(&Value::Null).as_str().unwrap_or("").to_string();
                    let company = contact.get("properties").and_then(|p| p.get("companies")).and_then(|c| c.get("value")).unwrap_or(&Value::Null).as_str().unwrap_or("").to_string();
                    let description = contact.get("properties").and_then(|p| p.get("fund_description")).and_then(|d| d.get("value")).unwrap_or(&Value::Null).as_str().unwrap_or("").to_string();
                    let city = contact.get("associated-company").and_then(|ac| ac.get("properties")).and_then(|p| p.get("city")).and_then(|c| c.get("value")).unwrap_or(&Value::Null).as_str().unwrap_or("").to_string();
                    let phone = contact.get("properties").and_then(|p| p.get("phone")).and_then(|ph| ph.get("value")).unwrap_or(&Value::Null).as_str().unwrap_or("").to_string();
                    let email = contact.get("properties").and_then(|p| p.get("email")).and_then(|e| e.get("value")).unwrap_or(&Value::Null).as_str().unwrap_or("").to_string();

                    let investor_info = InvestorInfo {
                        name: Some(name.to_string()),  // или None, если значение отсутствует
                        company: Some(company.to_string()),  // или None
                        description: Some(description.to_string()),  // или None
                        city: Some(city.to_string()),  // или None
                        phone_number: Some(phone.to_string()),  // или None
                        email: Some(email.to_string()),  // или None
                        preferred_sectors: Some("None".to_string()),  // или реальное значение
                    };

                    investor_infos.push(investor_info);
                }
            }
            println!("{:?}", investor_infos);
            return Ok(investor_infos);
        } else {
            println!("No contacts found with 'vid'.");
        }
    } else {
        println!("No contacts found.");
    }

    Ok(investor_infos)
}
