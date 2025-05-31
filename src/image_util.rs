use std::io::Cursor;

use image::{io::Reader as ImageReader, DynamicImage, ImageOutputFormat};

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Png,
    Jpeg(u8), // quality 0-100
    Webp(f32), // quality 0.0-100.0
}

impl OutputFormat {
    pub fn from_string(format_str: &str, quality: Option<u8>) -> Result<Self, String> {
        match format_str.to_lowercase().as_str() {
            "png" => Ok(OutputFormat::Png),
            "jpeg" | "jpg" => {
                let quality = quality.unwrap_or(85); // Default quality
                if quality > 100 {
                    return Err("JPEG quality must be between 0 and 100".to_string());
                }
                Ok(OutputFormat::Jpeg(quality))
            }
            "webp" => {
                let quality = quality.unwrap_or(80) as f32; // Default quality for WebP
                if quality > 100.0 {
                    return Err("WebP quality must be between 0 and 100".to_string());
                }
                Ok(OutputFormat::Webp(quality))
            }
            _ => Err(format!("Unsupported format: {}", format_str)),
        }
    }

    pub fn content_type(&self) -> &'static str {
        match self {
            OutputFormat::Png => "image/png",
            OutputFormat::Jpeg(_) => "image/jpeg",
            OutputFormat::Webp(_) => "image/webp",
        }
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            OutputFormat::Png => "png",
            OutputFormat::Jpeg(_) => "jpg",
            OutputFormat::Webp(_) => "webp",
        }
    }
}

pub fn read_image(bytes: Vec<u8>) -> DynamicImage {
    let reader = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .expect("This will never fail using Cursor");

    let img = reader.decode().expect("should decoded");

    img
}

pub fn write_image(image: DynamicImage, format: OutputFormat) -> Result<Vec<u8>, String> {
    let outbuf = vec![];
    let mut cursor = Cursor::new(outbuf);

    let image_format = match format {
        OutputFormat::Png => ImageOutputFormat::Png,
        OutputFormat::Jpeg(quality) => ImageOutputFormat::Jpeg(quality),
        OutputFormat::Webp(_quality) => {
            // Note: The image crate's WebP encoder doesn't currently support quality settings
            // through ImageOutputFormat::WebP. The quality parameter is accepted for API
            // consistency but WebP will use default encoding settings.
            ImageOutputFormat::WebP
        }
    };

    image
        .write_to(&mut cursor, image_format)
        .map_err(|e| format!("Failed to encode image: {}", e))?;

    let result = cursor.get_ref().to_vec();

    Ok(result)
}
