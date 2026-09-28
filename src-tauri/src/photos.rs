//! Lists JPEGs in the library folder with oriented dimensions.
//! The list command already returns the row shape a future index would.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use std::fs;

use image::metadata::Orientation;
use image::{ImageDecoder, ImageReader};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
    pub id: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub modified_ms: u64,
    #[serde(skip)]
    pub path: PathBuf,
}

/// Hardcoded for the POC.
pub fn library_dir() -> PathBuf {
    PathBuf::from(std::env::var("HOME").expect("HOME is set")).join("Desktop/GAL")
}

/// ponytail: identity is path + size + mtime, not a content hash.
/// Switch to hashing bytes if the same photo must dedupe across copies.
pub fn photo_id(path: &Path, len: u64, modified_ms: u64) -> String {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    path.hash(&mut h);
    len.hash(&mut h);
    modified_ms.hash(&mut h);
    format!("{:016x}", h.finish())
}

fn is_jpeg(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("jpg") || e.eq_ignore_ascii_case("jpeg"))
}

/// Reads only the JPEG header, no pixel decode. Swaps width/height for rotated EXIF orientations.
fn oriented_dims(path: &Path) -> image::ImageResult<(u32, u32)> {
    let mut decoder = ImageReader::open(path)?.with_guessed_format()?.into_decoder()?;
    let (w, h) = decoder.dimensions();
    let rotated = matches!(
        decoder.orientation().unwrap_or(Orientation::NoTransforms),
        Orientation::Rotate90 | Orientation::Rotate270 | Orientation::Rotate90FlipH | Orientation::Rotate270FlipH
    );
    Ok(if rotated { (h, w) } else { (w, h) })
}

type DimsCache = HashMap<String, (u32, u32)>;

/// Lists the folder (top level only), newest first. Dimensions are memoized in
/// `cache_file` keyed by photo id, so a changed file naturally misses the cache.
pub fn list(dir: &Path, cache_file: &Path) -> Result<Vec<Photo>, String> {
    let cache: DimsCache = fs::read(cache_file)
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default();
    let mut fresh = DimsCache::new();
    let mut photos = Vec::new();

    let entries = fs::read_dir(dir).map_err(|e| format!("cannot read {}: {e}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if !is_jpeg(&path) {
            continue;
        }
        let meta = match entry.metadata() {
            Ok(m) if m.is_file() => m,
            _ => continue,
        };
        let modified_ms = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        let id = photo_id(&path, meta.len(), modified_ms);
        let (width, height) = match cache.get(&id) {
            Some(&dims) => dims,
            None => match oriented_dims(&path) {
                Ok(dims) => dims,
                Err(e) => {
                    eprintln!("skipping {}: {e}", path.display());
                    continue;
                }
            },
        };
        fresh.insert(id.clone(), (width, height));
        photos.push(Photo {
            id,
            name: entry.file_name().to_string_lossy().into_owned(),
            width,
            height,
            modified_ms,
            path,
        });
    }

    if fresh != cache {
        if let Some(parent) = cache_file.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(bytes) = serde_json::to_vec(&fresh) {
            let _ = fs::write(cache_file, bytes);
        }
    }

    photos.sort_by(|a, b| b.modified_ms.cmp(&a.modified_ms).then_with(|| a.name.cmp(&b.name)));
    Ok(photos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_jpegs_and_reuses_the_dims_cache() {
        let dir = std::env::temp_dir().join(format!("gal-test-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        image::RgbImage::new(40, 30).save(dir.join("a.jpg")).unwrap();
        fs::write(dir.join("not-a-photo.txt"), b"x").unwrap();
        let cache = dir.join("cache/dims.json");

        let first = list(&dir, &cache).unwrap();
        assert_eq!(first.len(), 1);
        assert_eq!((first[0].width, first[0].height), (40, 30));
        assert!(cache.exists());

        let second = list(&dir, &cache).unwrap();
        assert_eq!(second[0].id, first[0].id);
        assert_eq!((second[0].width, second[0].height), (40, 30));

        fs::remove_dir_all(&dir).unwrap();
    }
}
