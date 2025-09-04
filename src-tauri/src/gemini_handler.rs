use crate::gemini_tagger::{AudioFile, GeminiTagger, TaggedFile, TaggingProgress};
use crate::database::TagMappingCache;
use crate::AppState;
use anyhow::Result;
use dotenv::dotenv;
use log::{error, info};
use rusqlite::params;
use serde_json::json;
use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::Semaphore;

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
    // Use connection pool instead of single connection
    let conn = state.db_pool.get_connection()
        .map_err(|e| format!("Failed to get database connection: {}", e))?;
    
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
        status: "Virtual folders reset completed. Starting auto-tagging...".to_string(),
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
    info!("=== PROCESS FILES ASYNC STARTED (POOLED VERSION) ===");
    info!("Processing {} files with batch size {} - POOLED CONNECTION MODE", files.len(), batch_size);
    
    let state = app_handle.state::<crate::AppState>();
    let total_files = files.len();
    let total_batches = (total_files + batch_size - 1) / batch_size;
    let mut processed_count = 0;
    let mut failed_count = 0;
    
    // Load cached mappings using connection pool
    let cached_mappings = {
        let conn = state.db_pool.get_connection()
            .map_err(|e| format!("Failed to get connection for cache loading: {}", e))?;
        TagMappingCache::get_all_cached_mappings(&conn)
            .map_err(|e| format!("Failed to load cached mappings: {}", e))?
    };
    
    // Process files in batches and save each batch immediately
    info!("About to process {} files in {} batches with incremental saving", files.len(), total_batches);
    
    // Create batches
    let batches: Vec<Vec<AudioFile>> = files
        .chunks(batch_size)
        .map(|chunk| chunk.to_vec())
        .collect();
    
    info!("Created {} batches for incremental processing", batches.len());
    
    // Process batches with controlled concurrency (3 simultaneous)
    let max_concurrent_batches = 3;
    let semaphore = Arc::new(Semaphore::new(max_concurrent_batches));
    let processed_counter = Arc::new(AtomicUsize::new(0));
    let failed_counter = Arc::new(AtomicUsize::new(0));
    let tagger_arc = Arc::new(tagger);
    
    info!("Processing {} batches with maximum {} concurrent batches", batches.len(), max_concurrent_batches);
    
    // Create tasks for concurrent batch processing
    let mut batch_tasks = Vec::new();
    
    for (batch_idx, batch) in batches.into_iter().enumerate() {
        let semaphore_clone = semaphore.clone();
        let tagger_clone = tagger_arc.clone();
        let cached_mappings_clone = cached_mappings.clone();
        let app_handle_clone = app_handle.clone();
        let processed_counter_clone = processed_counter.clone();
        let failed_counter_clone = failed_counter.clone();
        
        let task = tokio::spawn(async move {
            // Acquire semaphore permit (limits concurrent execution to 3)
            let _permit = semaphore_clone.acquire().await.unwrap();
            
            info!("=== PROCESSING BATCH {} OF {} (CONCURRENT) ===", batch_idx + 1, total_batches);
            
            let batch_len = batch.len();
            let mut batch_processed = 0;
            let mut batch_failed = 0;
            
            // Process this batch with Gemini API
            match tagger_clone.process_single_batch_with_cache(batch, batch_idx, &cached_mappings_clone).await {
                Ok((tagged_files, new_mappings)) => {
                    info!("Batch {} processed successfully with {} tagged files", batch_idx + 1, tagged_files.len());
                    
                    // Save tagged files to database immediately using connection pool  
                    let app_state_ref = app_handle_clone.state::<crate::AppState>();
                    match save_tagged_batch(&app_state_ref, &tagged_files).await {
                        Ok(saved_count) => {
                            batch_processed = saved_count;
                            info!("Saved batch {} with {} files to database", batch_idx + 1, saved_count);
                        }
                        Err(e) => {
                            error!("Failed to save batch {} to database: {}", batch_idx + 1, e);
                            batch_failed = tagged_files.len();
                        }
                    }
                    
                    // Store any new mappings from this batch using connection pool
                    if !new_mappings.genre_mappings.is_empty() || !new_mappings.mood_mappings.is_empty() ||
                       !new_mappings.occasion_mappings.is_empty() || !new_mappings.keyword_mappings.is_empty() {
                        
                        match app_state_ref.db_pool.get_connection() {
                            Ok(conn) => {
                                if let Err(e) = TagMappingCache::store_mappings(
                                    &conn,
                                    &new_mappings.genre_mappings,
                                    &new_mappings.mood_mappings,
                                    &new_mappings.occasion_mappings,
                                    &new_mappings.keyword_mappings,
                                ) {
                                    error!("Failed to store new mappings from batch {}: {}", batch_idx + 1, e);
                                } else {
                                    let stored_count = new_mappings.genre_mappings.len() + 
                                                     new_mappings.mood_mappings.len() + 
                                                     new_mappings.occasion_mappings.len() + 
                                                     new_mappings.keyword_mappings.len();
                                    info!("Stored {} new mappings from batch {} using pooled connection", stored_count, batch_idx + 1);
                                }
                            }
                            Err(e) => {
                                error!("Failed to get connection for storing mappings from batch {}: {}", batch_idx + 1, e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to process batch {}: {}", batch_idx + 1, e);
                    batch_failed = batch_len;
                }
            }
            
            // Update atomic counters
            processed_counter_clone.fetch_add(batch_processed, Ordering::Relaxed);
            failed_counter_clone.fetch_add(batch_failed, Ordering::Relaxed);
            
            // Emit progress update
            let current_processed = processed_counter_clone.load(Ordering::Relaxed);
            let current_failed = failed_counter_clone.load(Ordering::Relaxed);
            
            let _ = app_handle_clone.emit("tagging-progress", TaggingProgress {
                total_files,
                processed_files: current_processed,
                failed_files: current_failed,
                current_batch: batch_idx + 1,
                total_batches,
                status: format!("Batch {} completed (concurrent processing)", batch_idx + 1),
            });
            
            (batch_processed, batch_failed)
        });
        
        batch_tasks.push(task);
    }
    
    // Wait for all batch tasks to complete
    info!("Waiting for all {} concurrent batch tasks to complete...", batch_tasks.len());
    
    use futures::future::join_all;
    let results = join_all(batch_tasks).await;
    
    // Collect final results and update counters
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok((batch_processed, batch_failed)) => {
                info!("Batch {} completed: {} processed, {} failed", i + 1, batch_processed, batch_failed);
            }
            Err(e) => {
                error!("Batch {} task panicked: {}", i + 1, e);
                failed_count += batch_size;
            }
        }
    }
    
    // Get final counts from atomic counters
    processed_count = processed_counter.load(Ordering::Relaxed);
    failed_count = failed_counter.load(Ordering::Relaxed);
    
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

async fn save_tagged_batch(
    state: &tauri::State<'_, crate::AppState>, 
    batch: &[TaggedFile]
) -> Result<usize, String> {
    // Get a dedicated connection from the pool for this batch transaction
    let mut conn = state.db_pool.get_connection()
        .map_err(|e| format!("Failed to get database connection: {}", e))?;
    
    // Begin transaction for atomic batch operations
    let tx = conn.transaction()
        .map_err(|e| format!("Failed to begin transaction: {}", e))?;
    
    let mut saved_count = 0;
    
    for file in batch {
        // Update the audio file with genre and mood, and mark as auto-tagged
        let audio_file = crate::models::AudioFile {
            id: Some(file.id as i64),
            file_path: file.file_path.clone(),
            genre: Some(file.genre.clone()),
            mood: Some(file.mood.clone()),
            auto_tagged: Some(true), // Mark as auto-tagged to prevent reprocessing
            ..Default::default()
        };
        
        // Use the database abstraction methods with the transaction connection
        crate::database::AudioFileOps::update(&tx, &audio_file)
            .map_err(|e| format!("Failed to update audio file {}: {}", file.id, e))?;
        
        // Use repository instance for tag operations
        let rpg_repo = crate::database::RpgTagRepository::new();
        
        // Remove existing occasion and keyword tags
        for occasion in &file.rpg_occasion {
            let _ = rpg_repo.remove(&tx, file.id as i64, "occasion", occasion);
        }
        for keyword in &file.rpg_keywords {
            let _ = rpg_repo.remove(&tx, file.id as i64, "keyword", keyword);
        }
        
        // Add new occasion tags
        for occasion in &file.rpg_occasion {
            rpg_repo.add(&tx, file.id as i64, "occasion", occasion)
                .map_err(|e| format!("Failed to add occasion tag '{}' for file {}: {}", occasion, file.id, e))?;
        }
        
        // Add new keyword tags  
        for keyword in &file.rpg_keywords {
            rpg_repo.add(&tx, file.id as i64, "keyword", keyword)
                .map_err(|e| format!("Failed to add keyword tag '{}' for file {}: {}", keyword, file.id, e))?;
        }
        
        saved_count += 1;
    }
    
    // Commit the entire batch transaction
    tx.commit()
        .map_err(|e| format!("Failed to commit batch transaction: {}", e))?;
    
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