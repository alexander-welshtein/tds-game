use actix_files as fs;
use actix_web::{get, App, HttpServer, Result};

#[get("/")]
async fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("public/dist/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index)
        .service(fs::Files::new("/", "public/dist").show_files_listing()))
        .bind("localhost:3000")?
        .run()
        .await
}