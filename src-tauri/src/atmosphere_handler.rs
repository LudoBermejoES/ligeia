use tauri::{AppHandle, Manager};
use crate::models::{Atmosphere, AtmosphereWithSounds, AtmosphereCategory};
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
        
        db.get_atmosphere_with_sounds(atmosphere_id).map_err(|e| {
            log::error!("Failed to get atmosphere with sounds {}: {}", atmosphere_id, e);
            e.to_string()
        })
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
}