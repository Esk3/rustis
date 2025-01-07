use super::*;

fn set_input() -> Vec<(Value, [String; 2])> {
    [
        (
            [
                Value::BulkString("SET".into()),
                Value::BulkString("abc".into()),
                Value::BulkString("xyz123".into()),
            ]
            .to_vec(),
            ["abc", "xyz123"],
        ),
        (
            [
                Value::BulkString("SET".into()),
                Value::BulkString("AKey".into()),
                Value::BulkString("aValue".into()),
            ]
            .to_vec(),
            ["AKey", "aValue"],
        ),
    ]
    .into_iter()
    .map(|(v, e)| (Value::Array(v), e))
    .map(|(v, e)| (v, e.map(std::string::ToString::to_string)))
    .collect()
}

#[test]
fn try_deserialize_set_test() {
    let inputs = set_input();
    for (input, [expected_key, expected_value]) in inputs {
        let expected = Input::Set {
            key: expected_key,
            value: expected_value,
            expiry: None,
            get: false,
        };
        assert_eq!(
            try_deserialize(input.into_array().unwrap()),
            TryDeserializeResult::Ok(expected.into())
        );
    }
}

#[test]
fn deserialize_set() {
    let (key, value) = ("abc", "xyz123");
    let message = super::super::tests::f([
        Value::BulkString("SET".into()),
        Value::BulkString(key.into()),
        Value::BulkString(value.into()),
    ])
    .unwrap();
    assert_eq!(
        message,
        Input::Set {
            key: key.into(),
            value: value.into(),
            expiry: None,
            get: false
        }
        .into()
    );
}
