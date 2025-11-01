use crate::domain::subscriber_email::SubscriberEmail;
use reqwest::Client;
use secrecy::Secret;
#[derive(Clone)]
pub struct EmailClient {
    sender: SubscriberEmail,
    client: Client,
    base_url: String,
    authorization_token: Secret<String>,
}
#[derive(serde::Serialize)]
struct SendEmailRequest<'a> {
    //'a 是 lifetime 参数，用于表示请求的生命周期
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
}

impl EmailClient {
    pub fn new(sender: SubscriberEmail, base_url: String, authorization_token: Secret<String>) -> Self {
        Self { sender, client: Client::new(), base_url, authorization_token }
    }
}

impl EmailClient {
    pub async fn send_email(&self, recipient: SubscriberEmail, subject: &str, html_content: &str, text_content: &str) -> Result<(), String> {
        let url = format!("{}/email", self.base_url);
        //创建请求
        let request = SendEmailRequest::new(
          //as_ref() 用于将 SubscriberEmail 转换为 &str
          self.sender.as_ref(),
          recipient.as_ref(),
          subject,
          html_content,
          text_content,
        );
        let buffer = self.client
        .post(url)
        .json(&request)
        .header("Authorization", self.authorization_token.expose_secret())
        .send()
        .await
        .map_err(|e| e.to_string())?;
        match buffer.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl<'a> SendEmailRequest<'a> {
    pub fn new(from: &'a str, to: &'a str, subject: &'a str, html_body: &'a str, text_body: &'a str) -> Self {
        Self { from, to, subject, html_body, text_body }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_ok, assert_err};
    use fake::{Fake, Faker};
    use fake::faker::lorem::en::{Sentence, Paragraph};
    use rand::{rngs::StdRng, SeedableRng};
    use std::sync::Once;
    use wiremock::{MockServer, ResponseTemplate, Mock};
    use fake::faker::internet::en::SafeEmail;
    use crate::routes::telemetry::{get_subscriber, init_subscriber};
    use wiremock::matchers::{method, path, header, header_exists, body_json};

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
            //模拟服务器的 URI
            mock_server.uri(),
            Secret::new(Faker.fake::<String>()),
        );

        // 3. 模拟请求  Mock::given 用于定义匹配条件和响应，是 WireMock 的核心 API。

        Mock::given(method("POST"))// 在 WireMock 中匹配方法为 POST 的请求
        .and(path("/email"))// 在 WireMock 中匹配路径为 /email 的请求
        .respond_with(ResponseTemplate::new(200))// 在 WireMock 中响应为 200 的请求
        .expect(1)// 在 WireMock 中 期望请求次数为 1
        .mount(&mock_server)
        .await;//挂载到模拟服务器
        // 4. 发送邮件  
        //发送邮件的参数：接收者、主题、HTML 内容、文本内容

        let recipient = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let html_content: String = Paragraph(1..2).fake();
        let text_content: String = Paragraph(1..2).fake();
        //发送邮件
        let _ = email_client.send_email(recipient, &subject, &html_content, &text_content).await.unwrap();
    }

    #[tokio::test]
    async fn send_email_fails_if_the_server_returns_500() {
        let _ = init();
        let mock_server = MockServer::start().await;
        let email_client = EmailClient::new(
            SubscriberEmail::parse(SafeEmail().fake()).unwrap(),
            mock_server.uri(),
            Secret::new(Faker.fake::<String>()),
        );
        let recipient = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let html_content: String = Paragraph(1..2).fake();
        let text_content: String = Paragraph(1..2).fake();   
        let request = SendEmailRequest::new(
            email_client.sender.as_ref(),
            recipient.as_ref(),
            &subject,
            &html_content,
            &text_content,
        );
        //模拟请求 header_exists("Authorization") 用于在 WireMock 中验证请求头是否存在（不检查值）。
        //header("Content-Type", "application/json") 用于在 WireMock 中验证请求头 Content-Type 的值为 "application/json"。
        //body_json(request) 用于在 WireMock 中验证请求体是否为 JSON 格式。
        //path("/email") 用于在 WireMock 中验证请求路径是否为 "/email"。
        //respond_with(ResponseTemplate::new(500)) 用于在 WireMock 中响应 500 错误。
        //expect(1) 用于在 WireMock 中期望请求次数为 1。
        //mount(&mock_server) 用于在 WireMock 中挂载模拟服务器。
        Mock::given(header_exists("Authorization"))
        .and(header("Content-Type", "application/json"))
        .and(body_json(&request))
        .and(path("/email"))
        .respond_with(ResponseTemplate::new(500))
        .expect(1)
        .mount(&mock_server)
        .await;
        let result = email_client.send_email(recipient, &subject, &html_content, &text_content).await;
        assert_err!(result);
    }
//SendEmailMatcher 是一个自定义的 WireMock 匹配器，用于验证 HTTP 请求的请求体是否符合预期
#[derive(Debug, Clone)]
#[serde(rename_all = "pascal_case")]
//#[serde(rename_all = "pascal_case")] 用于将 JSON 字段名转换为 PascalCase 格式
//#[derive(Debug, Clone)] 用于将 SendEmailMatcher 类型转换为 Debug 和 Clone 类型
struct SendEmailMatcher<'a> {
        from: &'a str,
        to: &'a str,
        subject: &'a str,
        html_body: &'a str,
        text_body: &'a str,
    }
    impl<'a> wiremock::Match for SendEmailMatcher<'a> {
        fn matches(&self, request: &wiremock::Request) -> bool {
            //将请求体转换为 JSON
            let request_as_json : Result<serde_json::Value, serde_json::Error>
             = serde_json::from_slice(request.body);// 注意：request.body 是 &[u8]
             //如果请求体转换为 JSON 成功，则判断请求体是否与预期一致 body是serde_json::value::Value类型
             if let Ok(body) = request_as_json {
                //打印请求体 dbg!是 Rust 标准库提供的一个宏，用于方便地调试代码。
                // 它会将传入的参数的值打印到标准错误输出（stderr），同时还会输出该代码所在的文件名和行号等信息，有助于快速定位代码中的问题。
                dbg!(&body);
                //检查 JSON 是否包含 from、to、subject、html_body、text_body字段
                body.get("from").is_some() &&
                body.get("to").is_some() &&// 检查 JSON 是否包含 to 字段        
                body.get("subject").is_some() &&// 检查 JSON 是否包含 subject 字段
                body.get("html_body").is_some() &&// 检查 JSON 是否包含 html_body 字段
                body.get("text_body").is_some()// 检查 JSON 是否包含 text_body 字段
             }else{
                //如果请求体转换为 JSON 失败，则返回 false
                false
             }
        }
        fn describe(&self) -> String {
            format!("matches a request with from: {}, to: {}, subject: {}, html_body: {}, text_body: {}", self.from, self.to, self.subject, self.html_body, self.text_body)
        }
    }


    #[tokio::test]
    async fn send_email_succeeds_if_the_server_returns_200() {
        let _ = init();
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(
            sender.clone(),
            mock_server.uri(),
            Secret::new(Faker.fake::<String>()),
        );
        let recipient = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let html_content: String = Paragraph(1..2).fake();
        let text_content: String = Paragraph(1..2).fake();
        Mock::given(method("POST"))
        .and(path("/email"))
        .and(header_exists("Authorization"))
        .and(header("Content-Type", "application/json"))
        .and(SendEmailMatcher {
            from: sender.as_ref(),
            to: recipient.as_ref(),
            subject: &subject,
            html_body: &html_content,
            text_body: &text_content,
        })//SendEmailMatcher 用于匹配请求体
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;
        let result = email_client.send_email(recipient, &subject, &html_content, &text_content).await;
        assert_ok!(result);
    }
}
