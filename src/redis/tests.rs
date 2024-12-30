use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use rustis::{
    config::{RedisConfig, Role},
    connection::{Connection, ConnectionMessage, ConnectionResult},
    listner::RedisListner,
};

use super::*;
type DummyRedis = Redis<DummyListner>;

fn setup_follower() -> Redis<DummyListner> {
    DummyRedis::bind_from_config(RedisConfig::new_follower(
        1234,
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 4321)),
    ))
    .unwrap()
}

#[test]
fn create_redis_server() {
    let _ = DummyRedis::bind().unwrap();
}

#[test]
fn create_with_port() {
    let config = RedisConfig::new(6780);
    let _ = DummyRedis::bind_from_config(config);
}

#[test]
fn get_port() {
    let redis = DummyRedis::bind().unwrap();
    let _: u16 = redis.get_port();
}

#[test]
fn get_default_port() {
    let port = DummyRedis::bind().unwrap().get_port();
    assert_eq!(port, 6379);
}

#[test]
fn get_port_is_same_as_set() {
    let expected_port = 6380;
    let port = DummyRedis::bind_from_config(RedisConfig::new(expected_port))
        .unwrap()
        .get_port();
    assert_eq!(port, expected_port);
}

#[test]
fn role_is_leader_when_leader_addr_is_not_set() {
    let expected = Role::Leader;
    let role = DummyRedis::bind().unwrap().role();
    assert_eq!(role, expected);
}

#[test]
fn role_is_follower_when_leader_addr_is_set() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 1234));
    let expected = Role::Follower(addr);
    let role = DummyRedis::bind_from_config(RedisConfig::new_follower(1234, addr))
        .unwrap()
        .role();
    assert_eq!(role, expected);
}

#[test]
fn is_leader() {
    let redis = DummyRedis::bind().unwrap();
    assert!(redis.is_leader());
}

#[test]
fn is_follower() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 1234));
    let config = RedisConfig::new_follower(2134, addr);
    let redis = DummyRedis::bind_from_config(config).unwrap();
    assert!(redis.is_follower());
}

#[test]
fn get_listner() {
    let redis = DummyRedis::bind().unwrap();
    let _ = redis.listner;
}

#[test]
#[should_panic(expected = "incoming called on dummy listner")]
fn calls_listers_incoming_on_run() {
    let redis = DummyRedis::bind().unwrap();
    redis.run();
}

#[test]
#[ignore = "reason"]
fn listner_is_bound_to_right_port() {
    todo!()
}

#[test]
#[should_panic(expected = "called accept connection")]
#[ignore = "todo"]
fn creates_incoming_connection_on_listner_output() {
    let redis = Redis::<MockOnceListner>::bind().unwrap();
    redis.run();
}

#[test]
#[should_panic(expected = "is not follower")]
fn creating_outgoing_connection_as_leader_panics() {
    let mut redis = DummyRedis::bind().unwrap();
    redis.connect_to_leader();
}

#[test]
fn create_outgoing_connection_as_follower_is_ok() {
    let mut redis = setup_follower();
    redis.connect_to_leader();
}

#[test]
#[should_panic(expected = "incoming called on dummy listner")]
fn creates_outgoing_connection_on_run_as_follower() {
    let redis = DummyRedis::bind_from_config(RedisConfig::new_follower(
        1234,
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 4321)),
    ))
    .unwrap();
    redis.run();
}

struct DummyListner;
impl RedisListner for DummyListner {
    type Connection = DummyConnection;
    fn bind(_port: u16) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self)
    }

    fn incoming(self) -> impl Iterator<Item = Self::Connection> {
        panic!("incoming called on dummy listner");
        std::iter::once(DummyConnection)
    }
}

struct MockOnceListner;
impl RedisListner for MockOnceListner {
    type Connection = DummyConnection;

    fn bind(_port: u16) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self)
    }

    fn incoming(self) -> impl Iterator<Item = Self::Connection> {
        std::iter::once(DummyConnection)
    }
}

struct DummyConnection;
impl Connection for DummyConnection {
    fn read_resp(&mut self, buf: &mut [u8]) -> ConnectionResult<usize> {
        todo!()
    }

    fn write_resp(&mut self, buf: &[u8]) -> ConnectionResult<()> {
        todo!()
    }

    fn from_connection<C>(value: C) -> Self {
        todo!()
    }

    fn read_command(&mut self) -> ConnectionResult<ConnectionMessage> {
        todo!()
    }

    fn write_command(&mut self, command: ConnectionMessage) -> ConnectionResult<usize> {
        todo!()
    }
}
impl std::io::Read for DummyConnection {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}
impl std::io::Write for DummyConnection {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
