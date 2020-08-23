use actix_web::{error, get, Responder, web, HttpResponse};
use askama::Template;

use crate::database::model::post::Post;
use crate::server::template::IndexTemplate;
use crate::title;

macro_rules! map_err {
    ($e:expr) => {
        $e.map_err(|e| error::ErrorInternalServerError(e))
    };
}

#[get("/")]
pub async fn index() -> error::Result<impl Responder> {
    let temp = IndexTemplate {
        title: title(),
        posts: map_err!(Post::get_all().await)?
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(map_err!(temp.render())?)
    )
}
