use queue::{Queue, StoreResult};

use super::*;

#[test]
fn new_queue_is_not_active() {
    let queue = Queue::new();
    assert!(!queue.is_active());
}

#[test]
fn store_item_in_inactive_queue_fails() {
    let mut queue = Queue::new();
    let item = client::Request::epoch(Input::Ping, 0);
    let invalid = queue.store(item.clone());
    assert_eq!(invalid, StoreResult::InvalidStore(item));
}

fn store_multi() -> Queue {
    let mut queue = Queue::new();
    let ok = queue.store(client::Request::epoch(Input::Multi, 0));
    assert_eq!(ok, StoreResult::Ok);
    queue
}

#[test]
fn store_multi_in_inactive_queue_is_ok() {
    store_multi();
}

#[test]
fn queue_is_active_after_storing_multi() {
    let queue = store_multi();
    assert!(queue.is_active());
}

#[test]
fn store_item_in_active_queue_is_ok() {
    let mut queue = store_multi();
    let ok = queue.store(client::Request::epoch(Input::Ping, 0));
    assert_eq!(ok, StoreResult::Ok);
}

#[test]
fn store_commit_multi_on_active_queue_returns_items_and_sets_inactive() {
    let mut queue = store_multi();
    let items = [client::Request::epoch(Input::Ping, 0)];
    for item in items.clone() {
        let ok = queue.store(item);
        assert_eq!(ok, StoreResult::Ok);
    }
    let list = queue.store(client::Request::epoch(Input::CommitMulti, 0));
    assert_eq!(list, StoreResult::QueueFinished(items.to_vec()));
    assert!(!queue.is_active());
}

#[test]
fn store_multi_in_active_queue_is_err() {
    let mut queue = store_multi();
    let input = client::Request::epoch(Input::Multi, 0);
    let err = queue.store(input.clone());
    assert_eq!(err, StoreResult::InvalidStore(input));
}
