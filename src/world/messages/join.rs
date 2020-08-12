use actix::{Context, Handler};
use actix::prelude::*;

use crate::world::components::player::Player;
use crate::world::world::World;

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinInstance {
    pub session_id: usize,
    pub instance_id: usize,
}

impl Handler<JoinInstance> for World {
    type Result = ();

    fn handle(&mut self, msg: JoinInstance, _: &mut Context<Self>) {
        let JoinInstance { session_id, instance_id } = msg;

        if let Some(instance) = self.instances.get_mut(&instance_id) {
            instance.players.insert(session_id, Player::default());
            println!("JoinInstance: session_id = {:?} instance_id = {:?}", session_id, instance_id);
        } else {
            println!("[Failed] JoinInstance: instance_id = {:?}", instance_id);
        }
    }
}

