use actix::{Context, Handler};
use actix::prelude::*;

use crate::world::world::World;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize
}

impl Handler<Disconnect> for World {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
    }
}

