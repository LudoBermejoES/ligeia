use rusqlite::{Connection, Result, params};
use chrono::Utc;
use std::collections::HashMap;

/// System initialization operations for virtual folders
pub struct VirtualFolderSystemInit;

impl VirtualFolderSystemInit {
    /// Initialize default RPG folder structure on first run
    pub fn initialize_default_virtual_folders(conn: &Connection) -> Result<()> {
        // Check if default folders are already created
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM virtual_folders WHERE is_system_folder = 1",
            [],
            |row| row.get(0)
        )?;

        if count > 0 {
            return Ok(()); // Already initialized
        }

        log::info!("Initializing default virtual folder structure...");

        // Create the default RPG folder structure
        Self::create_rpg_folder_hierarchy(conn)?;
        
        log::info!("Default virtual folders initialized successfully");
        Ok(())
    }

    /// Get all audio files that are not in any virtual folder but have tags
    pub fn get_unorganized_tagged_files(conn: &Connection) -> Result<Vec<i64>> {
        let mut stmt = conn.prepare(
            "SELECT DISTINCT af.id 
             FROM audio_files af
             INNER JOIN rpg_tags rt ON af.id = rt.audio_file_id
             WHERE af.id NOT IN (
                 SELECT DISTINCT vfc.audio_file_id 
                 FROM virtual_folder_contents vfc
             )
             ORDER BY af.id"
        )?;
        
        let file_ids: Vec<i64> = stmt.query_map([], |row| {
            Ok(row.get(0)?)
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(file_ids)
    }

    fn create_rpg_folder_hierarchy(conn: &Connection) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        
        // Load folder structure from data file
        let folder_structure: &[(&str, Option<&str>, Option<&str>, &str)] = &include!("../../data/folder_structure.rs");
        
        // Helper function to create a system folder
        let create_system_folder = |conn: &Connection, name: &str, parent_id: Option<i64>, icon: Option<&str>, description: &str| -> Result<i64> {
            let mut stmt = conn.prepare(
                "INSERT INTO virtual_folders 
                 (name, description, parent_folder_id, color, icon, created_by, folder_order, is_system_folder, metadata, created_at, updated_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )?;
            
            stmt.execute(params![
                name,
                description,
                parent_id,
                None::<String>, // color
                icon,
                "system",
                0,
                true, // is_system_folder
                None::<String>, // metadata
                &now,
                &now
            ])?;
            
            Ok(conn.last_insert_rowid())
        };
        
        // Track created folders by their path for parent-child relationships
        let mut folder_map: HashMap<String, i64> = HashMap::new();
        
        // Create folders in order, handling parent-child relationships
        for (path, parent_path, icon, description) in folder_structure {
            let parent_id = if let Some(parent) = parent_path {
                folder_map.get(*parent).copied()
            } else {
                None
            };
            
            // Extract folder name from path (last segment after last '/')
            let folder_name = path.split('/').last().unwrap_or(path);
            
            let folder_id = create_system_folder(conn, folder_name, parent_id, *icon, description)?;
            folder_map.insert(path.to_string(), folder_id);
        }

        Ok(())
    }
}