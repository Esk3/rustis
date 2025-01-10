use super::KvRepository;

#[test]
fn new_repository_is_empty() {
    let repo = KvRepository::new();
    assert!(repo.is_empty());
}

#[test]
fn repository_is_not_empty_after_setting_value() {
    let repo = KvRepository::new();
    repo.set("key".to_string(), "value".to_string(), None)
        .unwrap();
    assert!(!repo.is_empty());
}

#[test]
fn getting_empty_repository_returns_none() {
    let repo = KvRepository::new();
    let none = repo.get("key", std::time::SystemTime::UNIX_EPOCH).unwrap();
    assert_eq!(none, None);
}

#[test]
fn getting_set_value_returns_some() {
    let repo = KvRepository::new();
    let key = "key";
    repo.set(key.to_string(), "value".to_string(), None)
        .unwrap();
    let some = repo.get(key, std::time::SystemTime::UNIX_EPOCH).unwrap();
    assert!(some.is_some());
}

#[test]
fn getting_set_value_returns_same_value() {
    let repo = KvRepository::new();
    let key = "key";
    let value = "value";
    repo.set(key.to_string(), value.to_string(), None).unwrap();
    let some_value = repo.get(key, std::time::SystemTime::UNIX_EPOCH).unwrap();
    assert_eq!(some_value, Some(value.to_string()));
}

fn getting_other_value_still_returns_none() {
    let repo = KvRepository::new();
    let key = "key";
    let value = "value";
    repo.set(key.to_string(), value.to_string(), None).unwrap();
    let some_value = repo.get(key, std::time::SystemTime::UNIX_EPOCH).unwrap();
    assert_eq!(some_value, Some(value.to_string()));

    let none = repo
        .get("other_key", std::time::SystemTime::UNIX_EPOCH)
        .unwrap();
    assert_eq!(none, None)
}

#[test]
fn set_value_with_expiry() {
    let repo = KvRepository::new();
    repo.set(
        "key".to_string(),
        "value".to_string(),
        Some(std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(10)),
    )
    .unwrap();
}

#[test]
fn get_value_with_expiry_is_some_before_timestamp() {
    let repo = KvRepository::new();
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
}
#[test]
fn get_value_with_expiry_is_none_after_timestamp() {
    let repo = KvRepository::new();
    let (key, value) = ("abc", "xyz");
    let timestamp = std::time::SystemTime::UNIX_EPOCH;
    repo.set(key.into(), value.into(), Some(timestamp)).unwrap();
    let none = repo
        .get(key, timestamp + std::time::Duration::from_secs(1))
        .unwrap();
    assert_eq!(none, None);
}
#[test]
fn get_value_with_expiry_is_always_none_after_one_get_after_timestamp() {
    let repo = KvRepository::new();
    let (key, value) = ("abc", "xyz");
    let timestamp = std::time::SystemTime::UNIX_EPOCH;
    repo.set(key.into(), value.into(), Some(timestamp)).unwrap();
    let none_value = repo
        .get(key, timestamp + std::time::Duration::from_secs(1))
        .unwrap();
    assert_eq!(none_value, None);

    let none = repo
        .get(key, timestamp - std::time::Duration::from_secs(1))
        .unwrap();
    assert_eq!(none, None);
}
