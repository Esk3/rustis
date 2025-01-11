use super::*;

#[test]
fn usage() {
    let value = resp::Value::bulk_strings("SET; SomeKey; SomeValue");
    let mut values = Parser::new(value)
        .ident("SET")
        .unwrap()
        .value("key")
        .unwrap()
        .value("value")
        .unwrap()
        .finish();
    let key = values.remove("key").unwrap();
    let value = values.remove("value").unwrap();
    assert_eq!(key, "SomeKey");
    assert_eq!(value, "SomeValue");
}

#[test]
fn wrong_ident_is_err() {
    let value = resp::Value::bulk_strings("SET; SomeKey; SomeValue");
    let res = Parser::new(value).ident("PING");
    assert!(res.is_err());
}

#[test]
fn ident_advances() {
    let value = resp::Value::bulk_strings("SET; SomeKey; SomeValue");
    let res = Parser::new(value).ident("SET").unwrap().ident("SET");
    assert!(res.is_err());
}
