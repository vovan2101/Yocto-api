#[cfg(test)]
mod tests {
    use reqwest::Client;
    use serde_json::json;
    use tokio;

    #[tokio::test]
    async fn test_send_form() {
        let client = Client::new();
        let res = client
            .post("http://localhost:3001/send-form")
            .json(&json!({
                "first_name": "John",
                "last_name": "Doe",
                "email": "john.doe@example.com",
                "relationship": "Founder",
                "company_name": "Doe Industries",
                "description": "We create innovative solutions for everyday problems.",
                "industry": "Technology",
                "website": "http://doeindustries.com",
                "pitch_deck": "http://doeindustries.com/pitch_deck.pdf"
            }))
            .send()
            .await
            .expect("Failed to send request");

        assert!(res.status().is_success(), "Request was not successful");
        let body = res.text().await.expect("Failed to read response body");
        println!("Response: {}", body);
    }
}
