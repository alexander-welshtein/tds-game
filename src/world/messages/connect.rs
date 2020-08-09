use actix::{Context, Handler, Recipient};
use actix::prelude::*;

use crate::world::world::{Message, World};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub session_id: usize,
    pub addr: Recipient<Message>,
}

impl Handler<Connect> for World {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        println!("Connected: id = {:?}", msg.session_id);
        self.sessions.insert(msg.session_id, msg.addr);
    }
}

