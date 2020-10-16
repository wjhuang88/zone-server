mod controller;
mod repository;

use actix_web::{App, HttpServer};
use controller::{hello, posts, post};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(posts)
            .service(post)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
