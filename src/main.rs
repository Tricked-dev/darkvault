use actix_web::dev::ResourcePath;
use actix_web::web::Bytes;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use mime_guess::from_path;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

use std::borrow::Cow;
use std::path::PathBuf;
use std::{env, fs};
use time::{format_description, OffsetDateTime};

#[derive(Deserialize, Serialize)]
struct Config {
    directory: PathBuf,
    name: String,
    theme: Option<String>,
    port: Option<i32>,
    host: Option<String>,
}

lazy_static! {
    static ref CONFIG: Config = {
        let data = std::fs::read_to_string("darkvault.toml")
            .expect("Create a darkvault.toml with the config values! to get it to work!");
        toml::from_str(&data).unwrap()
    };
    static ref DIRECTORY_PATH: String = {
        let dir = CONFIG.directory.clone();
        dir.canonicalize()
            .expect("Provided directory does not exist")
            .display()
            .to_string()
    };
}

#[derive(RustEmbed)]
#[folder = "build/"]
struct Assets;

fn handle_embedded_file(path: &str) -> Option<HttpResponse> {
    if let Some(content) = Assets::get(path) {
        let body: Bytes = match content.data {
            Cow::Borrowed(bytes) => bytes.into(),
            Cow::Owned(bytes) => bytes.into(),
        };
        Some(
            HttpResponse::Ok()
                .content_type(from_path(path).first_or_octet_stream().as_ref())
                .body(body),
        )
    } else {
        None
    }
}

#[get("/{_:.*}")]
async fn handle(path: web::Path<String>) -> impl Responder {
    if let Some(res) = handle_embedded_file(path.path()) {
        res
    } else {
        handle_embedded_file("200.html").unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct PathQuery {
    path: String,
}

fn systemtime_strftime<T>(dt: T, fmt: &str) -> String
where
    T: Into<OffsetDateTime>,
{
    let format = format_description::parse(fmt).unwrap();
    dt.into().format(&format).unwrap()
}

#[derive(Deserialize, Serialize)]
struct DarkFile {
    path: String,
    directory: bool,
    modified: Option<String>,
    size: Option<u64>,
}

#[get("/download/{_:.*}")]
//TODO ADD TORRENT DOWNLOAD
async fn download(path: web::Path<String>) -> impl Responder {
    let mut dir = CONFIG.directory.clone();

    dir.push(&path.path());
    if let Ok(dir) = dir.canonicalize() {
        if !dir.display().to_string().starts_with(&*DIRECTORY_PATH) {
            return HttpResponse::BadRequest()
                .content_type("application/json")
                .body(r#"{"error":"Invalid file"}"#.to_string());
        }
    } else {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .body(r#"{"error":"Could not find file"}"#.to_string());
    }
    if dir.is_dir() {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .body(r#"{"error":"Could not find file"}"#.to_string());
    }
    let file = fs::read(&dir).expect("Something seriously went wrong");

    HttpResponse::Ok()
        .content_type("application/content-stream")
        .body(file)
}

#[get("/list")]
async fn list(path: web::Query<PathQuery>) -> impl Responder {
    let mut dir = CONFIG.directory.clone();

    dir.push(&path.path);
    if let Ok(dir) = dir.canonicalize() {
        if !dir.display().to_string().starts_with(&*DIRECTORY_PATH) {
            return HttpResponse::BadRequest()
                .content_type("application/json")
                .body(r#"{"error":"Invalid directory"}"#.to_string());
        }
    } else {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .body(r#"{"error":"Could not find directory"}"#.to_string());
    }
    if dir.is_file() {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .body(r#"{"error":"Could not find directory"}"#.to_string());
    }
    let files = fs::read_dir(dir).expect("Something seriously went wrong");

    let data = files
        .map(|x| {
            let file = x.unwrap().path();
            let meta = fs::metadata(&file).unwrap();
            DarkFile {
                path: file
                    .canonicalize()
                    .unwrap()
                    .display()
                    .to_string()
                    .replace(&format!("{}/", *DIRECTORY_PATH), ""),
                directory: file.is_dir(),
                modified: meta.modified().ok().map(|x| {
                    systemtime_strftime(x, "[year]-[month]-[day] [hour]:[minute]:[second]")
                }),
                size: Some(meta.len()),
            }
        })
        .collect::<Vec<DarkFile>>();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&data).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = format!(
        "{}:{}",
        &*CONFIG.host.clone().unwrap_or("127.0.0.1".to_owned()),
        env::var("PORT")
            .map(|x| x.parse::<i32>().unwrap())
            .unwrap_or(CONFIG.port.unwrap_or(404))
    );
    println!("Darkvault started at: http://{}", url);
    HttpServer::new(|| {
        App::new()
            .wrap(
                middleware::DefaultHeaders::new()
                    .add((
                        "X-vault-theme",
                        &*CONFIG.theme.clone().unwrap_or("light".to_owned()),
                    ))
                    .add(("X-vault-name", &*CONFIG.name)),
            )
            .service(web::scope("/api").service(list).service(download))
            .service(handle)
    })
    .bind(&url)?
    .run()
    .await
}
