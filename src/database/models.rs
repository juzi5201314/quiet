use serde::Serialize;

use super::schema::posts;

#[derive(Queryable, Serialize, Debug)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub create_time: i64,
    pub comments: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub create_time: i64,
}

impl NewPost {
    pub fn new(title: String, content: String) -> Self {
        NewPost {
            title,
            content,
            create_time: chrono::Local::now().timestamp(),
        }
    }
}
