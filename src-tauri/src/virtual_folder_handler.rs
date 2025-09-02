use crate::models::{VirtualFolder, VirtualFolderTree, VirtualFolderWithContents, FolderTemplate, AutoOrganizationSuggestion, FolderSuggestion};
use tauri::{AppHandle, Manager};

// Folder Management Commands

#[tauri::command]
pub async fn create_virtual_folder(
    app_handle: AppHandle,
    folder: VirtualFolder,
) -> Result<i64, String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    db.create_virtual_folder(&folder)
        .map_err(|e| format!("Failed to create virtual folder: {}", e))
}

#[tauri::command]
pub async fn get_virtual_folder_by_id(
    app_handle: AppHandle,
    id: i64,
) -> Result<VirtualFolder, String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    db.get_virtual_folder_by_id(id)
        .map_err(|e| format!("Failed to get virtual folder: {}", e))
}

#[tauri::command]
pub async fn update_virtual_folder(
    app_handle: AppHandle,
    folder: VirtualFolder,
) -> Result<(), String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    db.update_virtual_folder(&folder)
        .map_err(|e| format!("Failed to update virtual folder: {}", e))
}

#[tauri::command]
pub async fn delete_virtual_folder(
    app_handle: AppHandle,
    id: i64,
) -> Result<(), String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    // Check if this is the "Unassigned" folder - prevent deletion
    let folder = db.get_virtual_folder_by_id(id)
        .map_err(|e| format!("Failed to get folder: {}", e))?;
    
    if folder.name == "Unassigned" {
        return Err("Cannot delete the Unassigned folder. This is a special system folder.".to_string());
    }
    
    db.delete_virtual_folder(id)
        .map_err(|e| format!("Failed to delete virtual folder: {}", e))
}

// Hierarchy Operations

#[tauri::command]
pub async fn get_virtual_folder_tree(
    app_handle: AppHandle,
) -> Result<Vec<VirtualFolderTree>, String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    db.get_virtual_folder_tree()
        .map_err(|e| format!("Failed to get virtual folder tree: {}", e))
}

#[tauri::command]
pub async fn get_folder_children(
    app_handle: AppHandle,
    parent_id: Option<i64>,
) -> Result<Vec<VirtualFolder>, String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    db.get_folder_children(parent_id)
        .map_err(|e| format!("Failed to get folder children: {}", e))
}

#[tauri::command]
pub async fn move_virtual_folder(
    app_handle: AppHandle,
    folder_id: i64,
    new_parent_id: Option<i64>,
) -> Result<(), String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    db.move_virtual_folder(folder_id, new_parent_id)
        .map_err(|e| format!("Failed to move virtual folder: {}", e))
}

#[tauri::command]
pub async fn get_folder_path(
    app_handle: AppHandle,
    folder_id: i64,
) -> Result<Vec<VirtualFolder>, String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    db.get_folder_path(folder_id)
        .map_err(|e| format!("Failed to get folder path: {}", e))
}

// Content Management Commands

#[tauri::command]
pub async fn add_files_to_virtual_folder(
    app_handle: AppHandle,
    folder_id: i64,
    file_ids: Vec<i64>,
) -> Result<(), String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    // Check if this is the "Unassigned" folder - prevent manual additions
    let folder = db.get_virtual_folder_by_id(folder_id)
        .map_err(|e| format!("Failed to get folder: {}", e))?;
    
    if folder.name == "Unassigned" {
        return Err("Cannot manually add files to the Unassigned folder. This folder automatically shows all unorganized files.".to_string());
    }
    
    for file_id in file_ids {
        // Ignore duplicates - the database constraint will handle this
        let _ = db.add_file_to_virtual_folder(folder_id, file_id);
    }
    
    Ok(())
}

#[tauri::command]
pub async fn remove_files_from_virtual_folder(
    app_handle: AppHandle,
    folder_id: i64,
    file_ids: Vec<i64>,
) -> Result<(), String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    // Check if this is the "Unassigned" folder - prevent manual removals
    let folder = db.get_virtual_folder_by_id(folder_id)
        .map_err(|e| format!("Failed to get folder: {}", e))?;
    
    if folder.name == "Unassigned" {
        return Err("Cannot manually remove files from the Unassigned folder. Files will automatically disappear when added to other folders.".to_string());
    }
    
    for file_id in file_ids {
        db.remove_file_from_virtual_folder(folder_id, file_id)
            .map_err(|e| format!("Failed to remove file {} from folder: {}", file_id, e))?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn get_virtual_folder_contents(
    app_handle: AppHandle,
    folder_id: i64,
) -> Result<VirtualFolderWithContents, String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    // First get the folder to check if it's the "Unassigned" folder
    let folder = db.get_virtual_folder_by_id(folder_id)
        .map_err(|e| format!("Failed to get folder: {}", e))?;
    
    if folder.name == "Unassigned" {
        // Special handling for "Unassigned" folder - show all unorganized files
        get_unassigned_folder_contents(&db, folder)
            .map_err(|e| format!("Failed to get unassigned folder contents: {}", e))
    } else {
        // Regular folder contents
        db.get_virtual_folder_contents(folder_id)
            .map_err(|e| format!("Failed to get virtual folder contents: {}", e))
    }
}

#[tauri::command]
pub async fn get_file_virtual_folders(
    app_handle: AppHandle,
    audio_file_id: i64,
) -> Result<Vec<VirtualFolder>, String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    db.get_file_virtual_folders(audio_file_id)
        .map_err(|e| format!("Failed to get file virtual folders: {}", e))
}

// Search and Discovery Commands

#[tauri::command]
pub async fn search_virtual_folders(
    app_handle: AppHandle,
    query: String,
) -> Result<Vec<VirtualFolder>, String> {
    println!("🔍 [HANDLER] Received search request for query: '{}'", query);
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    let result = db.search_virtual_folders(&query)
        .map_err(|e| format!("Failed to search virtual folders: {}", e))?;
    
    println!("🔍 [HANDLER] Returning {} folders to frontend", result.len());
    Ok(result)
}

#[tauri::command]
pub async fn get_folders_containing_files(
    app_handle: AppHandle,
    file_ids: Vec<i64>,
) -> Result<Vec<VirtualFolder>, String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    db.get_folders_containing_files(&file_ids)
        .map_err(|e| format!("Failed to get folders containing files: {}", e))
}

// Template Commands

#[tauri::command]
pub async fn create_folder_template(
    app_handle: AppHandle,
    template: FolderTemplate,
) -> Result<i64, String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    db.create_folder_template(&template)
        .map_err(|e| format!("Failed to create folder template: {}", e))
}

#[tauri::command]
pub async fn get_folder_templates(
    app_handle: AppHandle,
    category: Option<String>,
) -> Result<Vec<FolderTemplate>, String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    let category_ref = category.as_deref();
    db.get_folder_templates(category_ref)
        .map_err(|e| format!("Failed to get folder templates: {}", e))
}

// Tag-based Suggestion Commands

#[tauri::command]
pub async fn suggest_folders_for_file(
    app_handle: AppHandle,
    audio_file_id: i64,
    limit: Option<usize>,
) -> Result<Vec<FolderSuggestion>, String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    let suggestions = db.suggest_folders_for_file(audio_file_id, limit)
        .map_err(|e| format!("Failed to get folder suggestions: {}", e))?;
    
    // Convert to FolderSuggestion format with matching tags
    let mut result = Vec::new();
    for (folder, score) in suggestions {
        let matching_tags = db.get_matching_tags(audio_file_id, folder.id.unwrap())
            .map_err(|e| format!("Failed to get matching tags: {}", e))?;
        
        result.push(FolderSuggestion {
            folder,
            confidence_score: score,
            matching_tags,
        });
    }
    
    Ok(result)
}

#[tauri::command]
pub async fn get_auto_organization_suggestions(
    app_handle: AppHandle,
    threshold: Option<f64>,
) -> Result<Vec<AutoOrganizationSuggestion>, String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    let threshold = threshold.unwrap_or(0.8); // Default 80% confidence threshold with new scoring system
    
    db.get_auto_organization_suggestions(threshold)
        .map_err(|e| format!("Failed to get auto-organization suggestions: {}", e))
}

#[tauri::command]
pub async fn apply_auto_organization_suggestions(
    app_handle: AppHandle,
    suggestions: Vec<AutoOrganizationSuggestion>,
) -> Result<usize, String> {
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    let mut applied_count = 0;
    
    for suggestion in suggestions {
        match db.add_file_to_virtual_folder(
            suggestion.suggested_folder_id, 
            suggestion.audio_file_id
        ) {
            Ok(_) => applied_count += 1,
            Err(e) => {
                log::error!("Failed to apply suggestion for file {}: {}", 
                    suggestion.audio_file_id, e);
                // Continue with other suggestions rather than failing completely
            }
        }
    }
    
    Ok(applied_count)
}

#[tauri::command]
pub async fn auto_organize_sounds(
    app_handle: AppHandle,
    confidence_threshold: Option<f64>,
) -> Result<AutoOrganizeResult, String> {
    log::info!("Starting auto-organize process with virtual folder reset");
    
    let state = app_handle.state::<crate::AppState>();
    
    // First, reset virtual folders - delete all and recreate from scratch
    {
        let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
        
        log::info!("Deleting all existing virtual folders");
        db.delete_all_virtual_folders()
            .map_err(|e| format!("Failed to delete virtual folders: {}", e))?;
        
        log::info!("Reinitializing virtual folders from clean structure");
        db.reinitialize_virtual_folders()
            .map_err(|e| format!("Failed to reinitialize virtual folders: {}", e))?;
    }
    
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    let threshold = confidence_threshold.unwrap_or(0.8); // Default 80% with new confidence scoring (8+/10 requirement)
    
    // Find all unorganized sounds with tags
    let unorganized_files = db.get_unorganized_tagged_files()
        .map_err(|e| format!("Failed to get unorganized files: {}", e))?;
    
    let total_files = unorganized_files.len();
    log::info!("Starting to organize {} unorganized audio files", total_files);
    
    let mut organized_count = 0i32;
    let mut processed_count = 0i32;
    let mut results = Vec::new();
    let mut unorganized_files_info: Vec<(String, f64, String)> = Vec::new();
    
    for file_id in unorganized_files {
        processed_count += 1;
        let mut file_organized = false;
        
        // Log progress every 500 files
        if processed_count % 500 == 0 {
            let remaining = total_files as i32 - processed_count;
            log::info!("Progress: Processed {} files, {} remaining to process ({} organized so far)", 
                      processed_count, remaining, organized_count);
        }
        
        // Get suggestions for this file
        let suggestions = db.suggest_folders_for_file(file_id, Some(5))
            .map_err(|e| format!("Failed to get suggestions for file {}: {}", file_id, e))?;
        
        // Get best score before moving suggestions
        let best_score = suggestions.first().map(|(_, score)| *score).unwrap_or(0.0);
        
        // Filter by confidence threshold and apply the best match
        for (folder, score) in suggestions {
            if score >= threshold {
                match db.add_file_to_virtual_folder(folder.id.unwrap(), file_id) {
                    Ok(_) => {
                        organized_count += 1;
                        file_organized = true;
                        results.push(AutoOrganizeFileResult {
                            file_id,
                            folder_id: folder.id.unwrap(),
                            folder_name: folder.name.clone(),
                            confidence_score: score,
                        });
                        break; // Only add to one folder (the best match)
                    },
                    Err(e) => {
                        log::error!("Failed to add file {} to folder {}: {}", file_id, folder.name, e);
                    }
                }
            }
        }
        
        // If file wasn't organized, collect info for logging
        if !file_organized {
            if let Ok(audio_file) = get_audio_file_by_id(&*db, file_id) {
                let filename = std::path::Path::new(&audio_file.file_path)
                    .file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or("unknown");
                
                // Get tags for this file
                let file_tags = match db.get_rpg_tags_for_file(file_id) {
                    Ok(tags) => {
                        let tag_strings: Vec<String> = tags.iter()
                            .map(|tag| format!("{}:{}", tag.tag_type, tag.tag_value))
                            .collect();
                        if tag_strings.is_empty() {
                            "no tags".to_string()
                        } else {
                            tag_strings.join(", ")
                        }
                    },
                    Err(_) => "error reading tags".to_string()
                };
                
                unorganized_files_info.push((filename.to_string(), best_score, file_tags));
            }
        }
    }
    
    log::info!("Auto-organize completed: {} files processed, {} successfully organized", 
              processed_count, organized_count);
    
    // Log unorganized files with their best confidence scores and tags
    if !unorganized_files_info.is_empty() {
        log::info!("Files that were NOT organized ({} total):", unorganized_files_info.len());
        for (filename, best_score, tags) in &unorganized_files_info {
            log::info!("  - {} (best confidence: {:.2}%) | Tags: [{}]", filename, best_score * 100.0, tags);
        }
        
        // Summary of why files weren't organized
        let low_confidence_count = unorganized_files_info.iter()
            .filter(|(_, score, _)| *score > 0.0 && *score < threshold)
            .count();
        let no_suggestions_count = unorganized_files_info.iter()
            .filter(|(_, score, _)| *score == 0.0)
            .count();
            
        log::info!("Unorganized files breakdown: {} had suggestions below {:.0}% threshold, {} had no folder suggestions", 
                  low_confidence_count, threshold * 100.0, no_suggestions_count);
    }
    
    Ok(AutoOrganizeResult {
        processed_files: processed_count,
        organized_files: organized_count,
        results,
    })
}

/// Helper function to get contents for the special "Unassigned" folder
fn get_unassigned_folder_contents(
    db: &crate::database::Database, 
    folder: crate::models::VirtualFolder
) -> Result<crate::models::VirtualFolderWithContents, rusqlite::Error> {
    use crate::models::{VirtualFolderWithContents, AudioFile};
    use rusqlite::params;
    
    // Get all files that are not in any virtual folder (not just tagged ones)
    let conn = db.connection();
    let mut stmt = conn.prepare(
        "SELECT DISTINCT af.id 
         FROM audio_files af
         WHERE af.id NOT IN (
             SELECT DISTINCT vfc.audio_file_id 
             FROM virtual_folder_contents vfc
         )
         ORDER BY af.id"
    )?;
    
    let unorganized_files: Vec<i64> = stmt.query_map([], |row| {
        Ok(row.get(0)?)
    })?.collect::<Result<Vec<_>, _>>()?;
    
    log::info!("Found {} unassigned files, logging their tags for analysis", unorganized_files.len());
    
    // Convert file IDs to full AudioFile objects and log their tags
    let mut audio_files = Vec::new();
    for file_id in unorganized_files {
        if let Ok(audio_file) = get_audio_file_by_id(db, file_id) {
            // Get and log tags for this file to help with debugging organization
            log_file_tags_for_analysis(db, file_id, &audio_file.file_path)?;
            audio_files.push(audio_file);
        }
    }
    
    Ok(VirtualFolderWithContents {
        folder: folder.clone(),
        audio_files,
        subfolders: Vec::new(), // Unassigned folder has no subfolders
        breadcrumb: vec![folder], // Root level folder
    })
}

/// Helper function to get full AudioFile by ID
fn get_audio_file_by_id(db: &crate::database::Database, file_id: i64) -> Result<crate::models::AudioFile, rusqlite::Error> {
    use crate::models::AudioFile;
    use rusqlite::params;
    
    let conn = db.connection();
    let mut stmt = conn.prepare(
        "SELECT id, file_path, title, artist, album, duration, genre, year, track_number, bpm
         FROM audio_files WHERE id = ?"
    )?;
    
    stmt.query_row(params![file_id], |row| {
        Ok(AudioFile {
            id: Some(row.get(0)?),
            file_path: row.get(1)?,
            title: row.get(2)?,
            artist: row.get(3)?,
            album: row.get(4)?,
            duration: row.get(5)?,
            genre: row.get(6)?,
            year: row.get(7)?,
            track_number: row.get(8)?,
            bpm: row.get(9)?,
            ..Default::default()
        })
    })
}

/// Helper function to log tags of unassigned files for analysis
fn log_file_tags_for_analysis(db: &crate::database::Database, file_id: i64, file_path: &str) -> Result<(), rusqlite::Error> {
    use rusqlite::params;
    
    let conn = db.connection();
    let mut stmt = conn.prepare(
        "SELECT tag_type, tag_value FROM rpg_tags WHERE audio_file_id = ? ORDER BY tag_type, tag_value"
    )?;
    
    let mut genre_tags = Vec::new();
    let mut mood_tags = Vec::new(); 
    let mut occasion_tags = Vec::new();
    let mut keyword_tags = Vec::new();
    let mut other_tags = Vec::new();
    
    let tags: Result<Vec<(String, String)>, _> = stmt.query_map(params![file_id], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?.collect();
    
    if let Ok(tag_list) = tags {
        for (tag_type, tag_value) in tag_list {
            let tag_full = format!("{}:{}", tag_type, tag_value);
            match tag_type.as_str() {
                "genre" => genre_tags.push(tag_full),
                "mood" => mood_tags.push(tag_full),
                "occasion" => occasion_tags.push(tag_full),
                "keyword" | "keywords" => keyword_tags.push(tag_full),
                _ => other_tags.push(tag_full),
            }
        }
        
        // Only log files that have tags (to avoid spam from untagged files)
        if !genre_tags.is_empty() || !mood_tags.is_empty() || !occasion_tags.is_empty() || !keyword_tags.is_empty() {
            let filename = std::path::Path::new(file_path)
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or("unknown");
                
            log::info!("UNASSIGNED: {} | Genre: [{}] | Mood: [{}] | Occasion: [{}] | Keywords: [{}] | Other: [{}]", 
                filename,
                genre_tags.join(", "),
                mood_tags.join(", "),
                occasion_tags.join(", "),
                keyword_tags.join(", "),
                other_tags.join(", ")
            );
        }
    }
    
    Ok(())
}

// Auto-organization result types
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AutoOrganizeResult {
    pub processed_files: i32,
    pub organized_files: i32,
    pub results: Vec<AutoOrganizeFileResult>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AutoOrganizeFileResult {
    pub file_id: i64,
    pub folder_id: i64,
    pub folder_name: String,
    pub confidence_score: f64,
}

// Statistics types
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct FolderStatistics {
    pub total_folders: i64,
    pub total_associations: i64,
    pub top_folders: Vec<FolderStat>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct FolderStat {
    pub name: String,
    pub file_count: i64,
}