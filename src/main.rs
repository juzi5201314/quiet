#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::path::Path;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use tera::Tera;

use crate::config::Config;
use crate::database::Database;
use crate::database::{mongo::Mongo, sqlite::Sqlite};
use crate::routes::*;
use actix_http::cookie::SameSite;
use actix_session::CookieSession;

mod config;
mod database;

mod routes;

pub static CONFIG: Lazy<RwLock<Config>> =
    Lazy::new(|| RwLock::new(Config::load().expect("Configuration format error!")));

pub static TERA: Lazy<RwLock<Tera>> = Lazy::new(|| {
    RwLock::new({
        let mut tera = Tera::new(&format!("{}/**/*.html", CONFIG.read().templates_path()))
            .expect("Tera Error");
        tera.autoescape_on(vec![]);
        tera
    })
});

pub static DB: Lazy<Box<dyn Database + Send + Sync>> = Lazy::new(|| {
    match CONFIG
        .read()
        .database_mode
        .as_ref()
        .unwrap_or(&String::from("sqlite"))
        .as_str()
    {
        "mysql" => unimplemented!(), // TODO: mysql
        "sqlite" => Sqlite::open().expect("sqlite connection failed."),
        "mongodb" => Mongo::open().expect("MongoDB initialization failed."),
        _ => panic!("unknown database_mode"),
    }
});

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    Lazy::force(&DB);
    Lazy::force(&TERA);

    let workers = CONFIG.read().workers();

    let server = HttpServer::new(|| {
        App::new()
            .wrap(
                CookieSession::private(&[0x07; 32])
                    .name("quiet")
                    .secure(false)
                    .same_site(SameSite::None)
                    .max_age(3600 * 24 * 10),
            )
            // 首页
            .service(web::resource("/").route(web::get().to(index)))
            // posts 文章api
            .service(
                web::resource("/posts")
                    .route(web::post().to(new_post))
                    .route(web::get().to(get_post)),
            )
            // 静态资源
            .service(
                Files::new(
                    "/static",
                    Path::new(&CONFIG.read().templates_path()).join("static"),
                )
                .redirect_to_slash_directory(),
            )
    })
    .bind(CONFIG.read().listen_addr())?;

    server
        .addrs()
        .iter()
        .for_each(|addr| println!("listening on http://{}", addr.to_string()));
    println!("using workers: {}", workers);

    server.workers(workers).run().await
}

pub fn clean_html(html: &str) -> String {
    ammonia::Builder::default()
        .strip_comments(false)
        .clean(html)
        .to_string()
}

#[actix_rt::test]
async fn add_post() {
    use actix_web::test;

    let form = web::Json(PostData {
        title: "测试2".to_string(),
        content: r#"<h2>emm</h2>hello world<br/>2line<script>alert("xss")</script>"#.to_string(),
    });
    let resp = new_post(form).await.unwrap();
    dbg!(resp);
}
