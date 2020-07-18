use async_trait::async_trait;
use anyhow::Result;

use crate::database::model::post::{Post, NewPostBuilder};
use crate::database::mongo::MongoDB;

pub mod model;
pub mod mongo;

static mut DATABASE: Option<Box<dyn Database + Send + Sync>> = None;

async fn build_database() -> Box<dyn Database + Send + Sync> {
    let err = "Database URL is unqualified.";
    let url = env!("QUIET_DB"; required);
    let ty = url.get(..url.find(':').expect(err)).expect(err);
    match ty {
        "mongodb" => {
            Box::new(MongoDB::from_url(&url).await.expect(""))
        },
        _ => panic!("Unsupported database.")
    }
}

pub async fn get_db() -> &'static dyn Database {
    unsafe {
        if DATABASE.is_none() {
            DATABASE = Some(build_database().await);
        }
        DATABASE.as_ref().unwrap().as_ref()
    }

}

pub trait Database: AddPost + Sync + Send {

}

#[async_trait]
pub trait AddPost {
    async fn add_post(&self, post: NewPostBuilder) -> Result<()>;
}