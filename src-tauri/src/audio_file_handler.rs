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
    
    /// Load audio file metadata and import RPG tags from embedded TXXX fields
    pub fn load_audio_file_with_rpg_tags(_app_handle: AppHandle, file_path: String) -> Result<(AudioFile, Vec<(String, String)>), String> {
        log::debug!("Loading audio file with RPG tags: {}", file_path);
        
        // Load basic audio file metadata
        let audio_file = AudioHandler::load_audio_file_metadata(&file_path).map_err(|e| {
            log::error!("Failed to load audio file metadata {}: {}", file_path, e);
            e.to_string()
        })?;
        
        // Read RPG tags from file
        let rpg_tags = AudioHandler::read_rpg_tags_from_file(&file_path).map_err(|e| {
            log::error!("Failed to read RPG tags from file {}: {}", file_path, e);
            e.to_string()
        })?;
        
        if !rpg_tags.is_empty() {
            log::info!("Found {} RPG tags in file: {}", rpg_tags.len(), file_path);
        }
        
        Ok((audio_file, rpg_tags))
    }
    
    /// Save audio file with imported RPG tags to database
    pub fn save_audio_file_with_rpg_tags(app_handle: AppHandle, audio_file: AudioFile, rpg_tags: Vec<(String, String)>) -> Result<i64, String> {
        log::debug!("Saving audio file with RPG tags: path={}, rpg_tag_count={}", audio_file.file_path, rpg_tags.len());
        
        // Save the audio file first
        let audio_file_id = Self::save_audio_file(app_handle.clone(), audio_file)?;
        
        // Save RPG tags if any were found
        if !rpg_tags.is_empty() {
            let state = app_handle.state::<AppState>();
            let rpg_tags_count = rpg_tags.len();
            
            for (tag_type, tag_value) in &rpg_tags {
                match state.tag_manager.add_rpg_tag(audio_file_id, tag_type, tag_value) {
                    Ok(_) => {
                        log::debug!("Added RPG tag: {}:{} to audio file {}", tag_type, tag_value, audio_file_id);
                    }
                    Err(e) => {
                        log::warn!("Failed to add RPG tag {}:{} to audio file {}: {}", tag_type, tag_value, audio_file_id, e);
                        // Continue with other tags even if one fails
                    }
                }
            }
            
            log::info!("Imported {} RPG tags for audio file: {}", rpg_tags_count, audio_file_id);
        }
        
        Ok(audio_file_id)
    }

    /// Update audio file tags in both the file system and database
    pub fn update_audio_file_tags(app_handle: AppHandle, file_path: String, updates: AudioFile) -> Result<(), String> {
        log::info!("Updating audio file tags: {}", file_path);
        
        // Update ID3 tags in the file
        AudioHandler::update_audio_file_tags(&file_path, &updates).map_err(|e| {
            log::error!("Failed to update audio file tags {}: {}", file_path, e);
            e.to_string()
        })?;

        // Update database record
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
        // Find the audio file in database by file_path
        if let Ok(existing_file) = db.get_audio_file_by_path(&file_path) {
            // Create updated audio file with merged data
            let mut updated_file = existing_file;
            
            // Merge the updates into the existing file data
            if updates.title.is_some() { updated_file.title = updates.title; }
            if updates.artist.is_some() { updated_file.artist = updates.artist; }
            if updates.album.is_some() { updated_file.album = updates.album; }
            if updates.album_artist.is_some() { updated_file.album_artist = updates.album_artist; }
            if updates.genre.is_some() { updated_file.genre = updates.genre; }
            if updates.year.is_some() { updated_file.year = updates.year; }
            if updates.track_number.is_some() { updated_file.track_number = updates.track_number; }
            if updates.total_tracks.is_some() { updated_file.total_tracks = updates.total_tracks; }
            if updates.composer.is_some() { updated_file.composer = updates.composer; }
            if updates.conductor.is_some() { updated_file.conductor = updates.conductor; }
            if updates.producer.is_some() { updated_file.producer = updates.producer; }
            if updates.remixer.is_some() { updated_file.remixer = updates.remixer; }
            if updates.bpm.is_some() { updated_file.bpm = updates.bpm; }
            if updates.initial_key.is_some() { updated_file.initial_key = updates.initial_key; }
            if updates.mood.is_some() { updated_file.mood = updates.mood; }
            if updates.language.is_some() { updated_file.language = updates.language; }
            if updates.copyright.is_some() { updated_file.copyright = updates.copyright; }
            if updates.publisher.is_some() { updated_file.publisher = updates.publisher; }
            
            // Update the database record
            db.update_audio_file(&updated_file).map_err(|e| {
                log::error!("Failed to update audio file in database {}: {}", file_path, e);
                format!("Database update failed: {}", e)
            })?;
            
            log::info!("Successfully updated both file tags and database for: {}", file_path);
        } else {
            log::warn!("Audio file not found in database, only file tags were updated: {}", file_path);
        }

        Ok(())
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