use tauri::{AppHandle, Manager};
use crate::models::AudioFile;
use crate::{AppState, AudioHandler};

/// Handler for audio processing operations (BPM, duration calculations)
pub struct AudioProcessingHandler;

impl AudioProcessingHandler {
    pub fn new() -> Self {
        AudioProcessingHandler
    }

    /// Calculate missing durations and BPMs for all audio files
    pub fn calculate_missing_durations(app_handle: AppHandle) -> Result<String, String> {
        let state = app_handle.state::<AppState>();
        let db = state.db.lock().unwrap();
        
        log::info!("Starting calculation of missing durations and BPMs");
        
        // Get all audio files and filter those missing duration or BPM
        let audio_files = db.get_all_audio_files().map_err(|e| e.to_string())?;
        let files_to_process: Vec<_> = audio_files
            .into_iter()
            .filter(|file| file.duration.is_none() || file.bpm.is_none())
            .collect();
        
        let mut duration_updated = 0u32;
        let mut bpm_updated = 0u32;
        let total_files = files_to_process.len();
        
        log::info!("Found {} files needing processing", total_files);
        
        for (index, audio_file) in files_to_process.iter().enumerate() {
            log::info!("Processing file {} of {}: {}", index + 1, total_files, audio_file.file_path);
            
            // Check what needs to be calculated
            let needs_duration = audio_file.duration.is_none();
            let needs_bpm = audio_file.bpm.is_none();
            
            if needs_duration && needs_bpm {
                let (duration_count, bpm_count) = Self::calculate_both_duration_and_bpm(&db, &audio_file)?;
                duration_updated += duration_count;
                bpm_updated += bpm_count;
            } else if needs_duration {
                duration_updated += Self::calculate_duration_only(&db, &audio_file)?;
            } else if needs_bpm {
                bpm_updated += Self::calculate_bpm_only(&db, &audio_file)?;
            }
        }
        
        log::info!("Processing completed, duration_updated: {}, bpm_updated: {}", duration_updated, bpm_updated);
        
        // Return a summary message
        Self::create_summary_message(duration_updated, bpm_updated)
    }

    // Helper methods

    /// Calculate both duration and BPM for a file
    fn calculate_both_duration_and_bpm(db: &crate::database::Database, audio_file: &AudioFile) -> Result<(u32, u32), String> {
        match AudioHandler::calculate_duration_and_bpm(&audio_file.file_path) {
            Ok((duration, bpm)) => {
                if let Some(id) = audio_file.id {
                    let bpm_u32 = bpm.map(|b| b.round() as u32);
                    if let Err(e) = db.update_audio_file_duration_and_bpm(id, duration, bpm_u32) {
                        log::error!("Failed to update duration and BPM for {}: {}", audio_file.file_path, e);
                        return Ok((0, 0));
                    }
                    
                    let mut duration_count = 0;
                    let mut bpm_count = 0;
                    
                    if duration.is_some() {
                        duration_count = 1;
                        log::info!("Updated duration for {}: {:.2}s", audio_file.file_path, duration.unwrap());
                    }
                    if bpm.is_some() {
                        bpm_count = 1;
                        log::info!("Updated BPM for {}: {:.1}", audio_file.file_path, bpm.unwrap());
                    }
                    
                    Ok((duration_count, bpm_count))
                } else {
                    log::error!("Audio file has no ID: {}", audio_file.file_path);
                    Ok((0, 0))
                }
            }
            Err(e) => {
                log::error!("Failed to calculate duration and BPM for {}: {}", audio_file.file_path, e);
                Ok((0, 0))
            }
        }
    }

    /// Calculate only duration for a file
    fn calculate_duration_only(db: &crate::database::Database, audio_file: &AudioFile) -> Result<u32, String> {
        match AudioHandler::calculate_audio_duration(&audio_file.file_path) {
            Ok(duration) => {
                if let Some(id) = audio_file.id {
                    if let Err(e) = db.update_audio_file_duration(id, duration) {
                        log::error!("Failed to update duration for {}: {}", audio_file.file_path, e);
                        return Ok(0);
                    }
                    
                    log::info!("Updated duration for {}: {:.2}s", audio_file.file_path, duration);
                    Ok(1)
                } else {
                    log::error!("Audio file has no ID: {}", audio_file.file_path);
                    Ok(0)
                }
            }
            Err(e) => {
                log::error!("Failed to calculate duration for {}: {}", audio_file.file_path, e);
                Ok(0)
            }
        }
    }

    /// Calculate only BPM for a file
    fn calculate_bpm_only(db: &crate::database::Database, audio_file: &AudioFile) -> Result<u32, String> {
        match AudioHandler::calculate_audio_bpm(&audio_file.file_path) {
            Ok(bpm) => {
                if let Some(id) = audio_file.id {
                    let bpm_u32 = bpm.round() as u32;
                    if let Err(e) = db.update_audio_file_bpm(id, bpm_u32) {
                        log::error!("Failed to update BPM for {}: {}", audio_file.file_path, e);
                        return Ok(0);
                    }
                    
                    log::info!("Updated BPM for {}: {:.1}", audio_file.file_path, bpm);
                    Ok(1)
                } else {
                    log::error!("Audio file has no ID: {}", audio_file.file_path);
                    Ok(0)
                }
            }
            Err(e) => {
                log::error!("Failed to calculate BPM for {}: {}", audio_file.file_path, e);
                Ok(0)
            }
        }
    }

    /// Create summary message based on updated counts
    fn create_summary_message(duration_updated: u32, bpm_updated: u32) -> Result<String, String> {
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
}