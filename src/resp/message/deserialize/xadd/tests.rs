use super::*;

#[test]
fn de() {
    let input = [
        Value::BulkString("XADD".into()),
        Value::BulkString("TheStreamName".into()),
        Value::BulkString("TheValue".into()),
    ];
    let res = try_deserialize(input.to_vec());
    let TryDeserializeResult::Ok(msg) = res else {
        panic!()
    };
    let Input::XAdd {
        stream_key,
        entry_id,
        value,
    } = msg.expect_input().unwrap()
    else {
        panic!();
    };
    assert_eq!(stream_key, "TheStreamName");
    assert_eq!(value, "TheValue");
}
