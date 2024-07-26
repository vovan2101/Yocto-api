use reqwest::Client;
use std::error::Error;
use crate::models::form_data::ExternalFormData;

pub async fn send_form(form_data: ExternalFormData) -> Result<(), Box<dyn Error>> {
    println!("Sending form data: {:?}", form_data);
    let client = Client::new();
    let response = client.post("https://precursorvc.com/startup/")
        .form(&form_data)
        .send()
        .await?;
        
    if response.status().is_success() {
        println!("Response status: {}", response.status());
        println!("Form sent successfully!");
        Ok(())
    } else {
        println!("Response status: {}", response.status());
        Err(Box::from("Failed to send form"))
    }
}
