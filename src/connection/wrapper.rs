use crate::node_service::{ClientService, FollowerService, LeaderService};

use super::{client::Client, follower::Follower, leader::Leader};

pub enum ConnectionKind<C, F, L> {
    Client(Client<C>),
    Follower(Follower<F>),
    Leader(Leader<L>),
}

impl<C, F, L> ConnectionKind<C, F, L>
where
    C: ClientService<F = F>,
    F: FollowerService,
    L: LeaderService,
{
    #[must_use]
    pub fn into_follower(self) -> Self {
        match self {
            ConnectionKind::Client(client) => Self::Follower(client.into_follower()),
            ConnectionKind::Follower(_) => self,
            ConnectionKind::Leader(_) => todo!(),
        }
    }
}
