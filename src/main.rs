use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self};

#[derive(Deserialize)]
struct FileOperation {
    input_path: String,
    output_path: String,
}

async fn compress_file_endpoint(data: web::Json<FileOperation>) -> impl Responder {
    match compress_file(&data.input_path, &data.output_path) {
        Ok(_) => HttpResponse::Ok().body("File compressed successfully."),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error compressing file: {}", e)),
    }
}

async fn decompress_file_endpoint(data: web::Json<FileOperation>) -> impl Responder {
    match decompress_file(&data.input_path, &data.output_path) {
        Ok(_) => HttpResponse::Ok().body("File decompressed successfully."),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error decompressing file: {}", e)),
    }
}

fn compress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    
    io::copy(&mut &input_file, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}

fn decompress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;
    let mut decoder = flate2::read::GzDecoder::new(input_file);
    
    io::copy(&mut decoder, &mut &output_file)?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/compress", web::post().to(compress_file_endpoint))
            .route("/decompress", web::post().to(decompress_file_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
