use super::*;

#[test]
fn create_stream_repo() {
    let repo = StreamRepository::new();
    assert!(repo.is_empty());
}

#[test]
fn xadd_creates_stream_if_key_does_not_exsist() {
    let repo = StreamRepository::new();
    repo.xadd("myNewStream", None, "myValue");
    assert!(!repo.is_empty());
}

#[test]
fn xread_fails_if_stream_does_not_exsist() {
    let repo = StreamRepository::new();
    assert!(repo.xread("notFound", EntryId::min(), 1).is_err());
}

#[test]
fn xread_returns_newly_added_value_if_done_on_a_new_stream() {
    let repo = StreamRepository::new();
    let stream_key = "streamKey";
    let value = "abc";
    let key = repo.xadd(stream_key, None, value).unwrap();

    let found_value = repo.xread(stream_key, key, 1).unwrap();
    assert_eq!(found_value, [value]);

    let found_value = repo.xread(stream_key, EntryId::min(), 1).unwrap();
    assert_eq!(found_value, [value]);
}

#[test]
fn xread_returns_empty_list_if_keys_not_found() {
    let repo = StreamRepository::new();
    let stream_key = "streamKey";
    let value = "abc";
    let _ = repo.xadd(stream_key, None, value).unwrap();

    let empty_list = repo.xread(stream_key, EntryId::max(), 1).unwrap();
    assert!(empty_list.is_empty());
}

#[test]
fn xread_last() {
    let repo = StreamRepository::new();
    let stream_key = "streamKey";
    let value = "abc";
    let _ = repo.xadd(stream_key, None, value).unwrap();

    let last = repo.xread_last(stream_key).unwrap();
    assert_eq!(last, value);

    let value = "nextValue";
    let _ = repo.xadd(stream_key, None, value).unwrap();

    let last = repo.xread_last(stream_key).unwrap();
    assert_eq!(last, value);
}

// normal block returns imidiately once it has any data
// read with special $ symbol blocks and only returns newly recived data since blocking
// read with special + symbol reads last value in stream

#[test]
#[should_panic(expected = "stream not found")]
fn xrange_on_empty_repo_fails() {
    let repo = StreamRepository::new();
    repo.xrange("any", EntryId::min(), EntryId::max()).unwrap();
}

#[test]
fn xrange() {
    let repo = StreamRepository::new();
    let stream_key = "streamKey";
    let value = "abc";
    let key = repo.xadd(stream_key, None, value).unwrap();

    let found_value = repo.xrange(stream_key, key.clone(), key).unwrap();
    assert_eq!(found_value, [value]);

    let found_value = repo
        .xrange(stream_key, EntryId::new(0, 1), EntryId::new(0, 1))
        .unwrap();
    assert_eq!(found_value, [value]);
}
// special - and + returns are min and max

#[test]
fn blocking_query_does_not_block_when_it_finds_data() {
    let repo = StreamRepository::new();
    let stream_key = "streamKey";
    let value = "abc";
    let _ = repo.xadd(stream_key, None, value).unwrap();

    let xrange_result = repo
        .blocking_query(
            std::time::Duration::from_secs(1),
            |repo: &StreamRepository| {
                repo.xrange(stream_key, EntryId::min(), EntryId::max())
                    .map(|v| if v.is_empty() { None } else { Some(v) })
            },
        )
        .unwrap();
    let found_value = xrange_result.unwrap();
    assert_eq!(found_value, [value]);
}

#[test]
#[ignore = "sleep"]
fn blocking_query_blocks_on_no_data_and_returns_none() {
    let repo = StreamRepository::new();
    let stream_key = "streamKey";
    let value = "abc";
    let _ = repo.xadd(stream_key, None, value).unwrap();

    let start = std::time::Instant::now();
    let block_duration = std::time::Duration::from_millis(10);
    let none = repo.blocking_query(block_duration, |repo: &StreamRepository| {
        repo.xrange(stream_key, EntryId::new(2, 0), EntryId::max())
            .map(|v| if v.is_empty() { None } else { Some(v) })
    });
    let elapsed = start.elapsed();
    assert!(elapsed >= block_duration);
    assert!(none.is_none());
}

#[test]
fn values_are_vissable_in_all_clones_of_repo() {
    let repo = StreamRepository::new();
    let clone = repo.clone();
    let stream_key = "amazignKey";
    let value = "amazingValue";
    repo.xadd(stream_key, None, value).unwrap();
    let result = clone
        .xrange(stream_key, EntryId::new(0, 1), EntryId::new(0, 1))
        .unwrap();
    assert_eq!(result, [value]);
}

#[test]
fn blocking_returns_data_recived_during_block() {
    let repo = StreamRepository::new();
    let stream_key = "streamKey";
    let value = "abc";
    let _ = repo.xadd(stream_key, None, value).unwrap();

    let block_duration = std::time::Duration::from_millis(100);
    let repo2 = repo.clone();
    let handle = std::thread::spawn(move || {
        repo2.blocking_query(block_duration, |repo: &StreamRepository| {
            repo.xrange(stream_key, EntryId::new(0, 1), EntryId::new(0, 1))
                .map(|v| if v.is_empty() { None } else { Some(v) })
        })
    });
    std::thread::sleep(std::time::Duration::from_millis(1));
    assert!(!handle.is_finished());
    let new_value = "theNewValue";
    let key = repo.xadd(stream_key, None, new_value).unwrap();
    dbg!(key);
    std::thread::sleep(std::time::Duration::from_millis(1));
    assert!(handle.is_finished());
    let res = handle.join().unwrap().unwrap().unwrap();
    assert_eq!(res, [new_value]);
}
