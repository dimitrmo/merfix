include!(concat!(env!("OUT_DIR"), "/version.rs"));

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
        "jpg" | "jpeg" | "jfif" => Some(ImageFormat::Jpeg),
        "png" | "apng" => Some(ImageFormat::Png),
        "webp" => Some(ImageFormat::WebP),
        "tif" | "tiff" => Some(ImageFormat::Tiff),
        _ => None, // Unsupported format
    }
}

#[wasm_bindgen]
pub fn supported_mime_types() -> Vec<JsValue> {
    let metadata_types = vec![
        "image/jpeg",
        "image/png",
        "image/webp",
        "image/tiff", "image/tiff-fx"
    ];
    metadata_types.into_iter().map(JsValue::from).collect()
}

#[wasm_bindgen]
pub fn supported_extensions() -> Vec<JsValue> {
    let metadata_types = vec![
        "jpg", "jpeg",
        "png",
        "webp",
        "tiff"
    ];
    metadata_types.into_iter().map(JsValue::from).collect()
}

#[wasm_bindgen]
pub fn detect_image_mime_type(data: &[u8]) -> Option<String> {
    match image::guess_format(data) {
        Ok(format) => match format {
            image::ImageFormat::Jpeg => Some("image/jpeg".to_string()),  // JPEG & JFIF
            image::ImageFormat::Png => Some("image/png".to_string()),    // PNG
            image::ImageFormat::WebP => Some("image/webp".to_string()),  // WebP
            image::ImageFormat::Tiff => Some("image/tiff".to_string()),  // TIFF
            _ => None, // Other formats aren't included
        },
        Err(_) => None,
    }
}

#[wasm_bindgen]
pub fn detect_image_extension(data: &[u8]) -> Option<String> {
    match image::guess_format(data) {
        Ok(format) => match format {
            image::ImageFormat::Jpeg => Some("jpeg".to_string()),  // JPEG & JFIF
            image::ImageFormat::Png => Some("png".to_string()),    // PNG
            image::ImageFormat::WebP => Some("webp".to_string()),  // WebP            
            image::ImageFormat::Tiff => Some("tiff".to_string()),  // TIFF            
            _ => None, // Other formats aren't included
        },
        Err(_) => None,
    }
}

fn prepare_error(error_message: String) -> ExifRemovalResult {
    return ExifRemovalResult {
        status: ExifRemovalStatus::Error,
        data: None,
        error: Some(error_message),
    }
}

#[wasm_bindgen]
pub fn remove_exif(input: &[u8], extension: &str) -> ExifRemovalResult {
    let format = match get_image_format(extension) {
        Some(fmt) => fmt,
        None => return prepare_error(format!("Unsupported image format: {}", extension)),
    };

    let img = match ImageReader::new(
        std::io::Cursor::new(input)
    ).with_guessed_format() {
        Ok(reader) => match reader.decode() {
            Ok(decoded) => decoded,
            Err(err) => return prepare_error(format!("Failed to decode image: {}", err))
        },
        Err(err) => return prepare_error(format!("Failed to read image format: {}", err))
    };

    let mut output = Vec::new();
    if let Err(err) = img.write_to(&mut std::io::Cursor::new(&mut output), format) {
        return prepare_error(format!("Failed to write image: {}", err));
    }

    ExifRemovalResult {
        status: ExifRemovalStatus::Success,
        data: Some(output),
        error: None,
    }
}

#[wasm_bindgen]
pub fn version() -> String {
    VERSION.to_string()
}
