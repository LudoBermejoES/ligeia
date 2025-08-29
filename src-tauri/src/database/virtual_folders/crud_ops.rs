use rusqlite::{Connection, Result, params};
use crate::models::{VirtualFolder, FolderTemplate};
use chrono::Utc;

/// CRUD operations for virtual folders
pub struct VirtualFolderCrud;

impl VirtualFolderCrud {
    pub fn create_virtual_folder(conn: &Connection, folder: &VirtualFolder) -> Result<i64> {
        let now = Utc::now().to_rfc3339();
        let mut stmt = conn.prepare(
            "INSERT INTO virtual_folders 
             (name, description, parent_folder_id, color, icon, created_by, folder_order, is_system_folder, metadata, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )?;
        
        stmt.execute(params![
            &folder.name,
            &folder.description,
            &folder.parent_folder_id,
            &folder.color,
            &folder.icon,
            &folder.created_by,
            &folder.folder_order,
            &folder.is_system_folder,
            &folder.metadata,
            &now,
            &now
        ])?;
        
        Ok(conn.last_insert_rowid())
    }
    
    pub fn get_virtual_folder_by_id(conn: &Connection, id: i64) -> Result<VirtualFolder> {
        let mut stmt = conn.prepare(
            "SELECT id, name, description, parent_folder_id, color, icon, created_at, updated_at, 
             created_by, folder_order, is_system_folder, metadata
             FROM virtual_folders WHERE id = ?"
        )?;
        
        use crate::database::virtual_folders::utils::VirtualFolderUtils;
        let folder = stmt.query_row([id], |row| VirtualFolderUtils::row_to_virtual_folder(row))?;
        Ok(folder)
    }
    
    pub fn update_virtual_folder(conn: &Connection, folder: &VirtualFolder) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        let mut stmt = conn.prepare(
            "UPDATE virtual_folders 
             SET name = ?, description = ?, parent_folder_id = ?, color = ?, icon = ?,
                 created_by = ?, folder_order = ?, is_system_folder = ?, metadata = ?, updated_at = ?
             WHERE id = ?"
        )?;
        
        stmt.execute(params![
            &folder.name,
            &folder.description,
            &folder.parent_folder_id,
            &folder.color,
            &folder.icon,
            &folder.created_by,
            &folder.folder_order,
            &folder.is_system_folder,
            &folder.metadata,
            &now,
            &folder.id
        ])?;
        
        Ok(())
    }
    
    pub fn delete_virtual_folder(conn: &Connection, id: i64) -> Result<()> {
        // First check if folder has children
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM virtual_folders WHERE parent_folder_id = ?")?;
        let child_count: i64 = stmt.query_row([id], |row| row.get(0))?;
        
        if child_count > 0 {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
                Some("Cannot delete folder with children".to_string()),
            ));
        }
        
        // Delete the folder (cascade will handle contents)
        let mut stmt = conn.prepare("DELETE FROM virtual_folders WHERE id = ?")?;
        stmt.execute([id])?;
        
        Ok(())
    }

    // Template operations
    pub fn create_folder_template(conn: &Connection, template: &FolderTemplate) -> Result<i64> {
        let now = Utc::now().to_rfc3339();
        let mut stmt = conn.prepare(
            "INSERT INTO folder_templates (name, description, template_data, category, is_public, created_by, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )?;
        
        stmt.execute(params![
            &template.name,
            &template.description,
            &template.template_data,
            &template.category,
            &template.is_public,
            &template.created_by,
            &now
        ])?;
        
        Ok(conn.last_insert_rowid())
    }
    
    pub fn get_folder_templates(conn: &Connection, category: Option<&str>) -> Result<Vec<FolderTemplate>> {
        let mut stmt = match category {
            Some(_) => conn.prepare(
                "SELECT id, name, description, template_data, category, is_public, created_at, created_by
                 FROM folder_templates WHERE category = ? AND is_public = 1 ORDER BY name"
            )?,
            None => conn.prepare(
                "SELECT id, name, description, template_data, category, is_public, created_at, created_by
                 FROM folder_templates WHERE is_public = 1 ORDER BY category, name"
            )?,
        };
        
        use crate::database::virtual_folders::utils::VirtualFolderUtils;
        let templates: Vec<FolderTemplate> = match category {
            Some(cat) => {
                let rows = stmt.query_map([cat], |row| VirtualFolderUtils::row_to_folder_template(row))?;
                rows.collect::<Result<Vec<_>, _>>()?
            }
            None => {
                let rows = stmt.query_map([], |row| VirtualFolderUtils::row_to_folder_template(row))?;
                rows.collect::<Result<Vec<_>, _>>()?
            }
        };
        
        Ok(templates)
    }
}