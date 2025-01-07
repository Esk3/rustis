use super::*;

#[test]
fn get() {
    let assert_ok_key = |a, b| {
        assert_eq!(
            try_deserialize(a),
            TryDeserializeResult::Ok(Input::Get(b).into())
        );
    };
    let expect_ok = [
        (["abc"].to_vec(), "abc"),
        (["my key"].to_vec(), "my key"),
        (["someKey", "should be ignored"].to_vec(), "someKey"),
    ]
    .map(|(list, key)| {
        (
            [Value::SimpleString("GET".into())]
                .into_iter()
                .chain(
                    list.into_iter()
                        .map(std::convert::Into::into)
                        .map(Value::BulkString),
                )
                .collect::<Vec<Value>>(),
            key.to_string(),
        )
    });

    for (value, expected_key) in expect_ok {
        assert_ok_key(value, expected_key);
    }
}
