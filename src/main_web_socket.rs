use std::time::{Instant, Duration};
use actix::{Actor, StreamHandler, AsyncContext, ActorContext};
use actix_web_actors::ws;
use crate::client_state::{ClientState, Operation};
use crate::manager::change_client_state;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct MainWebSocket {
    hb: Instant,
    state: ClientState,
}

impl Actor for MainWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
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
                let operation: Operation = match serde_json::from_str(&*text) {
                  Ok(operation) => operation,
                    Err(_error) => Operation::new()
                };

                change_client_state(operation, &mut self.state);

                let state = match serde_json::to_string(&self.state) {
                    Ok(result) => result,
                    Err(_error) => String::from("{}")
                };

                ctx.text(state);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl MainWebSocket {
    pub fn new() -> Self {
        Self { hb: Instant::now(), state: ClientState::new() }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}