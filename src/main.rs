use anyhow::Result;

use crate::database::model::post::{NewPostBuilder, Post};

#[macro_use]
pub mod macros;

mod database;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}

#[tokio::test]
async fn add_post() -> Result<()> {
    dotenv::dotenv().ok();
    let new_post = NewPostBuilder::new()
        .title("标题".to_owned())
        .body("内容".to_owned())
        .can_comment(true)
        .stick(false);


    Post::add(new_post).await?;

    Ok(())
}