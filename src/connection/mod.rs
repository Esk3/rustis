use client::{ConnectionToClientService, Kind};
use follower::{ConnectionToFollower, FollowerService};

use crate::{
    io::{Io, Output},
    message_broker::manager::WorkerManager,
    service::Service,
};

pub mod client;
pub mod follower;
pub mod leader;
pub mod services;

#[derive(Debug)]
pub struct Connection<Io, C, F, L> {
    service: ConnectionService<C, F, L>,
    io: Io,
}

impl<IO, C, F, L> Connection<IO, C, F, L>
where
    IO: Io,
    C: for<'a> Service<&'a mut IO, Response = Kind<Option<Output>>>,
{
    pub fn new_client(manager: WorkerManager, io: IO) -> Self {
        Self {
            service: ConnectionService::Client(ConnectionToClientService::new(manager)),
            io,
        }
    }

    pub fn init_new_leader(manager: WorkerManager, io: IO) -> Self {
        todo!()
    }

    pub fn next(mut self) -> anyhow::Result<()> {
        match self.service {
            ConnectionService::Client(mut client) => {
                let kind = client.call(&mut self.io)?;
                match kind {
                    client::Kind::Response(res) => {
                        //if let Some(res) = res {
                        //    self.io.write_value(res)?;
                        //}
                        Ok(())
                    }
                    client::Kind::IntoFollower => {
                        let manager = client.into_manager();
                        ConnectionService::Follower::<(), ConnectionToFollower, ()>(
                            ConnectionToFollower::new(manager),
                        );
                        todo!()
                    }
                }
            }
            ConnectionService::Follower(mut follower) => {
                let res = follower.call(()).unwrap();
                match res {
                    follower::Response::Ok => todo!(),
                    follower::Response::GetAck => {
                        //self.io.write_output(output);
                        //self.io.read_input();
                    }
                }
                todo!()
            }
            ConnectionService::Leader(_) => todo!(),
        }
    }
}

#[derive(Debug)]
enum ConnectionService<C = ConnectionToClientService, F = ConnectionToFollower, L = ()> {
    Client(C),
    Follower(F),
    Leader(L),
}

// conn
//      io  value->Input Input      input->Response input->Response manager
// client -> req -> into follower -> queue -> handler
//                                  resposne
//   io           subscriber              manager
// follower -> get event from manager -> handler
// leader -> req -> handler
//                  response
