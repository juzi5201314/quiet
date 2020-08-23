use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use byteorder::{BigEndian, ByteOrder};
use mongodb::{Client, Collection, Cursor};
use mongodb::bson::{Array, Bson, doc, Document, from_bson, oid::ObjectId, to_bson};
use mongodb::options::{ClientOptions, FindOptions, UpdateModifications};
use tokio::stream::StreamExt;

use crate::database::model::post::{Post, PostId};
use crate::database::traits::{AddPost, DatabaseTrait, DelPost, GetPost, PostTrait, UpdatePost};
use crate::error::Error;
use tokio::macros::support::Future;

pub struct MongoDB {
    client: Client,
    client_options: ClientOptions,
    db: mongodb::Database
}

impl MongoDB {
    pub async fn from_url(url: &str) -> Result<Self> {
        let client_options = ClientOptions::parse(&url).await?;
        if client_options.credential.is_none() {
            panic!("For security, mongodb must fill in the credential.");
        }
        let client = Client::with_options(client_options.clone())?;

        Ok(MongoDB {
            db: client.database(
                client_options
                    .credential
                    .as_ref()
                    .unwrap()
                    .source
                    .as_ref()
                    .unwrap(),
            ),
            client,
            client_options
        })
    }

    fn get_posts_collection(&self) -> Collection {
        self.db.collection("posts")
    }
}

impl DatabaseTrait for MongoDB {}

impl Post {
    fn from_doc(doc: &Document) -> Result<Self> {
        let id = doc.get_object_id("_id")?;
        let time = BigEndian::read_u32(&id.bytes()) as i32;
        Ok(Post {
            _id: PostId::String(id.to_string()),
            title: doc.get_str("title")?.to_owned(),
            body:  doc.get_str("body")?.to_owned(),
            stick: doc.get_bool("stick")?,
            can_comment: doc.get_bool("can_comment")?,
            comments: doc.get_array("comments")?.iter().filter_map(
                |b| Some(PostId::String(b.as_object_id()?.to_string()))
            ).collect(),
            create_time: time,
            update_time: doc.get_i32("update_time")?
        })
    }
}

#[async_trait]
impl PostTrait for MongoDB {
    async fn post_count(&self) -> i64 {
        self.get_posts_collection().estimated_document_count(None).await.unwrap_or(0)
    }
}

#[async_trait]
impl UpdatePost for MongoDB {
    async fn update_post_with_id(&self, id: &PostId, post: &Post) -> Result<()> {
        self.get_posts_collection().update_one(
            doc! { "_id": Bson::ObjectId(ObjectId::with_string(&id.to_string())?) },
            UpdateModifications::Document({
                let mut doc = post.to_doc()?;
                doc.remove("_id");
                doc
            }),
            None
        ).await?;
        Ok(())
    }
}

#[async_trait]
impl GetPost for MongoDB {
    async fn get_all_posts(&self) -> Result<Vec<Post>> {
        let mut res = Vec::new();
        let mut cursor = self.get_posts_collection().find(
            None,
            Some(FindOptions::builder()
                .sort(Some(doc! { "update_time": -1 }))
                .build()),
        ).await?;

        while let Some(doc) = cursor.next().await {
            res.push(Post::from_doc(&doc?)?)
        }

        Ok(res)
    }

    async fn get_post_with_id(&self, id: &PostId) -> Result<Option<Post>> {
        Ok(self.get_posts_collection().find_one(doc! {
            "_id": Bson::ObjectId(ObjectId::with_string(&id.to_string())?)
        }, None).await?.map(|doc: Document| Post::from_doc(&doc).unwrap()))
    }
}

#[async_trait]
impl DelPost for MongoDB {
    async fn remove_post_with_id(&self, id: &PostId) -> Result<bool> {
        let del_cpunt = self.get_posts_collection()
            .delete_one(
                doc! {
                        "_id": Bson::ObjectId(ObjectId::with_string(&id.to_string())?)
                    },
                None,
            )
            .await?.deleted_count;
        Ok(del_cpunt == 1)
    }

    async fn remove_posts_with_id_list(&self, ids: Vec<&PostId>) -> Result<bool> {
        for res in futures::future::join_all({
            let mut v = Vec::new();
            for id in ids {
                v.push(self.remove_post_with_id(id))
            }
            v
        }).await {
            if !res? {
                return Ok(false)
            }
        }
        Ok(true)
    }
}

#[async_trait]
impl AddPost for MongoDB {
    async fn add_post(&self, post: &Post) -> Result<PostId> {
        let id: Bson = self.get_posts_collection()
            .insert_one(post.to_doc()?, None)
            .await?.inserted_id;
        Ok(PostId::String(id.as_object_id().ok_or(Error::None("ObjectId".to_owned()))?.to_string()))
    }
}
