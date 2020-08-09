use std::collections::HashMap;
use actix::prelude::*;
use crate::transfer::{Instance, Player, Transfer, Operation};
use actix::clock::Duration;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

pub struct World {
    sessions: HashMap<usize, Recipient<Message>>,
    instances: HashMap<usize, Instance>,
}

impl Default for World {
    fn default() -> Self {
        let mut instances = HashMap::new();
        instances.insert(0, Instance::new());

        Self {
            sessions: Default::default(),
            instances,
        }
    }
}

impl World {
    fn tick(&mut self, _: &mut Context<Self>) {
        for (_, instance) in &self.instances {
            for (session_id, player) in &instance.players {
                if let Some(addr) = self.sessions.get(&session_id) {
                    if let Ok(result) = serde_json::to_string(&Transfer {
                        player: Player {
                            x: player.x,
                            y: player.y,
                            speed: player.speed,
                            operation: Operation {
                                id: player.operation.id,
                                command: player.operation.command.clone()
                            }
                        },
                    }) {
                        addr.do_send(Message(result));
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
            println!("Session {:?} join to instance {:?}", session_id, instance_id);
        } else {
            println!("Unsuccessful attempt to join an instance {:?}", instance_id);
        }
    }
}

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
            if let Some(player) = instance.players.get_mut(&session_id) {
                match operation.command.as_str() {
                    "MoveLeft" => player.x -= player.speed,
                    "MoveRight" => player.x += player.speed,
                    "MoveUp" => player.y -= player.speed,
                    "MoveDown" => player.y += player.speed,
                    _ => {}
                };

                player.operation = operation;
            }
        };
    }
}