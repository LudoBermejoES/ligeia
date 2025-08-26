#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;
use tauri::AppHandle;


mod models;
mod database;
mod audio_handler;
mod tag_manager;
mod file_scanner;
mod atmosphere_handler;
mod import_export_handler;
mod audio_processing_handler;
mod audio_file_handler;
mod tag_handler;
mod store_tags_handler;
mod virtual_folder_handler;

use models::*;
use database::Database;
use audio_handler::AudioHandler;
use tag_manager::TagManager;
use file_scanner::FileScanner;
use atmosphere_handler::AtmosphereHandler;
use import_export_handler::ImportExportHandler;
use audio_processing_handler::AudioProcessingHandler;
use audio_file_handler::AudioFileHandler;
use tag_handler::TagHandler;

struct AppState {
    db: Mutex<Database>,
    tag_manager: TagManager,
}

#[tauri::command]
async fn load_audio_file(file_path: String) -> Result<AudioFile, String> {
    AudioFileHandler::load_audio_file(file_path)
}

#[tauri::command]
async fn save_audio_file(app_handle: AppHandle, audio_file: AudioFile) -> Result<i64, String> {
    AudioFileHandler::save_audio_file(app_handle, audio_file)
}

#[tauri::command]
async fn load_audio_file_with_rpg_tags(app_handle: AppHandle, file_path: String) -> Result<(AudioFile, Vec<(String, String)>), String> {
    AudioFileHandler::load_audio_file_with_rpg_tags(app_handle, file_path)
}

#[tauri::command]
async fn save_audio_file_with_rpg_tags(app_handle: AppHandle, audio_file: AudioFile, rpg_tags: Vec<(String, String)>) -> Result<i64, String> {
    AudioFileHandler::save_audio_file_with_rpg_tags(app_handle, audio_file, rpg_tags)
}

#[tauri::command]
async fn get_all_audio_files(app_handle: AppHandle) -> Result<Vec<AudioFile>, String> {
    AudioFileHandler::get_all_audio_files(app_handle)
}

#[tauri::command]
async fn update_audio_file_tags(file_path: String, updates: AudioFile) -> Result<(), String> {
    AudioFileHandler::update_audio_file_tags(file_path, updates)
}

#[tauri::command]
async fn write_rpg_tags_to_file(app_handle: AppHandle, file_path: String) -> Result<(), String> {
    AudioFileHandler::write_rpg_tags_to_file(app_handle, file_path)
}

#[tauri::command]
async fn scan_directory_recursive(dir_path: String) -> Result<Vec<String>, String> {
    AudioFileHandler::scan_directory_recursive(dir_path)
}

#[tauri::command]
async fn delete_audio_file(app_handle: AppHandle, id: i64) -> Result<(), String> {
    AudioFileHandler::delete_audio_file(app_handle, id)
}

// RPG Tag Commands
#[tauri::command]
async fn get_tag_vocabulary(app_handle: AppHandle, tag_type: Option<String>) -> Result<Vec<TagVocabulary>, String> {
    TagHandler::get_tag_vocabulary(app_handle, tag_type)
}

#[tauri::command]
async fn add_rpg_tag(app_handle: AppHandle, audio_file_id: i64, tag_type: String, tag_value: String) -> Result<i64, String> {
    TagHandler::add_rpg_tag(app_handle, audio_file_id, tag_type, tag_value)
}

#[tauri::command]
async fn remove_rpg_tag(app_handle: AppHandle, audio_file_id: i64, tag_type: String, tag_value: String) -> Result<(), String> {
    TagHandler::remove_rpg_tag(app_handle, audio_file_id, tag_type, tag_value)
}

#[tauri::command]
async fn get_rpg_tags_for_file(app_handle: AppHandle, audio_file_id: i64) -> Result<Vec<RpgTag>, String> {
    log::info!("Tauri command get_rpg_tags_for_file called with audio_file_id: {}", audio_file_id);
    let result = TagHandler::get_rpg_tags_for_file(app_handle, audio_file_id);
    match &result {
        Ok(tags) => log::info!("Tauri command get_rpg_tags_for_file returning {} tags", tags.len()),
        Err(e) => log::error!("Tauri command get_rpg_tags_for_file failed: {}", e),
    }
    result
}

#[tauri::command]
async fn bulk_tag_files(app_handle: AppHandle, request: BulkTagRequest) -> Result<(), String> {
    TagHandler::bulk_tag_files(app_handle, request)
}

#[tauri::command]
async fn search_files_by_tags(app_handle: AppHandle, request: TagSearchRequest) -> Result<Vec<AudioFileWithTags>, String> {
    TagHandler::search_files_by_tags(app_handle, request)
}

#[tauri::command]
async fn get_all_audio_files_with_tags(app_handle: AppHandle) -> Result<Vec<AudioFileWithTags>, String> {
    TagHandler::get_all_audio_files_with_tags(app_handle)
}

#[tauri::command]
async fn get_tag_statistics(app_handle: AppHandle) -> Result<tag_manager::TagStatistics, String> {
    TagHandler::get_tag_statistics(app_handle)
}

#[tauri::command]
async fn get_existing_tags(app_handle: AppHandle) -> Result<std::collections::HashMap<String, Vec<String>>, String> {
    TagHandler::get_existing_tags(app_handle)
}

#[tauri::command]
async fn export_library_data(app_handle: AppHandle) -> Result<ExportData, String> {
    ImportExportHandler::export_library_data(app_handle)
}

#[tauri::command]
async fn import_library_data(app_handle: AppHandle, data: String) -> Result<(), String> {
    log::info!("Starting library import: JSON size={}KB", data.len() / 1024);
    
    // Parse the JSON string into ExportData
    let export_data: ExportData = match serde_json::from_str(&data) {
        Ok(data) => data,
        Err(e) => {
            log::error!("Failed to parse import JSON: {}", e);
            return Err(format!("Failed to parse import data: {}", e));
        }
    };
    
    log::info!("Parsed library data: version={}, files={}", export_data.version, export_data.files.len());
    
    let result = ImportExportHandler::import_library_data(app_handle, export_data);
    match &result {
        Ok(_) => log::info!("Import completed successfully"),
        Err(e) => log::error!("Import failed with error: {}", e)
    }
    result
}

#[tauri::command]
async fn store_all_tags_in_files(app_handle: AppHandle) -> Result<StoreTagsResult, String> {
    store_tags_handler::store_all_tags_in_files(app_handle).await
}

// Atmosphere Commands
#[tauri::command]
async fn save_atmosphere(app_handle: AppHandle, atmosphere: AtmosphereSavePayload) -> Result<i64, String> {
    AtmosphereHandler::save_atmosphere(app_handle, atmosphere)
}

#[tauri::command]
async fn get_all_atmospheres(app_handle: AppHandle) -> Result<Vec<Atmosphere>, String> {
    AtmosphereHandler::get_all_atmospheres(app_handle)
}

#[tauri::command]
async fn get_atmosphere_by_id(app_handle: AppHandle, id: i64) -> Result<Atmosphere, String> {
    AtmosphereHandler::get_atmosphere_by_id(app_handle, id)
}

#[tauri::command]
async fn delete_atmosphere(app_handle: AppHandle, id: i64) -> Result<(), String> {
    AtmosphereHandler::delete_atmosphere(app_handle, id)
}

#[tauri::command]
async fn add_sound_to_atmosphere(app_handle: AppHandle, atmosphere_id: i64, audio_file_id: i64, volume: f32, is_looping: bool) -> Result<i64, String> {
    AtmosphereHandler::add_sound_to_atmosphere(app_handle, atmosphere_id, audio_file_id, volume, is_looping)
}

#[tauri::command]
async fn remove_sound_from_atmosphere(app_handle: AppHandle, atmosphere_id: i64, audio_file_id: i64) -> Result<(), String> {
    AtmosphereHandler::remove_sound_from_atmosphere(app_handle, atmosphere_id, audio_file_id)
}

#[tauri::command]
async fn update_atmosphere_sound(app_handle: AppHandle, atmosphere_id: i64, audio_file_id: i64, volume: f32, is_looping: bool, is_muted: bool, min_seconds: i32, max_seconds: i32) -> Result<(), String> {
    AtmosphereHandler::update_atmosphere_sound(app_handle, atmosphere_id, audio_file_id, volume, is_looping, is_muted, min_seconds, max_seconds)
}

#[tauri::command]
async fn get_atmosphere_with_sounds(app_handle: AppHandle, atmosphere_id: i64) -> Result<AtmosphereWithSounds, String> {
    log::debug!("Getting atmosphere details: id={}", atmosphere_id);
    AtmosphereHandler::get_atmosphere_with_sounds(app_handle, atmosphere_id)
}

#[tauri::command]
async fn get_atmosphere_categories(app_handle: AppHandle) -> Result<Vec<AtmosphereCategory>, String> {
    AtmosphereHandler::get_atmosphere_categories(app_handle)
}

#[tauri::command]
async fn duplicate_atmosphere(app_handle: AppHandle, id: i64, new_name: Option<String>) -> Result<i64, String> {
    AtmosphereHandler::duplicate_atmosphere(app_handle, id, new_name)
}

#[tauri::command]
async fn compute_atmosphere_integrity(app_handle: AppHandle, id: i64) -> Result<AtmosphereIntegrity, String> {
    AtmosphereHandler::compute_atmosphere_integrity(app_handle, id)
}

#[tauri::command]
async fn compute_all_atmosphere_integrities(app_handle: AppHandle) -> Result<Vec<AtmosphereIntegrityBatchEntry>, String> {
    AtmosphereHandler::compute_all_atmosphere_integrities(app_handle)
}

#[tauri::command]
async fn search_atmospheres(app_handle: AppHandle, query: Option<String>, category: Option<String>, keywords: Option<Vec<String>>) -> Result<Vec<Atmosphere>, String> {
    AtmosphereHandler::search_atmospheres(app_handle, query, category, keywords)
}

#[tauri::command]
async fn calculate_missing_durations(app_handle: AppHandle) -> Result<String, String> {
    AudioProcessingHandler::calculate_missing_durations(app_handle)
}

// Import virtual folder commands from handler
use virtual_folder_handler::{
    create_virtual_folder, get_virtual_folder_by_id, update_virtual_folder, delete_virtual_folder,
    get_virtual_folder_tree, get_folder_children, get_folder_path, move_virtual_folder, 
    add_files_to_virtual_folder, remove_files_from_virtual_folder, get_virtual_folder_contents,
    get_file_virtual_folders, search_virtual_folders, get_folders_containing_files
};

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
                    Target::new(TargetKind::Folder { 
                        path: std::path::PathBuf::from("../logs"), 
                        file_name: Some("ligeia".into()) 
                    }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            load_audio_file,
            save_audio_file,
            load_audio_file_with_rpg_tags,
            save_audio_file_with_rpg_tags,
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
            get_existing_tags,
            export_library_data,
            import_library_data,
            store_all_tags_in_files,
            calculate_missing_durations,
            save_atmosphere,
            get_all_atmospheres,
            get_atmosphere_by_id,
            delete_atmosphere,
            add_sound_to_atmosphere,
            remove_sound_from_atmosphere,
            update_atmosphere_sound,
            get_atmosphere_with_sounds,
            get_atmosphere_categories,
            duplicate_atmosphere,
            compute_atmosphere_integrity,
            compute_all_atmosphere_integrities,
            search_atmospheres,
            create_virtual_folder,
            get_virtual_folder_by_id,
            update_virtual_folder,
            delete_virtual_folder,
            get_virtual_folder_tree,
            get_folder_children,
            get_folder_path,
            move_virtual_folder,
            add_files_to_virtual_folder,
            remove_files_from_virtual_folder,
            get_virtual_folder_contents,
            get_file_virtual_folders,
            search_virtual_folders,
            get_folders_containing_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
