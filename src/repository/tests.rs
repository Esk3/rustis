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
    let none_value = repo.get("key").unwrap();
    assert_eq!(none_value, None);
}

#[test]
fn getting_set_value_returns_some() {
    let repo = Repository::new();
    let key = "key";
    repo.set(key.to_string(), "value".to_string(), None)
        .unwrap();
    let some_value = repo.get(key).unwrap();
    assert!(some_value.is_some());
}

#[test]
fn getting_set_value_returns_same_value() {
    let repo = Repository::new();
    let key = "key";
    let value = "value";
    repo.set(key.to_string(), value.to_string(), None).unwrap();
    let some_value = repo.get(key).unwrap();
    assert_eq!(some_value, Some(value.to_string()));
}

#[test]
fn getting_other_value_still_returns_none() {
    let repo = Repository::new();
    let key = "key";
    let value = "value";
    repo.set(key.to_string(), value.to_string(), None).unwrap();
    let some_value = repo.get(key).unwrap();
    assert_eq!(some_value, Some(value.to_string()));

    let none_value = repo.get("other_key").unwrap();
    assert_eq!(none_value, None);
}

#[test]
fn set_value_with_expiry() {
    let repo = Repository::new();
    repo.set(
        "key".to_string(),
        "value".to_string(),
        Some(std::time::Duration::from_secs(1)),
    )
    .unwrap();
}
