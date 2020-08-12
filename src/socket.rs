use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, StreamHandler, Running};
use actix_web_actors::ws;
use rand::prelude::ThreadRng;
use rand::Rng;

use crate::world::world::{World, Message};
use crate::transfer::Operation;
use crate::world::messages::connect::Connect;
use crate::world::messages::join::JoinInstance;
use crate::world::messages::update::UpdatePlayer;
use crate::world::messages::disconnect::Disconnect;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct MainWebSocket {
    session_id: usize,
    hb: Instant,
    world: Addr<World>,
    rng: ThreadRng,
}

impl Actor for MainWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        self.session_id = self.rng.gen::<usize>();

        self.world.do_send(Connect {
            session_id: self.session_id,
            addr: ctx.address().recipient(),
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.world.do_send(Disconnect {
            session_id: self.session_id
        });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MainWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let operation: Operation = match serde_json::from_str(&*text) {
                    Ok(operation) => operation,
                    Err(_error) => Operation::default()
                };

                match operation.command.trim() {
                    "JoinInstance" => self.world.do_send(JoinInstance {
                        session_id: self.session_id,
                        instance_id: 0,
                    }),
                    "MoveLeft" | "MoveRight" | "MoveUp" | "MoveDown" => self.world.do_send(UpdatePlayer {
                        session_id: self.session_id,
                        instance_id: 0,
                        operation,
                    }),
                    _ => ()
                };
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl Handler<Message> for MainWebSocket {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl MainWebSocket {
    pub fn new(world: Addr<World>) -> Self {
        Self {
            session_id: 0,
            hb: Instant::now(),
            world,
            rng: Default::default(),
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.world.do_send(Disconnect {
                    session_id: act.session_id
                });

                ctx.stop();

                return;
            }
            ctx.ping(b"");
        });
    }
}