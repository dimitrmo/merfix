use wasm_bindgen::prelude::*;
use image::ImageReader;
use image::ImageFormat;

#[wasm_bindgen]
pub fn remove_exif(input: &[u8]) -> Vec<u8> {
    // Load the image without metadata
    let img = ImageReader::new(std::io::Cursor::new(input))
        .with_guessed_format()
        .expect("Failed to read image")
        .decode()
        .expect("Failed to decode image");

    let mut output = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut output), ImageFormat::Jpeg)
        .expect("Failed to write image");

    output
}
