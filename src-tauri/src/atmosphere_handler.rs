use tauri::{AppHandle, Manager};
use crate::models::{Atmosphere, AtmosphereWithSounds, AtmosphereCategory, AtmosphereSavePayload};
use crate::models::{AtmosphereIntegrity, AtmosphereIntegrityBatchEntry};
use crate::AppState;

/// Handler for atmosphere-related operations
pub struct AtmosphereHandler;

impl AtmosphereHandler {

    /// Save or update an atmosphere with sounds
    pub fn save_atmosphere(app_handle: AppHandle, payload: AtmosphereSavePayload) -> Result<i64, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
        log::info!("Saving atmosphere: name={}, id={}, sounds_count={}", 
                  payload.atmosphere.name, 
                  payload.atmosphere.id.map_or(-1, |id| id),
                  payload.sounds.as_ref().map_or(0, |s| s.len()));
        
        // Log delay configurations if any sounds have delay settings
        if let Some(sounds) = &payload.sounds {
            let delay_sounds: Vec<_> = sounds.iter()
                .filter(|s| s.min_seconds > 0 || s.max_seconds > 0)
                .collect();
            if !delay_sounds.is_empty() {
                log::info!("Atmosphere has {} sounds with delay settings:", delay_sounds.len());
                for sound in delay_sounds {
                    log::info!("  Audio {} delay: {}s-{}s", sound.audio_file_id, sound.min_seconds, sound.max_seconds);
                }
            }
            
            let result = db.save_atmosphere_with_sounds(&payload.atmosphere, sounds).map_err(|e| {
                log::error!("Failed to save atmosphere with sounds: {}", e);
                e.to_string()
            });
            
            match &result {
                Ok(id) => log::info!("Successfully saved atmosphere with sounds, atmosphere_id={}", id),
                Err(e) => log::error!("Save atmosphere with sounds failed: {}", e)
            }
            
            result
        } else {
            // Legacy save without sounds
            log::info!("Saving atmosphere without sounds (legacy mode)");
            db.save_atmosphere(&payload.atmosphere).map_err(|e| {
                log::error!("Failed to save atmosphere: {}", e);
                e.to_string()
            })
        }
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
    pub fn update_atmosphere_sound(app_handle: AppHandle, atmosphere_id: i64, audio_file_id: i64, volume: f32, is_looping: bool, is_muted: bool, min_seconds: i32, max_seconds: i32) -> Result<(), String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
        if min_seconds > 0 || max_seconds > 0 {
            log::info!("Updating atmosphere sound with delay: atmosphere_id={}, audio_file_id={}, delay={}s-{}s, volume={}, loop={}, muted={}", 
                      atmosphere_id, audio_file_id, min_seconds, max_seconds, volume, is_looping, is_muted);
        }
        
        db.update_atmosphere_sound(atmosphere_id, audio_file_id, volume, is_looping, is_muted, min_seconds, max_seconds).map_err(|e| {
            log::error!("Failed to update atmosphere sound settings: {}", e);
            e.to_string()
        })
    }

    /// Get atmosphere with all its sounds
    pub fn get_atmosphere_with_sounds(app_handle: AppHandle, atmosphere_id: i64) -> Result<AtmosphereWithSounds, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
        log::debug!("Getting atmosphere details: id={}", atmosphere_id);
        
        match db.get_atmosphere_with_sounds(atmosphere_id) {
            Ok(res) => {
                log::info!("Loaded atmosphere '{}' with {} sounds", res.atmosphere.name, res.sounds.len());
                
                // Log each sound for debugging
                for sound in &res.sounds {
                    log::debug!("  Sound: audio_file_id={}, volume={}, looping={}, muted={}, delay={}s-{}s", 
                               sound.audio_file_id, sound.volume, sound.is_looping, sound.is_muted,
                               sound.min_seconds, sound.max_seconds);
                }
                
                // Log delay configurations if any sounds have delay settings
                let delay_sounds: Vec<_> = res.sounds.iter()
                    .filter(|s| s.min_seconds > 0 || s.max_seconds > 0)
                    .collect();
                if !delay_sounds.is_empty() {
                    log::info!("Atmosphere {} has {} sounds with delay settings:", atmosphere_id, delay_sounds.len());
                }
                Ok(res)
            }
            Err(e) => {
                log::error!("Failed to get atmosphere with sounds {}: {}", atmosphere_id, e);
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
        crate::database::AtmosphereOps::duplicate(db.connection(), id, new_name.as_deref()).map_err(|e| {
            log::error!("Failed to duplicate atmosphere {}: {}", id, e);
            e.to_string()
        })
    }

    /// Compute integrity (missing audio file IDs) for an atmosphere
    pub fn compute_atmosphere_integrity(app_handle: AppHandle, id: i64) -> Result<AtmosphereIntegrity, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        crate::database::AtmosphereOps::compute_integrity(db.connection(), id).map_err(|e| {
            log::error!("Failed to compute integrity for atmosphere {}: {}", id, e);
            e.to_string()
        })
    }

    /// Batch compute integrity for all atmospheres
    pub fn compute_all_atmosphere_integrities(app_handle: AppHandle) -> Result<Vec<AtmosphereIntegrityBatchEntry>, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        crate::database::AtmosphereOps::compute_all_integrities(db.connection()).map_err(|e| {
            log::error!("Failed to batch compute atmosphere integrities: {}", e);
            e.to_string()
        })
    }

    /// Search atmospheres
    pub fn search_atmospheres(app_handle: AppHandle, query: Option<String>, category: Option<String>, keywords: Option<Vec<String>>) -> Result<Vec<Atmosphere>, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
    crate::database::AtmosphereOps::search(db.connection(), query.as_deref(), category.as_deref(), keywords.as_deref()).map_err(|e| {
            log::error!("Failed to search atmospheres: {}", e);
            e.to_string()
        })
    }
}