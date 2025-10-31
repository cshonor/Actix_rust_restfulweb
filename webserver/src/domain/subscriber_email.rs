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
    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully_quickcheck(email: String) -> bool {
        SubscriberEmail::parse(email).is_ok()
    }
    #[quickcheck_macros::quickcheck]
    fn invalid_emails_are_rejected_quickcheck(email: String) -> bool {
        SubscriberEmail::parse(email).is_err()
    }
}
  
