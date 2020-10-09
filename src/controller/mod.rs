use actix_web::{get, HttpResponse, Responder, Result, error, web};
use crate::repository::RepositoryManager;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/posts")]
pub async fn posts() -> Result<HttpResponse> {
    let manager = RepositoryManager::new();
    let result = manager.get_post_list().await.unwrap();
    let resp = HttpResponse::Ok().json(result);
    Ok(resp)
}

#[get("/posts/{id}")]
pub async fn post(web::Path((id,)): web::Path<(i64,)>) -> Result<String> {
    let manager = RepositoryManager::new();
    let result = manager.get_post_instance(id).await;
    if let Ok(post) = result {
        Ok(post.to_markdown())
    } else {
        Err(error::ErrorNotFound("Post not found"))
    }
}