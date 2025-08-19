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
            get_tag_statistics
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
