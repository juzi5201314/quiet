use std::time::SystemTime;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub create_time: i64,
    pub comments: Option<i32>
}

use super::schema::posts;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub create_time: i64
}

impl NewPost {
    pub fn new(title: String, content: String) -> Self {
        NewPost { title, content, create_time: SystemTime::now().elapsed().unwrap().as_secs() as i64 }
    }
}
