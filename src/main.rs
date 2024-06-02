extern crate flate2;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, Read};
use flate2::read::GzDecoder;

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
    let mut decoder = GzDecoder::new(input_file);
    
    io::copy(&mut decoder, &mut &output_file)?;
    Ok(())
}


fn main() {
    let input_path = "example.txt";
    let compressed_path = "example.txt.gz";
    let decompressed_path = "example_decompressed.txt";
    
    match compress_file(input_path, compressed_path) {
        Ok(_) => println!("File compressed successfully."),
        Err(e) => eprintln!("Error compressing file: {}", e),
    }
    
    match decompress_file(compressed_path, decompressed_path) {
        Ok(_) => println!("File decompressed successfully."),
        Err(e) => eprintln!("Error decompressing file: {}", e),
    }
}
