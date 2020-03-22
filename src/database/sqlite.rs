use diesel::{
    r2d2 as diesel_r2d2, r2d2::ConnectionManager, QueryDsl, RunQueryDsl, SqliteConnection,
};

use crate::database::models::{NewPost, Post};
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

use super::schema::posts;

impl Database for Sqlite {
    fn add_post(&self, title: String, content: String) -> Result<(), Error> {
        diesel::insert_into(posts::table)
            .values(NewPost::new(title, content))
            .execute(&self.pool.get()?)?;
        Ok(())
    }

    fn get_posts(&self) -> Result<Vec<Post>, Error> {
        let conn = self.pool.get()?;
        Ok(posts::table.load::<Post>(&conn)?)
    }

    fn get_post(&self, post_id: String) -> Result<Post, Error> {
        let conn = self.pool.get()?;
        Ok(posts::table.find(post_id).first(&conn)?)
    }

    fn delete_post(&self, post_id: String) -> Result<(), Error> {
        unimplemented!()
    }

    fn update_post(&self, post_id: String, new_title: String, new_content: String) -> Result<(), Error> {
        unimplemented!()
    }

    fn search_posts(&self, keyword: String) -> Result<Vec<Post>, Error> {
        unimplemented!()
    }
}
