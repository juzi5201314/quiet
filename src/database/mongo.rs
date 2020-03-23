use bson::{doc, oid, Bson};
use mongodb::{error as mongo_error, Client};

use crate::database::models::Post;
use crate::database::{Database, Error, ErrorKind};
use crate::CONFIG;

static DATABASE_NAME: &str = "quiet_db";
static POST_COLLECTION: &str = "posts";

#[derive(Clone)]
pub struct Mongo {
    client: Client,
}

impl Mongo {
    pub fn open() -> Result<Box<Self>, mongo_error::Error> {
        Ok(Box::new(Mongo {
            client: Client::with_uri_str(CONFIG.read().database_url.as_str())?,
        }))
    }
    fn get_post_collection(&self) -> mongodb::Collection {
        self.client
            .database(DATABASE_NAME)
            .collection(POST_COLLECTION)
    }
}

use chrono::prelude::*;

impl Database for Mongo {
    fn add_post(&self, title: String, content: String) -> Result<(), Error> {
        self.get_post_collection()
            .insert_one(
                doc! {
                   "title": title,
                   "content": content,
                   "create_time": Local::now().timestamp()
                },
                None,
            )?;
        Ok(())
    }

    fn get_posts(&self) -> Result<Vec<Post>, Error> {
        let mut posts = Vec::new();
        for result in self
            .get_post_collection()
            .find(None, None)? {
            match result {
                Ok(document) => {
                    posts.push(Post {
                        id: document.get_object_id("_id").unwrap().to_hex(),
                        title: if let Some(title) =
                            document.get("title").and_then(Bson::as_str) {
                            title.to_string()
                        } else {
                            String::from("unnamed")
                        },
                        content: if let Some(content) =
                            document.get("content").and_then(Bson::as_str)
                        {
                            content.to_string()
                        } else {
                            String::new()
                        },
                        create_time: if let Some(create_time) =
                            document.get("create_time").and_then(Bson::as_i64)
                        {
                            create_time
                        } else {
                            continue;
                        },
                        update_time: if let Some(update_time) =
                            document.get("update_time").and_then(Bson::as_i64)
                        {
                            update_time
                        } else {
                            continue;
                        },
                        comments: if let Some(comments) =
                            document.get("comments").and_then(Bson::as_i32)
                        {
                            Some(comments)
                        } else {
                            None
                        },
                    });
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(posts)
    }

    fn get_post(&self, post_id: String) -> Result<Post, Error> {
        match self
            .get_post_collection()
            .find_one(
                doc! { "_id":  oid::ObjectId::with_string(post_id.as_str())? },
                None,
            )? {
            Some(document) => Ok(Post {
                id: document.get_object_id("_id").unwrap().to_hex(),
                title: if let Some(title) =
                    document.get("title").and_then(Bson::as_str) {
                    title.to_string()
                } else {
                    String::from("unnamed")
                },
                content: if let Some(content) =
                    document.get("content").and_then(Bson::as_str) {
                    content.to_string()
                } else {
                    String::new()
                },
                create_time: if let Some(create_time) =
                    document.get("create_time").and_then(Bson::as_i64)
                {
                    create_time
                } else {
                    return Err(Error(ErrorKind::Mongo, "invalid create_time".to_string()));
                },
                update_time: if let Some(update_time) =
                document.get("update_time").and_then(Bson::as_i64)
                {
                    update_time
                } else {
                    return Err(Error(ErrorKind::Mongo, "invalid create_time".to_string()));
                },
                comments: if let Some(comments) = document.get("comments").and_then(Bson::as_i32) {
                    Some(comments)
                } else {
                    None
                },
            }),
            None => Err(Error(ErrorKind::Mongo, "not found".to_string())),
        }
    }

    fn delete_post(&self, post_id: String) -> Result<(), Error> {
        self.get_post_collection()
            .delete_one(doc! { "_id": oid::ObjectId::with_string(post_id.as_str())? }, None)?;
        Ok(())
    }

    fn update_post(&self, post_id: String, new_title: Option<String>, new_content: Option<String>) -> Result<(), Error> {
        self.get_post_collection()
            .update_one(doc! {"_id": oid::ObjectId::with_string(post_id.as_str())?},
                        doc! {"$set": if new_title.is_some() && new_content.is_some() {
                            doc! {"title": new_title.unwrap(), "content": new_content.unwrap()}
                        } else if new_title.is_none() && new_content.is_some() {
                            doc! {"content": new_content.unwrap()}
                        } else if new_title.is_some() && new_content.is_none() {
                            doc! {"title": new_title.unwrap()}
                        } else {
                            doc! {}
                        }
                        }, None)?;
        Ok(())
    }

    fn search_posts(&self, keyword: String) -> Result<Vec<Post>, Error> {
        let mut posts = Vec::new();
        for result in self
            .get_post_collection()
            .find(doc! {
                    "$or": [
                        {"content": {"$regex": keyword.as_str()}},
                        {"title": {"$regex": keyword.as_str()}}
                    ]
                }, None)? {
            match result {
                Ok(document) => {
                    posts.push(Post {
                        id: document.get_object_id("_id").unwrap().to_hex(),
                        title: if let Some(title) =
                        document.get("title").and_then(Bson::as_str) {
                            title.to_string()
                        } else {
                            String::from("unnamed")
                        },
                        content: if let Some(content) =
                        document.get("content").and_then(Bson::as_str)
                        {
                            content.to_string()
                        } else {
                            String::new()
                        },
                        create_time: if let Some(create_time) =
                        document.get("create_time").and_then(Bson::as_i64)
                        {
                            create_time
                        } else {
                            continue;
                        },
                        update_time: if let Some(update_time) =
                        document.get("update_time").and_then(Bson::as_i64)
                        {
                            update_time
                        } else {
                            continue;
                        },
                        comments: if let Some(comments) =
                        document.get("comments").and_then(Bson::as_i32)
                        {
                            Some(comments)
                        } else {
                            None
                        },
                    });
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(posts)
    }
}
