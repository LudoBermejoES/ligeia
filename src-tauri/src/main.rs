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
    let state = app_handle.state::<AppState>();
    let db = state.db.lock().unwrap();
    db.get_all_audio_files().map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_audio_file_tags(file_path: String, updates: AudioFile) -> Result<(), String> {
    AudioHandler::update_audio_file_tags(&file_path, &updates)
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
    let db = state.db.lock().unwrap();
    
    // Get all audio files
    let audio_files = db.get_all_audio_files().map_err(|e| e.to_string())?;
    
    // Convert to export format
    let export_files: Vec<ExportAudioFile> = audio_files.into_iter().map(|af| ExportAudioFile {
        id: af.id,
        file_path: af.file_path,
        title: af.title,
        artist: af.artist,
        album: af.album,
        genre: af.genre,
        year: af.year,
        duration: af.duration,
        album_artist: af.album_artist,
        track_number: af.track_number,
        bpm: af.bpm,
        initial_key: af.initial_key,
        mood: af.mood,
        language: af.language,
    }).collect();
    
    // Get all RPG tags
    let tag_results = state.tag_manager.get_all_audio_files_with_tags().map_err(|e| e.to_string())?;
    
    // Extract tags and convert to export format
    let mut export_tags = Vec::new();
    for result in tag_results {
        for tag in result.rpg_tags {
            export_tags.push(ExportRpgTag {
                audio_file_id: tag.audio_file_id,
                tag_type: tag.tag_type,
                tag_value: tag.tag_value,
            });
        }
    }
    
    Ok(ExportData {
        version: 1,
        files: export_files,
        tags: export_tags,
    })
}

#[tauri::command]
async fn import_library_data(app_handle: AppHandle, data: ExportData) -> Result<(), String> {
    let state = app_handle.state::<AppState>();
    let db = state.db.lock().unwrap();
    
    // Clear existing data
    db.clear_all_data().map_err(|e| e.to_string())?;
    
    // Import audio files
    for export_file in data.files {
        let audio_file = AudioFile {
            id: None, // Let database assign new IDs
            file_path: export_file.file_path,
            title: export_file.title,
            artist: export_file.artist,
            album: export_file.album,
            genre: export_file.genre,
            year: export_file.year,
            duration: export_file.duration,
            // Use exported fields when available, otherwise None
            album_artist: export_file.album_artist,
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
            initial_key: export_file.initial_key,
            bpm: export_file.bpm,
            language: export_file.language,
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
            mood: export_file.mood,
            occasion: None,
            tempo: None,
            content_type: None,
            category: None,
        };
        
        let new_id = db.save_audio_file(&audio_file).map_err(|e| e.to_string())?;
        
        // Import tags for this file
        for export_tag in &data.tags {
            if export_tag.audio_file_id == export_file.id.unwrap_or(0) {
                state.tag_manager.add_rpg_tag(new_id, &export_tag.tag_type, &export_tag.tag_value).map_err(|e| e.to_string())?;
            }
        }
    }
    
    Ok(())
}

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
        .invoke_handler(tauri::generate_handler![
            load_audio_file,
            save_audio_file,
            get_all_audio_files,
            delete_audio_file,
            update_audio_file_tags,
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
            import_library_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
