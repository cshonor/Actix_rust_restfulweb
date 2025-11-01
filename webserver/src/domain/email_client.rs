use crate::domain::subscriber_email::SubscriberEmail;
use reqwest::Client;
#[derive(Clone)]
pub struct EmailClient {
    sender: SubscriberEmail,
    client: Client,
    base_url: String,
}

impl EmailClient {
    pub fn new(sender: SubscriberEmail, client: Client, base_url: String) -> Self {
        Self { sender, client :Client::new(), base_url }
    }
}

impl EmailClient {
    pub async fn send_email(&self, recipient: SubscriberEmail, subject: String, content: String) -> Result<(), String> {
        todo!()
    }
}