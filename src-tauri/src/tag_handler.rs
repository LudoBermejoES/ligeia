use tauri::{AppHandle, Manager};
use crate::models::{TagVocabulary, RpgTag, BulkTagRequest, TagSearchRequest, AudioFileWithTags};
use crate::AppState;

/// Handler for RPG tag operations
pub struct TagHandler;

impl TagHandler {

    /// Get tag vocabulary
    pub fn get_tag_vocabulary(app_handle: AppHandle, tag_type: Option<String>) -> Result<Vec<TagVocabulary>, String> {
        let state = app_handle.state::<AppState>();
        
        log::debug!("Getting tag vocabulary, tag_type: {:?}", tag_type);
        
        state.tag_manager.get_tag_vocabulary(tag_type.as_deref()).map_err(|e| {
            log::error!("Failed to get tag vocabulary: {}", e);
            e.to_string()
        })
    }

    /// Add RPG tag to an audio file
    pub fn add_rpg_tag(app_handle: AppHandle, audio_file_id: i64, tag_type: String, tag_value: String) -> Result<i64, String> {
        let state = app_handle.state::<AppState>();
        
        log::debug!("Adding RPG tag: file_id={}, type={}, value={}", audio_file_id, tag_type, tag_value);
        
        state.tag_manager.add_rpg_tag(audio_file_id, &tag_type, &tag_value).map_err(|e| {
            log::error!("Failed to add RPG tag: file_id={}, type={}, value={}, error={}", audio_file_id, tag_type, tag_value, e);
            e.to_string()
        })
    }

    /// Remove RPG tag from an audio file
    pub fn remove_rpg_tag(app_handle: AppHandle, audio_file_id: i64, tag_type: String, tag_value: String) -> Result<(), String> {
        let state = app_handle.state::<AppState>();
        
        log::debug!("Removing RPG tag: file_id={}, type={}, value={}", audio_file_id, tag_type, tag_value);
        
        state.tag_manager.remove_rpg_tag(audio_file_id, &tag_type, &tag_value).map_err(|e| {
            log::error!("Failed to remove RPG tag: file_id={}, type={}, value={}, error={}", audio_file_id, tag_type, tag_value, e);
            e.to_string()
        })
    }

    /// Get all RPG tags for a specific audio file
    pub fn get_rpg_tags_for_file(app_handle: AppHandle, audio_file_id: i64) -> Result<Vec<RpgTag>, String> {
        let state = app_handle.state::<AppState>();
        
        log::debug!("Getting RPG tags for file: {}", audio_file_id);
        
        state.tag_manager.get_rpg_tags_for_file(audio_file_id).map_err(|e| {
            log::error!("Failed to get RPG tags for file {}: {}", audio_file_id, e);
            e.to_string()
        })
    }

    /// Bulk tag multiple files
    pub fn bulk_tag_files(app_handle: AppHandle, request: BulkTagRequest) -> Result<(), String> {
        let state = app_handle.state::<AppState>();
        
        log::info!("Bulk tagging files: file_paths_count={}, tags_to_add_count={}, tags_to_remove_count={}", 
                  request.file_paths.len(), request.tags_to_add.len(), request.tags_to_remove.len());
        
        state.tag_manager.bulk_tag_files(request).map_err(|e| {
            log::error!("Failed to bulk tag files: {}", e);
            e.to_string()
        })
    }

    /// Search files by tags
    pub fn search_files_by_tags(app_handle: AppHandle, request: TagSearchRequest) -> Result<Vec<AudioFileWithTags>, String> {
        let state = app_handle.state::<AppState>();
        
        log::debug!("Searching files by tags: tag_types={:?}, tag_values={:?}, match_all={}", 
                   request.tag_types, request.tag_values, request.match_all);
        
        state.tag_manager.search_files_by_tags(request).map_err(|e| {
            log::error!("Failed to search files by tags: {}", e);
            e.to_string()
        })
    }

    /// Get all audio files with their RPG tags
    pub fn get_all_audio_files_with_tags(app_handle: AppHandle) -> Result<Vec<AudioFileWithTags>, String> {
        let state = app_handle.state::<AppState>();
        
        log::debug!("Getting all audio files with tags");
        
        state.tag_manager.get_all_audio_files_with_tags().map_err(|e| {
            log::error!("Failed to get all audio files with tags: {}", e);
            e.to_string()
        })
    }

    /// Get tag statistics
    pub fn get_tag_statistics(app_handle: AppHandle) -> Result<crate::tag_manager::TagStatistics, String> {
        let state = app_handle.state::<AppState>();
        
        log::debug!("Getting tag statistics");
        
        state.tag_manager.get_tag_statistics().map_err(|e| {
            log::error!("Failed to get tag statistics: {}", e);
            e.to_string()
        })
    }
}