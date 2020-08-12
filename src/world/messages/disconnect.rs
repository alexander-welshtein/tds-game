use actix::{Context, Handler};
use actix::prelude::*;

use crate::world::world::World;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub session_id: usize
}

impl Handler<Disconnect> for World {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.session_id);

        for (_, instance) in &mut self.instances {
            instance.remove_player(msg.session_id);
        }

        println!("Disconnect: session_id = {:?}", msg.session_id);
    }
}

