use tauri::{AppHandle, Manager};
use crate::models::{Atmosphere, AtmosphereWithSounds, AtmosphereCategory};
use crate::models::{AtmosphereIntegrity, AtmosphereIntegrityBatchEntry};
use crate::AppState;

/// Handler for atmosphere-related operations
pub struct AtmosphereHandler;

impl AtmosphereHandler {
    pub fn new() -> Self {
        AtmosphereHandler
    }

    /// Save or update an atmosphere
    pub fn save_atmosphere(app_handle: AppHandle, atmosphere: Atmosphere) -> Result<i64, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
    log::info!("Saving atmosphere: name={}, category={}, subcategory={}, crossfade_ms={}, curve={}", 
          atmosphere.name, atmosphere.category, atmosphere.subcategory, atmosphere.default_crossfade_ms, atmosphere.fade_curve);
        
        db.save_atmosphere(&atmosphere).map_err(|e| {
            log::error!("Failed to save atmosphere: {}", e);
            e.to_string()
        })
    }

    /// Get all atmospheres
    pub fn get_all_atmospheres(app_handle: AppHandle) -> Result<Vec<Atmosphere>, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
        log::debug!("Retrieving all atmospheres");
        
        db.get_all_atmospheres().map_err(|e| {
            log::error!("Failed to get all atmospheres: {}", e);
            e.to_string()
        })
    }

    /// Get atmosphere by ID
    pub fn get_atmosphere_by_id(app_handle: AppHandle, id: i64) -> Result<Atmosphere, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
        log::debug!("Retrieving atmosphere by ID: {}", id);
        
        db.get_atmosphere_by_id(id).map_err(|e| {
            log::error!("Failed to get atmosphere by ID {}: {}", id, e);
            e.to_string()
        })
    }

    /// Delete atmosphere
    pub fn delete_atmosphere(app_handle: AppHandle, id: i64) -> Result<(), String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
        log::info!("Deleting atmosphere: ID={}", id);
        
        db.delete_atmosphere(id).map_err(|e| {
            log::error!("Failed to delete atmosphere {}: {}", id, e);
            e.to_string()
        })
    }

    /// Add sound to atmosphere
    pub fn add_sound_to_atmosphere(app_handle: AppHandle, atmosphere_id: i64, audio_file_id: i64, volume: f32, is_looping: bool) -> Result<i64, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
        log::info!("Adding sound to atmosphere: atmosphere_id={}, audio_file_id={}, volume={}, is_looping={}", 
                  atmosphere_id, audio_file_id, volume, is_looping);
        
        db.add_sound_to_atmosphere(atmosphere_id, audio_file_id, volume, is_looping).map_err(|e| {
            log::error!("Failed to add sound to atmosphere {}: {}", atmosphere_id, e);
            e.to_string()
        })
    }

    /// Remove sound from atmosphere
    pub fn remove_sound_from_atmosphere(app_handle: AppHandle, atmosphere_id: i64, audio_file_id: i64) -> Result<(), String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
        log::info!("Removing sound from atmosphere: atmosphere_id={}, audio_file_id={}", 
                  atmosphere_id, audio_file_id);
        
        db.remove_sound_from_atmosphere(atmosphere_id, audio_file_id).map_err(|e| {
            log::error!("Failed to remove sound from atmosphere {}: {}", atmosphere_id, e);
            e.to_string()
        })
    }

    /// Update atmosphere sound settings
    pub fn update_atmosphere_sound(app_handle: AppHandle, atmosphere_id: i64, audio_file_id: i64, volume: f32, is_looping: bool, is_muted: bool) -> Result<(), String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
        log::debug!("Updating atmosphere sound: atmosphere_id={}, audio_file_id={}, volume={}, is_looping={}, is_muted={}", 
                   atmosphere_id, audio_file_id, volume, is_looping, is_muted);
        
        db.update_atmosphere_sound(atmosphere_id, audio_file_id, volume, is_looping, is_muted).map_err(|e| {
            log::error!("Failed to update atmosphere sound settings: {}", e);
            e.to_string()
        })
    }

    /// Get atmosphere with all its sounds
    pub fn get_atmosphere_with_sounds(app_handle: AppHandle, atmosphere_id: i64) -> Result<AtmosphereWithSounds, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
        log::debug!("Retrieving atmosphere with sounds: atmosphere_id={}", atmosphere_id);
        // First test if atmosphere exists
        if let Err(exists_err) = db.get_atmosphere_by_id(atmosphere_id) {
            log::warn!("Atmosphere {} not found prior to detail fetch: {}", atmosphere_id, exists_err);
        }
        let start = std::time::Instant::now();
        match db.get_atmosphere_with_sounds(atmosphere_id) {
            Ok(res) => {
                let ms = start.elapsed().as_millis();
                log::debug!("Fetched atmosphere {} with {} sound mappings ({} audio files) in {}ms", atmosphere_id, res.sounds.len(), res.audio_files.len(), ms);
                Ok(res)
            }
            Err(e) => {
                let ms = start.elapsed().as_millis();
                log::error!("Failed to get atmosphere with sounds {} after {}ms: {}", atmosphere_id, ms, e);
                Err(e.to_string())
            }
        }
    }

    /// Get all atmosphere categories
    pub fn get_atmosphere_categories(app_handle: AppHandle) -> Result<Vec<AtmosphereCategory>, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
        log::debug!("Retrieving atmosphere categories");
        
        db.get_atmosphere_categories().map_err(|e| {
            log::error!("Failed to get atmosphere categories: {}", e);
            e.to_string()
        })
    }

    /// Duplicate atmosphere (metadata + sounds)
    pub fn duplicate_atmosphere(app_handle: AppHandle, id: i64, new_name: Option<String>) -> Result<i64, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        log::info!("Duplicating atmosphere id={} new_name={:?}", id, new_name);
        db.atmospheres.duplicate(db.connection(), id, new_name.as_deref()).map_err(|e| {
            log::error!("Failed to duplicate atmosphere {}: {}", id, e);
            e.to_string()
        })
    }

    /// Compute integrity (missing audio file IDs) for an atmosphere
    pub fn compute_atmosphere_integrity(app_handle: AppHandle, id: i64) -> Result<AtmosphereIntegrity, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        db.atmospheres.compute_integrity(db.connection(), id).map_err(|e| {
            log::error!("Failed to compute integrity for atmosphere {}: {}", id, e);
            e.to_string()
        })
    }

    /// Batch compute integrity for all atmospheres
    pub fn compute_all_atmosphere_integrities(app_handle: AppHandle) -> Result<Vec<AtmosphereIntegrityBatchEntry>, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        db.atmospheres.compute_all_integrities(db.connection()).map_err(|e| {
            log::error!("Failed to batch compute atmosphere integrities: {}", e);
            e.to_string()
        })
    }

    /// Search atmospheres
    pub fn search_atmospheres(app_handle: AppHandle, query: Option<String>, category: Option<String>, keywords: Option<Vec<String>>) -> Result<Vec<Atmosphere>, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
    db.atmospheres.search(db.connection(), query.as_deref(), category.as_deref(), keywords.as_deref()).map_err(|e| {
            log::error!("Failed to search atmospheres: {}", e);
            e.to_string()
        })
    }
}