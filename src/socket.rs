use std::time::{Instant, Duration};
use actix::{Actor, StreamHandler, AsyncContext, ActorContext, Addr, Handler};
use actix_web_actors::ws;
use crate::{world, transfer};
use rand::prelude::ThreadRng;
use rand::Rng;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct MainWebSocket {
    session_id: usize,
    hb: Instant,
    world: Addr<world::World>,
    rng: ThreadRng,
}

impl Actor for MainWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        self.session_id = self.rng.gen::<usize>();

        self.world.do_send(world::Connect {
            session_id: self.session_id,
            addr: ctx.address().recipient(),
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MainWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("message: {:?}", msg);

        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let operation: transfer::Operation = match serde_json::from_str(&*text) {
                    Ok(operation) => operation,
                    Err(_error) => transfer::Operation::default()
                };

                match operation.command.trim() {
                    "JoinInstance" => self.world.do_send(world::JoinInstance {
                        session_id: self.session_id,
                        instance_id: 0,
                    }),
                    "MoveLeft" | "MoveRight" | "MoveUp" | "MoveDown" => self.world.do_send(world::UpdatePlayer {
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

impl Handler<world::Message> for MainWebSocket {
    type Result = ();

    fn handle(&mut self, msg: world::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl MainWebSocket {
    pub fn new(world: Addr<world::World>) -> Self {
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
                println!("Websocket Client heartbeat failed, disconnecting!");

                act.world.do_send(world::Disconnect {
                    id: act.session_id
                });

                ctx.stop();

                return;
            }
            ctx.ping(b"");
        });
    }
}