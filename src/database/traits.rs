use anyhow::Result;
use async_trait::async_trait;

use crate::database::model::post::{Post, PostId};

pub trait DatabaseTrait: Sync + Send + PostTrait {

}

#[async_trait]
pub trait PostTrait: AddPost + DelPost + GetPost + UpdatePost {
    async fn post_count(&self) -> i64;
}

#[async_trait]
pub trait AddPost {
    async fn add_post(&self, post: &Post) -> Result<PostId>;
}

#[async_trait]
pub trait DelPost {
    async fn remove_post_with_id(&self, id: &PostId) -> Result<bool>;
}

#[async_trait]
pub trait GetPost {
    async fn get_all_posts(&self) -> Result<Vec<Post>>;
    async fn get_post_with_id(&self, id: &PostId) -> Result<Option<Post>>;
}

#[async_trait]
pub trait UpdatePost {
    async fn update_post_with_id(&self, id: &PostId, post: &Post) -> Result<()>;
}