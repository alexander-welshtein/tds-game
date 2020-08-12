use std::collections::HashMap;

use actix::clock::Duration;
use actix::prelude::*;

use crate::transfer::{Transfer};
use crate::world::components::instance::Instance;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

pub struct World {
    pub(crate) sessions: HashMap<usize, Recipient<Message>>,
    pub(crate) instances: HashMap<usize, Instance>,
}

impl Default for World {
    fn default() -> Self {
        let mut instances = HashMap::new();
        instances.insert(0, Instance::default());

        Self {
            sessions: Default::default(),
            instances,
        }
    }
}

impl World {
    fn tick(&mut self, _: &mut Context<Self>) {
        for (_, instance) in &self.instances {
            for (session_id, player) in instance.get_players() {
                if let Some(addr) = self.sessions.get(&session_id) {
                    if let Ok(result) = serde_json::to_string(&Transfer {
                        player: player.clone(),
                        players: instance.get_players()
                            .iter()
                            .filter(|(player_session_id, _)| *player_session_id != session_id)
                            .map(|(_, player)| player.clone())
                            .collect(),
                    }) {
                        addr.do_send(Message(result)).ok();
                    }
                }
            }
        }
    }
}

impl Actor for World {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        IntervalFunc::new(Duration::from_millis(200), Self::tick).finish().spawn(ctx);
    }
}