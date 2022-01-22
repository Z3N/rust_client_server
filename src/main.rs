use actix_files::NamedFile;
use actix_web::middleware::Logger;
use actix_web::web::Buf;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use awc::Client;
use image::imageops::FilterType;
use image::GenericImageView;
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::Hasher;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    image_url: String,
}

#[get("/load_file")]
async fn load_file(info: web::Query<Info>) -> HttpResponse {
    let client = Client::default();
    let decoded_uri = match urlencoding::decode(&info.image_url) {
        Ok(x) => x.to_string(),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("ERROR: URI decoding.\n{:?}", e))
        }
    };
    let res = client.get(decoded_uri.clone()).header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36").send().await;

    let mut res = match res {
        Ok(x) => x,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!(
                "ERROR: Can't download image file specified.\n{:?}",
                e
            ))
        }
    };
    let image = match res.body().await {
        Ok(x) => x,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("ERROR: Can't get response body.\n{:?}", e))
        }
    };
    let image = match image::load_from_memory(image.bytes()) {
        Ok(x) => x,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("ERROR: Invalid image file.\n{:?}", e))
        }
    };
    let image = image.resize(image.width() * 2, image.height() * 2, FilterType::Lanczos3);
    let mut hasher = DefaultHasher::new();
    hasher.write(decoded_uri.as_bytes());
    let file_name = hasher.finish();
    match image.save(format!("static/{}.jpg", file_name)) {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("ERROR: Can't write file.\n{:?}", e))
        }
    }
    let image_location = format!("http://127.0.0.1:8080/static/{}.jpg", file_name);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .header("X-Hdr", "sample")
        .body(format!(
            "<a href='{}'>{}</a>",
            image_location, image_location
        ))
}

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(load_file)
            .route("/{filename:.*}", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
