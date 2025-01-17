use stream::Field;

use super::*;

fn tester<F>(f: F) -> StreamRepository
where
    F: FnOnce(&mut StreamRepository) + std::panic::UnwindSafe,
{
    let mut repo = StreamRepository::new();
    match std::panic::catch_unwind(move || {
        f(&mut repo);
        repo
    }) {
        Ok(repo) => repo,
        Err(err) => std::panic::resume_unwind(err),
    }
}

#[test]
fn create_stream_repo() {
    tester(|repo| {
        assert!(repo.is_empty());
    });
}

#[test]
fn xadd_creates_stream_if_key_does_not_exsist() {
    tester(|repo| {
        repo.add_auto_increment(
            "myNewStream",
            vec![Field::new("mykey", "myValue")],
            &std::time::UNIX_EPOCH,
        );
        assert!(!repo.is_empty());
    });
}

#[test]
#[should_panic(expected = "stream not found")]
fn xread_fails_if_stream_does_not_exsist() {
    tester(|repo| {
        let _err = repo.read("notFound", &EntryId::min(), usize::MAX).unwrap();
    });
}

#[test]
fn xread_returns_newly_added_field_if_done_on_a_new_stream() {
    tester(|repo| {
        let stream_key = "streamkey123";
        let fields = vec![Field::new("abc", "xyz")];
        repo.add_auto_increment(stream_key, fields.clone(), &std::time::UNIX_EPOCH);
        let read = repo
            .read(stream_key, &EntryId::new(0, 0), usize::MAX)
            .unwrap();
        assert_eq!(read.first().unwrap().fields(), fields);
    });
}

#[test]
fn xread_returns_empty_list_if_keys_not_found() {
    tester(|repo| {
        let stream_key = "streamKey";
        let _ = repo.add_auto_increment(
            stream_key,
            vec![Field::new("not gonna", "be found")],
            &std::time::UNIX_EPOCH,
        );

        let empty_list = repo.read(stream_key, &EntryId::max(), 1).unwrap();
        assert!(empty_list.is_empty());
    });
}

fn seed_tester<F>(f: F) -> StreamRepository
where
    F: FnOnce(&mut StreamRepository, &mut Vec<(String, Vec<Entry>)>) + std::panic::UnwindSafe,
{
    let fields = [
        [
            Field::new("fsfsaf", "djfldasføl"),
            Field::new("hello", "world"),
            Field::new("what is", "up"),
        ]
        .to_vec(),
        [Field::new("dfjsalkf", "djfldasføl")].to_vec(),
        [
            Field::new("fjk", "dfjk"),
            Field::new("hello", "fsadfasdf"),
            Field::new("SF", "left"),
        ]
        .to_vec(),
    ];
    let keys = [("abc", fields)];
    tester(|repo| {
        let mut keys = keys
            .into_iter()
            .map(|(stream_key, fields)| {
                (
                    stream_key.to_string(),
                    fields
                        .into_iter()
                        .map(|fields| {
                            Entry::new(
                                repo.add_auto_increment(
                                    stream_key,
                                    fields.clone(),
                                    &std::time::UNIX_EPOCH,
                                ),
                                fields,
                            )
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<(String, Vec<Entry>)>>();
        f(repo, &mut keys);
    })
}

#[test]
fn keys_are_unique() {
    seed_tester(|_repo, stream_keys| {
        for (_stream_key, entires) in stream_keys {
            while let Some(entry) = entires.pop() {
                assert_eq!(entires.iter().find(|e| e.id() == entry.id()), None);
            }
        }
    });
}

#[test]
fn xread_last() {
    seed_tester(|repo, keys| {
        let (stream_key, data) = keys.first().unwrap();
        let expected_last_entry = data.last().unwrap();
        let read = repo.read_last(stream_key).unwrap();
        assert_eq!(read, *expected_last_entry);
    });
}

// normal block returns imidiately once it has any data
// read with special $ symbol blocks and only returns newly recived data since blocking
// read with special + symbol reads last value in stream

#[test]
#[should_panic(expected = "stream not found")]
fn xrange_on_empty_repo_fails() {
    tester(|repo| {
        repo.range("any", &EntryId::min(), &EntryId::max()).unwrap();
    });
}

#[test]
fn xrange_test() {
    seed_tester(|repo, stream_keys| {
        for (stream_key, entries) in stream_keys {
            let entry = entries.first().unwrap();
            let found_values = repo.range(stream_key, entry.id(), entry.id()).unwrap();
            assert_eq!(found_values, [entry.clone()], "{entry:?}, {entries:?}");
        }
    });
}

// special - and + returns are min and max

#[test]
fn blocking_query_does_not_block_when_it_finds_data() {
    seed_tester(|repo, streams| {
        for (key, entries) in streams {
            let start = std::time::Instant::now();
            let block_duration = std::time::Duration::from_millis(10);

            let entry = entries.first().unwrap();
            let result = repo.blocking_query(Some(block_duration), |repo| {
                let res = repo.range(key.clone(), entry.id(), entry.id()).unwrap();
                if res.is_empty() {
                    BlockResult::NotFound
                } else {
                    BlockResult::Found(res)
                }
            });
            let BlockResult::Found(value) = result else {
                panic!()
            };
            assert_eq!(value, [entry.clone()]);

            let elapsed = start.elapsed();
            assert!(elapsed < block_duration);
        }
    });
}

#[test]
fn blocking_query_blocks_on_no_data_and_returns_none() {
    seed_tester(|repo, streams| {
        for (key, entries) in streams {
            let start = std::time::Instant::now();
            let block_duration = std::time::Duration::from_millis(10);

            let entry_id = entries.last().unwrap().id() + 1;
            let result = repo.blocking_query(Some(std::time::Duration::from_millis(100)), |repo| {
                let res = repo.range(key.clone(), &entry_id, &entry_id).unwrap();
                if res.is_empty() {
                    BlockResult::NotFound
                } else {
                    BlockResult::Found(res)
                }
            });

            let elapsed = start.elapsed();
            assert_eq!(result, BlockResult::NotFound);
            assert!(elapsed >= block_duration);
        }
    });
}

#[test]
#[ignore = "todo"]
fn values_are_vissable_in_all_clones_of_repo() {
    todo!()
}

#[test]
fn blocking_returns_data_recived_during_block() {
    seed_tester(|repo, streams| {
        for (stream_key, entries) in streams {
            let block_duration = std::time::Duration::from_millis(100);
            let repo2 = repo.clone();
            let handle;
            {
                let stream_key = stream_key.clone();
                let entry = entries.last().unwrap().id() + 1;
                handle = std::thread::spawn(move || {
                    repo2.blocking_query(Some(block_duration), |repo: &StreamRepository| {
                        repo.range(stream_key.clone(), &entry, &entry)
                            .map(|v| {
                                if v.is_empty() {
                                    BlockResult::NotFound
                                } else {
                                    BlockResult::Found(v)
                                }
                            })
                            .unwrap()
                    })
                });
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
            assert!(!handle.is_finished(), "{:?}", handle.join());

            let new_field = vec![Field::new("thenewKey", "theNewValue")];
            let key =
                repo.add_auto_increment(stream_key, new_field.clone(), &std::time::UNIX_EPOCH);

            dbg!(key);
            std::thread::sleep(std::time::Duration::from_millis(4));
            assert!(handle.is_finished());
            let res = handle.join().unwrap();
            let BlockResult::Found(entries) = res else {
                panic!()
            };
            assert_eq!(entries.first().unwrap().fields(), new_field);
        }
    });
}

#[test]
fn blocking_read_returns_not_found_if_no_data_in_range() {
    seed_tester(|repo, streams| {
        for (key, entries) in streams {
            let result = repo.read_blocking(
                key,
                &(entries.last().unwrap().id() + 1),
                10,
                Some(std::time::Duration::from_millis(10)),
            );
            assert_eq!(result, BlockResult::NotFound);
        }
    });
}

#[test]
fn blocking_read_returns_found_data_as_normal() {
    seed_tester(|repo, streams| {
        for (key, entries) in streams {
            let id = entries.first().unwrap().id();
            let count = 2;
            let expected = repo.read(key.clone(), id, count).unwrap();
            assert!(!expected.is_empty(), "bad test. read is empty");
            let actual =
                repo.read_blocking(key, id, count, Some(std::time::Duration::from_millis(10)));
            let BlockResult::Found(actual) = actual else {
                panic!("expected BlockResult::Found got: {actual:?}");
            };
            assert_eq!(actual, expected);
        }
    });
}

#[test]
fn blocking_read_returns_data_recived_during_block() {
    seed_tester(|repo, streams| {
        for (stream_key, entries) in streams {
            let block_duration = std::time::Duration::from_millis(100);
            let repo2 = repo.clone();
            let handle;
            {
                let stream_key = stream_key.clone();
                let entry_id = entries.last().unwrap().id().clone();
                handle = std::thread::spawn(move || {
                    repo2.read_blocking(stream_key, &entry_id, 10, Some(block_duration))
                });
            }

            std::thread::sleep(std::time::Duration::from_millis(1));
            assert!(!handle.is_finished(), "{:?}", handle.join());

            let new_value = vec![Field::new("someNewKey", "theNewValue")];
            repo.add_auto_increment(stream_key, new_value.clone(), &std::time::UNIX_EPOCH);

            std::thread::sleep(std::time::Duration::from_millis(4));
            assert!(handle.is_finished());
            let res = handle.join().unwrap();
            let BlockResult::Found(entries) = res else {
                panic!()
            };
            assert_eq!(entries.first().unwrap().fields(), new_value);
        }
    });
}
