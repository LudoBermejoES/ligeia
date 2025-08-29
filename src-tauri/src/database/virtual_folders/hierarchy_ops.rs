use rusqlite::{Connection, Result, params};
use crate::models::{VirtualFolder, VirtualFolderTree};
use chrono::Utc;

/// Hierarchy operations for virtual folders
pub struct VirtualFolderHierarchy;

impl VirtualFolderHierarchy {
    pub fn get_folder_children(conn: &Connection, parent_id: Option<i64>) -> Result<Vec<VirtualFolder>> {
        let mut stmt = match parent_id {
            Some(_) => conn.prepare(
                "SELECT id, name, description, parent_folder_id, color, icon, created_at, updated_at,
                 created_by, folder_order, is_system_folder, metadata
                 FROM virtual_folders WHERE parent_folder_id = ? ORDER BY folder_order, name"
            )?,
            None => conn.prepare(
                "SELECT id, name, description, parent_folder_id, color, icon, created_at, updated_at,
                 created_by, folder_order, is_system_folder, metadata
                 FROM virtual_folders WHERE parent_folder_id IS NULL ORDER BY folder_order, name"
            )?,
        };
        
        use crate::database::virtual_folders::utils::VirtualFolderUtils;
        let folders: Vec<VirtualFolder> = match parent_id {
            Some(pid) => {
                let rows = stmt.query_map([pid], |row| VirtualFolderUtils::row_to_virtual_folder(row))?;
                rows.collect::<Result<Vec<_>, _>>()?
            }
            None => {
                let rows = stmt.query_map([], |row| VirtualFolderUtils::row_to_virtual_folder(row))?;
                rows.collect::<Result<Vec<_>, _>>()?
            }
        };
        
        Ok(folders)
    }
    
    pub fn get_all_virtual_folders(conn: &Connection) -> Result<Vec<VirtualFolder>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, description, parent_folder_id, color, icon, created_at, updated_at,
             created_by, folder_order, is_system_folder, metadata
             FROM virtual_folders ORDER BY name"
        )?;
        
        use crate::database::virtual_folders::utils::VirtualFolderUtils;
        let folders: Vec<VirtualFolder> = stmt.query_map([], |row| VirtualFolderUtils::row_to_virtual_folder(row))?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(folders)
    }
    
    pub fn get_folder_tree(conn: &Connection) -> Result<Vec<VirtualFolderTree>> {
        let root_folders = Self::get_folder_children(conn, None)?;
        let mut tree = Vec::new();
        
        for folder in root_folders {
            let tree_node = Self::build_folder_tree_node(conn, folder)?;
            tree.push(tree_node);
        }
        
        Ok(tree)
    }
    
    fn build_folder_tree_node(conn: &Connection, folder: VirtualFolder) -> Result<VirtualFolderTree> {
        let folder_id = folder.id.unwrap_or(0);
        
        // Get direct file count
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM virtual_folder_contents WHERE folder_id = ?")?;
        let file_count: i64 = stmt.query_row([folder_id], |row| row.get(0))?;
        
        // Get children
        let children_folders = Self::get_folder_children(conn, Some(folder_id))?;
        let mut children = Vec::new();
        let mut total_file_count = file_count;
        
        for child_folder in children_folders {
            let child_tree = Self::build_folder_tree_node(conn, child_folder)?;
            total_file_count += child_tree.total_file_count;
            children.push(child_tree);
        }
        
        Ok(VirtualFolderTree {
            folder,
            children,
            file_count,
            total_file_count,
        })
    }
    
    pub fn get_folder_path(conn: &Connection, folder_id: i64) -> Result<Vec<VirtualFolder>> {
        let mut path = Vec::new();
        let mut current_id = Some(folder_id);
        
        while let Some(id) = current_id {
            use crate::database::virtual_folders::crud_ops::VirtualFolderCrud;
            let folder = VirtualFolderCrud::get_virtual_folder_by_id(conn, id)?;
            current_id = folder.parent_folder_id;
            path.insert(0, folder); // Insert at beginning to build path from root
        }
        
        Ok(path)
    }
    
    pub fn move_folder(conn: &Connection, folder_id: i64, new_parent_id: Option<i64>) -> Result<()> {
        // Prevent circular references
        if let Some(new_parent) = new_parent_id {
            if Self::is_ancestor(conn, folder_id, new_parent)? {
                return Err(rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
                    Some("Cannot move folder to its own descendant".to_string()),
                ));
            }
        }
        
        let now = Utc::now().to_rfc3339();
        let mut stmt = conn.prepare(
            "UPDATE virtual_folders SET parent_folder_id = ?, updated_at = ? WHERE id = ?"
        )?;
        stmt.execute(params![new_parent_id, &now, folder_id])?;
        
        Ok(())
    }
    
    fn is_ancestor(conn: &Connection, folder_id: i64, potential_descendant: i64) -> Result<bool> {
        if folder_id == potential_descendant {
            return Ok(true);
        }
        
        let mut stmt = conn.prepare("SELECT parent_folder_id FROM virtual_folders WHERE id = ?")?;
        let parent_id: Option<i64> = stmt.query_row([potential_descendant], |row| {
            Ok(row.get::<_, Option<i64>>(0)?)
        })?;
        
        if let Some(parent) = parent_id {
            Self::is_ancestor(conn, folder_id, parent)
        } else {
            Ok(false)
        }
    }
}