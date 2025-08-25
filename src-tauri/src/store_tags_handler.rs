use crate::models::{StoreTagsResult, FileTagComparison, TagDifference};
use crate::database::Database;
use tauri::AppHandle;
use id3::{Tag, TagLike, Frame, Content};
use std::path::Path;
use std::time::Instant;
use log::{info, warn, error};

/// Store all database metadata and RPG tags into the actual audio files
pub async fn store_all_tags_in_files(_app_handle: AppHandle) -> Result<StoreTagsResult, String> {
    let start_time = Instant::now();
    info!("Starting store tags in files operation");

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
    info!("Processing {} audio files", result.total_files);

    for audio_file in audio_files {
        match process_single_file(&db, &audio_file, &mut result) {
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
        "Store tags operation completed in {:.2}s: {} total, {} updated, {} skipped, {} failed",
        result.duration_seconds,
        result.total_files,
        result.updated_files,
        result.skipped_files,
        result.failed_files
    );

    Ok(result)
}

/// Process a single audio file - compare current tags with database and update if needed
fn process_single_file(
    db: &Database,
    audio_file: &crate::models::AudioFile,
    _result: &mut StoreTagsResult,
) -> Result<bool, String> {
    let file_path = &audio_file.file_path;
    
    // Check if file exists and is readable
    if !Path::new(file_path).exists() {
        return Err("File does not exist".to_string());
    }

    // Read current tags from file
    let current_tag = match Tag::read_from_path(file_path) {
        Ok(tag) => Some(tag),
        Err(_) => {
            // File might not have tags yet, create new tag
            None
        }
    };

    // Get RPG tags for this file from database
    let rpg_tags = if let Some(audio_file_id) = audio_file.id {
        match db.get_rpg_tags_for_file(audio_file_id) {
            Ok(tags) => tags,
            Err(e) => {
                warn!("Failed to get RPG tags for file {}: {}", file_path, e);
                Vec::new()
            }
        }
    } else {
        Vec::new()
    };

    // Compare current file tags with database values
    let comparison = compare_file_tags_with_database(&current_tag, audio_file, &rpg_tags);
    
    if !comparison.needs_update {
        return Ok(false); // No update needed
    }

    // Create new tag with all database metadata
    let mut new_tag = current_tag.unwrap_or_else(Tag::new);
    
    // Write all metadata fields to the tag
    write_metadata_to_tag(&mut new_tag, audio_file, &rpg_tags)?;
    
    // Write the updated tag back to the file
    match new_tag.write_to_path(file_path, id3::Version::Id3v24) {
        Ok(_) => {
            info!("Successfully updated tags for: {}", file_path);
            Ok(true)
        }
        Err(e) => {
            error!("Failed to write tags to {}: {}", file_path, e);
            Err(format!("Failed to write tags: {}", e))
        }
    }
}

/// Compare current file tags with database values to determine what needs updating
fn compare_file_tags_with_database(
    current_tag: &Option<Tag>,
    audio_file: &crate::models::AudioFile,
    rpg_tags: &[crate::models::RpgTag],
) -> FileTagComparison {
    let mut comparison = FileTagComparison {
        file_path: audio_file.file_path.clone(),
        needs_update: false,
        missing_tags: Vec::new(),
        different_values: Vec::new(),
    };

    if let Some(tag) = current_tag {
        // Compare standard metadata fields
        compare_standard_fields(tag, audio_file, &mut comparison);
        
        // Compare RPG tags
        compare_rpg_tags(tag, rpg_tags, &mut comparison);
    } else {
        // No existing tag, so we need to create everything
        comparison.needs_update = true;
        comparison.missing_tags.push("All metadata (no existing tag)".to_string());
    }

    comparison
}

/// Compare standard ID3 metadata fields
fn compare_standard_fields(
    tag: &Tag,
    audio_file: &crate::models::AudioFile,
    comparison: &mut FileTagComparison,
) {
    // Title
    if let Some(ref title) = audio_file.title {
        if tag.title().map(|t| t.to_string()) != Some(title.clone()) {
            comparison.different_values.push(TagDifference {
                field_name: "title".to_string(),
                current_value: tag.title().unwrap_or("").to_string(),
                new_value: title.clone(),
            });
            comparison.needs_update = true;
        }
    }

    // Artist
    if let Some(ref artist) = audio_file.artist {
        if tag.artist().map(|a| a.to_string()) != Some(artist.clone()) {
            comparison.different_values.push(TagDifference {
                field_name: "artist".to_string(),
                current_value: tag.artist().unwrap_or("").to_string(),
                new_value: artist.clone(),
            });
            comparison.needs_update = true;
        }
    }

    // Album
    if let Some(ref album) = audio_file.album {
        if tag.album().map(|a| a.to_string()) != Some(album.clone()) {
            comparison.different_values.push(TagDifference {
                field_name: "album".to_string(),
                current_value: tag.album().unwrap_or("").to_string(),
                new_value: album.clone(),
            });
            comparison.needs_update = true;
        }
    }

    // Genre
    if let Some(ref genre) = audio_file.genre {
        if tag.genre().map(|g| g.to_string()) != Some(genre.clone()) {
            comparison.different_values.push(TagDifference {
                field_name: "genre".to_string(),
                current_value: tag.genre().unwrap_or("").to_string(),
                new_value: genre.clone(),
            });
            comparison.needs_update = true;
        }
    }

    // BPM
    if let Some(bpm) = audio_file.bpm {
        let bpm_u32 = bpm as u32;
        if tag.get("TBPM").and_then(|frame| {
            if let Content::Text(text) = &frame.content() {
                text.parse::<u32>().ok()
            } else {
                None
            }
        }) != Some(bpm_u32) {
            comparison.different_values.push(TagDifference {
                field_name: "bpm".to_string(),
                current_value: tag.get("TBPM").map(|f| format!("{:?}", f.content())).unwrap_or_default(),
                new_value: bpm_u32.to_string(),
            });
            comparison.needs_update = true;
        }
    }

    // Add more field comparisons as needed...
}

/// Compare RPG tags stored in TXXX fields
fn compare_rpg_tags(
    tag: &Tag,
    rpg_tags: &[crate::models::RpgTag],
    comparison: &mut FileTagComparison,
) {
    // Group RPG tags by type
    let mut genre_tags = Vec::new();
    let mut mood_tags = Vec::new();
    let mut occasion_tags = Vec::new();
    let mut keyword_tags = Vec::new();

    for rpg_tag in rpg_tags {
        match rpg_tag.tag_type.as_str() {
            "genre" => genre_tags.push(rpg_tag.tag_value.clone()),
            "mood" => mood_tags.push(rpg_tag.tag_value.clone()),
            "occasion" => occasion_tags.push(rpg_tag.tag_value.clone()),
            "keywords" => keyword_tags.push(rpg_tag.tag_value.clone()),
            _ => {}
        }
    }

    // Compare each RPG tag type
    compare_txxx_field(tag, "RPG_GENRE", &genre_tags.join(";"), comparison);
    compare_txxx_field(tag, "RPG_MOOD", &mood_tags.join(";"), comparison);
    compare_txxx_field(tag, "RPG_OCCASION", &occasion_tags.join(";"), comparison);
    compare_txxx_field(tag, "RPG_KEYWORDS", &keyword_tags.join(";"), comparison);
}

/// Compare a TXXX (user-defined text) field
fn compare_txxx_field(
    tag: &Tag,
    field_name: &str,
    expected_value: &str,
    comparison: &mut FileTagComparison,
) {
    let current_value = get_txxx_value(tag, field_name).unwrap_or_default();
    
    if current_value != expected_value && !expected_value.is_empty() {
        comparison.different_values.push(TagDifference {
            field_name: format!("TXXX:{}", field_name),
            current_value,
            new_value: expected_value.to_string(),
        });
        comparison.needs_update = true;
    }
}

/// Get a TXXX field value from tag
fn get_txxx_value(tag: &Tag, description: &str) -> Option<String> {
    for frame in tag.frames() {
        if let Some(extended_text) = frame.content().extended_text() {
            if extended_text.description == description {
                return Some(extended_text.value.clone());
            }
        }
    }
    None
}

/// Write all metadata and RPG tags to the ID3 tag
fn write_metadata_to_tag(
    tag: &mut Tag,
    audio_file: &crate::models::AudioFile,
    rpg_tags: &[crate::models::RpgTag],
) -> Result<(), String> {
    // Write standard metadata fields
    if let Some(ref title) = audio_file.title {
        tag.set_title(title);
    }
    
    if let Some(ref artist) = audio_file.artist {
        tag.set_artist(artist);
    }
    
    if let Some(ref album) = audio_file.album {
        tag.set_album(album);
    }
    
    if let Some(ref genre) = audio_file.genre {
        tag.set_genre(genre);
    }

    if let Some(year) = audio_file.year {
        tag.set_year(year as i32);
    }

    if let Some(track) = audio_file.track_number {
        tag.set_track(track as u32);
    }

    if let Some(ref album_artist) = audio_file.album_artist {
        tag.set_album_artist(album_artist);
    }

    // Write extended metadata
    if let Some(ref composer) = audio_file.composer {
        set_text_frame(tag, "TCOM", composer);
    }

    if let Some(ref conductor) = audio_file.conductor {
        set_text_frame(tag, "TPE3", conductor);
    }

    if let Some(ref producer) = audio_file.producer {
        set_text_frame(tag, "TPRO", producer);
    }

    if let Some(ref publisher) = audio_file.publisher {
        set_text_frame(tag, "TPUB", publisher);
    }

    if let Some(ref copyright) = audio_file.copyright {
        set_text_frame(tag, "TCOP", copyright);
    }

    if let Some(ref language) = audio_file.language {
        set_text_frame(tag, "TLAN", language);
    }

    if let Some(ref initial_key) = audio_file.initial_key {
        set_text_frame(tag, "TKEY", initial_key);
    }

    if let Some(bpm) = audio_file.bpm {
        set_text_frame(tag, "TBPM", &bpm.to_string());
    }

    if let Some(duration) = audio_file.duration {
        let duration_ms = (duration * 1000.0) as u32;
        set_text_frame(tag, "TLEN", &duration_ms.to_string());
    }

    if let Some(ref encoding_settings) = audio_file.encoding_settings {
        set_text_frame(tag, "TSSE", encoding_settings);
    }

    if let Some(ref encoded_by) = audio_file.encoded_by {
        set_text_frame(tag, "TENC", encoded_by);
    }

    // Write RPG tags as TXXX fields
    write_rpg_tags_to_txxx(tag, rpg_tags)?;

    // Add Ligeia-specific metadata
    set_txxx_frame(tag, "LIGEIA_VERSION", "1.0")?;
    set_txxx_frame(tag, "LIGEIA_TIMESTAMP", &chrono::Utc::now().to_rfc3339())?;
    if let Some(id) = audio_file.id {
        set_txxx_frame(tag, "LIGEIA_DATABASE_ID", &id.to_string())?;
    }
    set_txxx_frame(tag, "ORIGINAL_PATH", &audio_file.file_path)?;

    Ok(())
}

/// Write RPG tags as TXXX user-defined text frames
fn write_rpg_tags_to_txxx(tag: &mut Tag, rpg_tags: &[crate::models::RpgTag]) -> Result<(), String> {
    // Group tags by type
    let mut genre_tags = Vec::new();
    let mut mood_tags = Vec::new();
    let mut occasion_tags = Vec::new();
    let mut keyword_tags = Vec::new();
    let mut all_tags = Vec::new();

    for rpg_tag in rpg_tags {
        all_tags.push(rpg_tag.tag_value.clone());
        
        match rpg_tag.tag_type.as_str() {
            "genre" => genre_tags.push(rpg_tag.tag_value.clone()),
            "mood" => mood_tags.push(rpg_tag.tag_value.clone()),
            "occasion" => occasion_tags.push(rpg_tag.tag_value.clone()),
            "keywords" => keyword_tags.push(rpg_tag.tag_value.clone()),
            _ => {}
        }
    }

    // Write grouped RPG tags
    if !genre_tags.is_empty() {
        set_txxx_frame(tag, "RPG_GENRE", &genre_tags.join(";"))?;
    }
    
    if !mood_tags.is_empty() {
        set_txxx_frame(tag, "RPG_MOOD", &mood_tags.join(";"))?;
    }
    
    if !occasion_tags.is_empty() {
        set_txxx_frame(tag, "RPG_OCCASION", &occasion_tags.join(";"))?;
    }
    
    if !keyword_tags.is_empty() {
        set_txxx_frame(tag, "RPG_KEYWORDS", &keyword_tags.join(";"))?;
    }
    
    // Write all tags combined for maximum compatibility
    if !all_tags.is_empty() {
        set_txxx_frame(tag, "RPG_ALL_TAGS", &all_tags.join(";"))?;
    }

    Ok(())
}

/// Set a text frame in the tag
fn set_text_frame(tag: &mut Tag, frame_id: &str, value: &str) {
    let frame = Frame::with_content(frame_id, Content::Text(value.to_string()));
    tag.add_frame(frame);
}

/// Set a TXXX (user-defined text) frame
fn set_txxx_frame(tag: &mut Tag, description: &str, value: &str) -> Result<(), String> {
    let content = Content::ExtendedText(id3::frame::ExtendedText {
        description: description.to_string(),
        value: value.to_string(),
    });
    
    let frame = Frame::with_content("TXXX", content);
    tag.add_frame(frame);
    Ok(())
}