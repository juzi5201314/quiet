use std::ops::Add;

use anyhow::Result;

use crate::database::model::post::Post;

#[macro_use]
pub mod macros;
mod error;
mod database;

#[tokio::main(threaded_scheduler)]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    database::init().await;

    Ok(())
}

#[macros::test]
async fn get_posts() -> Result<()> {
    let posts = Post::get_all().await?;
    dbg!(posts);

    Ok(())
}

#[macros::test]
async fn test_post() -> Result<()> {
    let new_post = Post::new("标题", "内容", false, true);
    let id = Post::add(&new_post).await?;
    let post = Post::get(&id).await?.unwrap();
    let count = Post::count().await;
    assert!(post.remove().await?);

    assert_eq!(&post._id, &id);
    assert_eq!(Post::count().await, count - 1);

    Ok(())
}
