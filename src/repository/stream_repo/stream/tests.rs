use std::collections::HashSet;

use super::*;

#[test]
fn new_radix_is_empty() {
    let radix = Stream::new();
    assert!(radix.is_empty());
}

#[test]
fn radix_is_not_empty_after_add() {
    let mut radix = Stream::new();
    radix.add_default_key("key".to_string(), "value");
    assert!(!(radix.is_empty()));
}

const DATA: [&str; 3] = ["ValueOne", "TwoItems", "LastItem"];
const DEFAULT: &str = "ConstDefaultValue";

fn radix_data() -> Stream {
    let mut radix = Stream::new();
    for value in DATA {
        radix.add_default_key(DEFAULT, value);
    }
    radix
}

#[test]
fn key_returned_from_add_default_is_always_unique() {
    let mut radix = Stream::new();
    let mut set = HashSet::new();
    let default = "the_default";
    for value in ["first", "second", "third"] {
        let key = radix.add_default_key(default, value);
        assert!(set.insert(key.clone()), "key: {key}, value: {value}");
    }
}

#[test]
fn read_returns_value() {
    let mut radix = Stream::new();
    let default = "the_default";
    for value in ["first", "second", "third"] {
        let key = radix.add_default_key(default, value);
        let query = radix.read(key, 1);
        assert_eq!(query, [value]);
    }
}

#[test]
fn read_returns_later_values_but_not_earlier_values() {
    let mut radix = Stream::new();
    let default = "the_default";
    let values = ["first", "second", "third"];
    let mut keys = Vec::with_capacity(values.len());
    for value in &values {
        let key = radix.add_default_key(default, value);
        keys.push(key);
    }

    for (i, key) in keys.into_iter().enumerate() {
        let expected = &values[i..];
        let values = radix.read(key, values.len());
        assert_eq!(values, expected);
    }
}

#[test]
fn read_returns_later_values_when_key_is_not_matched() {
    let mut radix = Stream::new();
    let default = "the_default";
    let values = ["first", "second", "third"];
    let mut keys = Vec::with_capacity(values.len());
    for value in &values {
        let key = radix.add_default_key(default, value);
        keys.push(key);
    }

    let list = radix.read(0, values.len());
    assert_eq!(list, values);
}

#[test]
fn read_last_on_empty_returns_none() {
    let radix = Stream::new();
    let none_value = radix.read_last();
    assert_eq!(none_value, None);
}

#[test]
fn read_last_returns_last_value() {
    let radix = radix_data();
    let last = radix.read_last().unwrap();
    assert_eq!(last, *DATA.last().unwrap());
}

#[test]
fn range_returns_nothing_on_empty() {
    let radix = Stream::new();
    let empty = radix.range(0.to_string(), u64::MAX.to_string());
    assert_eq!(empty, Vec::<String>::new());
}

#[test]
fn range_returns_everything_between_0_and_u64_MAX() {
    let radix = radix_data();
    let range = radix.range(0, u64::MAX);
    assert_eq!(range, DATA);
}

#[test]
fn range_returns_nothing_when_no_keys_are_in_range() {
    let radix = radix_data();
    let range = radix.range(10, u64::MAX);
    assert_eq!(range, Vec::<String>::new());
}

#[test]
fn range_returns_keys_in_range_inclusive() {
    let radix = radix_data();
    let range = radix.range(1, 2);
    assert_eq!(range, &DATA[..=1]);
}
