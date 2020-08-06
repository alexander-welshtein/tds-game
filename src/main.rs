mod transfer;
mod main_web_socket;

use actix_files as fs;
use actix_web::{web, App, HttpServer, Result, HttpRequest, HttpResponse, Error, middleware};
use actix_web_actors::{ws};

async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    ws::start(main_web_socket::MainWebSocket::new(), &req, stream)
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