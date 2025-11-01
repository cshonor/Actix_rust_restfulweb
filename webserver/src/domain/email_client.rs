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
#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_ok, assert_err};
    use fake::{Fake, Faker};
    use rand::{rngs::StdRng, SeedableRng};
    use std::sync::Once;
    use wiremock::{MockServer, ResponseTemplate};
    use fake::faker::internet::en::SafeEmail;
    static TRACING: Once = Once::new();
    
    fn init() {
        TRACING.call_once(|| {
            let subscriber = get_subscriber("test".into(), "info".into(), std::io::stdout());
            init_subscriber(subscriber);
        });
    }

    #[tokio::test]
    async fn send_email_sends_the_expected_request() {
        let _ = init();
        let mock_server = MockServer::start().await;
        let email_client = EmailClient::new(
            SubscriberEmail::parse(SafeEmail().fake()).unwrap(),
            Client::new(),
            mock_server.uri(),
        );
        let response = email_client.send_email(
            SubscriberEmail::parse(SafeEmail().fake()).unwrap(),
            "subject".to_string(),
            "content".to_string(),
        ).await;
        assert_ok!(response);
    }
}