use diesel::{Connection, r2d2 as diesel_r2d2, r2d2::ConnectionManager, SqliteConnection, RunQueryDsl};

use crate::CONFIG;
use crate::database::{Database, Error};
use crate::database::models::{Post, NewPost};

#[derive(Clone)]
pub struct Sqlite {
    pool: diesel_r2d2::Pool<ConnectionManager<SqliteConnection>>
}

impl Sqlite {
    pub fn open() -> Result<Box<Self>, r2d2::Error> {
        let s = Sqlite {
            pool: diesel_r2d2::Pool::builder()
                .build(ConnectionManager::<SqliteConnection>::new(CONFIG.read().database_url.as_str()))?
        };
        super::embedded_migrations::run_with_output(&s.pool.get().unwrap(), &mut std::io::stdout());
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
}