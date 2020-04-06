use diesel::prelude::*;
use serde::Serialize;

use super::schema::posts;

#[derive(Queryable, Serialize, Debug)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub create_time: i64,
    pub update_time: i64,
    pub comments: Option<i32>,
}

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
    pub update_time: i64,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub id: String,
    pub title: String,
    pub content: String,
    pub update_time: i64,
    pub create_time: i64,
}

impl NewPost {
    pub fn new(title: String, content: String) -> Self {
        let now = chrono::Local::now().timestamp();
        NewPost {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            content,
            update_time: now,
            create_time: now,
        }
    }
}
