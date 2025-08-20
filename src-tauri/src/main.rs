#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;
use tauri::{AppHandle, Manager};


mod models;
mod database;
mod audio_handler;
mod tag_manager;
mod file_scanner;

use models::*;
use database::Database;
use audio_handler::AudioHandler;
use tag_manager::TagManager;
use file_scanner::FileScanner;

struct AppState {
    db: Mutex<Database>,
    tag_manager: TagManager,
}

#[tauri::command]
async fn load_audio_file(file_path: String) -> Result<AudioFile, String> {
    AudioHandler::load_audio_file_metadata(&file_path)
}

#[tauri::command]
async fn save_audio_file(app_handle: AppHandle, audio_file: AudioFile) -> Result<i64, String> {
    let state = app_handle.state::<AppState>();
    let db = state.db.lock().unwrap();
    db.save_audio_file(&audio_file).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_all_audio_files(app_handle: AppHandle) -> Result<Vec<AudioFile>, String> {
    // Test unified logging system
    log::info!("Frontend requested all audio files - unified logging test successful!");
    
    let state = app_handle.state::<AppState>();
    let db = state.db.lock().unwrap();
    db.get_all_audio_files().map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_audio_file_tags(file_path: String, updates: AudioFile) -> Result<(), String> {
    AudioHandler::update_audio_file_tags(&file_path, &updates)
}

#[tauri::command]
async fn write_rpg_tags_to_file(app_handle: AppHandle, file_path: String) -> Result<(), String> {
    let state = app_handle.state::<AppState>();
    
    // Get the audio file from database to get its ID
    let db = state.db.lock().unwrap();
    let audio_files = db.get_all_audio_files().map_err(|e| e.to_string())?;
    let audio_file = audio_files.iter().find(|f| f.file_path == file_path)
        .ok_or_else(|| "Audio file not found in database".to_string())?;
    
    if let Some(audio_file_id) = audio_file.id {
        // Get all RPG tags for this file
        let rpg_tags = state.tag_manager.get_rpg_tags_for_file(audio_file_id)
            .map_err(|e| e.to_string())?;
        
        // Group tags by type
        let mut tag_groups: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        for tag in rpg_tags {
            tag_groups.entry(tag.tag_type).or_default().push(tag.tag_value);
        }
        
        // Convert to the format expected by write_rpg_tags_to_file
        let rpg_tag_tuples: Vec<(String, Vec<String>)> = tag_groups.into_iter().collect();
        
        // Write RPG tags to the actual audio file
        AudioHandler::write_rpg_tags_to_file(&file_path, &rpg_tag_tuples)?;
        
        log::info!("Successfully wrote RPG tags to file: {}, tag_count: {}", file_path, rpg_tag_tuples.len());
    }
    
    Ok(())
}

#[tauri::command]
async fn scan_directory_recursive(dir_path: String) -> Result<Vec<String>, String> {
    FileScanner::scan_directory_recursive(&dir_path)
}

#[tauri::command]
async fn delete_audio_file(app_handle: AppHandle, id: i64) -> Result<(), String> {
    let state = app_handle.state::<AppState>();
    let db = state.db.lock().unwrap();
    db.delete_audio_file(id).map_err(|e| e.to_string())
}

// New RPG Tag Commands
#[tauri::command]
async fn get_tag_vocabulary(app_handle: AppHandle, tag_type: Option<String>) -> Result<Vec<TagVocabulary>, String> {
    let state = app_handle.state::<AppState>();
    state.tag_manager.get_tag_vocabulary(tag_type.as_deref())
}

#[tauri::command]
async fn add_rpg_tag(app_handle: AppHandle, audio_file_id: i64, tag_type: String, tag_value: String) -> Result<i64, String> {
    let state = app_handle.state::<AppState>();
    state.tag_manager.add_rpg_tag(audio_file_id, &tag_type, &tag_value)
}

#[tauri::command]
async fn remove_rpg_tag(app_handle: AppHandle, audio_file_id: i64, tag_type: String, tag_value: String) -> Result<(), String> {
    let state = app_handle.state::<AppState>();
    state.tag_manager.remove_rpg_tag(audio_file_id, &tag_type, &tag_value)
}

#[tauri::command]
async fn get_rpg_tags_for_file(app_handle: AppHandle, audio_file_id: i64) -> Result<Vec<RpgTag>, String> {
    let state = app_handle.state::<AppState>();
    state.tag_manager.get_rpg_tags_for_file(audio_file_id)
}

#[tauri::command]
async fn bulk_tag_files(app_handle: AppHandle, request: BulkTagRequest) -> Result<(), String> {
    let state = app_handle.state::<AppState>();
    state.tag_manager.bulk_tag_files(request)
}

#[tauri::command]
async fn search_files_by_tags(app_handle: AppHandle, request: TagSearchRequest) -> Result<Vec<AudioFileWithTags>, String> {
    let state = app_handle.state::<AppState>();
    state.tag_manager.search_files_by_tags(request)
}

#[tauri::command]
async fn get_all_audio_files_with_tags(app_handle: AppHandle) -> Result<Vec<AudioFileWithTags>, String> {
    let state = app_handle.state::<AppState>();
    state.tag_manager.get_all_audio_files_with_tags()
}

#[tauri::command]
async fn get_tag_statistics(app_handle: AppHandle) -> Result<tag_manager::TagStatistics, String> {
    let state = app_handle.state::<AppState>();
    state.tag_manager.get_tag_statistics()
}

#[tauri::command]
async fn export_library_data(app_handle: AppHandle) -> Result<ExportData, String> {
    let state = app_handle.state::<AppState>();
    let _db = state.db.lock().unwrap();
    
    log::info!("Starting library export");
    
    // Get all audio files with their RPG tags
    let tag_results = state.tag_manager.get_all_audio_files_with_tags().map_err(|e| e.to_string())?;
    
    log::info!("Retrieved files with tags, file_count: {}", tag_results.len());
    
    // Convert to enhanced export format
    let mut export_files = Vec::new();
    
    for result in tag_results {
        let af = result.audio_file;
        
        // Group RPG tags by type for this file
        let mut occasions = Vec::new();
        let mut keywords = Vec::new();
        let mut genres = Vec::new();
        let mut moods = Vec::new();
        let mut quality = None;
        
        for tag in result.rpg_tags {
            match tag.tag_type.as_str() {
                "occasion" => occasions.push(tag.tag_value),
                "keyword" => keywords.push(tag.tag_value),
                "genre" => genres.push(tag.tag_value),
                "mood" => moods.push(tag.tag_value),
                "quality" => quality = Some(tag.tag_value),
                _ => {} // Skip other tag types
            }
        }
        
        // Create comma-separated strings for genre and mood (as per import format)
        let genre_string = if !genres.is_empty() {
            Some(genres.join(", "))
        } else {
            af.genre
        };
        
        let mood_string = if !moods.is_empty() {
            Some(moods.join(", "))
        } else {
            af.mood
        };
        
        let export_file = ExportAudioFile {
            id: af.id,
            file_path: af.file_path,
            title: af.title,
            artist: af.artist,
            album: af.album,
            genre: genre_string,
            year: af.year,
            duration: af.duration,
            album_artist: af.album_artist,
            track_number: af.track_number,
            bpm: af.bpm,
            initial_key: af.initial_key,
            mood: mood_string,
            language: af.language,
            rpg_occasion: if occasions.is_empty() { None } else { Some(occasions) },
            rpg_keywords: if keywords.is_empty() { None } else { Some(keywords) },
            rpg_quality: quality,
        };
        
        export_files.push(export_file);
    }
    
    // Get tag vocabulary for enhanced export
    let tag_vocabulary = match state.tag_manager.get_tag_vocabulary(None) {
        Ok(vocab) => {
            let mut genres = Vec::new();
            let mut moods = Vec::new();
            let mut occasions = Vec::new();
            let mut keywords = Vec::new();
            
            for tag in vocab {
                match tag.tag_type.as_str() {
                    "genre" => genres.push(tag.tag_value),
                    "mood" => moods.push(tag.tag_value),
                    "occasion" => occasions.push(tag.tag_value),
                    "keyword" => keywords.push(tag.tag_value),
                    _ => {}
                }
            }
            
            Some(serde_json::json!({
                "version": "1.0",
                "description": "RPG Audio Tagging Spec — Deep Taxonomy based on TAGS.md",
                "last_updated": chrono::Utc::now().to_rfc3339(),
                "genres": genres,
                "moods": moods,
                "occasions": occasions,
                "keywords": keywords
            }))
        },
        Err(e) => {
            log::error!("Failed to get tag vocabulary, error: {}", e.to_string());
            None
        }
    };
    
    log::info!("Export completed successfully, files_exported: {}, has_vocabulary: {}", export_files.len(), tag_vocabulary.is_some());
    
    Ok(ExportData {
        version: 1,
        files: export_files,
        tags: Vec::new(), // Empty for enhanced format
        tag_vocabulary,
    })
}

#[tauri::command]
async fn import_library_data(app_handle: AppHandle, data: ExportData) -> Result<(), String> {
    let state = app_handle.state::<AppState>();
    let db = state.db.lock().unwrap();
    
    log::info!("Starting library import, version: {}, files_count: {}, tags_count: {}, has_vocabulary: {}", data.version, data.files.len(), data.tags.len(), data.tag_vocabulary.is_some());
    
    // Clear existing data
    log::info!("Clearing existing data");
    db.clear_all_data().map_err(|e| {
        log::error!("Failed to clear existing data, error: {}", e.to_string());
        e.to_string()
    })?;
    log::info!("Existing data cleared successfully");
    
    let mut files_imported = 0;
    let mut tags_imported = 0;
    let mut rpg_occasions_imported = 0;
    let mut rpg_keywords_imported = 0;
    
    // Import audio files
    log::info!("Starting file import, total_files: {}", data.files.len());
    
    for (index, export_file) in data.files.iter().enumerate() {
        log::debug!("Processing file {} of {}, file_path: {}, title: {:?}, id: {:?}, has_rpg_occasion: {}, has_rpg_keywords: {}", index + 1, data.files.len(), export_file.file_path, export_file.title, export_file.id, export_file.rpg_occasion.is_some(), export_file.rpg_keywords.is_some());
        
        let audio_file = AudioFile {
            id: None, // Let database assign new IDs
            file_path: export_file.file_path.clone(),
            title: export_file.title.clone(),
            artist: export_file.artist.clone(),
            album: export_file.album.clone(),
            genre: export_file.genre.clone(),
            year: export_file.year,
            duration: export_file.duration,
            // Use exported fields when available, otherwise None
            album_artist: export_file.album_artist.clone(),
            date: None,
            track_number: export_file.track_number,
            total_tracks: None,
            disc_number: None,
            total_discs: None,
            composer: None,
            conductor: None,
            lyricist: None,
            original_artist: None,
            remixer: None,
            arranger: None,
            engineer: None,
            producer: None,
            dj_mixer: None,
            mixer: None,
            content_group: None,
            subtitle: None,
            initial_key: export_file.initial_key.clone(),
            bpm: export_file.bpm,
            language: export_file.language.clone(),
            media_type: None,
            original_filename: None,
            original_lyricist: None,
            original_release_time: None,
            playlist_delay: None,
            recording_time: None,
            release_time: None,
            tagging_time: None,
            encoding_time: None,
            encoding_settings: None,
            encoded_by: None,
            copyright: None,
            file_owner: None,
            internet_radio_station_name: None,
            internet_radio_station_owner: None,
            isrc: None,
            publisher: None,
            mood: export_file.mood.clone(),
            occasion: None,
            tempo: None,
            content_type: None,
            category: None,
        };
        
        let new_id = db.save_audio_file(&audio_file).map_err(|e| {
            log::error!("Failed to save audio file, file_path: {}, error: {}", export_file.file_path, e.to_string());
            e.to_string()
        })?;
        
        files_imported += 1;
        log::debug!("Audio file saved, old_id: {:?}, new_id: {}, file_path: {}", export_file.id, new_id, export_file.file_path);
        
        // Process genre field as semicolon/comma-separated tags
        if let Some(genre_str) = &export_file.genre {
            let genre_tags: Vec<String> = genre_str
                .split(|c| c == ';' || c == ',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();
            
            log::debug!("Processing genre tags, file_path: {}, genre_string: {}, split_tags: {:?}", export_file.file_path, genre_str, genre_tags);
            
            for genre_tag in genre_tags {
                match state.tag_manager.add_rpg_tag(new_id, "genre", &genre_tag) {
                    Ok(_) => {
                        tags_imported += 1;
                        log::debug!("Genre tag added, file_id: {}, genre: {}", new_id, genre_tag);
                    },
                    Err(e) => {
                        log::error!("Failed to add genre tag, file_id: {}, genre: {}, error: {}", new_id, genre_tag, e.to_string());
                    }
                }
            }
        }
        
        // Process mood field as semicolon/comma-separated tags
        if let Some(mood_str) = &export_file.mood {
            let mood_tags: Vec<String> = mood_str
                .split(|c| c == ';' || c == ',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();
            
            log::debug!("Processing mood tags, file_path: {}, mood_string: {}, split_tags: {:?}", export_file.file_path, mood_str, mood_tags);
            
            for mood_tag in mood_tags {
                match state.tag_manager.add_rpg_tag(new_id, "mood", &mood_tag) {
                    Ok(_) => {
                        tags_imported += 1;
                        log::debug!("Mood tag added, file_id: {}, mood: {}", new_id, mood_tag);
                    },
                    Err(e) => {
                        log::error!("Failed to add mood tag, file_id: {}, mood: {}, error: {}", new_id, mood_tag, e.to_string());
                    }
                }
            }
        }
        
        // Import traditional RPG tags for this file (skip genre/mood/occasion/keyword/quality as they're processed separately)
        let matching_tags: Vec<_> = data.tags.iter()
            .filter(|tag| tag.audio_file_id == export_file.id.unwrap_or(0))
            .filter(|tag| tag.tag_type != "genre" && tag.tag_type != "mood" && tag.tag_type != "occasion" && tag.tag_type != "keyword" && tag.tag_type != "quality") // Avoid duplicates
            .collect();
            
        log::debug!("Processing traditional RPG tags (excluding genre/mood/occasion/keyword/quality), file_path: {}, matching_tags_count: {}, original_file_id: {:?}", export_file.file_path, matching_tags.len(), export_file.id);
        
        for export_tag in matching_tags {
            match state.tag_manager.add_rpg_tag(new_id, &export_tag.tag_type, &export_tag.tag_value) {
                Ok(_) => {
                    tags_imported += 1;
                    log::debug!("RPG tag added, file_id: {}, tag_type: {}, tag_value: {}", new_id, export_tag.tag_type, export_tag.tag_value);
                },
                Err(e) => {
                    log::error!("Failed to add RPG tag, file_id: {}, tag_type: {}, tag_value: {}, error: {}", new_id, export_tag.tag_type, export_tag.tag_value, e.to_string());
                }
            }
        }
        
        // Import enhanced RPG fields as tags
        if let Some(occasions) = &export_file.rpg_occasion {
            log::debug!("Processing RPG occasions, file_path: {}, occasions_count: {}, occasions: {:?}", export_file.file_path, occasions.len(), occasions);
            
            for occasion in occasions {
                match state.tag_manager.add_rpg_tag(new_id, "occasion", occasion) {
                    Ok(_) => {
                        rpg_occasions_imported += 1;
                        log::debug!("RPG occasion added, file_id: {}, occasion: {}", new_id, occasion);
                    },
                    Err(e) => {
                        log::error!("Failed to add RPG occasion, file_id: {}, occasion: {}, error: {}", new_id, occasion, e.to_string());
                    }
                }
            }
        }
        
        if let Some(keywords) = &export_file.rpg_keywords {
            log::debug!("Processing RPG keywords, file_path: {}, keywords_count: {}, keywords: {:?}", export_file.file_path, keywords.len(), keywords);
            
            for keyword in keywords {
                match state.tag_manager.add_rpg_tag(new_id, "keyword", keyword) {
                    Ok(_) => {
                        rpg_keywords_imported += 1;
                        log::debug!("RPG keyword added, file_id: {}, keyword: {}", new_id, keyword);
                    },
                    Err(e) => {
                        log::error!("Failed to add RPG keyword, file_id: {}, keyword: {}, error: {}", new_id, keyword, e.to_string());
                    }
                }
            }
        }
        
        // Import quality if available
        if let Some(quality) = &export_file.rpg_quality {
            log::debug!("Processing RPG quality, file_path: {}, quality: {}", export_file.file_path, quality);
            
            match state.tag_manager.add_rpg_tag(new_id, "quality", quality) {
                Ok(_) => {
                    tags_imported += 1;
                    log::debug!("RPG quality added, file_id: {}, quality: {}", new_id, quality);
                },
                Err(e) => {
                    log::error!("Failed to add RPG quality, file_id: {}, quality: {}, error: {}", new_id, quality, e.to_string());
                }
            }
        }
    }
    
    log::info!("Library import completed successfully, files_imported: {}, traditional_tags_imported: {}, rpg_occasions_imported: {}, rpg_keywords_imported: {}, total_tags_imported: {}", files_imported, tags_imported, rpg_occasions_imported, rpg_keywords_imported, tags_imported + rpg_occasions_imported + rpg_keywords_imported);
    
    Ok(())
}

#[tauri::command]
async fn calculate_missing_durations(app_handle: AppHandle) -> Result<String, String> {
    let state = app_handle.state::<AppState>();
    let db = state.db.lock().unwrap();
    
    // Get all audio files and filter those missing duration or BPM
    let audio_files = db.get_all_audio_files().map_err(|e| e.to_string())?;
    let files_to_process: Vec<_> = audio_files
        .into_iter()
        .filter(|file| file.duration.is_none() || file.bpm.is_none())
        .collect();
    
    let mut duration_updated = 0u32;
    let mut bpm_updated = 0u32;
    let total_files = files_to_process.len();
    
    for (index, audio_file) in files_to_process.iter().enumerate() {
        println!("Processing file {} of {}: {}", index + 1, total_files, audio_file.file_path);
        
        // Check what needs to be calculated
        let needs_duration = audio_file.duration.is_none();
        let needs_bpm = audio_file.bpm.is_none();
        
        if needs_duration && needs_bpm {
            // Calculate both duration and BPM
            match AudioHandler::calculate_duration_and_bpm(&audio_file.file_path) {
                Ok((duration, bpm)) => {
                    if let Some(id) = audio_file.id {
                        let bpm_u32 = bpm.map(|b| b.round() as u32);
                        if let Err(e) = db.update_audio_file_duration_and_bpm(id, duration, bpm_u32) {
                            eprintln!("Failed to update duration and BPM for {}: {}", audio_file.file_path, e);
                            continue;
                        }
                        
                        if duration.is_some() {
                            duration_updated += 1;
                            println!("Updated duration for {}: {:.2}s", audio_file.file_path, duration.unwrap());
                        }
                        if bpm.is_some() {
                            bpm_updated += 1;
                            println!("Updated BPM for {}: {:.1}", audio_file.file_path, bpm.unwrap());
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to calculate duration and BPM for {}: {}", audio_file.file_path, e);
                    continue;
                }
            }
        } else if needs_duration {
            // Calculate only duration
            match AudioHandler::calculate_audio_duration(&audio_file.file_path) {
                Ok(duration) => {
                    if let Some(id) = audio_file.id {
                        if let Err(e) = db.update_audio_file_duration(id, duration) {
                            eprintln!("Failed to update duration for {}: {}", audio_file.file_path, e);
                            continue;
                        }
                        
                        duration_updated += 1;
                        println!("Updated duration for {}: {:.2}s", audio_file.file_path, duration);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to calculate duration for {}: {}", audio_file.file_path, e);
                    continue;
                }
            }
        } else if needs_bpm {
            // Calculate only BPM
            match AudioHandler::calculate_audio_bpm(&audio_file.file_path) {
                Ok(bpm) => {
                    if let Some(id) = audio_file.id {
                        let bpm_u32 = bpm.round() as u32;
                        if let Err(e) = db.update_audio_file_bpm(id, bpm_u32) {
                            eprintln!("Failed to update BPM for {}: {}", audio_file.file_path, e);
                            continue;
                        }
                        
                        bpm_updated += 1;
                        println!("Updated BPM for {}: {:.1}", audio_file.file_path, bpm);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to calculate BPM for {}: {}", audio_file.file_path, e);
                    continue;
                }
            }
        }
    }
    
    // Return a summary message
    let mut summary_parts = Vec::new();
    if duration_updated > 0 {
        summary_parts.push(format!("{} durations", duration_updated));
    }
    if bpm_updated > 0 {
        summary_parts.push(format!("{} BPMs", bpm_updated));
    }
    
    if summary_parts.is_empty() {
        Ok("All files already have complete duration and BPM information.".to_string())
    } else {
        Ok(format!("Successfully calculated and updated {}.", summary_parts.join(" and ")))
    }
}

use tauri_plugin_log::{Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db = Database::new().expect("Failed to create database");
    let tag_manager = TagManager::new().expect("Failed to create tag manager");
    
    tauri::Builder::default()
        .manage(AppState {
            db: Mutex::new(db),
            tag_manager,
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Debug)
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: Some("ligeia".into()) }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            load_audio_file,
            save_audio_file,
            get_all_audio_files,
            delete_audio_file,
            update_audio_file_tags,
            write_rpg_tags_to_file,
            scan_directory_recursive,
            get_tag_vocabulary,
            add_rpg_tag,
            remove_rpg_tag,
            get_rpg_tags_for_file,
            bulk_tag_files,
            search_files_by_tags,
            get_all_audio_files_with_tags,
            get_tag_statistics,
            export_library_data,
            import_library_data,
            calculate_missing_durations
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
