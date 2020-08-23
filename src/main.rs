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
async fn del_post() -> Result<()> {
    //Post::remove(q).await?;

    Ok(())
}

#[macros::test]
async fn add_post() -> Result<()> {
    let new_post = Post {
        title: "标题".to_owned(),
        body: "内容".to_owned(),
        stick: false,
        can_comment: true,
        create_time: 0,
        update_time: 0,
        ..Default::default()
    };

    Post::add(new_post).await?;

    Ok(())
}
