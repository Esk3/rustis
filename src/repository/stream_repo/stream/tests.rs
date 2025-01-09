use std::collections::HashSet;

use super::*;

#[test]
fn new_stream_is_empty() {
    let stream = Stream::new();
    assert!(stream.is_empty());
}

#[test]
fn stream_is_not_empty_after_add() {
    let mut stream = Stream::new();
    stream.add_default_key(EmptyEntryId, "value");
    assert!(!(stream.is_empty()));
}

const DATA: [&str; 3] = ["ValueOne", "TwoItems", "LastItem"];
const DEFAULT: EmptyEntryId = EmptyEntryId;

fn stream_data() -> Stream {
    let mut stream = Stream::new();
    for value in DATA {
        stream.add_default_key(DEFAULT, value);
    }
    stream
}

#[test]
fn key_returned_from_add_default_is_always_unique() {
    let mut stream = Stream::new();
    let mut set = HashSet::new();
    for value in ["first", "second", "third"] {
        let key = stream.add_default_key(DEFAULT, value);
        assert!(set.insert(key.clone()), "key: {key:?}, value: {value}");
    }
}

#[test]
fn read_returns_value() {
    let mut stream = Stream::new();
    for value in ["first", "second", "third"] {
        let key = stream.add_default_key(DEFAULT, value);
        let query = stream.read(&key, 1);
        assert_eq!(query, [value]);
    }
}

#[test]
fn read_returns_later_values_but_not_earlier_values() {
    let mut stream = Stream::new();
    let values = ["first", "second", "third"];
    let mut keys = Vec::with_capacity(values.len());
    for value in &values {
        let key = stream.add_default_key(DEFAULT, value);
        keys.push(key);
    }

    for (i, key) in keys.into_iter().enumerate() {
        let expected = &values[i..];
        let values = stream.read(&key, values.len());
        assert_eq!(values, expected);
    }
}

#[test]
fn read_returns_later_values_when_key_is_not_matched() {
    let mut stream = Stream::new();
    let default = "the_default";
    let values = ["first", "second", "third"];
    let mut keys = Vec::with_capacity(values.len());
    for value in &values {
        let key = stream.add_default_key(DEFAULT, value);
        keys.push(key);
    }

    let list = stream.read(&EntryId::min(), values.len());
    assert_eq!(list, values);
}

#[test]
fn read_last_on_empty_returns_none() {
    let stream = Stream::new();
    let none_value = stream.read_last();
    assert_eq!(none_value, None);
}

#[test]
fn read_last_returns_last_value() {
    let stream = stream_data();
    let last = stream.read_last().unwrap();
    assert_eq!(last, *DATA.last().unwrap());
}

#[test]
fn range_returns_nothing_on_empty() {
    let stream = Stream::new();
    let empty = stream.range(&EntryId::min(), &EntryId::max());
    assert_eq!(empty, Vec::<String>::new());
}

#[test]
fn range_returns_everything_between_entry_min_and_max() {
    let stream = stream_data();
    let range = stream.range(&EntryId::min(), &EntryId::max());
    assert_eq!(range, DATA);
}

#[test]
fn range_returns_nothing_when_no_keys_are_in_range() {
    let stream = stream_data();
    let range = stream.range(&EntryId::new(10, 0), &EntryId::max());
    assert_eq!(range, Vec::<String>::new());
}

#[test]
fn range_returns_keys_in_range_inclusive() {
    let stream = stream_data();
    let range = stream.range(&EntryId::new(0, 1), &EntryId::new(0, 2));
    dbg!(&stream);
    assert_eq!(range, &DATA[..=1]);
}

#[test]
fn stream_uses_default_key_if_empty() {
    let mut stream = Stream::new();
    let set_key = EntryId::new(1234, 567);
    let recived_key = stream.add_default_key(set_key.clone(), "myVal");
    assert_eq!(recived_key, set_key);
}
