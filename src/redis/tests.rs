use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use crate::{
    config::Role,
    connection::{self, Connection, ConnectionResult},
    listner::RedisListner,
    resp::Message,
};
use builder::RedisBuilder;

use super::*;
type DummyRedis = Redis<DummyListner, DummyConnection>;

fn setup_leader() -> DummyRedis {
    RedisBuilder::new()
        .listner(DummyListner)
        .repo(Repository::new())
        .emitter(EventEmitter::new())
        .build()
        .unwrap()
}

fn setup_follower() -> DummyRedis {
    RedisBuilder::new()
        .listner(DummyListner)
        .leader_connection(DummyConnection)
        .repo(Repository::new())
        .emitter(EventEmitter::new())
        .build()
        .unwrap()
}

#[test]
fn create_redis_server() {
    let redis_server = RedisBuilder::<_, DummyConnection>::new()
        .listner(DummyListner)
        .repo(Repository::new())
        .emitter(EventEmitter::new())
        .build()
        .unwrap();
}

#[test]
fn get_port() {
    let redis = setup_leader();
    let _: u16 = redis.get_port();
}

#[test]
fn get_default_port() {
    let port = setup_leader().get_port();
    assert_eq!(port, 6379);
}

#[test]
#[ignore = "todo"]
fn get_port_is_same_as_set() {
    todo!()
    //let expected_port = 6380;
    //let port = DummyRedis::bind_from_config(RedisConfig::new(expected_port))
    //    .unwrap()
    //    .get_port();
    //assert_eq!(port, expected_port);
}

#[test]
fn role_is_leader_when_leader_leader_connection_is_none() {
    let expected = Role::Leader;
    let role = setup_leader().role();
    assert_eq!(role, expected);
}

#[test]
fn role_is_follower_when_leader_connection_is_some() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0));
    let expected = Role::Follower(addr);
    let role = setup_follower().role();
    assert_eq!(role, expected);
}

#[test]
fn get_listner() {
    let redis = setup_leader();
    let _ = redis.listner;
}

#[test]
#[should_panic(expected = "incoming called on dummy listner")]
fn calls_listers_incoming_on_run() {
    let redis = setup_leader();
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
    let redis = RedisBuilder::<MockOnceListner, DummyConnection>::new()
        .listner(MockOnceListner)
        .repo(Repository::new())
        .emitter(EventEmitter::new())
        .build()
        .unwrap();
    redis.run();
}

#[test]
#[should_panic(expected = "is not follower")]
fn creating_outgoing_connection_as_leader_panics() {
    let mut redis = setup_leader();
    redis.connect_to_leader();
}

#[test]
fn create_outgoing_connection_as_follower_is_ok() {
    let mut redis = setup_follower();
    let _connection_to_leader = redis.connect_to_leader().unwrap();
}

#[test]
#[should_panic(expected = "incoming called on dummy listner")]
fn creates_outgoing_connection_on_run_as_follower() {
    let redis = setup_follower();
    redis.run();
}

#[test]
#[ignore = "todo"]
fn still_listens_for_incoming_connection_after_connection_to_leader() {
    todo!()
}

#[test]
#[ignore = "todo"]
fn sends_handshake_to_leader_when_follower() {
    todo!()
}

#[test]
#[ignore = "todo"]
fn can_accept_multiple_incoming_connections() {
    todo!()
}

#[derive(Debug)]
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

    fn get_port(&self) -> u16 {
        6379
    }
}

#[derive(Debug)]
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

    fn get_port(&self) -> u16 {
        0
    }
}

struct DummyConnection;
impl Connection for DummyConnection {
    fn connect(addr: std::net::SocketAddr) -> ConnectionResult<Self>
    where
        Self: Sized,
    {
        Ok(Self)
    }

    fn read_message(&mut self) -> ConnectionResult<connection::Message> {
        todo!()
    }

    fn write_message(&mut self, command: Message) -> ConnectionResult<usize> {
        todo!()
    }

    fn get_peer_addr(&self) -> std::net::SocketAddr {
        std::net::SocketAddr::new(std::net::Ipv4Addr::LOCALHOST.into(), 0)
    }
}
