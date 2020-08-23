use actix_web::{App, HttpServer};
use anyhow::Result;

use crate::server::routes::index;

mod template;
mod routes;

pub async fn start() -> Result<()> {
    Ok(HttpServer::new(|| App::new().service(index))
        .bind(env!("QUIET_ADDR", "127.0.0.1:7070"))?
        .run()
        .await?)
}