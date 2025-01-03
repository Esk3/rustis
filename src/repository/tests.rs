use super::*;

#[test]
fn create_repository() {
    let _: Repository = Repository::new();
}

#[test]
fn new_repository_is_empty() {
    let repo = Repository::new();
    assert!(repo.is_empty());
}

#[test]
fn repository_is_not_empty_after_setting_value() {
    let repo = Repository::new();
    repo.set("key".to_string(), "value".to_string(), None)
        .unwrap();
    assert!(!repo.is_empty());
}

#[test]
fn getting_empty_repository_returns_none() {
    let repo = Repository::new();
    let none_value = repo.get("key", std::time::SystemTime::now()).unwrap();
    assert_eq!(none_value, None);
}

#[test]
fn getting_set_value_returns_some() {
    let repo = Repository::new();
    let key = "key";
    repo.set(key.to_string(), "value".to_string(), None)
        .unwrap();
    let some_value = repo.get(key, std::time::SystemTime::now()).unwrap();
    assert!(some_value.is_some());
}

#[test]
fn getting_set_value_returns_same_value() {
    let repo = Repository::new();
    let key = "key";
    let value = "value";
    repo.set(key.to_string(), value.to_string(), None).unwrap();
    let some_value = repo.get(key, std::time::SystemTime::now()).unwrap();
    assert_eq!(some_value, Some(value.to_string()));
}

#[test]
fn getting_other_value_still_returns_none() {
    let repo = Repository::new();
    let key = "key";
    let value = "value";
    repo.set(key.to_string(), value.to_string(), None).unwrap();
    let some_value = repo.get(key, std::time::SystemTime::now()).unwrap();
    assert_eq!(some_value, Some(value.to_string()));

    let none_value = repo.get("other_key", std::time::SystemTime::now()).unwrap();
    assert_eq!(none_value, None);
}

#[test]
fn set_value_with_expiry() {
    let repo = Repository::new();
    repo.set(
        "key".to_string(),
        "value".to_string(),
        Some(std::time::SystemTime::now() + std::time::Duration::from_secs(10)),
    )
    .unwrap();
}

#[test]
fn get_value_with_expiry_is_some_before_timestamp() {
    let repo = Repository::new();
    let (key, value) = ("abc", "xyz");
    let timestamp = std::time::SystemTime::now();
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
    let repo = Repository::new();
    let (key, value) = ("abc", "xyz");
    let timestamp = std::time::SystemTime::now();
    repo.set(key.into(), value.into(), Some(timestamp)).unwrap();
    let none_value = repo
        .get(key, timestamp + std::time::Duration::from_secs(1))
        .unwrap();
    assert_eq!(none_value, None);
}
#[test]
fn get_value_with_expiry_is_always_none_after_one_get_after_timestamp() {
    let repo = Repository::new();
    let (key, value) = ("abc", "xyz");
    let timestamp = std::time::SystemTime::now();
    repo.set(key.into(), value.into(), Some(timestamp)).unwrap();
    let none_value = repo
        .get(key, timestamp + std::time::Duration::from_secs(1))
        .unwrap();
    assert_eq!(none_value, None);

    let none_value = repo
        .get(key, timestamp - std::time::Duration::from_secs(1))
        .unwrap();
    assert_eq!(none_value, None);
}
