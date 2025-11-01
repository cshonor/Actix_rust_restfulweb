use crate::domain::SubscriberEmail;
use crate::domain::SubscriberName;
use claim::{assert_err, assert_ok};

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
#[cfg(test)]
mod tests {
    use crate::domain::NewSubscriber;
    use claim::{assert_err, assert_ok};
    #[test]
    fn a_200_OK_result_indicates_success() {
        let result = NewSubscriber::parse("Ursula Le Guin".to_string());
        assert_ok!(result);
    }
}