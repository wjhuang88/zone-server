use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub title: Option<String>,
    pub content: Option<String>,
    pub read_count: Option<i64>,
    pub create_time: Option<i64>,
    pub update_time: Option<i64>,
    pub bref: Option<String>,
    pub tag_id: Option<i64>,
    pub like_count: Option<i64>,
    pub commet_count: Option<i64>,
    pub valid: Option<i8>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct PostListItem {
    pub id: i64,
    pub title: Option<String>,
    pub read_count: Option<i64>,
    pub create_time: Option<i64>,
    pub update_time: Option<i64>,
    pub bref: Option<String>,
    pub tag_id: Option<i64>,
    pub like_count: Option<i64>,
    pub commet_count: Option<i64>,
}

impl Post {
    pub fn to_markdown(&self) -> String {
        let title_str = self.title.as_ref().unwrap();
        let date_str = Utc.timestamp_millis(self.create_time.unwrap());
        let mut result = String::new();
        result += "---\n";
        result += &format!("{}: {}\n", "title: ", title_str);
        result += &format!("{}: {}\n", "date: ", date_str);
        result += &format!("{}: {}\n", "bref: ", self.bref.as_ref().unwrap_or(&String::new()));
        result += &format!("{}: {}\n", "like_count: ", self.like_count.unwrap_or(0));
        result += &format!("{}: {}\n", "commet_count: ", self.commet_count.unwrap_or(0));
        result += &format!("{}: {}\n", "read_count: ", self.read_count.unwrap_or(0));
        result += "---\n";
        result += self.content.as_ref().unwrap();
        result
    }
}

impl std::fmt::Display for Post {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.to_markdown())
    }
}
