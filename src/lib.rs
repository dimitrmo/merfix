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

fn get_image_format(extension: &str) -> Option<ImageFormat> {
    match extension.to_lowercase().as_str() {
        "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
        "png" => Some(ImageFormat::Png),
        "webp" => Some(ImageFormat::WebP),
        "bmp" => Some(ImageFormat::Bmp),
        "tiff" => Some(ImageFormat::Tiff),
        "gif" => Some(ImageFormat::Gif),
        _ => None, // Unsupported format
    }
}

#[wasm_bindgen]
pub fn supported_formats() -> Vec<JsValue> {
    let metadata_types = vec![
        "jpg", "jpeg",
        "png",
        "webp",
        "bmp",
        "tiff",
        "gif"
    ];
    metadata_types.into_iter().map(JsValue::from).collect()
}

#[wasm_bindgen]
pub fn remove_exif(input: &[u8], extension: &str) -> ExifRemovalResult {
    let format = match get_image_format(extension) {
        Some(fmt) => fmt,
        None => return ExifRemovalResult {
            status: ExifRemovalStatus::Error,
            data: None,
            error: Some("Unsupported image format".to_string()),
        },
    };

    let img = match ImageReader::new(
        std::io::Cursor::new(input)
    ).with_guessed_format() {
        Ok(reader) => match reader.decode() {
            Ok(decoded) => decoded,
            Err(_) => return ExifRemovalResult {
                status: ExifRemovalStatus::Error,
                data: None,
                error: Some("Failed to decode image".to_string()),
            },
        },
        Err(_) => return ExifRemovalResult {
            status: ExifRemovalStatus::Error,
            data: None,
            error: Some("Failed to read image format".to_string()),
        },
    };

    let mut output = Vec::new();
    if img.write_to(&mut std::io::Cursor::new(&mut output), format).is_err() {
        return ExifRemovalResult {
            status: ExifRemovalStatus::Error,
            data: None,
            error: Some("Failed to write image".to_string()),
        };
    }

    ExifRemovalResult {
        status: ExifRemovalStatus::Success,
        data: Some(output),
        error: None,
    }
}
