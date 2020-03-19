#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

use std::collections::HashMap;
use std::path::Path;

use actix_files::Files;
use actix_web::{App, Error as WebError, HttpResponse, HttpServer, web};
use actix_web::http::StatusCode;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use tera::Tera;

use crate::config::Config;
use crate::database::Database;
use crate::database::sqlite::Sqlite;

mod config;
mod database;

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

async fn index(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse, WebError> {
    let mut content = tera::Context::new();

    let body = TERA.read().render("index.html", &content)
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

    Ok(HttpResponse::Ok().body(body))
}