use crate::repository::stream_repo::tests::EqEntryValues;

use super::*;

fn tester<F>(f: F) -> Stream
where
    F: FnOnce(&mut Stream) + std::panic::UnwindSafe,
{
    let mut stream = Stream::new();
    let result = std::panic::catch_unwind(move || {
        f(&mut stream);
        stream
    });
    match result {
        Ok(stream) => stream,
        Err(err) => std::panic::resume_unwind(err),
    }
}

#[test]
fn new_stream_is_empty() {
    let stream = tester(|_| ());
    assert!(stream.is_empty());
}

#[test]
fn stream_is_not_empty_after_single_add() {
    let stream = tester(|stream| {
        stream.add_with_auto_key("value", &std::time::UNIX_EPOCH);
    });
    assert!(!(stream.is_empty()));
}

fn seed_tester<F>(f: F) -> Stream
where
    F: FnOnce(&mut Stream, &mut Vec<EntryId>, &mut Vec<String>, &mut Vec<Entry>)
        + std::panic::UnwindSafe,
{
    tester(|stream| {
        let data = ["ValueOne", "TwoItems", "LastItem"]
            .into_iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        let mut entries = data
            .into_iter()
            .map(|value| {
                Entry::new(
                    stream.add_with_auto_key(value.clone(), &std::time::UNIX_EPOCH),
                    value,
                )
            })
            .collect::<Vec<_>>();
        let (mut ids, mut values) = entries
            .clone()
            .into_iter()
            .map(|entry| (entry.id.clone(), entry.value.clone()))
            .unzip();
        f(stream, &mut ids, &mut values, &mut entries);
    })
}

#[test]
fn key_returned_from_add_auto_key_is_always_unique() {
    seed_tester(|_stream, key, _data, all| {
        while let Some(last) = key.pop() {
            assert!(!key.contains(&last));
        }
    });
}

#[test]
fn read_returns_value() {
    seed_tester(|stream, keys, values, _| {
        let read = stream.read(keys.first().unwrap(), 1);

        read.eq_values(&values[0..1]);
    });
    seed_tester(|stream, keys, values, _| {
        let read = stream.read(keys.get(1).unwrap(), 1);
        read.eq_values(&values[1..2]);
    });
}

#[test]
fn read_returns_later_values_but_not_earlier_values() {
    seed_tester(|stream, keys, values, _| {
        for (i, key) in keys.iter().enumerate() {
            let read = stream.read(key, usize::MAX);
            read.eq_values(&values[i..]);
        }
    });
}

#[test]
fn read_returns_later_values_when_key_is_not_matched() {
    seed_tester(|stream, _, values, _| {
        let read = stream.read(unsafe { &EntryId::null() }, usize::MAX);
        read.eq_values(&values);
    });
}

#[test]
fn read_last_on_empty_returns_none() {
    tester(|stream| {
        let none_value = stream.read_last();
        assert_eq!(none_value, None);
    });
}

#[test]
fn read_last_returns_last_value() {
    seed_tester(|stream, _, data, entries| {
        let last = stream.read_last().unwrap();
        assert_eq!(last, *entries.last().unwrap());
    });
}

#[test]
fn range_returns_nothing_on_empty() {
    tester(|stream| {
        let empty = stream.range(&EntryId::min(), &EntryId::max());
        assert_eq!(empty, Vec::<Entry>::new());
    });
}

#[test]
fn range_returns_everything_between_entry_min_and_max() {
    seed_tester(|stream, _, _, entries| {
        let read = stream.range(&EntryId::min(), &EntryId::max());
        assert_eq!(read, *entries);
    });
}

#[test]
fn range_returns_nothing_when_no_keys_are_in_range() {
    seed_tester(|stream, keys, _, _| {
        let read = stream.range(&(keys.last().unwrap() + 1), &EntryId::max());
        assert_eq!(read, Vec::<Entry>::new());
    });
}
#[test]
fn range_returns_keys_in_range_inclusive() {
    seed_tester(|stream, keys, _, entires| {
        let read = stream.range(keys.first().unwrap(), keys.get(1).unwrap());
        assert_eq!(read, entires[0..=1]);
    });
}
