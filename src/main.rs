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
    port: Option<i32>,
    host: Option<String>,
    hidedotfiles: Option<bool>,
    #[serde(flatten)]
    /// Basically makes it so serde merges these, this allows for cleaner code see [`handle`]
    meta: Meta,
}
#[derive(Deserialize, Serialize)]

struct Meta {
    name: String,
    theme: Option<String>,
    urls: Vec<Url>,
    description: String,
}

#[derive(Deserialize, Serialize)]

struct Url {
    to: String,
    name: String,
    icon: Option<String>,
}

lazy_static! {
    static ref CONFIG: Config = {
        let data = std::fs::read_to_string("darkvault.toml")
            .expect("Create a darkvault.toml with the config values! to get it to work!");
        toml::from_str(&data).expect("Config structure is invalid please look at the readme")
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
    if path.path() == "meta.json" {
        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&CONFIG.meta).unwrap())
    } else if let Ok(file) = get_file(path.path().to_owned()) {
        HttpResponse::Ok()
            .content_type(from_path(&path.path()).first_or_octet_stream().as_ref())
            .body(file)
    } else if let Some(res) = handle_embedded_file(path.path()) {
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
    let file = get_file(path.path().to_owned());
    match file {
        Ok(file) => HttpResponse::Ok()
            .content_type("application/content-stream")
            .body(file),
        Err(e) => e,
    }
}

fn validate_dir(path: String) -> Result<PathBuf, HttpResponse> {
    let mut dir = CONFIG.directory.clone();

    dir.push(path);
    if let Ok(dir) = dir.canonicalize() {
        let display = dir.display().to_string();

        if CONFIG.hidedotfiles.unwrap_or(false) {
            for dir in &display
                .replace(&*DIRECTORY_PATH, "")
                .split("/")
                .collect::<Vec<&str>>()
            {
                if dir.starts_with('.') {
                    return Err(HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(r#"{"error":"Could not find file"}"#.to_string()));
                }
            }
        }

        if !display.starts_with(&*DIRECTORY_PATH) {
            return Err(HttpResponse::BadRequest()
                .content_type("application/json")
                .body(r#"{"error":"Invalid file"}"#.to_string()));
        }
    } else {
        return Err(HttpResponse::BadRequest()
            .content_type("application/json")
            .body(r#"{"error":"Could not find file"}"#.to_string()));
    }
    Ok(dir)
}

fn get_file(path: String) -> Result<Vec<u8>, HttpResponse> {
    let dir = validate_dir(path.path().to_owned());
    match dir {
        Ok(dir) => {
            if dir.is_dir() {
                return Err(HttpResponse::BadRequest()
                    .content_type("application/json")
                    .body(r#"{"error":"Could not find file"}"#.to_string()));
            }
            Ok(fs::read(&dir).expect("Something seriously went wrong"))
        }
        Err(e) => Err(e),
    }
}

fn get_dir(path: String) -> Result<Vec<DarkFile>, HttpResponse> {
    let dir = validate_dir(path.path().to_owned());
    match dir {
        Ok(dir) => {
            if dir.is_file() {
                return Err(HttpResponse::BadRequest()
                    .content_type("application/json")
                    .body(r#"{"error":"Could not find file"}"#.to_string()));
            }
            let files = fs::read_dir(dir).expect("Something seriously went wrong");

            let data = files
                .filter(|x| {
                    if CONFIG.hidedotfiles.unwrap_or(false) {
                        if let Ok(k) = x {
                            let display = k.path().canonicalize().unwrap().display().to_string();
                            if CONFIG.hidedotfiles.unwrap_or(false) {
                                for dir in &display
                                    .replace(&*DIRECTORY_PATH, "")
                                    .split("/")
                                    .collect::<Vec<&str>>()
                                {
                                    if dir.starts_with('.') {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                    return true;
                })
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

            Ok(data)
        }
        Err(e) => Err(e),
    }
}

#[get("/list")]
async fn list(path: web::Query<PathQuery>) -> impl Responder {
    let data = get_dir(path.path.to_owned());

    match data {
        Ok(data) => HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&data).unwrap()),
        Err(e) => e,
    }
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
                        &*CONFIG.meta.theme.clone().unwrap_or("light".to_owned()),
                    ))
                    .add(("X-vault-name", &*CONFIG.meta.name)),
            )
            .service(web::scope("/api").service(list).service(download))
            .service(handle)
    })
    .bind(&url)?
    .run()
    .await
}
