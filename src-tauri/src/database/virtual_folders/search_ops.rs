use rusqlite::{Connection, Result, params};
use crate::models::VirtualFolder;

/// Search and discovery operations for virtual folders
pub struct VirtualFolderSearch;

impl VirtualFolderSearch {
    pub fn search_folders(conn: &Connection, query: &str) -> Result<Vec<VirtualFolder>> {
        let search_pattern = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, name, description, parent_folder_id, color, icon, created_at, updated_at,
             created_by, folder_order, is_system_folder, metadata
             FROM virtual_folders 
             WHERE name LIKE ? OR description LIKE ?
             ORDER BY name"
        )?;
        
        use crate::database::virtual_folders::utils::VirtualFolderUtils;
        let folder_iter = stmt.query_map(params![&search_pattern, &search_pattern], |row| {
            VirtualFolderUtils::row_to_virtual_folder(row)
        })?;
        
        let mut folders = Vec::new();
        for folder in folder_iter {
            folders.push(folder?);
        }
        
        Ok(folders)
    }
    
    pub fn get_folders_containing_files(conn: &Connection, file_ids: &[i64]) -> Result<Vec<VirtualFolder>> {
        if file_ids.is_empty() {
            return Ok(Vec::new());
        }
        
        let placeholders = file_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!(
            "SELECT DISTINCT vf.* FROM virtual_folders vf
             JOIN virtual_folder_contents vfc ON vf.id = vfc.folder_id
             WHERE vfc.audio_file_id IN ({})
             ORDER BY vf.name",
            placeholders
        );
        
        let mut stmt = conn.prepare(&query)?;
        let params: Vec<&dyn rusqlite::ToSql> = file_ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();
        
        use crate::database::virtual_folders::utils::VirtualFolderUtils;
        let folder_iter = stmt.query_map(&params[..], |row| VirtualFolderUtils::row_to_virtual_folder(row))?;
        
        let mut folders = Vec::new();
        for folder in folder_iter {
            folders.push(folder?);
        }
        
        Ok(folders)
    }
}