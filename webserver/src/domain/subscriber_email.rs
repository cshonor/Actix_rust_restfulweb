use claim::{assert_err, assert_ok};
#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(email: String) -> Result<SubscriberEmail, String> {
        if validator::validate_email(&email) {
            Ok(Self(email.to_lowercase()))
        } else {
            Err("Email is not valid".to_string())
        }
    }
}
impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SubscriberEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[cfg(test)]
mod tests { 
    use crate::domain::SubscriberEmail;
    use claim::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use quickcheck::{Arbitrary, Gen};
    
    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl Arbitrary for ValidEmailFixture {
        // quickcheck 要求 "可生成随机数据的类型" 必须实现此 trait，arbitrary 方法定义了数据的生成规则。
        fn arbitrary(_g: &mut Gen) -> Self {
            let email = SafeEmail().fake();
            Self(email)
        }
    }

    #[derive(Debug, Clone)]
    struct InvalidEmailFixture(pub String);

    // 必须实现 Arbitrary trait，才能使用 quickcheck 进行随机测试
    impl Arbitrary for InvalidEmailFixture {
        // 生成无效邮箱的规则：随机选择预定义的无效邮箱模式 
        // arbitrary 方法定义数据生成规则
        fn arbitrary(g: &mut Gen) -> Self {
            // 生成一个随机索引，确保不会 panic 通过 Gen 参数生成随机数据
            // 预定义的无效邮箱模式列表
            let invalid_patterns = vec![
                "".to_string(),                    // 空字符串
                "not-an-email".to_string(),        // 完全无效
                "user".to_string(),                // 缺少 @ 和域名
                "user@".to_string(),               // 缺少域名
                "user@domain".to_string(),         // 缺少顶级域名
                "userdomain.com".to_string(),      // 缺少 @
                "@domain.com".to_string(),         // 缺少用户名
            ];
            // 使用索引选择，确保不会 panic
            let index = (u32::arbitrary(g) % invalid_patterns.len() as u32) as usize;
            Self(invalid_patterns[index].clone())
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully_quickcheck(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }

    #[quickcheck_macros::quickcheck]
    fn invalid_emails_are_rejected_quickcheck(invalid_email: InvalidEmailFixture) -> bool {
        SubscriberEmail::parse(invalid_email.0).is_err()
    }
    #[test]
    fn a_200_OK_result_indicates_success() {
        let result = SubscriberEmail::parse("ursula_le_guin@gmail.com".to_string());
        assert_ok!(result);
    }
    #[test]
    fn a_400_bad_request_result_indicates_validation_error() {
        let result = SubscriberEmail::parse("ursula_le_guin@gmail".to_string());
        assert_err!(result);
    }
    #[test]
    fn empty_string_is_rejected() {
        let result = SubscriberEmail::parse("".to_string());
        assert_err!(result);
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let result = SubscriberEmail::parse("ursulagmail.com".to_string());
        assert_err!(result);
    }
    #[test]
    fn email_missing_dot_is_rejected() {
        let result = SubscriberEmail::parse("ursula@gmailcom".to_string());
        assert_err!(result);
    }
    #[test]
    fn valid_emails_are_parsed_successfully() {
        let email = SafeEmail().fake();
        assert_ok!(SubscriberEmail::parse(email));
    }
    #[test]
    fn invalid_emails_are_rejected() {
        let emails = vec![
            "ursulagmail.com",      // 缺少 @ 符号
            "ursula@gmailcom",      // 缺少域名点
            "not-an-email",         // 完全无效
       ];
        for email in emails {
            assert_err!(SubscriberEmail::parse(email.to_string()));
        }
    }
}
  
