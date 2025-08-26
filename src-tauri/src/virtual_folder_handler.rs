use crate::database::Database;
use crate::models::{VirtualFolder, VirtualFolderTree, VirtualFolderWithContents, FolderTemplate};
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
    let state = app_handle.state::<crate::AppState>();
    let db = state.db.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    db.search_virtual_folders(&query)
        .map_err(|e| format!("Failed to search virtual folders: {}", e))
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