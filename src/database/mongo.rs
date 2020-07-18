use anyhow::Result;
use async_trait::async_trait;
use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use mongodb::bson::{Document, doc};

use crate::database::{AddPost, Database};
use crate::database::model::post::{NewPostBuilder, Post};

pub struct MongoDB {
    client: Client,
    client_options: ClientOptions,
    db: mongodb::Database
}

impl MongoDB {
    pub async fn from_url(url: &str) -> Result<Self> {
        let mut client_options = ClientOptions::parse(&url).await?;
        if client_options.credential.is_none() {
            panic!("For security, mongodb must fill in the credential.");
        }
        let client = Client::with_options(client_options.clone())?;

        Ok(MongoDB {
            db: client.database(client_options.credential.as_ref().unwrap().source.as_ref().unwrap()),
            client,
            client_options
        })
    }

    fn get_posts_collection(&self) -> Collection{
        self.db.collection("posts")
    }
}

impl Database for MongoDB {

}

impl NewPostBuilder {
    pub fn to_doc(&self) -> Document {
        mongodb::bson::to_bson(self).unwrap().as_document().unwrap().to_owned()
    }
}


#[async_trait]
impl AddPost for MongoDB {
    async fn add_post(&self, post: NewPostBuilder) -> Result<()> {
        self.get_posts_collection().insert_one(post.to_doc(), None).await?;
        Ok(())
    }
}