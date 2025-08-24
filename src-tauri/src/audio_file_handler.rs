use tauri::{AppHandle, Manager};
use crate::models::AudioFile;
use crate::{AppState, AudioHandler};

/// Handler for audio file CRUD operations
pub struct AudioFileHandler;

impl AudioFileHandler {

    /// Save audio file to database
    pub fn save_audio_file(app_handle: AppHandle, audio_file: AudioFile) -> Result<i64, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
        log::debug!("Saving audio file: path={}, title={:?}", audio_file.file_path, audio_file.title);
        
        db.save_audio_file(&audio_file).map_err(|e| {
            log::error!("Failed to save audio file {}: {}", audio_file.file_path, e);
            e.to_string()
        })
    }

    /// Get all audio files from database
    pub fn get_all_audio_files(app_handle: AppHandle) -> Result<Vec<AudioFile>, String> {
        log::debug!("Getting all audio files from database");
        
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
        db.get_all_audio_files().map_err(|e| {
            log::error!("Failed to get all audio files: {}", e);
            e.to_string()
        })
    }

    /// Delete audio file from database
    pub fn delete_audio_file(app_handle: AppHandle, id: i64) -> Result<(), String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
        log::info!("Deleting audio file: id={}", id);
        
        db.delete_audio_file(id).map_err(|e| {
            log::error!("Failed to delete audio file {}: {}", id, e);
            e.to_string()
        })
    }

    /// Load audio file metadata from file system
    pub fn load_audio_file(file_path: String) -> Result<AudioFile, String> {
        log::debug!("Loading audio file metadata: {}", file_path);
        
        AudioHandler::load_audio_file_metadata(&file_path).map_err(|e| {
            log::error!("Failed to load audio file metadata {}: {}", file_path, e);
            e.to_string()
        })
    }

    /// Update audio file tags in the file system
    pub fn update_audio_file_tags(file_path: String, updates: AudioFile) -> Result<(), String> {
        log::info!("Updating audio file tags: {}", file_path);
        
        AudioHandler::update_audio_file_tags(&file_path, &updates).map_err(|e| {
            log::error!("Failed to update audio file tags {}: {}", file_path, e);
            e.to_string()
        })
    }

    /// Write RPG tags to file system
    pub fn write_rpg_tags_to_file(app_handle: AppHandle, file_path: String) -> Result<(), String> {
        let state = app_handle.state::<AppState>();
        
        log::info!("Writing RPG tags to file: {}", file_path);
        
        // Get the audio file from database to get its ID
        let db = state.db.lock().unwrap();
        let audio_files = db.get_all_audio_files().map_err(|e| {
            log::error!("Failed to get audio files for RPG tag lookup: {}", e);
            e.to_string()
        })?;
        
        let audio_file = audio_files.iter().find(|f| f.file_path == file_path)
            .ok_or_else(|| {
                log::error!("Audio file not found in database: {}", file_path);
                "Audio file not found in database".to_string()
            })?;
        
        if let Some(audio_file_id) = audio_file.id {
            // Get all RPG tags for this file
            let rpg_tags = state.tag_manager.get_rpg_tags_for_file(audio_file_id)
                .map_err(|e| {
                    log::error!("Failed to get RPG tags for file {}: {}", file_path, e);
                    e.to_string()
                })?;
            
            // Group tags by type
            let mut tag_groups: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
            for tag in rpg_tags {
                tag_groups.entry(tag.tag_type).or_default().push(tag.tag_value);
            }
            
            // Convert to the format expected by write_rpg_tags_to_file
            let rpg_tag_tuples: Vec<(String, Vec<String>)> = tag_groups.into_iter().collect();
            
            // Write RPG tags to the actual audio file
            AudioHandler::write_rpg_tags_to_file(&file_path, &rpg_tag_tuples).map_err(|e| {
                log::error!("Failed to write RPG tags to file {}: {}", file_path, e);
                e.to_string()
            })?;
            
            log::info!("Successfully wrote RPG tags to file: {}, tag_count: {}", file_path, rpg_tag_tuples.len());
        } else {
            log::error!("Audio file has no ID: {}", file_path);
            return Err("Audio file has no ID".to_string());
        }
        
        Ok(())
    }

    /// Scan directory recursively for audio files
    pub fn scan_directory_recursive(dir_path: String) -> Result<Vec<String>, String> {
        log::info!("Scanning directory recursively: {}", dir_path);
        
        crate::FileScanner::scan_directory_recursive(&dir_path).map_err(|e| {
            log::error!("Failed to scan directory {}: {}", dir_path, e);
            e.to_string()
        })
    }
}