use super::*;

pub(super) trait EqEntryValues<Rhs: ?Sized = Self> {
    fn eq_values(self, rhs: &Rhs);
}

impl EqEntryValues<[String]> for Vec<Entry> {
    fn eq_values(self, rhs: &[String]) {
        let v = self.into_iter().map(|e| e.value).collect::<Vec<String>>();
        assert_eq!(v, rhs);
    }
}

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
        repo.xadd_auto_increment("myNewStream", "myValue", &std::time::UNIX_EPOCH);
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
fn xread_returns_newly_added_value_if_done_on_a_new_stream() {
    tester(|repo| {
        let stream_key = "streamkey123";
        let value = "abc";
        let key = repo.xadd_auto_increment(stream_key, value, &std::time::UNIX_EPOCH);
        let read = repo.read(stream_key, &key, usize::MAX).unwrap();
        read.eq_values(&[value.to_string()]);
        let read = repo.read(stream_key, &EntryId::min(), usize::MAX).unwrap();
        read.eq_values(&[value.to_string()]);
    });
}

#[test]
fn xread_returns_empty_list_if_keys_not_found() {
    tester(|repo| {
        let stream_key = "streamKey";
        let _ = repo.xadd_auto_increment(stream_key, "abc", &std::time::UNIX_EPOCH);

        let empty_list = repo.read(stream_key, &EntryId::max(), 1).unwrap();
        assert!(empty_list.is_empty());
    });
}

fn seed_tester<F>(f: F) -> StreamRepository
where
    F: FnOnce(&mut StreamRepository, &mut Vec<(String, Vec<Entry>)>) + std::panic::UnwindSafe,
{
    let values1 = ["First", "iOne", "Last"]
        .into_iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>();
    let keys = [("abc", values1)];
    tester(|repo| {
        let mut keys = keys
            .into_iter()
            .map(|(stream_key, values)| {
                (
                    stream_key.to_string(),
                    values
                        .into_iter()
                        .map(|value| {
                            Entry::new(
                                repo.xadd_auto_increment(
                                    stream_key,
                                    value.clone(),
                                    &std::time::UNIX_EPOCH,
                                ),
                                value,
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
                assert_eq!(entires.iter().find(|e| e.id == entry.id), None);
            }
        }
    });
}

#[test]
fn xread_last() {
    seed_tester(|repo, keys| {
        let (stream_key, data) = keys.first().unwrap();
        let expected_last_entry = data.last().unwrap();
        let read = repo.xread_last(stream_key).unwrap();
        assert_eq!(read.value, *expected_last_entry.value);
    });
}

// normal block returns imidiately once it has any data
// read with special $ symbol blocks and only returns newly recived data since blocking
// read with special + symbol reads last value in stream

#[test]
#[should_panic(expected = "stream not found")]
fn xrange_on_empty_repo_fails() {
    tester(|repo| {
        repo.xrange("any", &EntryId::min(), &EntryId::max())
            .unwrap();
    });
}

#[test]
fn xrange_test() {
    seed_tester(|repo, stream_keys| {
        for (stream_key, entries) in stream_keys {
            let entry = entries.first().unwrap();
            let found_values = repo.xrange(stream_key, &entry.id, &entry.id).unwrap();
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
            let result = repo.blocking_query(block_duration, |repo| {
                let res = repo.xrange(key.clone(), &entry.id, &entry.id).unwrap();
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

            let entry_id = &entries.last().unwrap().id + 1;
            let result = repo.blocking_query(std::time::Duration::from_millis(100), |repo| {
                let res = repo.xrange(key.clone(), &entry_id, &entry_id).unwrap();
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
                let entry = &entries.last().unwrap().id + 1;
                handle = std::thread::spawn(move || {
                    repo2.blocking_query(block_duration, |repo: &StreamRepository| {
                        repo.xrange(stream_key.clone(), &entry, &entry)
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

            let new_value = "theNewValue";
            let key = repo.xadd_auto_increment(stream_key, new_value, &std::time::UNIX_EPOCH);

            dbg!(key);
            std::thread::sleep(std::time::Duration::from_millis(4));
            assert!(handle.is_finished());
            let res = handle.join().unwrap();
            let BlockResult::Found(entry) = res else {
                panic!()
            };
            assert_eq!(entry.first().unwrap().value, new_value);
        }
    });
}

#[test]
fn blocking_read_returns_not_found_if_no_data_in_range() {
    seed_tester(|repo, streams| {
        for (key, entries) in streams {
            let result = repo.read_blocking(
                key,
                &(&entries.last().unwrap().id + 1),
                10,
                std::time::Duration::from_millis(10),
            );
            assert_eq!(result, BlockResult::NotFound);
        }
    });
}

#[test]
fn blocking_read_returns_found_data_as_normal() {
    seed_tester(|repo, streams| {
        for (key, entries) in streams {
            let id = &entries.first().unwrap().id;
            let count = 2;
            let expected = repo.read(key.clone(), id, count).unwrap();
            assert!(!expected.is_empty(), "bad test. read is empty");
            let actual = repo.read_blocking(key, id, count, std::time::Duration::from_millis(10));
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
                let entry_id = &entries.last().unwrap().id + 1;
                handle = std::thread::spawn(move || {
                    repo2.read_blocking(stream_key, &entry_id, 10, block_duration)
                });
            }

            std::thread::sleep(std::time::Duration::from_millis(1));
            assert!(!handle.is_finished(), "{:?}", handle.join());

            let new_value = "theNewValue";
            repo.xadd_auto_increment(stream_key, new_value, &std::time::UNIX_EPOCH);

            std::thread::sleep(std::time::Duration::from_millis(4));
            assert!(handle.is_finished());
            let res = handle.join().unwrap();
            let BlockResult::Found(entry) = res else {
                panic!()
            };
            assert_eq!(entry.first().unwrap().value, new_value);
        }
    });
}

#[test]
fn blocking_range_returns_not_found_if_no_data_in_range() {
    seed_tester(|repo, streams| {
        for (key, entries) in streams {
            let entry_id = &entries.last().unwrap().id + 1;
            let result = repo.range_blocking(
                key,
                &entry_id,
                &entry_id,
                std::time::Duration::from_millis(10),
            );
            assert_eq!(result, BlockResult::NotFound);
        }
    });
}

#[test]
fn blocking_range_returns_found_data_as_normal() {
    seed_tester(|repo, streams| {
        for (key, entries) in streams {
            let start = &(&entries.first().unwrap().id + 1);
            let end = &(start + 1);
            let expected = repo.xrange(key.clone(), start, end).unwrap();
            assert!(!expected.is_empty(), "bad test. read is empty");
            let actual = repo.range_blocking(key, start, end, std::time::Duration::from_millis(10));
            let BlockResult::Found(actual) = actual else {
                panic!("expected BlockResult::Found got: {actual:?}");
            };
            assert_eq!(actual, expected);
        }
    });
}

#[test]
fn blocking_range_returns_data_recived_during_block() {
    seed_tester(|repo, streams| {
        for (stream_key, entries) in streams {
            let block_duration = std::time::Duration::from_millis(100);
            let repo2 = repo.clone();
            let handle;
            {
                let stream_key = stream_key.clone();
                let start = &entries.last().unwrap().id + 1;
                let end = &start + 1;
                handle = std::thread::spawn(move || {
                    repo2.range_blocking(stream_key, &start, &end, block_duration)
                });
            }

            std::thread::sleep(std::time::Duration::from_millis(1));
            assert!(!handle.is_finished(), "{:?}", handle.join());

            let new_value = "theNewValue";
            repo.xadd_auto_increment(stream_key, new_value, &std::time::UNIX_EPOCH);

            std::thread::sleep(std::time::Duration::from_millis(4));
            assert!(handle.is_finished());
            let res = handle.join().unwrap();
            let BlockResult::Found(entry) = res else {
                panic!()
            };
            assert_eq!(entry.first().unwrap().value, new_value);
        }
    });
}
