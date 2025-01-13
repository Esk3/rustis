use resp::value::{serialize::Serialize, IntoRespArray};

use super::*;

fn empty_stream() -> std::io::Cursor<Vec<u8>> {
    std::io::Cursor::new(Vec::new())
}

fn stream<I>(values: I) -> std::io::Cursor<Vec<u8>>
where
    I: IntoIterator<Item = resp::Value>,
{
    std::io::Cursor::new(values.into_iter().flat_map(|v| v.serialize()).collect())
}

fn stream_single(value: resp::Value) -> impl Stream + Debug {
    stream([value])
}

#[test]
fn connection_reads_single_value_from_stream() {
    fn test(expected_value: resp::Value) {
        let s = stream_single(expected_value.clone());
        let mut connection = RedisConnection::new(s);
        let value = connection.read().unwrap().value;
        assert_eq!(value, expected_value);
    }

    for value in [
        resp::Value::simple_string("hello_world"),
        resp::Value::simple_string("abcd"),
        resp::Value::bulk_string("My cool bulkstring"),
        resp::Value::bulk_strings("My cool; array; of; bulk strings ").into_array(),
    ] {
        test(value);
    }
}
fn test_values() -> [Vec<resp::Value>; 4] {
    [
        [
            resp::Value::simple_string("hello_world"),
            resp::Value::simple_string("nest_str"),
        ]
        .to_vec(),
        [
            resp::Value::simple_string("abcd"),
            resp::value::Value::bulk_string("1234"),
        ]
        .to_vec(),
        [
            resp::Value::bulk_string("My cool bulkstring"),
            resp::Value::simple_string("aaaaa"),
        ]
        .to_vec(),
        resp::Value::bulk_strings("My cool; array; of; bulk strings "),
    ]
}

#[test]
fn connection_reads_multiple_values_from_stream() {
    fn test(expected_values: Vec<resp::Value>) {
        let s = stream(expected_values.clone());
        let mut connection = RedisConnection::new(s);
        let values = (0..expected_values.len())
            .map(|_| connection.read().unwrap().value)
            .collect::<Vec<resp::Value>>();
        assert_eq!(values, expected_values);
    }

    for (i, values) in test_values().into_iter().enumerate() {
        dbg!(i);
        test(values);
        dbg!(i, "ok");
    }
}

#[test]
fn read_all_returns_all_values() {
    fn test(values: Vec<resp::Value>) {
        let s = stream(values.clone());
        let mut connection = RedisConnection::new(s);
        let actual = connection
            .read_all()
            .unwrap()
            .into_iter()
            .map(|r| r.value)
            .collect::<Vec<_>>();
        assert_eq!(actual, values);
    }

    for (i, values) in test_values().into_iter().enumerate() {
        dbg!(i);
        test(values);
        dbg!(i, "ok");
    }
}

#[test]
fn written_value_can_be_read_from_stream() {
    fn test(value: resp::Value) {
        let s = empty_stream();
        let mut conn = RedisConnection::new(s);
        conn.write(&value).unwrap();
        conn.inner().set_position(0);
        let actual = conn.read().unwrap().value;
        assert_eq!(actual, value);
    }
    for value in test_values() {
        test(value[0].clone());
    }
}

#[test]
fn write_returns_bytes_written_to_stream() {
    fn test(value: resp::Value) {
        let s = empty_stream();
        let mut conn = RedisConnection::new(s);
        let bytes_written = conn.write(&value).unwrap();
        let expected = conn.inner().position();
        assert_eq!(bytes_written, expected.try_into().unwrap());
    }
    for value in test_values() {
        test(value[0].clone());
    }
}

#[test]
fn write_all_returns_bytes_wirtten_to_stream() {
    fn test(values: Vec<resp::Value>) {
        let s = empty_stream();
        let mut conn = RedisConnection::new(s);
        let bytes_written = conn.write_all(&values).unwrap();
        let expected = conn.inner().position();
        assert_eq!(bytes_written, expected.try_into().unwrap());
    }
    for value in test_values() {
        test(value);
    }
}

#[test]
fn values_written_are_avalible_for_read_all() {
    fn test(values: Vec<resp::Value>) {
        let s = empty_stream();
        let mut conn = RedisConnection::new(s);
        _ = conn.write_all(&values).unwrap();
        conn.inner().set_position(0);
        let read = conn
            .read_all()
            .unwrap()
            .into_iter()
            .map(|r| r.value)
            .collect::<Vec<_>>();
        assert_eq!(read, values);
    }
    for value in test_values() {
        test(value);
    }
}

#[test]
fn pipeline_buffer_reads_single_value() {
    fn test(expected_value: resp::Value) {
        let s = stream_single(expected_value.clone());
        let mut connection = PipelineBuffer::new(s);
        let value = connection.read().unwrap().value;
        assert_eq!(value, expected_value);
    }

    for value in [
        resp::Value::simple_string("hello_world"),
        resp::Value::simple_string("abcd"),
        resp::Value::bulk_string("My cool bulkstring"),
        resp::Value::bulk_strings("My cool; array; of; bulk strings ").into_array(),
    ] {
        test(value);
    }
}

#[test]
fn pipeline_buffer_writes_single_value() {
    fn test(expected_value: resp::Value) {
        let s = empty_stream();
        let mut connection = PipelineBuffer::new(s);
        connection.write(&expected_value).unwrap();
        connection.connection.stream.set_position(0);
        let value = connection.read().unwrap().value;
        assert_eq!(value, expected_value);
    }

    for value in [
        resp::Value::simple_string("hello_world"),
        resp::Value::simple_string("abcd"),
        resp::Value::bulk_string("My cool bulkstring"),
        resp::Value::bulk_strings("My cool; array; of; bulk strings ").into_array(),
    ] {
        test(value);
    }
}

#[test]
fn pipeline_buffer_write_returns_bytes_of_value_serialized() {
    fn test(value: resp::Value) {
        let s = empty_stream();
        let mut conn = PipelineBuffer::new(s);
        let bytes_written = conn.write(&value).unwrap();
        let expected = serialize_value(&value).len();
        assert_eq!(bytes_written, expected);
    }
    for value in test_values() {
        test(value[0].clone());
    }
}

#[test]
fn pipeline_buffer_buffers_single_write_when_reading_more_than_one_value() {
    fn test(input_values: Vec<resp::Value>) {
        let len = input_values.len();
        let s = stream(input_values.clone());
        let expected_position = s.get_ref().len();
        let mut connection = PipelineBuffer::new(s);
        connection.read().unwrap();
        connection
            .write(&resp::Value::simple_string("dummy"))
            .unwrap();
        if len > 1 {
            assert_eq!(
                connection.connection.stream.position(),
                expected_position.try_into().unwrap(),
                "{input_values:?}"
            );
        }
    }

    for value in test_values() {
        test(value);
    }
}

#[test]
fn pipeline_buffer_writes_to_stream_on_first_write_after_read_buffer_is_empty() {
    fn test(input_values: Vec<resp::Value>) {
        let len = input_values.len();
        let s = stream(input_values.clone());
        let mut connection = PipelineBuffer::new(s);
        let mut expected_position = 0;
        for i in 0..(len - 1) {
            connection.read().unwrap();
            if i == 0 {
                expected_position = connection.connection.stream.position();
            }
            assert_eq!(connection.connection.stream.position(), expected_position);
            connection
                .write(&resp::Value::simple_string("dummy"))
                .unwrap();
            assert_eq!(connection.connection.stream.position(), expected_position);
        }
        connection.read().unwrap();
        assert_eq!(connection.connection.stream.position(), expected_position);
        connection
            .write(&resp::Value::simple_string("dummy"))
            .unwrap();
        let mut s = connection.connection.stream;
        assert_ne!(s.position(), expected_position);
        s.set_position(expected_position);
        let mut conn = RedisConnection::new(s);
        let res = conn
            .read_all()
            .unwrap()
            .into_iter()
            .map(|v| v.value)
            .collect::<Vec<_>>();
        assert_eq!(
            res,
            std::iter::repeat(resp::Value::simple_string("dummy"))
                .take(len)
                .collect::<Vec<resp::Value>>()
        );
    }

    for value in test_values() {
        test(value);
    }
}

impl Stream for std::io::Cursor<Vec<u8>> {
    type Addr = ();

    fn connect((): Self::Addr) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        unimplemented!("called connect on dummy stream")
    }

    fn peer_addr(&self) -> Self::Addr {
        unimplemented!("called peer_addr on dummy stream")
    }
}

impl Stream for &mut std::io::Cursor<Vec<u8>> {
    type Addr = ();

    fn connect((): Self::Addr) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        unimplemented!("called connect on dummy stream")
    }

    fn peer_addr(&self) -> Self::Addr {
        unimplemented!("called peer_addr on dummy stream")
    }
}
