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
    pub async fn send_email(&self, recipient: SubscriberEmail, subject: String, content: String) -> Result<(), String> {
        todo!()
    }
}