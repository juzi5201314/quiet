//! 木有标准，这里的api全部都是瞎jb设计的。
//! There is no standard, all APIs here are designed blindly.

use std::collections::HashMap;

use actix_session::Session;
use actix_web::http::header;
use actix_web::{web, Error as WebError, HttpResponse};
use serde::Deserialize;

use crate::{clean_html, DB, TERA};

macro_rules! web_error {
    ($err:expr) => {
        $err.map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))
    };
}

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

pub async fn login(session: &Session) -> Result<HttpResponse, WebError> {
    //TODO: 鉴权
    unimplemented!()
}

/// GET /
pub async fn index(_: web::Query<HashMap<String, String>>) -> Result<HttpResponse, WebError> {
    let mut content = tera::Context::new();

    content.insert("posts", &web_error!(DB.get_posts())?);

    let body = web_error!(TERA.read().render("index.html", &content))?;

    Ok(HttpResponse::Ok().body(body))
}

#[derive(Deserialize)]
pub struct UpdatePostData {
    pub id: String,
    pub title: Option<String>,
    pub content: Option<String>,
}

/// POST /posts/update
pub async fn update_post(data: web::Json<UpdatePostData>) -> Result<HttpResponse, WebError> {
    web_error!(DB.update_post(
        data.id.clone(),
        data.title.clone(),
        data.content.as_ref().map(|content| clean_html(content))
    ))?;
    Ok(HttpResponse::Found().header(header::LOCATION, "/").finish())
}

#[derive(Deserialize)]
pub struct NewPostData {
    pub title: String,
    pub content: String,
}

/// POST /posts/new
pub async fn new_post(data: web::Json<NewPostData>) -> Result<HttpResponse, WebError> {
    web_error!(DB.add_post(data.title.clone(), clean_html(&data.content)))?;
    Ok(HttpResponse::Found().header(header::LOCATION, "/").finish())
}

/// GET /posts or /posts?id={postid}
pub async fn get_post(
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, WebError> {
    Ok(if query.contains_key("id") {
        HttpResponse::Ok().json(web_error!(
            DB.get_post(web_error!(query.get("id").unwrap().parse::<String>())?)
        )?)
    } else {
        HttpResponse::Ok().json(web_error!(DB.get_posts())?)
    })
}
