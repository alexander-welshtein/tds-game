use actix::{Context, Handler};
use actix::prelude::*;

use crate::transfer::Operation;
use crate::world::world::World;

#[derive(Message)]
#[rtype(result = "()")]
pub struct UpdatePlayer {
    pub session_id: usize,
    pub instance_id: usize,
    pub operation: Operation,
}

impl Handler<UpdatePlayer> for World {
    type Result = ();

    fn handle(&mut self, msg: UpdatePlayer, _: &mut Context<Self>) {
        let UpdatePlayer { session_id, instance_id, operation } = msg;

        if let Some(instance) = self.instances.get_mut(&instance_id) {
            if let Some(player) = instance.get_player(session_id) {
                player.apply_operation(operation)
            }
        };
    }
}