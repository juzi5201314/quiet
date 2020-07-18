
use serde::{Serialize, Deserialize};
use anyhow::Result;
use crate::database::get_db;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    _id: String,
    /// 文章标题
    pub title: String,
    /// 文章内容
    pub body: String,
    /// 顶置
    pub stick: bool,
    /// 开启评论
    pub can_comment: bool,
    /// 评论id列表
    comments: Vec<String>,
    /// 创建时间
    pub create_time: u64,
    /// 更新时间
    pub update_time: u64
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

impl Post {
    pub async fn add(new_post: NewPostBuilder) -> Result<()> {
        get_db().await.add_post(new_post).await
    }
}
