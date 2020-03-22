use crate::database::models::Post;

// 开启了sqlite并且没有开启mysql，如果开启了mysql就不使用sqlite而使用mysql
// 没有开启mysql也没有开启sqlite（默认使用sqlite）
#[cfg(any(
    all(feature = "sqlite", not(feature = "mysql")),
    all(not(feature = "sqlite"), not(feature = "mysql"))
))]
embed_migrations!("./migrations/sqlite");

#[cfg(all(feature = "mysql"))]
embed_migrations!("./migrations/mysql");

pub mod models;
pub mod mongo;
mod schema;
pub mod sqlite;

#[derive(Debug)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error(s)
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Self {
        Error::from(err.to_string())
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::from(err.to_string())
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(err: mongodb::error::Error) -> Self {
        Error::from(err.kind.clone().to_string())
    }
}

impl From<bson::oid::Error> for Error {
    fn from(err: bson::oid::Error) -> Self { Error::from(err.to_string()) }
}

pub trait Database {
    fn add_post(&self, title: String, content: String) -> Result<(), Error>;
    fn get_posts(&self) -> Result<Vec<Post>, Error>;
    fn get_post(&self, post_id: String) -> Result<Post, Error>;
    fn delete_post(&self, post_id: String) -> Result<(), Error>;
    fn update_post(&self, post_id: String, new_title: String, new_content: String) -> Result<(), Error> {
        self.delete_post(post_id)?;
        self.add_post(new_title, new_content)
    }
    fn search_posts(&self, keyword: String) -> Result<Vec<Post>, Error>;
}
