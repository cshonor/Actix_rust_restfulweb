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
    pub async fn send_email(&self, recipient: SubscriberEmail, subject: &str, html_content: &str, text_content: &str) -> Result<(), String> {
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
    use crate::routes::telemetry::{get_subscriber, init_subscriber};
    use wiremock::matchers::{method, path};

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
        // 1. 启动一个模拟服务器，用于测试邮件发送
        let mock_server = MockServer::start().await;
        // 2. 创建一个邮件客户端
        let email_client = EmailClient::new(
            SubscriberEmail::parse(SafeEmail().fake()).unwrap(),
            Client::new(),
            mock_server.uri(),
        );

        // 3. 模拟请求  Mock::given 用于定义匹配条件和响应，是 WireMock 的核心 API。

        Mock::given(method("POST"))//匹配方法为 POST 的请求
        .and(path("/email"))//匹配路径为 /email 的请求
        .respond_with(ResponseTemplate::new(200))//响应为 200 的请求
        .expect(1)//期望请求次数为 1
        .mount(&mock_server)
        .await;//挂载到模拟服务器
        // 4. 发送邮件  
        let _ = email_client.send_email(
            SubscriberEmail::parse(SafeEmail().fake()).unwrap(),
            "subject".into(),
            "content".into(),
            "text_content".into(),
        ).await.unwrap();

        // 5. 验证请求是否发送
        let request_sent = request_was_sent(&mock_server).await;
        assert!(request_sent);
    }
}