use rusqlite::{Row, Result};
use crate::models::{VirtualFolder, FolderTemplate};

/// Utility functions for virtual folders
pub struct VirtualFolderUtils;

impl VirtualFolderUtils {
    pub fn row_to_virtual_folder(row: &Row) -> Result<VirtualFolder> {
        Ok(VirtualFolder {
            id: Some(row.get("id")?),
            name: row.get("name")?,
            description: row.get("description")?,
            parent_folder_id: row.get("parent_folder_id")?,
            color: row.get("color")?,
            icon: row.get("icon")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
            created_by: row.get("created_by")?,
            folder_order: row.get("folder_order")?,
            is_system_folder: row.get("is_system_folder")?,
            metadata: row.get("metadata")?,
        })
    }
    
    pub fn row_to_folder_template(row: &Row) -> Result<FolderTemplate> {
        Ok(FolderTemplate {
            id: Some(row.get("id")?),
            name: row.get("name")?,
            description: row.get("description")?,
            template_data: row.get("template_data")?,
            category: row.get("category")?,
            is_public: row.get("is_public")?,
            created_at: row.get("created_at")?,
            created_by: row.get("created_by")?,
        })
    }
}