pub struct Node;
impl Node {
    #[must_use]
    pub fn get<'a>(
        key: String,
        repo: &'a mut super::repository::Repository,
        timestamp: &std::time::SystemTime,
    ) -> Option<&'a String> {
        repo.get(key, timestamp)
    }

    pub fn set(
        key: String,
        value: String,
        repo: &mut super::repository::Repository,
        expiry: Option<std::time::Duration>,
        timestamp: &std::time::SystemTime,
    ) -> Option<String> {
        repo.set(key, value, expiry, timestamp)
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::Repository;

    use super::*;

    #[test]
    fn unset_key_is_none() {
        let mut repo = Repository::new();
        let result = Node::get("abc".to_string(), &mut repo, &std::time::SystemTime::now());
        assert!(result.is_none());
    }

    #[test]
    fn set_key_returns_value() {
        let mut repo = Repository::new();
        let key = "my key";
        let value = "my value";
        Node::set(
            key.to_string(),
            value.to_string(),
            &mut repo,
            None,
            &std::time::SystemTime::now(),
        );
        let result = Node::get(key.to_string(), &mut repo, &std::time::SystemTime::now()).unwrap();
        assert_eq!(result, value);
    }

    #[test]
    fn value_expires() {
        let mut repo = Repository::new();
        let key = "my key";
        let value = "my value";

        let before_expiry_timestamp = std::time::SystemTime::now();
        let expiry_duration = std::time::Duration::from_secs(1);
        let after_expiry_timestamp = before_expiry_timestamp + expiry_duration * 2;

        Node::set(
            key.to_string(),
            value.to_string(),
            &mut repo,
            Some(expiry_duration),
            &before_expiry_timestamp,
        );

        let result = Node::get(key.to_string(), &mut repo, &before_expiry_timestamp).unwrap();
        assert_eq!(result, value);

        let result = Node::get(key.to_string(), &mut repo, &after_expiry_timestamp);
        assert!(result.is_none());
    }
}
