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
    
    db.get_virtual_folder_contents(folder_id)
        .map_err(|e| format!("Failed to get virtual folder contents: {}", e))
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
    
    let threshold = threshold.unwrap_or(0.3); // Default 30% confidence threshold
    
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
                eprintln!("Failed to apply suggestion for file {}: {}", 
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
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    let threshold = confidence_threshold.unwrap_or(0.7); // Default 70% as requested
    
    // Find all unorganized sounds with tags
    let unorganized_files = db.get_unorganized_tagged_files()
        .map_err(|e| format!("Failed to get unorganized files: {}", e))?;
    
    let mut organized_count = 0;
    let mut processed_count = 0;
    let mut results = Vec::new();
    
    for file_id in unorganized_files {
        processed_count += 1;
        
        // Get suggestions for this file
        let suggestions = db.suggest_folders_for_file(file_id, Some(5))
            .map_err(|e| format!("Failed to get suggestions for file {}: {}", file_id, e))?;
        
        // Filter by confidence threshold and apply the best match
        for (folder, score) in suggestions {
            if score >= threshold {
                match db.add_file_to_virtual_folder(folder.id.unwrap(), file_id) {
                    Ok(_) => {
                        organized_count += 1;
                        results.push(AutoOrganizeFileResult {
                            file_id,
                            folder_id: folder.id.unwrap(),
                            folder_name: folder.name.clone(),
                            confidence_score: score,
                        });
                        break; // Only add to one folder (the best match)
                    },
                    Err(e) => {
                        eprintln!("Failed to add file {} to folder {}: {}", file_id, folder.name, e);
                    }
                }
            }
        }
    }
    
    Ok(AutoOrganizeResult {
        processed_files: processed_count,
        organized_files: organized_count,
        results,
    })
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