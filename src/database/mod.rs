use std::ops::Deref;
use std::sync::Arc;

use anyhow::Result;

use crate::database::model::post::Post;
use crate::database::mongo::MongoDB;
use crate::database::traits::DatabaseTrait;

pub mod traits;

pub mod model;
pub mod mongo;

type DatabaseBox = Box<dyn DatabaseTrait + Send + Sync>;

static mut DATABASE: Option<Arc<Database>> = None;

async fn build_database() -> Database {
    let err = "Database URL is unqualified.";
    let url = env!("QUIET_DB"; required);
    let ty = url.get(..url.find(':').expect(err)).expect(err);
    match ty {
        "mongodb" => Database::new(
            DatabaseType::Mongo,
            Box::new(MongoDB::from_url(&url).await.unwrap()),
        ),
        _ => panic!("Unsupported database."),
    }
}

pub async fn init() {
    unsafe {
        DATABASE = Some(Arc::new(build_database().await));
    }
}

pub fn get_db() -> Arc<Database> {
    unsafe {
        DATABASE.as_ref().expect("The database is not initialized.").clone()
    }
}

pub struct Database {
    _type: DatabaseType,
    db: DatabaseBox,
}

impl Database {
    fn new(ty: DatabaseType, db: DatabaseBox) -> Self {
        Database { _type: ty, db }
    }

    pub fn get_type(&self) -> &DatabaseType {
        &self._type
    }
}

impl Deref for Database {
    type Target = DatabaseBox;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

pub enum DatabaseType {
    Mongo,
}
