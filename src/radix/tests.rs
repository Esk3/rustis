use super::*;

#[test]
fn new_radix_is_empty() {
    let radix = Radix::<String>::new();
    assert!(radix.is_empty());
}

#[test]
fn radix_is_not_empty_after_adding_value() {
    let mut radix = Radix::<String>::new();
    let key = "abc";
    let value = "xyz";
    radix.add(key.as_bytes(), value.into()).unwrap();
    assert!(!radix.is_empty());
}

#[test]
fn add_duplicates_fails() {
    let mut radix = Radix::<String>::new();
    let key = "abc";
    let value = "xyz";
    let other_value = "otherValue";
    radix.add(key.as_bytes(), value.into()).unwrap();
    let expect_other_value = radix.add(key.as_bytes(), other_value.into()).unwrap_err();
    assert_eq!(expect_other_value, other_value);
}

#[test]
fn add_multiple_unique_is_ok() {
    let mut radix = Radix::<String>::new();
    let kv = [("abc", "xyz"), ("123", "456"), ("hello", "world")];
    for (key, value) in kv {
        radix.add(key.as_bytes(), value.into()).unwrap();
    }
}

#[test]
fn get_added() {
    let mut radix = Radix::<String>::new();
    let key = "abc";
    let value = "xyz";
    radix.add(key.as_bytes(), value.into()).unwrap();
    let res_value = radix.get(key.as_bytes()).unwrap();
    assert_eq!(res_value, value);
}

#[test]
fn get_before_add_returns_none_after_returns_some() {
    let mut radix = Radix::<String>::new();
    let kv = [("abc", "xyz"), ("123", "456"), ("hello", "world")];
    for (key, value) in kv {
        let none_value = radix.get(key.as_bytes());
        assert_eq!(none_value, None);
        radix.add(key.as_bytes(), value.into()).unwrap();
        let some_value = radix.get(key.as_bytes());
        assert_eq!(some_value, Some(&value.to_string()));
    }
    for (key, value) in kv {
        assert_eq!(radix.get(key.as_bytes()), Some(&value.to_string()));
    }
}

#[test]
fn add_nested() {
    let mut radix = Radix::<String>::new();
    let kv = [
        ("a12", "first"),
        ("ab3", "second"),
        ("abc", "nr 3"),
        ("ab2", "four"),
    ];
    for (key, value) in kv {
        assert_eq!(radix.get(key.as_bytes()), None, "radix:?");
        radix.add(key.as_bytes(), value.into()).unwrap();
        assert_eq!(
            radix.get(key.as_bytes()),
            Some(&value.to_string()),
            "key: [{key}], value: [{value}]; {radix:?}"
        );
    }
    for (key, value) in kv {
        assert_eq!(
            radix.get(key.as_bytes()),
            Some(&value.to_string()),
            "{radix:?}"
        );
    }
    assert_eq!(
        radix.get(b"super invalid key not in radix"),
        None,
        "{radix:?}"
    );
}
