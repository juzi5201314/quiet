//! 木有标准，这里的api全部都是瞎jb设计的。
//! There is no standard, all APIs here are designed blindly.

use std::collections::HashMap;

use actix_web::{Error as WebError, HttpResponse, web};

use crate::{DB, TERA};

macro_rules! web_error {
    ($err:expr) => {
        $err.map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))
    };
}

/// GET /
pub async fn index(_: web::Query<HashMap<String, String>>) -> Result<HttpResponse, WebError> {
    let mut content = tera::Context::new();

    content.insert("posts", &web_error!(DB.get_posts())?);

    let body = web_error!(TERA.read().render("index.html", &content))?;

    Ok(HttpResponse::Ok().body(body))
}

/// POST /posts
pub async fn new_post() -> Result<HttpResponse, WebError> {
    unimplemented!()
}

/// GET /posts or /posts?id={postid}
pub async fn get_post(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse, WebError> {
    Ok(if query.contains_key("id") {
        HttpResponse::Ok().json(web_error!(DB.get_post(web_error!(query.get("id").unwrap().parse::<i32>())?))?)
    } else {
        HttpResponse::Ok().json(web_error!(DB.get_posts())?)
    })
}