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
        stream.add_with_auto_key(vec![Field::new("k", "value")], &std::time::UNIX_EPOCH);
    });
    assert!(!(stream.is_empty()));
}

fn seed_tester<F>(f: F) -> Stream
where
    F: FnOnce(&mut Stream, Vec<Entry>) + std::panic::UnwindSafe,
{
    tester(|stream| {
        let fields = [
            [Field::new("abc", "123"), Field::new("xyz", "bca")].to_vec(),
            [Field::new("keyTwo", "ValueFirstTwo")].to_vec(),
        ];
        let entries = fields
            .into_iter()
            .map(|fields| {
                Entry::new(
                    stream.add_with_auto_key(fields.clone(), &std::time::UNIX_EPOCH),
                    fields,
                )
            })
            .collect::<Vec<Entry>>();
        f(stream, entries);
    })
}

#[test]
fn key_returned_from_add_auto_key_is_always_unique() {
    seed_tester(|_stream, mut entries| {
        while let Some(last) = entries.pop() {
            assert_eq!(entries.iter().find(|entry| entry.id == last.id), None);
        }
    });
}

#[test]
fn read_returns_value_bigger_than_key() {
    seed_tester(|stream, entries| {
        let id = entries.first().unwrap().id();
        let read = stream.read(id, 1);

        assert_eq!(
            read,
            entries.into_iter().skip(1).take(1).collect::<Vec<_>>()
        );
    });
    seed_tester(|stream, entries| {
        let read = stream.read(entries.get(1).unwrap().id(), 1);
        assert_eq!(
            read,
            entries.into_iter().skip(2).take(1).collect::<Vec<_>>()
        );
    });
}

#[test]
fn read_returns_later_values_but_not_earlier_values() {
    seed_tester(|stream, entries| {
        for (i, key) in entries.iter().map(super::Entry::id).enumerate() {
            let read = stream.read(key, usize::MAX);
            assert_eq!(read, entries[i + 1..]);
        }
    });
}

#[test]
fn read_returns_later_values_when_key_is_not_matched() {
    seed_tester(|stream, entries| {
        let read = stream.read(unsafe { &EntryId::null() }, usize::MAX);
        assert_eq!(read, entries);
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
    seed_tester(|stream, entries| {
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
fn range_returns_everything_between_entry_min_and_max_inclusive() {
    seed_tester(|stream, entries| {
        let read = stream.range(&EntryId::min(), &EntryId::max());
        assert_eq!(read, *entries);
    });
}

#[test]
fn range_returns_nothing_when_no_keys_are_in_range() {
    seed_tester(|stream, entries| {
        let read = stream.range(&(entries.last().unwrap().id() + 1), &EntryId::max());
        assert_eq!(read, Vec::<Entry>::new());
    });
}
#[test]
fn range_returns_keys_in_range_inclusive() {
    seed_tester(|stream, entries| {
        let read = stream.range(entries.first().unwrap().id(), entries.get(1).unwrap().id());
        assert_eq!(read, entries[0..=1]);
    });
}
