use actix::prelude::*;
use actix_files as fs;
use actix_web::{web, App, HttpServer, Result, HttpRequest, HttpResponse, Error, middleware};
use actix_web_actors::{ws};
use std::time::{Instant, Duration};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct MainWebSocket {
    hb: Instant
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
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl MainWebSocket {
    fn new() -> Self {
        Self { hb: Instant::now() }
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

async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    ws::start(MainWebSocket::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| App::new()
        .wrap(middleware::Logger::default())
        .service(web::resource("/ws/").route(web::get().to(ws_index)))
        .service(fs::Files::new("/", "public/dist").index_file("index.html")))
        .bind("localhost:3000")?
        .run()
        .await
}