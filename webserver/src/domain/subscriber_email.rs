use claim::{assert_err, assert_ok};
use validator::Validate;
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
#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use claim::{assert_err, assert_ok};
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
}   