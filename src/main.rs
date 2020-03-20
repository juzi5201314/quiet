#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::path::Path;

use actix_files::Files;
use actix_web::{App, HttpServer, web};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use tera::Tera;

use crate::config::Config;
use crate::database::Database;
use crate::database::sqlite::Sqlite;
use crate::routes::*;

mod config;
mod database;

mod routes;

pub static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| RwLock::new(Config::load().expect("Configuration format error!")));

pub static TERA: Lazy<RwLock<Tera>> = Lazy::new(|| RwLock::new(Tera::new(&format!("{}/**/*.html", CONFIG.read().templates_path())).expect("Tera Error")));

pub static DB: Lazy<Box<dyn Database + Send + Sync>> = Lazy::new(||
    if cfg!(feature="mysql") {
        // TODO: mysql
        unimplemented!()
    } else {
        Sqlite::open().expect("sqlite connection failed.")
    }
);

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    Lazy::force(&DB);
    Lazy::force(&TERA);

    let workers = CONFIG.read().workers();

    let server = HttpServer::new(|| {
        App::new()
            // 首页
            .service(web::resource("/")
                .route(web::get().to(index))
            )
            // posts 文章api
            .service(web::resource("/posts")
                .route(web::post().to(new_post))
                .route(web::get().to(get_post))
            )
            // 静态资源
            .service(Files::new("/static", Path::new(&CONFIG.read().templates_path()).join("static")).redirect_to_slash_directory())
    }).bind(CONFIG.read().listen_addr())?;

    server.addrs().iter().for_each(|addr|
        println!("listening on http://{}", addr.to_string())
    );
    println!("using workers: {}", workers);

    server
        .workers(workers)
        .run()
        .await
}

#[test]
fn add_post() {
    DB.add_post("测试".to_owned(), "hello world<br/>2line".to_owned()).unwrap()
}