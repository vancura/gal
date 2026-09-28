//! Lazily generated, disk-cached thumbnails: `<cache>/thumbs/<id>.jpg`.

use std::fs::{self, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use image::codecs::jpeg::JpegEncoder;
use image::metadata::Orientation;
use image::{DynamicImage, ImageDecoder, ImageReader};

pub const THUMB_HEIGHT: u32 = 512;

/// Returns the cached thumbnail path, generating it on first request.
/// Written temp-then-rename, so a concurrent duplicate request is harmless.
pub fn ensure(src: &Path, thumbs_dir: &Path, id: &str) -> Result<PathBuf, String> {
    let out = thumbs_dir.join(format!("{id}.jpg"));

    if out.exists() {
        return Ok(out);
    }

    fs::create_dir_all(thumbs_dir).map_err(|e| e.to_string())?;

    // ponytail: full decode of the source JPEG per thumbnail. If a 100k backfill
    // is too slow, switch to turbojpeg scaled decode (1/4 or 1/8 DCT scaling).
    let mut decoder = ImageReader::open(src)
        .and_then(|r| r.with_guessed_format())
        .map_err(|e| e.to_string())?
        .into_decoder()
        .map_err(|e| e.to_string())?;
    let orientation = decoder.orientation().unwrap_or(Orientation::NoTransforms);
    let mut img = DynamicImage::from_decoder(decoder).map_err(|e| e.to_string())?;

    img.apply_orientation(orientation);

    let thumb = if img.height() > THUMB_HEIGHT {
        let width = (img.width() as f64 * THUMB_HEIGHT as f64 / img.height() as f64)
            .round()
            .max(1.0) as u32;
        img.thumbnail(width, THUMB_HEIGHT)
    } else {
        img
    };

    let tmp = thumbs_dir.join(format!(".{id}.tmp"));
    let mut file = BufWriter::new(File::create(&tmp).map_err(|e| e.to_string())?);

    thumb
        .to_rgb8()
        .write_with_encoder(JpegEncoder::new_with_quality(&mut file, 80))
        .map_err(|e| e.to_string())?;
    drop(file);
    fs::rename(&tmp, &out).map_err(|e| e.to_string())?;
    Ok(out)
}
