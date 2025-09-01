use crate::gemini_tagger::{AudioFile, GeminiTagger, TaggedFile, TaggingProgress};
use crate::AppState;
use anyhow::Result;
use dotenv::dotenv;
use log::{error, info};
use rusqlite::params;
use serde_json::json;
use std::env;
use tauri::{AppHandle, Emitter, Manager};

// Check if Gemini API key exists in .env
#[tauri::command]
pub async fn check_gemini_api_key() -> Result<bool, String> {
    dotenv().ok();
    Ok(env::var("GEMINI_API_KEY").is_ok())
}

// Get untagged files from database
#[tauri::command]
pub async fn get_untagged_files(
    app_handle: AppHandle,
) -> Result<Vec<AudioFile>, String> {
    let state = app_handle.state::<AppState>();
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    
    let query = r#"
        SELECT DISTINCT af.id, af.file_path, af.title, af.artist, af.album, af.genre, af.mood
        FROM audio_files af
        WHERE (af.genre IS NULL OR af.genre = '' OR af.genre = 'Unknown')
           OR (af.mood IS NULL OR af.mood = '')
           OR af.id NOT IN (
               SELECT DISTINCT audio_file_id FROM rpg_tags 
               WHERE tag_type IN ('occasion', 'keyword')
           )
           AND (af.auto_tagged IS NULL OR af.auto_tagged = 0)
        ORDER BY af.file_path
    "#;
    
    let mut stmt = conn.prepare(query)
        .map_err(|e| format!("Failed to prepare query: {}", e))?;
    
    let files = stmt.query_map([], |row| {
        Ok(AudioFile {
            id: row.get(0)?,
            file_path: row.get(1)?,
            title: row.get(2)?,
            artist: row.get(3)?,
            album: row.get(4)?,
            genre: row.get(5)?,
            mood: row.get(6)?,
        })
    })
    .map_err(|e| format!("Failed to query untagged files: {}", e))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| format!("Failed to collect files: {}", e))?;
    
    info!("Found {} untagged files", files.len());
    Ok(files)
}

// Main auto-tagging command
#[tauri::command]
pub async fn auto_tag_files(
    app_handle: AppHandle,
    batch_size: Option<usize>,
    max_parallel: Option<usize>,
) -> Result<String, String> {
    info!("Starting auto-tag process");
    
    // Get untagged files
    let untagged_files = get_untagged_files(app_handle.clone()).await?;
    
    if untagged_files.is_empty() {
        return Ok("No untagged files found".to_string());
    }
    
    let total_files = untagged_files.len();
    let batch_size = batch_size.unwrap_or(50);
    let total_batches = (total_files + batch_size - 1) / batch_size;
    
    // Send initial progress
    let _ = app_handle.emit("tagging-progress", TaggingProgress {
        total_files,
        processed_files: 0,
        failed_files: 0,
        current_batch: 0,
        total_batches,
        status: "Starting...".to_string(),
    });
    
    // Initialize tagger
    let tagger = if let (Some(bs), Some(mp)) = (batch_size.into(), max_parallel) {
        GeminiTagger::with_config(bs, mp)
    } else {
        GeminiTagger::new()
    }
    .map_err(|e| format!("Failed to initialize Gemini tagger: {}", e))?;
    
    // Process files in a separate task
    let app_handle_clone = app_handle.clone();
    
    tauri::async_runtime::spawn(async move {
        match process_files_async(
            tagger,
            untagged_files,
            app_handle_clone.clone(),
            batch_size,
        ).await {
            Ok(result) => {
                let _ = app_handle_clone.emit("tagging-complete", json!({
                    "success": true,
                    "message": result
                }));
            }
            Err(e) => {
                error!("Auto-tagging failed: {}", e);
                let _ = app_handle_clone.emit("tagging-complete", json!({
                    "success": false,
                    "message": format!("Auto-tagging failed: {}", e)
                }));
            }
        }
    });
    
    Ok("Auto-tagging started".to_string())
}

async fn process_files_async(
    tagger: GeminiTagger,
    files: Vec<AudioFile>,
    app_handle: AppHandle,
    batch_size: usize,
) -> Result<String, String> {
    let state = app_handle.state::<crate::AppState>();
    let total_files = files.len();
    let total_batches = (total_files + batch_size - 1) / batch_size;
    let mut processed_count = 0;
    let mut failed_count = 0;
    
    // Process files
    let tagged_files = tagger.process_untagged_files(files).await
        .map_err(|e| format!("Failed to process files: {}", e))?;
    
    // Process in batches for database operations
    
    for (batch_idx, batch) in tagged_files.chunks(batch_size).enumerate() {
        // Update progress
        let _ = app_handle.emit("tagging-progress", TaggingProgress {
            total_files,
            processed_files: processed_count,
            failed_files: failed_count,
            current_batch: batch_idx + 1,
            total_batches,
            status: format!("Processing batch {} of {}", batch_idx + 1, total_batches),
        });
        
        // Save batch to database  
        match save_tagged_batch(&state, batch) {
            Ok(saved_count) => {
                processed_count += saved_count;
                info!("Saved batch {} with {} files", batch_idx, saved_count);
            }
            Err(e) => {
                error!("Failed to save batch {}: {}", batch_idx, e);
                failed_count += batch.len();
            }
        }
    }
    
    // Final progress update
    let _ = app_handle.emit("tagging-progress", TaggingProgress {
        total_files,
        processed_files: processed_count,
        failed_files: failed_count,
        current_batch: total_batches,
        total_batches,
        status: "Complete".to_string(),
    });
    
    Ok(format!(
        "Auto-tagging complete: {} files processed, {} failed",
        processed_count, failed_count
    ))
}

fn save_tagged_batch(
    state: &tauri::State<crate::AppState>, 
    batch: &[TaggedFile]
) -> Result<usize, String> {
    let mut saved_count = 0;
    
    for file in batch {
        // Use the database methods that handle the connection properly
        let db = state.db.lock().unwrap();
        
        // Update the audio file with genre and mood
        let audio_file = crate::models::AudioFile {
            id: Some(file.id as i64),
            file_path: file.file_path.clone(),
            genre: Some(file.genre.clone()),
            mood: Some(file.mood.clone()),
            ..Default::default()
        };
        
        db.update_audio_file(&audio_file)
            .map_err(|e| format!("Failed to update audio file {}: {}", file.id, e))?;
        
        // Remove existing occasion and keyword tags
        for occasion in &file.rpg_occasion {
            let _ = db.remove_rpg_tag(file.id as i64, "occasion", occasion);
        }
        for keyword in &file.rpg_keywords {
            let _ = db.remove_rpg_tag(file.id as i64, "keyword", keyword);
        }
        
        // Add new occasion tags
        for occasion in &file.rpg_occasion {
            db.add_rpg_tag(file.id as i64, "occasion", occasion)
                .map_err(|e| format!("Failed to add occasion tag '{}' for file {}: {}", occasion, file.id, e))?;
        }
        
        // Add new keyword tags  
        for keyword in &file.rpg_keywords {
            db.add_rpg_tag(file.id as i64, "keyword", keyword)
                .map_err(|e| format!("Failed to add keyword tag '{}' for file {}: {}", keyword, file.id, e))?;
        }
        
        saved_count += 1;
    }
    
    Ok(saved_count)
}

// Get tagging history for a file
#[tauri::command]
pub async fn get_tagging_history(
    app_handle: AppHandle,
    file_id: i32,
) -> Result<Vec<serde_json::Value>, String> {
    let state = app_handle.state::<AppState>();
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    
    let mut stmt = conn.prepare(
        "SELECT id, tagged_at, tags_applied, api_version 
         FROM auto_tag_history 
         WHERE file_id = ? 
         ORDER BY tagged_at DESC"
    )
    .map_err(|e| format!("Failed to prepare query: {}", e))?;
    
    let history = stmt.query_map(params![file_id], |row| {
        Ok(json!({
            "id": row.get::<_, i32>(0)?,
            "tagged_at": row.get::<_, String>(1)?,
            "tags_applied": row.get::<_, String>(2)?,
            "api_version": row.get::<_, String>(3)?
        }))
    })
    .map_err(|e| format!("Failed to query history: {}", e))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| format!("Failed to collect history: {}", e))?;
    
    Ok(history)
}