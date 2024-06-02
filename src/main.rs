use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use image::imageops::FilterType;
use serde::Deserialize;
use std::io::{Write, Read, Cursor};

#[derive(Deserialize)]
struct ResizeParams {
    width: u32,
    height: u32,
}

async fn compress_file_endpoint(mut payload: Multipart) -> Result<HttpResponse> {
    let mut buffer = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        while let Ok(Some(chunk)) = field.try_next().await {
            buffer.extend_from_slice(&chunk);
        }
    }

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&buffer)?;
    let compressed_data = encoder.finish()?;

    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/gzip"))
        .body(compressed_data))
}

async fn decompress_file_endpoint(mut payload: Multipart) -> Result<HttpResponse> {
    let mut buffer = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        while let Ok(Some(chunk)) = field.try_next().await {
            buffer.extend_from_slice(&chunk);
        }
    }

    let mut decoder = GzDecoder::new(&buffer[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;

    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/octet-stream"))
        .body(decompressed_data))
}

async fn resize_image_endpoint(mut payload: Multipart, params: web::Query<ResizeParams>) -> Result<HttpResponse> {
    let params = params.into_inner();
    let width = params.width;
    let height = params.height;
    let mut buffer = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        while let Ok(Some(chunk)) = field.try_next().await {
            buffer.extend_from_slice(&chunk);
        }
    }

    let img = image::load_from_memory(&buffer).map_err(|e| {
        actix_web::error::ErrorBadRequest(format!("Failed to load image: {}", e))
    })?;

    let resized_img = img.resize(width, height, FilterType::Nearest);

    let mut resized_buffer = Vec::new();
    resized_img.write_to(&mut Cursor::new(&mut resized_buffer), image::ImageFormat::Png).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Failed to write image: {}", e))
    })?;

    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "image/png"))
        .body(resized_buffer))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/compress", web::post().to(compress_file_endpoint))
            .route("/decompress", web::post().to(decompress_file_endpoint))
            .route("/resize", web::post().to(resize_image_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
