use crate::capture::content::{compute_hash, detect_url};
use crate::storage::database::Database;
use crate::storage::models::{CaptureEvent, CapturedContent, ContentType};
use chrono::Utc;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tauri::State;

/// The application data directory name for storing captured images.
const APP_DATA_DIR: &str = "com.xiaoyun.app";
const CAPTURES_SUBDIR: &str = "captures";
const THUMBNAILS_SUBDIR: &str = "thumbnails";
const THUMBNAIL_WIDTH: u32 = 200;

pub struct AppState {
    pub db: Arc<Database>,
}

/// Get the captures directory, creating it if necessary.
fn get_captures_dir() -> Result<PathBuf, String> {
    let base = dirs::data_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("Library").join("Application Support")))
        .ok_or_else(|| "Cannot determine application data directory".to_string())?;

    let captures_dir = base.join(APP_DATA_DIR).join(CAPTURES_SUBDIR);
    std::fs::create_dir_all(&captures_dir)
        .map_err(|e| format!("Failed to create captures directory: {}", e))?;

    Ok(captures_dir)
}

/// Get the thumbnails directory, creating it if necessary.
fn get_thumbnails_dir() -> Result<PathBuf, String> {
    let base = dirs::data_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("Library").join("Application Support")))
        .ok_or_else(|| "Cannot determine application data directory".to_string())?;

    let thumbnails_dir = base.join(APP_DATA_DIR).join(THUMBNAILS_SUBDIR);
    std::fs::create_dir_all(&thumbnails_dir)
        .map_err(|e| format!("Failed to create thumbnails directory: {}", e))?;

    Ok(thumbnails_dir)
}

/// Copy a source image to the captures directory and return the new path.
fn copy_image_to_captures(source_path: &str, id: &str) -> Result<String, String> {
    let source = Path::new(source_path);
    if !source.exists() {
        return Err(format!("Source image does not exist: {}", source_path));
    }

    let extension = source
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_else(|| "png".to_string());

    let captures_dir = get_captures_dir()?;
    let dest_filename = format!("{}.{}", id, extension);
    let dest_path = captures_dir.join(&dest_filename);

    std::fs::copy(source, &dest_path)
        .map_err(|e| format!("Failed to copy image to captures: {}", e))?;

    let dest_str = dest_path.to_string_lossy().to_string();
    log::info!("Image copied to captures: {}", dest_str);
    Ok(dest_str)
}

/// Generate a thumbnail (200px wide, preserving aspect ratio) and save it.
/// Returns the thumbnail path if successful.
fn generate_thumbnail(source_path: &str, id: &str) -> Result<String, String> {
    let img = image::open(source_path)
        .map_err(|e| format!("Failed to open image for thumbnail: {}", e))?;

    let (orig_width, orig_height) = (img.width(), img.height());
    if orig_width == 0 || orig_height == 0 {
        return Err("Image has zero dimensions".to_string());
    }

    // Calculate new height preserving aspect ratio
    let new_width = THUMBNAIL_WIDTH.min(orig_width);
    let new_height = (orig_height as f64 * new_width as f64 / orig_width as f64) as u32;

    let thumbnail = img.thumbnail(new_width, new_height);

    let thumbnails_dir = get_thumbnails_dir()?;
    let thumb_filename = format!("{}_thumb.png", id);
    let thumb_path = thumbnails_dir.join(&thumb_filename);

    thumbnail
        .save(&thumb_path)
        .map_err(|e| format!("Failed to save thumbnail: {}", e))?;

    let thumb_str = thumb_path.to_string_lossy().to_string();
    log::info!(
        "Thumbnail generated: {} ({}x{} -> {}x{})",
        thumb_str,
        orig_width,
        orig_height,
        new_width,
        new_height
    );
    Ok(thumb_str)
}

/// Internal auto-save function called directly from CaptureDetector.
/// Does not require Tauri State — takes a Database reference directly.
pub fn save_content_auto(db: &Arc<Database>, event: CaptureEvent) -> Result<CapturedContent, String> {
    let now = Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();

    // Detect content type and extract URL if applicable
    let (content_type, raw_text, image_path, detected_url) = match event.content_type.as_str() {
        "image" => (ContentType::Image, None, event.image_path, None),
        "url" => {
            let url = event.raw_text.as_deref().and_then(detect_url);
            (ContentType::Url, event.raw_text.clone(), None, url)
        }
        _ => {
            if let Some(ref text) = event.raw_text {
                if let Some(url) = detect_url(text) {
                    (ContentType::Url, event.raw_text.clone(), None, Some(url))
                } else {
                    (ContentType::Text, event.raw_text.clone(), None, None)
                }
            } else {
                (ContentType::Text, None, None, None)
            }
        }
    };

    let (final_image_path, thumbnail_path) = if content_type.as_str() == "image" {
        if let Some(ref src_path) = image_path {
            let copied_path = match copy_image_to_captures(src_path, &id) {
                Ok(p) => Some(p),
                Err(e) => {
                    log::error!("Failed to copy image: {}", e);
                    image_path.clone()
                }
            };

            let thumb_source = copied_path.as_deref().unwrap_or(src_path.as_str());
            let thumb_path = match generate_thumbnail(thumb_source, &id) {
                Ok(p) => Some(p),
                Err(e) => {
                    log::error!("Failed to generate thumbnail: {}", e);
                    None
                }
            };

            (copied_path, thumb_path)
        } else {
            (None, None)
        }
    } else {
        (image_path, None)
    };

    // For hash computation, use detected_url (trimmed) for URL content to ensure consistent dedup
    let hash_data = if let Some(ref path) = final_image_path {
        let file_size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        format!("img:{}:{}", path, file_size)
    } else if let Some(ref url) = detected_url {
        url.clone()
    } else {
        raw_text.as_deref().unwrap_or("").to_string()
    };
    let content_hash = compute_hash(hash_data.as_bytes());

    let byte_size = if let Some(ref path) = final_image_path {
        std::fs::metadata(path).map(|m| m.len() as i64).unwrap_or(0)
    } else {
        raw_text.as_ref().map(|t| t.len() as i64).unwrap_or(0)
    };

    // For URL content, use the clean detected URL (trimmed) as source_url
    let source_url = detected_url.clone();

    // Check for duplicate content using content_hash before saving
    let repo = crate::storage::repository::Repository::new(db.clone());
    if repo
        .content_exists_by_hash(&content_hash)
        .unwrap_or(false)
    {
        log::info!(
            "Duplicate content detected (hash={}), skipping save",
            &content_hash[..16]
        );
        return Err("Duplicate content".to_string());
    }

    let content = CapturedContent {
        id: id.clone(),
        content_type,
        raw_text,
        image_path: final_image_path,
        thumbnail_path,
        source_app: event.source_app,
        source_bundle_id: None,
        source_url,
        captured_at: now.clone(),
        content_hash,
        byte_size,
        is_deleted: false,
        created_at: now.clone(),
        updated_at: now,
    };

    repo.save_content(&content).map_err(|e| e.to_string())?;

    log::info!(
        "Content auto-saved: {} (type={}, size={} bytes)",
        id,
        content.content_type.as_str(),
        content.byte_size
    );

    Ok(content)
}

#[tauri::command]
pub fn save_captured_content(
    state: State<'_, AppState>,
    event: CaptureEvent,
) -> Result<CapturedContent, String> {
    save_content_auto(&state.db, event)
}
