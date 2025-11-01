use crate::domain::subscriber_email::SubscriberEmail;
use reqwest::Client;

pub struct EmailClient {
    sender: SubscriberEmail,
    client: Client,
}

impl EmailClient {
    pub fn new(sender: SubscriberEmail, client: Client) -> Self {
        Self { sender, client }
    }
}

impl EmailClient {
    pub fn send_email(&self, recipient: SubscriberEmail, subject: String, content: String) -> Result<(), String> {
        let url = format!("{}/email", self.base_url);
        let request_body = serde_json::json!({
            "from": self.sender,
            "to": recipient,
            "subject": subject,
            "content": content,
        });
        let response = self.client.post(url).json(&request_body).send().await.map_err(|e| e.to_string())?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(response.text().await.map_err(|e| e.to_string())?)
        }
    }
}