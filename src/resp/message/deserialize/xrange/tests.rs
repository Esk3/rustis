use crate::repository::stream_repo::stream::EntryId;

use super::*;

#[test]
fn deserialize_test() {
    let result = try_deserialize(
        [
            Value::BulkString("XRANGE".into()),
            Value::BulkString("streamName".into()),
            Value::BulkString("0".into()),
            Value::BulkString("2".into()),
        ]
        .to_vec(),
    );
    match result {
        TryDeserializeResult::Ok(message) => {
            let Input::XRange {
                stream_key,
                start,
                end,
            } = message.expect_input().unwrap()
            else {
                panic!();
            };
            assert_eq!(stream_key, "streamName");
            assert_eq!(start, EntryId::new(0, 0));
            assert_eq!(end, EntryId::new(2, 0));
        }
        TryDeserializeResult::Err(_) => todo!(),
        TryDeserializeResult::Ignore(_) => todo!(),
    }
}
