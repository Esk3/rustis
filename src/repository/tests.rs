use crate::test_helper;

use super::*;

test_helper! {
    RepoTester {repo: Repository, Repository::new()}
    new_repository_is_empty() {
        assert!(repo.is_empty());
    };

    repository_is_not_empty_after_setting_value() {
        repo.set("key".to_string(), "value".to_string(), None)
            .unwrap();
        assert!(!repo.is_empty());
    };

    [none]
    getting_empty_repository_returns_none() {
        repo.get("key", std::time::SystemTime::UNIX_EPOCH).unwrap()
    };

    [some]
    getting_set_value_returns_some() {
        let key = "key";
        repo.set(key.to_string(), "value".to_string(), None)
            .unwrap();
        repo.get(key, std::time::SystemTime::UNIX_EPOCH).unwrap()
    };

    [eq Some("value".to_string())]
    getting_set_value_returns_same_value() {
        let key = "key";
        let value = "value";
        repo.set(key.to_string(), value.to_string(), None).unwrap();
        repo.get(key, std::time::SystemTime::UNIX_EPOCH).unwrap()
    };

    [none]
    getting_other_value_still_returns_none() {
        let key = "key";
        let value = "value";
        repo.set(key.to_string(), value.to_string(), None).unwrap();
        let some_value = repo.get(key, std::time::SystemTime::UNIX_EPOCH).unwrap();
        assert_eq!(some_value, Some(value.to_string()));

        repo.get("other_key", std::time::SystemTime::UNIX_EPOCH).unwrap()
    };

    set_value_with_expiry() {
        repo.set(
            "key".to_string(),
            "value".to_string(),
            Some(std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(10)),
        )
            .unwrap();
        };

    get_value_with_expiry_is_some_before_timestamp() {
        let (key, value) = ("abc", "xyz");
        let timestamp = std::time::SystemTime::UNIX_EPOCH;
        repo.set(
            key.into(),
            value.into(),
            Some(timestamp + std::time::Duration::from_secs(1)),
        )
            .unwrap();
        let some_value = repo.get(key, timestamp).unwrap();
        assert_eq!(some_value, Some(value.into()));
    };
    [none]
    get_value_with_expiry_is_none_after_timestamp() {
        let (key, value) = ("abc", "xyz");
        let timestamp = std::time::SystemTime::UNIX_EPOCH;
        repo.set(key.into(), value.into(), Some(timestamp)).unwrap();
        repo
            .get(key, timestamp + std::time::Duration::from_secs(1))
            .unwrap()
    };
    [none]
    get_value_with_expiry_is_always_none_after_one_get_after_timestamp() {
        let (key, value) = ("abc", "xyz");
        let timestamp = std::time::SystemTime::UNIX_EPOCH;
        repo.set(key.into(), value.into(), Some(timestamp)).unwrap();
        let none_value = repo
            .get(key, timestamp + std::time::Duration::from_secs(1))
            .unwrap();
        assert_eq!(none_value, None);

        repo
            .get(key, timestamp - std::time::Duration::from_secs(1))
            .unwrap()
    };
}
