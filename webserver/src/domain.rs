pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(name: String) -> SubscriberName {
        let is_empty_or_whitespace = name.trim().is_empty();
        let is_too_long = name.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = name.chars().any(|c| forbidden_characters.contains(&c));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            panic!("Subscriber name is not valid");
        }
        Self(name)
    }

    pub fn inner(&self) -> &str {
        &self.0
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
