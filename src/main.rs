mod world;
mod transfer;
mod socket;

use actix_files as fs;
use actix_web::{web, App, HttpServer, Result, HttpRequest, HttpResponse, Error, middleware};
use actix_web_actors::{ws};
use crate::world::World;
use actix::{Actor, Addr};

async fn ws_index(req: HttpRequest, stream: web::Payload, srv: web::Data<Addr<World>>) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    ws::start(socket::MainWebSocket::new(srv.get_ref().clone()), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| App::new()
        .wrap(middleware::Logger::default())
        .data(World::default().start())
        .service(web::resource("/ws/").route(web::get().to(ws_index)))
        .service(fs::Files::new("/", "public/dist").index_file("index.html")))
        .bind("localhost:3000")?
        .run()
        .await
}