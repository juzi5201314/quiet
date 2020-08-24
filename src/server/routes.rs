use actix_web::{error, get, HttpResponse, post, Responder, web};
use askama::Template;
use serde::Deserialize;

use crate::database::model::post::{Post, PostId};
use crate::server::template::{EditorTemplate, IndexTemplate};
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
        posts: map_err!(Post::get_all().await)?,
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(map_err!(temp.render())?)
    )
}

#[get("/editor/{id}")]
pub async fn editor(web::Path((id)): web::Path<(String)>) -> error::Result<impl Responder> {
    let post = map_err!(Post::get(&PostId::from(id.as_str())).await);
    let is_new = id == "new";
    if post.as_ref().unwrap_or(&None).is_none() && !is_new {
        return Err(error::ErrorNotFound("Post does not exist."))
    }
    let post = post.unwrap_or(None).unwrap_or_else(|| Post::new("", "", false, true));
    let temp = EditorTemplate {
        is_new,
        post,
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(map_err!(temp.render())?)
    )
}

#[derive(Deserialize)]
struct EditPost {
    pub is_new: bool,
    pub post: Post
}

#[post("/post")]
pub async fn edit_post(data: web::Json<EditPost>) -> error::Result<impl Responder> {
    if data.is_new {
        map_err!(Post::add(&data.post).await)?;
    } else {
        let has_post = map_err!(Post::get(&data.post._id).await).unwrap_or_default().is_some();
        if has_post {
            map_err!(data.post.update().await)?
        } else {
            return Err(error::ErrorNotFound("Post does not exist."))
        }
    }

    Ok(HttpResponse::Ok())
}