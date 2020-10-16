mod controller;
mod repository;

use actix_web::{App, HttpServer};
use controller::{hello, posts, post};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(posts)
            .service(post)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
