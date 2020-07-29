use anyhow::Result;
use serde::{Deserialize, Serialize, Deserializer};

use crate::database::get_db;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

gen_model_builder!(NewPostBuilder {
    // 文章标题
    title: String,
    // 文章内容
    body: String,
    // 顶置
    stick: bool,
    // 开启评论
    can_comment: bool
});

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostId {
    String(String),
    Number(i64),
}

impl Post {
    pub async fn add(new_post: NewPostBuilder) -> Result<PostId> {
        get_db().add_post(new_post).await
    }

    pub async fn remove(&self) -> Result<bool> {
        get_db().remove_post_with_id(self._id.clone()).await
    }

    pub async fn get_all() -> Result<Vec<Post>> {
        get_db().get_all_posts().await
    }
}
