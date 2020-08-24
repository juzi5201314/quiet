use anyhow::Result;
use serde::{Deserialize, Serialize, Deserializer};
use mongodb::bson::{Document, to_bson};

use crate::database::get_db;

use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum PostId {
    String(String),
    Number(i64),
}

impl Display for PostId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PostId::String(s) => s.to_owned(),
            PostId::Number(i) => i.to_string()
        })
    }
}

impl Default for PostId {
    fn default() -> Self {
        PostId::Number(-1)
    }
}

impl From<&str> for PostId {
    fn from(s: &str) -> Self {
        if let Ok(i) = s.parse::<i64>() {
            PostId::Number(i)
        } else {
            PostId::String(s.to_owned())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Post {
    pub _id: PostId,
    /// 文章标题
    pub title: String,
    /// 文章内容
    pub body: String,
    /// 顶置
    pub stick: bool,
    /// 开启评论
    pub can_comment: bool,
    /// 评论id列表
    pub comments: Vec<PostId>,
    /// 创建时间
    pub create_time: i32,
    /// 更新时间
    pub update_time: i32,
}

impl Display for Post {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Post {
    pub fn new(title: &str, body: &str, stick: bool, can_comment: bool) -> Self {
        let now = chrono::Local::now().timestamp();
        Post {
            title: title.to_owned(),
            body: body.to_owned(),
            stick,
            can_comment,
            create_time: now as i32,
            update_time: now as i32,
            ..Default::default()
        }
    }

    pub async fn update(&self) -> Result<()> {
        get_db().update_post_with_id(&self._id, self).await
    }

    pub async fn add(new_post: &Post) -> Result<PostId> {
        get_db().add_post(new_post).await
    }

    pub async fn remove(&self) -> Result<bool> {
        get_db().remove_post_with_id(&self._id).await
    }

    pub async fn get_all() -> Result<Vec<Post>> {
        get_db().get_all_posts().await
    }

    pub async fn get(id: &PostId) -> Result<Option<Post>> {
        get_db().get_post_with_id(id).await
    }

    pub async fn count() -> i64 {
        get_db().post_count().await
    }

    pub fn to_doc(&self) -> Result<Document> {
        Ok(to_bson(self).map(|b| b.as_document().expect("Cannot convert `Post` to document").to_owned())?)
    }
}
