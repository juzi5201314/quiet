use std::ops::Add;

use anyhow::Result;

use crate::database::model::post::{Post, PostId};

#[macro_use]
pub mod macros;
mod error;
mod database;
mod server;

#[actix_web::main(threaded_scheduler)]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    database::init().await;
    server::start().await?;

    Ok(())
}

pub fn title() -> String {
    env!("QUIET_TITLE", "Quiet Blog").to_owned()
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
    let mut post = Post::get(&id).await?.unwrap();
    let count = Post::count().await;

    post.title = "t".to_owned();
    post.update().await?;

    assert_eq!(Post::get(&id).await?.unwrap().title, "t".to_owned());
    assert_eq!(&post._id, &id);
    //assert!(post.remove().await?);
    //assert_eq!(Post::count().await, count - 1);

    Ok(())
}
