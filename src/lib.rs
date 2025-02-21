use wasm_bindgen::prelude::*;
use image::ImageReader;
use image::ImageFormat;

#[wasm_bindgen]
pub enum ExifRemovalStatus {
    Success,
    Error,
}

#[wasm_bindgen]
pub struct ExifRemovalResult {
    status: ExifRemovalStatus,
    data: Option<Vec<u8>>,
    error: Option<String>,
}

#[wasm_bindgen]
impl ExifRemovalResult {
    pub fn status(&self) -> String {
        match self.status {
            ExifRemovalStatus::Success => {
                String::from("success")
            }
            ExifRemovalStatus::Error => {
                String::from("error")
            }
        }
    }

    pub fn is_error(&self) -> bool {
        match self.status {
            ExifRemovalStatus::Success => {
                false
            }
            ExifRemovalStatus::Error => {
                true
            }
        }
    }

    pub fn get_data(&self) -> Option<Vec<u8>> {
        self.data.clone()
    }

    pub fn get_error(&self) -> Option<String> {
        self.error.clone()
    }
}

#[wasm_bindgen]
pub fn remove_exif(input: &[u8]) -> ExifRemovalResult {
    // Load the image without metadata
    let img = ImageReader::new(std::io::Cursor::new(input))
        .with_guessed_format()
        .expect("Failed to read image")
        .decode()
        .expect("Failed to decode image");

    let mut output = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut output), ImageFormat::Jpeg)
        .expect("Failed to write image");

    ExifRemovalResult {
        status: ExifRemovalStatus::Success,
        data: Some(output),
        error: None,
    }
}
