use claim::{assert_err, assert_ok};
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(name: String) ->Result<SubscriberName, String> {
        let is_empty_or_whitespace = name.trim().is_empty();
        let is_too_long = name.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = name.chars().any(|c| forbidden_characters.contains(&c));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err("Subscriber name is not valid".to_string())
        } else {
            Ok(Self(name))
        }
    }


}


impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        //&self.0则是获取其不可变引用并返回。
        &self.0
    }
}

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claim::{assert_err, assert_ok};
    #[test]
    fn a_200_OK_result_indicates_success() {
        let result = SubscriberName::parse("Ursula Le Guin".to_string());
        assert_ok!(result);
    }
    #[test]
    fn a_400_bad_request_result_indicates_validation_error() {
        let result = SubscriberName::parse("".to_string());
        assert_err!(result);
    }
    #[test]
    fn a_400_bad_request_result_indicates_name_too_long() {
        let result = SubscriberName::parse("a".repeat(257));
        assert_err!(result);
    }
    #[test]
    fn a_400_bad_request_result_indicates_name_contains_forbidden_characters() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let result = SubscriberName::parse(name.to_string());
            assert_err!(result);
        }
    }
    #[test]
    fn whitespace_only_names_are_rejected() {
        let result = SubscriberName::parse(" ".to_string());
        assert_err!(result);
    }
    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let result = SubscriberName::parse("Ursula Le Guin".to_string());
        assert_ok!(result);
    }
}