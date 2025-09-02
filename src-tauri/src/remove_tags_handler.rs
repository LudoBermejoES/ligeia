use crate::models::{StoreTagsResult};
use crate::database::Database;
use tauri::AppHandle;
use id3::{Tag, TagLike};
use std::path::Path;
use std::time::Instant;
use log::{info, warn, error};

/// Remove all RPG tags and metadata from actual audio files
pub async fn remove_all_tags_from_files(_app_handle: AppHandle) -> Result<StoreTagsResult, String> {
    let start_time = Instant::now();
    info!("Starting remove tags from files operation");

    let mut result = StoreTagsResult {
        total_files: 0,
        updated_files: 0,
        skipped_files: 0,
        failed_files: 0,
        errors: Vec::new(),
        duration_seconds: 0.0,
    };

    // Get database instance
    let db = match Database::new() {
        Ok(db) => db,
        Err(e) => {
            error!("Failed to create database connection: {}", e);
            return Err(format!("Failed to create database connection: {}", e));
        }
    };

    // Get all audio files with metadata
    let audio_files = match db.get_all_audio_files() {
        Ok(files) => files,
        Err(e) => {
            error!("Failed to get audio files from database: {}", e);
            return Err(format!("Failed to get audio files: {}", e));
        }
    };

    result.total_files = audio_files.len();
    info!("Processing {} audio files for tag removal", result.total_files);

    for audio_file in audio_files {
        match process_single_file_removal(&audio_file, &mut result) {
            Ok(updated) => {
                if updated {
                    result.updated_files += 1;
                } else {
                    result.skipped_files += 1;
                }
            }
            Err(e) => {
                result.failed_files += 1;
                result.errors.push(format!("{}: {}", audio_file.file_path, e));
                warn!("Failed to process file {}: {}", audio_file.file_path, e);
            }
        }
    }

    result.duration_seconds = start_time.elapsed().as_secs_f64();
    
    info!(
        "Remove tags operation completed in {:.2}s: {} total, {} updated, {} skipped, {} failed",
        result.duration_seconds,
        result.total_files,
        result.updated_files,
        result.skipped_files,
        result.failed_files
    );

    Ok(result)
}

/// Process a single audio file - remove all tags from the actual file
fn process_single_file_removal(
    audio_file: &crate::models::AudioFile,
    _result: &mut StoreTagsResult,
) -> Result<bool, String> {
    let file_path = &audio_file.file_path;
    
    // Check if file exists and is readable
    if !Path::new(file_path).exists() {
        return Err("File does not exist".to_string());
    }

    // Check if file is writable
    let metadata = std::fs::metadata(file_path)
        .map_err(|e| format!("Cannot access file metadata: {}", e))?;
    
    if metadata.permissions().readonly() {
        return Err("File is read-only".to_string());
    }

    // Read current tags from file
    let current_tag = match Tag::read_from_path(file_path) {
        Ok(tag) => tag,
        Err(_) => {
            // File has no tags, nothing to remove
            return Ok(false);
        }
    };

    // Check if there are any tags to remove
    let has_tags = has_removable_tags(&current_tag);
    
    if !has_tags {
        // No tags to remove, skip this file
        return Ok(false);
    }

    // Create a new empty tag to replace the current one
    let mut new_tag = Tag::new();
    
    // Preserve only basic file identification info (keep title if it's the filename)
    if let Some(title) = current_tag.title() {
        let filename_without_ext = Path::new(file_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
            
        // Only preserve title if it's different from filename (user-set title)
        if title != filename_without_ext && !title.trim().is_empty() {
            new_tag.set_title(title);
        }
    }

    // Write the cleaned tag back to the file
    match new_tag.write_to_path(file_path, id3::Version::Id3v24) {
        Ok(_) => {
            info!("Removed tags from: {}", file_path);
            Ok(true)
        }
        Err(e) => {
            error!("Failed to write cleaned tags to {}: {}", file_path, e);
            Err(format!("Failed to write to file: {}", e))
        }
    }
}

/// Check if the tag has removable content (RPG tags, metadata, etc.)
fn has_removable_tags(tag: &Tag) -> bool {
    // Check for standard metadata fields that we want to remove
    tag.artist().is_some() ||
    tag.album().is_some() ||
    tag.year().is_some() ||
    tag.genre().is_some() ||
    tag.comments().next().is_some() ||
    
    // Check for custom frames (TXXX frames that contain RPG tags)
    tag.frames().any(|frame| {
        match frame.content() {
            id3::Content::ExtendedText(ext_text) => {
                // Remove custom frames like Mood, Occasion, Keywords, etc.
                matches!(ext_text.description.as_str(), 
                    "Mood" | "Occasion" | "Keywords" | "RPG_Genre" | 
                    "RPG_Mood" | "RPG_Occasion" | "RPG_Keywords" | "Quality" |
                    "Ligeia_ID" | "Ligeia_Version" | "BPM_Detected" | 
                    "Duration_Seconds" | "Encoding_Info"
                )
            }
            _ => false
        }
    }) ||
    
    // Check for other metadata we might want to remove
    tag.frames().any(|frame| {
        matches!(frame.id(), 
            // Standard ID3 frames we want to remove
            "TPE1" | // Artist
            "TALB" | // Album  
            "TDRC" | // Recording time
            "TCON" | // Genre
            "TBPM" | // BPM
            "TPOS" | // Part of set
            "TRCK" | // Track number
            "TMOO" | // Mood
            "COMM"   // Comments
        )
    })
}