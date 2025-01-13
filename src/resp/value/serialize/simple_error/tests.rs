use crate::resp::{value::serialize_value, Value};

use super::*;

#[test]
fn serialzie_simple_error_test() {
    let value = "myError";
    assert_eq!(
        serialize_value(&Value::SimpleError(value.into()),),
        serialize_simple_error(value)
    )
}
