use mongodb::{Client, options::ClientOptions};

use crate::CONFIG;
use crate::database::{Database, Error};
use crate::database::models::{Post, NewPost};

static DATABASE_NAME: &str = "quiet_db";

#[derive(Clone)]
pub struct Mongo {

}


