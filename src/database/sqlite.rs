use diesel::{
    r2d2 as diesel_r2d2, r2d2::ConnectionManager, r2d2::PooledConnection, AppearsOnTable, Column,
    ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SqliteConnection,
};

use crate::database::models::{NewPost, Post, UpdatePost};
use crate::database::{Database, Error};
use crate::CONFIG;

#[derive(Clone)]
pub struct Sqlite {
    pool: diesel_r2d2::Pool<ConnectionManager<SqliteConnection>>,
}

impl Sqlite {
    pub fn open() -> Result<Box<Self>, r2d2::Error> {
        let s = Sqlite {
            pool: diesel_r2d2::Pool::builder().build(
                ConnectionManager::<SqliteConnection>::new(CONFIG.read().database_url.as_str()),
            )?,
        };
        super::embedded_migrations::run_with_output(&s.pool.get().unwrap(), &mut std::io::stdout())
            .expect("load embedded migration failed.");
        Ok(Box::new(s))
    }
}

impl Sqlite {
    fn get_conn(&self) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, Error> {
        Ok(self.pool.get()?)
    }
}

use super::schema::posts;
use diesel::backend::Backend;
use diesel::query_builder::{AsChangeset, AstPass, QueryFragment};

impl Database for Sqlite {
    fn add_post(&self, title: String, content: String) -> Result<(), Error> {
        diesel::insert_into(posts::table)
            .values(NewPost::new(title, content))
            .execute(&self.pool.get()?)?;
        Ok(())
    }

    fn get_posts(&self) -> Result<Vec<Post>, Error> {
        Ok(posts::table.load::<Post>(&self.get_conn()?)?)
    }

    fn get_post(&self, post_id: String) -> Result<Post, Error> {
        Ok(posts::table.find(post_id).first(&self.get_conn()?)?)
    }

    fn delete_post(&self, post_id: String) -> Result<(), Error> {
        let del_num = diesel::delete(posts::table.find(post_id)).execute(&self.get_conn()?)?;
        if del_num != 1 {
            Err(Error::Other(String::from(
                "number of deletes is not equal to 1.",
            )))
        } else {
            Ok(())
        }
    }

    fn update_post(
        &self,
        post_id: String,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<(), Error> {
        if diesel::update(posts::table.find(post_id))
            .set(&UpdatePost {
                title,
                content,
                update_time: chrono::Local::now().timestamp(),
            })
            .execute(&self.get_conn()?)?
            != 1
        {
            Err(Error::Other(String::from(
                "number of updates is not equal to 1.",
            )))
        } else {
            Ok(())
        }
    }

    fn search_posts(&self, keyword: String) -> Result<Vec<Post>, Error> {
        unimplemented!()
    }
}
