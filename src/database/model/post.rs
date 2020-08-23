use anyhow::Result;
use serde::{Deserialize, Serialize, Deserializer};
use mongodb::bson::{Document, to_bson};

use crate::database::get_db;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostId {
    String(String),
    Number(i64),
}

impl Default for PostId {
    fn default() -> Self {
        PostId::Number(0)
    }
}

impl Post {
    pub async fn add(new_post: Post) -> Result<PostId> {
        get_db().add_post(new_post).await
    }

    pub async fn remove(&self) -> Result<bool> {
        get_db().remove_post_with_id(self._id.clone()).await
    }

    pub async fn get_all() -> Result<Vec<Post>> {
        get_db().get_all_posts().await
    }

    pub fn to_doc(&self) -> Result<Document> {
        let mut doc = to_bson(self).map(|b| b.as_document().expect("Cannot convert `Post` to document").to_owned())?;
        // 去掉_id字段，避免数据库不自动生成id
        doc.remove("_id");
        Ok(doc)
    }
}
