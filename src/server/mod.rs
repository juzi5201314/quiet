use actix_web::{App, HttpServer, web};
use anyhow::Result;

use crate::server::routes::{index, editor, edit_post};

mod template;
mod routes;

pub async fn start() -> Result<()> {
    Ok(HttpServer::new(|| App::new()
        .service(actix_files::Files::new("/static", env!("QUIET_STATIC", "templates/static")))
        .service(index)
        .service(editor)
        .service(web::scope("/api")
            .service(edit_post))
    )
        .bind(env!("QUIET_ADDR", "127.0.0.1:7070"))?
        .run()
        .await?)
}