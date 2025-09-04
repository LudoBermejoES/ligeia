use tauri::{AppHandle, Manager};
use crate::models::{ExportData, ExportAudioFile, AudioFile};
use crate::AppState;

/// Handler for library import/export operations
pub struct ImportExportHandler;

impl ImportExportHandler {

    /// Export all library data to enhanced format
    pub fn export_library_data(app_handle: AppHandle) -> Result<ExportData, String> {
        let state = app_handle.state::<AppState>();
        let _db = state.db.lock().unwrap();
        
        log::info!("Starting library export");
        
        // Get all audio files with their RPG tags
        let tag_results = state.tag_manager.get_all_audio_files_with_tags().map_err(|e| e.to_string())?;
        
        log::info!("Retrieved files with tags, file_count: {}", tag_results.len());
        
        // Convert to enhanced export format
        let mut export_files = Vec::new();
        
        for result in tag_results {
            let af = result.audio_file;
            
            // Group RPG tags by type for this file
            let mut occasions = Vec::new();
            let mut keywords = Vec::new();
            let mut genres = Vec::new();
            let mut moods = Vec::new();
            let mut quality = None;
            
            for tag in result.rpg_tags {
                match tag.tag_type.as_str() {
                    "occasion" => occasions.push(tag.tag_value),
                    "keyword" => keywords.push(tag.tag_value),
                    "genre" => genres.push(tag.tag_value),
                    "mood" => moods.push(tag.tag_value),
                    "quality" => quality = Some(tag.tag_value),
                    _ => {} // Skip other tag types
                }
            }
            
            // Create comma-separated strings for genre and mood (as per import format)
            let genre_string = if !genres.is_empty() {
                Some(genres.join(", "))
            } else {
                af.genre
            };
            
            let mood_string = if !moods.is_empty() {
                Some(moods.join(", "))
            } else {
                af.mood
            };
            
            let export_file = ExportAudioFile {
                id: af.id,
                file_path: af.file_path,
                title: af.title,
                artist: af.artist,
                album: af.album,
                genre: genre_string,
                year: af.year,
                duration: af.duration,
                album_artist: af.album_artist,
                track_number: af.track_number,
                bpm: af.bpm,
                initial_key: af.initial_key,
                mood: mood_string,
                language: af.language,
                rpg_occasion: if occasions.is_empty() { None } else { Some(occasions) },
                rpg_keywords: if keywords.is_empty() { None } else { Some(keywords) },
                rpg_quality: quality,
            };
            
            export_files.push(export_file);
        }
        
        // Get tag vocabulary for enhanced export
        let tag_vocabulary = match state.tag_manager.get_tag_vocabulary(None) {
            Ok(vocab) => {
                let mut genres = Vec::new();
                let mut moods = Vec::new();
                let mut occasions = Vec::new();
                let mut keywords = Vec::new();
                
                for tag in vocab {
                    match tag.tag_type.as_str() {
                        "genre" => genres.push(tag.tag_value),
                        "mood" => moods.push(tag.tag_value),
                        "occasion" => occasions.push(tag.tag_value),
                        "keyword" => keywords.push(tag.tag_value),
                        _ => {}
                    }
                }
                
                Some(serde_json::json!({
                    "version": "1.0",
                    "description": "RPG Audio Tagging Spec — Deep Taxonomy based on TAGS.md",
                    "last_updated": chrono::Utc::now().to_rfc3339(),
                    "genres": genres,
                    "moods": moods,
                    "occasions": occasions,
                    "keywords": keywords
                }))
            },
            Err(e) => {
                log::error!("Failed to get tag vocabulary, error: {}", e.to_string());
                None
            }
        };
        
        log::info!("Export completed successfully, files_exported: {}, has_vocabulary: {}", export_files.len(), tag_vocabulary.is_some());
        
        Ok(ExportData {
            version: 1,
            files: export_files,
            tags: Vec::new(), // Empty for enhanced format
            tag_vocabulary,
        })
    }

    /// Import library data from enhanced format
    pub fn import_library_data(app_handle: AppHandle, data: ExportData) -> Result<(), String> {
        log::info!("Import library data called - function entry");
        
        let state = app_handle.state::<AppState>();
        log::info!("Got app state successfully");
        
        let db = state.db.lock().unwrap();
        log::info!("Acquired database lock successfully");
        
        log::info!("Starting library import, version: {}, files_count: {}, tags_count: {}, has_vocabulary: {}", data.version, data.files.len(), data.tags.len(), data.tag_vocabulary.is_some());
        
        // Validate data structure
        if data.files.is_empty() {
            log::error!("Validation failed: No files to import");
            return Err("No files to import".to_string());
        }
        log::info!("Data structure validation passed");
        
        // Clear existing data
        log::info!("About to clear existing data");
        match db.clear_all_data() {
            Ok(_) => {
                log::info!("Existing data cleared successfully");
            },
            Err(e) => {
                log::error!("Failed to clear existing data, error: {}", e.to_string());
                return Err(format!("Failed to clear existing data: {}", e.to_string()));
            }
        }
        
        let mut files_imported = 0;
        let mut tags_imported = 0;
        let mut rpg_occasions_imported = 0;
        let mut rpg_keywords_imported = 0;
        
        // Import audio files
        log::info!("Starting file import, total_files: {}", data.files.len());
        
        for (index, export_file) in data.files.iter().enumerate() {
            log::debug!("Processing file {} of {}, file_path: {}, title: {:?}, id: {:?}, has_rpg_occasion: {}, has_rpg_keywords: {}", index + 1, data.files.len(), export_file.file_path, export_file.title, export_file.id, export_file.rpg_occasion.is_some(), export_file.rpg_keywords.is_some());
            
            let audio_file = Self::create_audio_file_from_export(export_file);
            
            let new_id = db.save_audio_file(&audio_file).map_err(|e| {
                log::error!("Failed to save audio file, file_path: {}, error: {}", export_file.file_path, e.to_string());
                format!("Failed to save audio file '{}': {}", export_file.file_path, e.to_string())
            })?;
            
            files_imported += 1;
            log::debug!("Audio file saved, old_id: {:?}, new_id: {}, file_path: {}", export_file.id, new_id, export_file.file_path);
            
            // Process genre tags
            if let Some(genre_str) = &export_file.genre {
                match Self::process_tag_field(&state, new_id, "genre", genre_str) {
                    Ok(count) => tags_imported += count,
                    Err(e) => log::error!("Failed to process genre tags for file_id: {}, file_path: {}, error: {}", new_id, export_file.file_path, e)
                }
            }
            
            // Process mood tags
            if let Some(mood_str) = &export_file.mood {
                match Self::process_tag_field(&state, new_id, "mood", mood_str) {
                    Ok(count) => tags_imported += count,
                    Err(e) => log::error!("Failed to process mood tags for file_id: {}, file_path: {}, error: {}", new_id, export_file.file_path, e)
                }
            }
            
            // Import traditional RPG tags (excluding processed ones)
            let matching_tags: Vec<_> = data.tags.iter()
                .filter(|tag| tag.audio_file_id == export_file.id.unwrap_or(0))
                .filter(|tag| !["genre", "mood", "occasion", "keyword", "quality"].contains(&tag.tag_type.as_str()))
                .collect();
                
            log::debug!("Processing traditional RPG tags (excluding genre/mood/occasion/keyword/quality), file_path: {}, matching_tags_count: {}, original_file_id: {:?}", export_file.file_path, matching_tags.len(), export_file.id);
            
            for export_tag in matching_tags {
                match state.tag_manager.add_rpg_tag(new_id, &export_tag.tag_type, &export_tag.tag_value) {
                    Ok(_) => {
                        tags_imported += 1;
                        log::debug!("RPG tag added, file_id: {}, tag_type: {}, tag_value: {}", new_id, export_tag.tag_type, export_tag.tag_value);
                    },
                    Err(e) => {
                        log::error!("Failed to add RPG tag, file_id: {}, tag_type: {}, tag_value: {}, error: {}", new_id, export_tag.tag_type, export_tag.tag_value, e.to_string());
                    }
                }
            }
            
            // Import enhanced RPG fields as tags
            if let Some(occasions) = &export_file.rpg_occasion {
                match Self::process_tag_array(&state, new_id, "occasion", occasions) {
                    Ok(count) => rpg_occasions_imported += count,
                    Err(e) => {
                        log::error!("Failed to process RPG occasions for file_id: {}, file_path: {}, error: {}", new_id, export_file.file_path, e);
                        // Continue processing other files
                    }
                }
            }
            
            if let Some(keywords) = &export_file.rpg_keywords {
                match Self::process_tag_array(&state, new_id, "keyword", keywords) {
                    Ok(count) => rpg_keywords_imported += count,
                    Err(e) => {
                        log::error!("Failed to process RPG keywords for file_id: {}, file_path: {}, error: {}", new_id, export_file.file_path, e);
                        // Continue processing other files
                    }
                }
            }
            
            // Import quality if available
            if let Some(quality) = &export_file.rpg_quality {
                log::debug!("Processing RPG quality, file_path: {}, quality: {}", export_file.file_path, quality);
                
                match state.tag_manager.add_rpg_tag(new_id, "quality", quality) {
                    Ok(_) => {
                        tags_imported += 1;
                        log::debug!("RPG quality added, file_id: {}, quality: {}", new_id, quality);
                    },
                    Err(e) => {
                        log::error!("Failed to add RPG quality, file_id: {}, quality: {}, error: {}", new_id, quality, e.to_string());
                    }
                }
            }
        }
        
        log::info!("Library import completed successfully, files_imported: {}, traditional_tags_imported: {}, rpg_occasions_imported: {}, rpg_keywords_imported: {}, total_tags_imported: {}", files_imported, tags_imported, rpg_occasions_imported, rpg_keywords_imported, tags_imported + rpg_occasions_imported + rpg_keywords_imported);
        
        Ok(())
    }

    // Helper methods

    /// Create AudioFile from ExportAudioFile
    fn create_audio_file_from_export(export_file: &ExportAudioFile) -> AudioFile {
        AudioFile {
            id: None, // Let database assign new IDs
            file_path: export_file.file_path.clone(),
            title: export_file.title.clone(),
            artist: export_file.artist.clone(),
            album: export_file.album.clone(),
            genre: export_file.genre.clone(),
            year: export_file.year,
            duration: export_file.duration,
            album_artist: export_file.album_artist.clone(),
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
            initial_key: export_file.initial_key.clone(),
            bpm: export_file.bpm,
            language: export_file.language.clone(),
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
            mood: export_file.mood.clone(),
            occasion: None,
            tempo: None,
            content_type: None,
            category: None,
            auto_tagged: None,
            auto_tag_date: None,
            auto_tag_version: None,
        }
    }

    /// Process semicolon/comma-separated tag field
    fn process_tag_field(state: &tauri::State<AppState>, file_id: i64, tag_type: &str, tag_string: &str) -> Result<u32, String> {
        let tags: Vec<String> = tag_string
            .split(|c| c == ';' || c == ',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        
        log::debug!("Processing {} tags, file_id: {}, tag_string: {}, split_tags: {:?}", tag_type, file_id, tag_string, tags);
        
        let mut imported_count = 0;
        for tag in tags {
            match state.tag_manager.add_rpg_tag(file_id, tag_type, &tag) {
                Ok(_) => {
                    imported_count += 1;
                    log::debug!("{} tag added, file_id: {}, {}: {}", tag_type, file_id, tag_type, tag);
                },
                Err(e) => {
                    log::error!("Failed to add {} tag, file_id: {}, {}: {}, error: {}", tag_type, file_id, tag_type, tag, e.to_string());
                }
            }
        }
        
        Ok(imported_count)
    }

    /// Process array of tags
    fn process_tag_array(state: &tauri::State<AppState>, file_id: i64, tag_type: &str, tags: &[String]) -> Result<u32, String> {
        log::debug!("Processing {} tags, file_id: {}, tags_count: {}, tags: {:?}", tag_type, file_id, tags.len(), tags);
        
        let mut imported_count = 0;
        for tag in tags {
            match state.tag_manager.add_rpg_tag(file_id, tag_type, tag) {
                Ok(_) => {
                    imported_count += 1;
                    log::debug!("{} tag added, file_id: {}, {}: {}", tag_type, file_id, tag_type, tag);
                },
                Err(e) => {
                    log::error!("Failed to add {} tag, file_id: {}, {}: {}, error: {}", tag_type, file_id, tag_type, tag, e.to_string());
                }
            }
        }
        
        Ok(imported_count)
    }
}