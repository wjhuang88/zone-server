mod entity;

use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;
use std::sync::Arc;

static mut POOL: Option<Arc<MySqlPool>> = None;

async fn get_pool() -> Result<Arc<MySqlPool>, sqlx::Error> {
    unsafe {
        if let Some(pool) = &POOL {
            Ok(pool.clone())
        } else {
            dotenv().ok();
            let url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
            let new_pool = MySqlPool::connect(&url).await?;
            let boxed = Arc::new(new_pool);
            POOL.replace(boxed.clone());
            Ok(boxed)
        }
    }
}

pub struct RepositoryManager;

impl RepositoryManager {
    pub fn new() -> RepositoryManager {
        RepositoryManager
    }

    pub async fn get_post_list(&self) -> Result<Vec<entity::PostListItem>, sqlx::Error> {
        let pool = get_pool().await?;
        let posts = sqlx::query_as!(entity::PostListItem, "select id, title, read_count, create_time, update_time, bref, tag_id, like_count, commet_count from t_o_post")
            .fetch_all(&*pool)
            .await?;
        Ok(posts)
    }

    pub async fn get_post_instance(&self, id: i64) -> Result<entity::Post, sqlx::Error> {
        let pool = get_pool().await?;
        let post = sqlx::query_as!(entity::Post, "select * from t_o_post where id=?", id)
            .fetch_one(&*pool)
            .await?;
        Ok(post)
    }
}
