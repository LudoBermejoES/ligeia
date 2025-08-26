use rusqlite::{Connection, Result, Row, params};
use crate::models::{VirtualFolder, VirtualFolderContent, VirtualFolderTree, VirtualFolderWithContents, FolderTemplate, AudioFile};
use chrono::Utc;

/// Database operations for virtual folders
pub struct VirtualFolderOps;

impl VirtualFolderOps {
    // CRUD Operations for VirtualFolder
    
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
        
        let folder = stmt.query_row([id], |row| Self::row_to_virtual_folder(row))?;
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
    
    // Hierarchy Operations
    
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
        
        let folders: Vec<VirtualFolder> = match parent_id {
            Some(pid) => {
                let rows = stmt.query_map([pid], |row| Self::row_to_virtual_folder(row))?;
                rows.collect::<Result<Vec<_>, _>>()?
            }
            None => {
                let rows = stmt.query_map([], |row| Self::row_to_virtual_folder(row))?;
                rows.collect::<Result<Vec<_>, _>>()?
            }
        };
        
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
            let folder = Self::get_virtual_folder_by_id(conn, id)?;
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
    
    // Content Management
    
    pub fn add_file_to_folder(conn: &Connection, folder_id: i64, audio_file_id: i64) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        let mut stmt = conn.prepare(
            "INSERT INTO virtual_folder_contents (folder_id, audio_file_id, added_at, file_order)
             VALUES (?, ?, ?, (SELECT COALESCE(MAX(file_order), 0) + 1 FROM virtual_folder_contents WHERE folder_id = ?))"
        )?;
        
        stmt.execute(params![folder_id, audio_file_id, &now, folder_id])?;
        Ok(())
    }
    
    pub fn remove_file_from_folder(conn: &Connection, folder_id: i64, audio_file_id: i64) -> Result<()> {
        let mut stmt = conn.prepare(
            "DELETE FROM virtual_folder_contents WHERE folder_id = ? AND audio_file_id = ?"
        )?;
        stmt.execute(params![folder_id, audio_file_id])?;
        Ok(())
    }
    
    pub fn get_folder_contents(conn: &Connection, folder_id: i64) -> Result<VirtualFolderWithContents> {
        // Get folder info
        let folder = Self::get_virtual_folder_by_id(conn, folder_id)?;
        
        // Get breadcrumb path
        let breadcrumb = Self::get_folder_path(conn, folder_id)?;
        
        // Get subfolders
        let subfolders = Self::get_folder_children(conn, Some(folder_id))?;
        
        // Get audio files in this folder
        let mut stmt = conn.prepare(
            "SELECT af.* FROM audio_files af
             JOIN virtual_folder_contents vfc ON af.id = vfc.audio_file_id
             WHERE vfc.folder_id = ?
             ORDER BY vfc.file_order, af.title"
        )?;
        
        let audio_file_iter = stmt.query_map([folder_id], |row| {
            // Map row to AudioFile - this is a simplified version, you may need to adjust based on your AudioFile struct
            Ok(AudioFile {
                id: Some(row.get("id")?),
                file_path: row.get("file_path")?,
                title: row.get("title")?,
                artist: row.get("artist")?,
                album: row.get("album")?,
                album_artist: row.get("album_artist")?,
                genre: row.get("genre")?,
                year: row.get("year")?,
                date: row.get("date")?,
                track_number: row.get("track_number")?,
                total_tracks: row.get("total_tracks")?,
                disc_number: row.get("disc_number")?,
                total_discs: row.get("total_discs")?,
                duration: row.get("duration")?,
                composer: row.get("composer")?,
                conductor: row.get("conductor")?,
                lyricist: row.get("lyricist")?,
                original_artist: row.get("original_artist")?,
                remixer: row.get("remixer")?,
                arranger: row.get("arranger")?,
                engineer: row.get("engineer")?,
                producer: row.get("producer")?,
                dj_mixer: row.get("dj_mixer")?,
                mixer: row.get("mixer")?,
                content_group: row.get("content_group")?,
                subtitle: row.get("subtitle")?,
                initial_key: row.get("initial_key")?,
                bpm: row.get("bpm")?,
                language: row.get("language")?,
                media_type: row.get("media_type")?,
                original_filename: row.get("original_filename")?,
                original_lyricist: row.get("original_lyricist")?,
                original_release_time: row.get("original_release_time")?,
                playlist_delay: row.get("playlist_delay")?,
                recording_time: row.get("recording_time")?,
                release_time: row.get("release_time")?,
                tagging_time: row.get("tagging_time")?,
                encoding_time: row.get("encoding_time")?,
                encoding_settings: row.get("encoding_settings")?,
                encoded_by: row.get("encoded_by")?,
                copyright: row.get("copyright")?,
                file_owner: row.get("file_owner")?,
                internet_radio_station_name: row.get("internet_radio_station_name")?,
                internet_radio_station_owner: row.get("internet_radio_station_owner")?,
                isrc: row.get("isrc")?,
                publisher: row.get("publisher")?,
                mood: row.get("mood")?,
                occasion: row.get("occasion")?,
                tempo: row.get("tempo")?,
                content_type: row.get("content_type")?,
                category: row.get("category")?,
            })
        })?;
        
        let mut audio_files = Vec::new();
        for audio_file in audio_file_iter {
            audio_files.push(audio_file?);
        }
        
        Ok(VirtualFolderWithContents {
            folder,
            audio_files,
            subfolders,
            breadcrumb,
        })
    }
    
    pub fn get_file_folders(conn: &Connection, audio_file_id: i64) -> Result<Vec<VirtualFolder>> {
        let mut stmt = conn.prepare(
            "SELECT vf.* FROM virtual_folders vf
             JOIN virtual_folder_contents vfc ON vf.id = vfc.folder_id
             WHERE vfc.audio_file_id = ?
             ORDER BY vf.name"
        )?;
        
        let folder_iter = stmt.query_map([audio_file_id], |row| Self::row_to_virtual_folder(row))?;
        
        let mut folders = Vec::new();
        for folder in folder_iter {
            folders.push(folder?);
        }
        
        Ok(folders)
    }
    
    // Search and Discovery
    
    pub fn search_folders(conn: &Connection, query: &str) -> Result<Vec<VirtualFolder>> {
        let search_pattern = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, name, description, parent_folder_id, color, icon, created_at, updated_at,
             created_by, folder_order, is_system_folder, metadata
             FROM virtual_folders 
             WHERE name LIKE ? OR description LIKE ?
             ORDER BY name"
        )?;
        
        let folder_iter = stmt.query_map(params![&search_pattern, &search_pattern], |row| {
            Self::row_to_virtual_folder(row)
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
        
        let folder_iter = stmt.query_map(&params[..], |row| Self::row_to_virtual_folder(row))?;
        
        let mut folders = Vec::new();
        for folder in folder_iter {
            folders.push(folder?);
        }
        
        Ok(folders)
    }
    
    // Templates (basic implementation)
    
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
        
        let templates: Vec<FolderTemplate> = match category {
            Some(cat) => {
                let rows = stmt.query_map([cat], |row| Self::row_to_folder_template(row))?;
                rows.collect::<Result<Vec<_>, _>>()?
            }
            None => {
                let rows = stmt.query_map([], |row| Self::row_to_folder_template(row))?;
                rows.collect::<Result<Vec<_>, _>>()?
            }
        };
        
        Ok(templates)
    }
    
    // Helper methods
    
    fn row_to_virtual_folder(row: &Row) -> Result<VirtualFolder> {
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
    
    fn row_to_folder_template(row: &Row) -> Result<FolderTemplate> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::schema::SchemaManager;
    
    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute("PRAGMA foreign_keys = ON", []).expect("Failed to enable foreign keys");
        
        let schema = SchemaManager::new(&conn);
        schema.create_tables(&conn).expect("Failed to create tables");
        
        conn
    }
    
    #[test]
    fn test_create_and_get_virtual_folder() {
        let conn = setup_test_db();
        
        let folder = VirtualFolder {
            id: None,
            name: "Test Folder".to_string(),
            description: Some("A test folder".to_string()),
            parent_folder_id: None,
            color: Some("#ff0000".to_string()),
            icon: Some("📁".to_string()),
            created_at: "".to_string(), // Will be set by create
            updated_at: "".to_string(),
            created_by: Some("test_user".to_string()),
            folder_order: 1,
            is_system_folder: false,
            metadata: None,
        };
        
        let folder_id = VirtualFolderOps::create_virtual_folder(&conn, &folder).unwrap();
        assert!(folder_id > 0);
        
        let retrieved_folder = VirtualFolderOps::get_virtual_folder_by_id(&conn, folder_id).unwrap();
        assert_eq!(retrieved_folder.name, "Test Folder");
        assert_eq!(retrieved_folder.description, Some("A test folder".to_string()));
        assert_eq!(retrieved_folder.color, Some("#ff0000".to_string()));
    }
    
    #[test]
    fn test_folder_hierarchy() {
        let conn = setup_test_db();
        
        // Create parent folder
        let parent_folder = VirtualFolder {
            name: "Parent Folder".to_string(),
            ..Default::default()
        };
        let parent_id = VirtualFolderOps::create_virtual_folder(&conn, &parent_folder).unwrap();
        
        // Create child folder
        let child_folder = VirtualFolder {
            name: "Child Folder".to_string(),
            parent_folder_id: Some(parent_id),
            ..Default::default()
        };
        let child_id = VirtualFolderOps::create_virtual_folder(&conn, &child_folder).unwrap();
        
        // Test getting children
        let children = VirtualFolderOps::get_folder_children(&conn, Some(parent_id)).unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name, "Child Folder");
        
        // Test getting path
        let path = VirtualFolderOps::get_folder_path(&conn, child_id).unwrap();
        assert_eq!(path.len(), 2);
        assert_eq!(path[0].name, "Parent Folder");
        assert_eq!(path[1].name, "Child Folder");
    }
    
    #[test]
    fn test_prevent_circular_reference() {
        let conn = setup_test_db();
        
        // Create two folders
        let folder1 = VirtualFolder {
            name: "Folder 1".to_string(),
            ..Default::default()
        };
        let id1 = VirtualFolderOps::create_virtual_folder(&conn, &folder1).unwrap();
        
        let folder2 = VirtualFolder {
            name: "Folder 2".to_string(),
            parent_folder_id: Some(id1),
            ..Default::default()
        };
        let id2 = VirtualFolderOps::create_virtual_folder(&conn, &folder2).unwrap();
        
        // Try to make folder1 a child of folder2 (circular reference)
        let result = VirtualFolderOps::move_folder(&conn, id1, Some(id2));
        assert!(result.is_err());
    }
    
    #[test]
    fn test_search_folders() {
        let conn = setup_test_db();
        
        // Create test folders
        let folder1 = VirtualFolder {
            name: "Combat Sounds".to_string(),
            description: Some("Battle and combat audio".to_string()),
            ..Default::default()
        };
        VirtualFolderOps::create_virtual_folder(&conn, &folder1).unwrap();
        
        let folder2 = VirtualFolder {
            name: "Peaceful Ambience".to_string(),
            description: Some("Calm and serene sounds".to_string()),
            ..Default::default()
        };
        VirtualFolderOps::create_virtual_folder(&conn, &folder2).unwrap();
        
        // Search by name
        let results = VirtualFolderOps::search_folders(&conn, "Combat").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Combat Sounds");
        
        // Search by description
        let results = VirtualFolderOps::search_folders(&conn, "calm").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Peaceful Ambience");
    }
}