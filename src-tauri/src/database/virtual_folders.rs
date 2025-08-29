use rusqlite::{Connection, Result, Row, params};
use crate::models::{VirtualFolder, VirtualFolderTree, VirtualFolderWithContents, FolderTemplate, AudioFile, AutoOrganizationSuggestion};
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
    
    pub fn get_all_virtual_folders(conn: &Connection) -> Result<Vec<VirtualFolder>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, description, parent_folder_id, color, icon, created_at, updated_at,
             created_by, folder_order, is_system_folder, metadata
             FROM virtual_folders ORDER BY name"
        )?;
        
        let folders: Vec<VirtualFolder> = stmt.query_map([], |row| Self::row_to_virtual_folder(row))?
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

    fn create_rpg_folder_hierarchy(conn: &Connection) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        
        // Helper function to create a system folder
        let create_system_folder = |conn: &Connection, name: &str, parent_id: Option<i64>, icon: Option<&str>| -> Result<i64> {
            let mut stmt = conn.prepare(
                "INSERT INTO virtual_folders 
                 (name, description, parent_folder_id, color, icon, created_by, folder_order, is_system_folder, metadata, created_at, updated_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )?;
            
            stmt.execute(params![
                name,
                format!("System folder for {} audio files", name.to_lowercase()),
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

        // === COMBAT ===
        let combat_id = create_system_folder(conn, "Combat", None, Some("⚔️"))?;
        
        // Weapons
        let weapons_id = create_system_folder(conn, "Weapons", Some(combat_id), Some("⚡"))?;
        
        let melee_id = create_system_folder(conn, "Melee", Some(weapons_id), Some("🗡️"))?;
        create_system_folder(conn, "Swords", Some(melee_id), None)?;
        create_system_folder(conn, "Axes", Some(melee_id), None)?;
        create_system_folder(conn, "Clubs", Some(melee_id), None)?;
        create_system_folder(conn, "Hammers", Some(melee_id), None)?;
        create_system_folder(conn, "Daggers", Some(melee_id), None)?;
        
        let ranged_id = create_system_folder(conn, "Ranged", Some(weapons_id), Some("🏹"))?;
        create_system_folder(conn, "Bows", Some(ranged_id), None)?;
        create_system_folder(conn, "Crossbows", Some(ranged_id), None)?;
        let firearms_id = create_system_folder(conn, "Firearms", Some(ranged_id), Some("🔫"))?;
        create_system_folder(conn, "Pistols", Some(firearms_id), None)?;
        create_system_folder(conn, "Rifles", Some(firearms_id), None)?;
        create_system_folder(conn, "Machine Guns", Some(firearms_id), None)?;
        create_system_folder(conn, "Thrown", Some(ranged_id), None)?;
        
        let magical_weapons_id = create_system_folder(conn, "Magical", Some(weapons_id), Some("✨"))?;
        create_system_folder(conn, "Battle Magic", Some(magical_weapons_id), None)?;
        create_system_folder(conn, "Spell Impacts", Some(magical_weapons_id), None)?;
        create_system_folder(conn, "Enchanted Weapons", Some(magical_weapons_id), None)?;
        
        // Armor & Defense
        let armor_id = create_system_folder(conn, "Armor & Defense", Some(combat_id), Some("🛡️"))?;
        create_system_folder(conn, "Leather", Some(armor_id), None)?;
        create_system_folder(conn, "Chain Mail", Some(armor_id), None)?;
        create_system_folder(conn, "Plate", Some(armor_id), None)?;
        create_system_folder(conn, "Shields", Some(armor_id), None)?;
        
        // Combat Phases
        let combat_phases_id = create_system_folder(conn, "Combat Phases", Some(combat_id), Some("⚡"))?;
        create_system_folder(conn, "Ambush", Some(combat_phases_id), None)?;
        create_system_folder(conn, "Skirmish", Some(combat_phases_id), None)?;
        create_system_folder(conn, "Siege", Some(combat_phases_id), None)?;
        create_system_folder(conn, "Final Battle", Some(combat_phases_id), None)?;
        
        // Victory & Defeat
        let victory_id = create_system_folder(conn, "Victory & Defeat", Some(combat_id), Some("🏆"))?;
        create_system_folder(conn, "Triumph", Some(victory_id), None)?;
        create_system_folder(conn, "Retreat", Some(victory_id), None)?;
        create_system_folder(conn, "Last Stand", Some(victory_id), None)?;

        // === ENVIRONMENTS ===
        let environments_id = create_system_folder(conn, "Environments", None, Some("🌍"))?;
        
        // Natural
        let natural_id = create_system_folder(conn, "Natural", Some(environments_id), Some("🌲"))?;
        
        let forest_id = create_system_folder(conn, "Forest", Some(natural_id), Some("🌳"))?;
        create_system_folder(conn, "Ancient Forest", Some(forest_id), None)?;
        create_system_folder(conn, "Dark Woods", Some(forest_id), None)?;
        create_system_folder(conn, "Fairy Groves", Some(forest_id), None)?;
        
        let mountains_id = create_system_folder(conn, "Mountains", Some(natural_id), Some("⛰️"))?;
        create_system_folder(conn, "High Peaks", Some(mountains_id), None)?;
        create_system_folder(conn, "Cave Systems", Some(mountains_id), None)?;
        create_system_folder(conn, "Mining Areas", Some(mountains_id), None)?;
        
        let water_id = create_system_folder(conn, "Water", Some(natural_id), Some("🌊"))?;
        create_system_folder(conn, "Ocean", Some(water_id), None)?;
        create_system_folder(conn, "Rivers", Some(water_id), None)?;
        create_system_folder(conn, "Swamps", Some(water_id), None)?;
        
        let weather_id = create_system_folder(conn, "Weather", Some(natural_id), Some("⛈️"))?;
        create_system_folder(conn, "Storms", Some(weather_id), None)?;
        create_system_folder(conn, "Blizzards", Some(weather_id), None)?;
        create_system_folder(conn, "Calm", Some(weather_id), None)?;
        
        // Urban
        let urban_id = create_system_folder(conn, "Urban", Some(environments_id), Some("🏘️"))?;
        
        let cities_id = create_system_folder(conn, "Cities", Some(urban_id), Some("🏙️"))?;
        create_system_folder(conn, "Noble Districts", Some(cities_id), None)?;
        create_system_folder(conn, "Markets", Some(cities_id), None)?;
        create_system_folder(conn, "Slums", Some(cities_id), None)?;
        
        let villages_id = create_system_folder(conn, "Villages", Some(urban_id), Some("🏠"))?;
        create_system_folder(conn, "Peaceful", Some(villages_id), None)?;
        create_system_folder(conn, "Under Threat", Some(villages_id), None)?;
        
        let buildings_id = create_system_folder(conn, "Buildings", Some(urban_id), Some("🏢"))?;
        create_system_folder(conn, "Taverns", Some(buildings_id), None)?;
        create_system_folder(conn, "Temples", Some(buildings_id), None)?;
        create_system_folder(conn, "Shops", Some(buildings_id), None)?;
        
        // Dungeons
        let dungeons_id = create_system_folder(conn, "Dungeons", Some(environments_id), Some("🏰"))?;
        create_system_folder(conn, "Stone Corridors", Some(dungeons_id), None)?;
        create_system_folder(conn, "Trap Rooms", Some(dungeons_id), None)?;
        create_system_folder(conn, "Boss Chambers", Some(dungeons_id), None)?;
        create_system_folder(conn, "Treasure Vaults", Some(dungeons_id), None)?;

        // === CREATURES ===
        let creatures_id = create_system_folder(conn, "Creatures", None, Some("🐲"))?;
        
        // Humanoids
        let humanoids_id = create_system_folder(conn, "Humanoids", Some(creatures_id), Some("👥"))?;
        
        let civilized_id = create_system_folder(conn, "Civilized", Some(humanoids_id), Some("👑"))?;
        create_system_folder(conn, "Humans", Some(civilized_id), None)?;
        create_system_folder(conn, "Elves", Some(civilized_id), None)?;
        create_system_folder(conn, "Dwarves", Some(civilized_id), None)?;
        
        let hostile_id = create_system_folder(conn, "Hostile", Some(humanoids_id), Some("⚔️"))?;
        create_system_folder(conn, "Orcs", Some(hostile_id), None)?;
        create_system_folder(conn, "Goblins", Some(hostile_id), None)?;
        create_system_folder(conn, "Bandits", Some(hostile_id), None)?;
        
        // Beasts
        let beasts_id = create_system_folder(conn, "Beasts", Some(creatures_id), Some("🐺"))?;
        
        let predators_id = create_system_folder(conn, "Predators", Some(beasts_id), Some("🦁"))?;
        create_system_folder(conn, "Wolves", Some(predators_id), None)?;
        create_system_folder(conn, "Bears", Some(predators_id), None)?;
        create_system_folder(conn, "Big Cats", Some(predators_id), None)?;
        
        let magical_beasts_id = create_system_folder(conn, "Magical", Some(beasts_id), Some("🦄"))?;
        create_system_folder(conn, "Dragons", Some(magical_beasts_id), None)?;
        create_system_folder(conn, "Griffons", Some(magical_beasts_id), None)?;
        create_system_folder(conn, "Unicorns", Some(magical_beasts_id), None)?;
        
        let mounts_id = create_system_folder(conn, "Mounts", Some(beasts_id), Some("🐎"))?;
        create_system_folder(conn, "Horses", Some(mounts_id), None)?;
        create_system_folder(conn, "Pegasi", Some(mounts_id), None)?;
        create_system_folder(conn, "War Beasts", Some(mounts_id), None)?;
        
        // Undead
        let undead_id = create_system_folder(conn, "Undead", Some(creatures_id), Some("☠️"))?;
        
        let lesser_undead_id = create_system_folder(conn, "Lesser", Some(undead_id), Some("💀"))?;
        create_system_folder(conn, "Skeletons", Some(lesser_undead_id), None)?;
        create_system_folder(conn, "Zombies", Some(lesser_undead_id), None)?;
        
        let greater_undead_id = create_system_folder(conn, "Greater", Some(undead_id), Some("👑"))?;
        create_system_folder(conn, "Liches", Some(greater_undead_id), None)?;
        create_system_folder(conn, "Vampires", Some(greater_undead_id), None)?;
        create_system_folder(conn, "Death Knights", Some(greater_undead_id), None)?;
        
        // Supernatural
        let supernatural_id = create_system_folder(conn, "Supernatural", Some(creatures_id), Some("👻"))?;
        create_system_folder(conn, "Demons", Some(supernatural_id), None)?;
        create_system_folder(conn, "Angels", Some(supernatural_id), None)?;
        create_system_folder(conn, "Fae", Some(supernatural_id), None)?;
        create_system_folder(conn, "Elementals", Some(supernatural_id), None)?;

        // === MAGIC & POWERS ===
        let magic_id = create_system_folder(conn, "Magic & Powers", None, Some("✨"))?;
        
        // Schools of Magic
        let schools_id = create_system_folder(conn, "Schools of Magic", Some(magic_id), Some("📚"))?;
        
        let evocation_id = create_system_folder(conn, "Evocation", Some(schools_id), Some("🔥"))?;
        create_system_folder(conn, "Fire", Some(evocation_id), None)?;
        create_system_folder(conn, "Ice", Some(evocation_id), None)?;
        create_system_folder(conn, "Lightning", Some(evocation_id), None)?;
        
        let necromancy_id = create_system_folder(conn, "Necromancy", Some(schools_id), Some("💀"))?;
        create_system_folder(conn, "Death Magic", Some(necromancy_id), None)?;
        create_system_folder(conn, "Soul Binding", Some(necromancy_id), None)?;
        create_system_folder(conn, "Undead Control", Some(necromancy_id), None)?;
        
        let illusion_id = create_system_folder(conn, "Illusion", Some(schools_id), Some("🎭"))?;
        create_system_folder(conn, "Mind Control", Some(illusion_id), None)?;
        create_system_folder(conn, "Deception", Some(illusion_id), None)?;
        create_system_folder(conn, "Invisibility", Some(illusion_id), None)?;
        
        let divination_id = create_system_folder(conn, "Divination", Some(schools_id), Some("🔮"))?;
        create_system_folder(conn, "Prophecy", Some(divination_id), None)?;
        create_system_folder(conn, "Scrying", Some(divination_id), None)?;
        create_system_folder(conn, "Truth Seeking", Some(divination_id), None)?;
        
        // Magical Events
        let magical_events_id = create_system_folder(conn, "Magical Events", Some(magic_id), Some("🌟"))?;
        create_system_folder(conn, "Rituals", Some(magical_events_id), None)?;
        create_system_folder(conn, "Summoning", Some(magical_events_id), None)?;
        create_system_folder(conn, "Portal Travel", Some(magical_events_id), None)?;
        create_system_folder(conn, "Time Manipulation", Some(magical_events_id), None)?;
        
        // Technology
        let technology_id = create_system_folder(conn, "Technology", Some(magic_id), Some("⚙️"))?;
        
        let medieval_id = create_system_folder(conn, "Medieval", Some(technology_id), Some("🔧"))?;
        create_system_folder(conn, "Clockwork", Some(medieval_id), None)?;
        create_system_folder(conn, "Alchemical", Some(medieval_id), None)?;
        
        let steampunk_id = create_system_folder(conn, "Steampunk", Some(technology_id), Some("🚂"))?;
        create_system_folder(conn, "Steam Engines", Some(steampunk_id), None)?;
        create_system_folder(conn, "Airships", Some(steampunk_id), None)?;
        
        let scifi_id = create_system_folder(conn, "Sci-Fi", Some(technology_id), Some("🚀"))?;
        create_system_folder(conn, "Cybernetics", Some(scifi_id), None)?;
        create_system_folder(conn, "Spaceships", Some(scifi_id), None)?;
        create_system_folder(conn, "AI Systems", Some(scifi_id), None)?;

        // === SOCIAL ENCOUNTERS ===
        let social_id = create_system_folder(conn, "Social Encounters", None, Some("🗣️"))?;
        
        // Taverns & Inns
        let taverns_id = create_system_folder(conn, "Taverns & Inns", Some(social_id), Some("🍺"))?;
        create_system_folder(conn, "Cheerful", Some(taverns_id), None)?;
        create_system_folder(conn, "Seedy", Some(taverns_id), None)?;
        create_system_folder(conn, "Haunted", Some(taverns_id), None)?;
        
        // Courts & Politics
        let courts_id = create_system_folder(conn, "Courts & Politics", Some(social_id), Some("👑"))?;
        create_system_folder(conn, "Royal Court", Some(courts_id), None)?;
        create_system_folder(conn, "Negotiations", Some(courts_id), None)?;
        create_system_folder(conn, "Intrigue", Some(courts_id), None)?;
        
        // Markets & Trade
        let markets_id = create_system_folder(conn, "Markets & Trade", Some(social_id), Some("🏪"))?;
        create_system_folder(conn, "Bustling Markets", Some(markets_id), None)?;
        create_system_folder(conn, "Black Markets", Some(markets_id), None)?;
        create_system_folder(conn, "Merchant Caravans", Some(markets_id), None)?;
        
        // Religious
        let religious_id = create_system_folder(conn, "Religious", Some(social_id), Some("⛪"))?;
        create_system_folder(conn, "Temples", Some(religious_id), None)?;
        create_system_folder(conn, "Ceremonies", Some(religious_id), None)?;
        create_system_folder(conn, "Divine Intervention", Some(religious_id), None)?;
        
        // Investigation
        let investigation_id = create_system_folder(conn, "Investigation", Some(social_id), Some("🔍"))?;
        create_system_folder(conn, "Crime Scenes", Some(investigation_id), None)?;
        create_system_folder(conn, "Library Research", Some(investigation_id), None)?;
        create_system_folder(conn, "Interrogation", Some(investigation_id), None)?;
        create_system_folder(conn, "Clue Discovery", Some(investigation_id), None)?;

        // === HORROR & TERROR ===
        let horror_id = create_system_folder(conn, "Horror & Terror", None, Some("👻"))?;
        
        // Classic Horror Locations
        let classic_horror_id = create_system_folder(conn, "Classic Horror Locations", Some(horror_id), Some("🏚️"))?;
        
        let haunted_houses_id = create_system_folder(conn, "Haunted Houses", Some(classic_horror_id), Some("🏚️"))?;
        create_system_folder(conn, "Victorian Mansions", Some(haunted_houses_id), None)?;
        create_system_folder(conn, "Abandoned Estates", Some(haunted_houses_id), None)?;
        create_system_folder(conn, "Cursed Residences", Some(haunted_houses_id), None)?;
        create_system_folder(conn, "Basement Horrors", Some(haunted_houses_id), None)?;
        
        let cemeteries_id = create_system_folder(conn, "Cemeteries & Graveyards", Some(classic_horror_id), Some("⚰️"))?;
        create_system_folder(conn, "Ancient Burial Grounds", Some(cemeteries_id), None)?;
        create_system_folder(conn, "Forgotten Graveyards", Some(cemeteries_id), None)?;
        create_system_folder(conn, "Mausoleums", Some(cemeteries_id), None)?;
        create_system_folder(conn, "Crypts", Some(cemeteries_id), None)?;
        
        // Cosmic Horror
        let cosmic_horror_id = create_system_folder(conn, "Cosmic Horror", Some(horror_id), Some("🌌"))?;
        
        let eldritch_id = create_system_folder(conn, "Eldritch Entities", Some(cosmic_horror_id), Some("👁️"))?;
        create_system_folder(conn, "Great Old Ones", Some(eldritch_id), None)?;
        create_system_folder(conn, "Outer Gods", Some(eldritch_id), None)?;
        create_system_folder(conn, "Tentacled Horrors", Some(eldritch_id), None)?;
        create_system_folder(conn, "Incomprehensible Beings", Some(eldritch_id), None)?;

        // === SUPERHERO & COMIC BOOK ===
        let superhero_id = create_system_folder(conn, "Superhero & Comic Book", None, Some("🦸"))?;
        
        // Urban Settings
        let comic_urban_id = create_system_folder(conn, "Urban Settings", Some(superhero_id), Some("🏢"))?;
        
        let metropolis_id = create_system_folder(conn, "Metropolis", Some(comic_urban_id), Some("🌆"))?;
        create_system_folder(conn, "Daily Planet", Some(metropolis_id), None)?;
        create_system_folder(conn, "LexCorp Tower", Some(metropolis_id), None)?;
        create_system_folder(conn, "City Center", Some(metropolis_id), None)?;
        
        let gotham_id = create_system_folder(conn, "Gotham City", Some(comic_urban_id), Some("🦇"))?;
        create_system_folder(conn, "Wayne Manor", Some(gotham_id), None)?;
        create_system_folder(conn, "Arkham Asylum", Some(gotham_id), None)?;
        create_system_folder(conn, "Crime Alley", Some(gotham_id), None)?;
        create_system_folder(conn, "GCPD", Some(gotham_id), None)?;
        
        // Powers & Abilities
        let powers_id = create_system_folder(conn, "Powers & Abilities", Some(superhero_id), Some("⚡"))?;
        
        let flight_id = create_system_folder(conn, "Flight", Some(powers_id), Some("🚁"))?;
        create_system_folder(conn, "Supersonic Flight", Some(flight_id), None)?;
        create_system_folder(conn, "Jetpack Flight", Some(flight_id), None)?;
        create_system_folder(conn, "Magical Flight", Some(flight_id), None)?;
        
        let strength_id = create_system_folder(conn, "Strength & Combat", Some(powers_id), Some("💪"))?;
        create_system_folder(conn, "Super Strength", Some(strength_id), None)?;
        create_system_folder(conn, "Martial Arts", Some(strength_id), None)?;
        create_system_folder(conn, "Energy Blasts", Some(strength_id), None)?;
        
        // Comic Book SFX
        let comic_sfx_id = create_system_folder(conn, "Comic Book SFX", Some(superhero_id), Some("💥"))?;
        
        let classic_sfx_id = create_system_folder(conn, "Classic Onomatopoeia", Some(comic_sfx_id), Some("💥"))?;
        create_system_folder(conn, "POW!", Some(classic_sfx_id), None)?;
        create_system_folder(conn, "BAM!", Some(classic_sfx_id), None)?;
        create_system_folder(conn, "ZAP!", Some(classic_sfx_id), None)?;
        create_system_folder(conn, "KAPOW!", Some(classic_sfx_id), None)?;
        create_system_folder(conn, "WHAM!", Some(classic_sfx_id), None)?;
        create_system_folder(conn, "BOOM!", Some(classic_sfx_id), None)?;

        // === MOODS & ATMOSPHERE ===
        let moods_id = create_system_folder(conn, "Moods & Atmosphere", None, Some("🎭"))?;
        
        let positive_id = create_system_folder(conn, "Positive", Some(moods_id), Some("😊"))?;
        create_system_folder(conn, "Heroic & Triumphant", Some(positive_id), None)?;
        create_system_folder(conn, "Peaceful & Serene", Some(positive_id), None)?;
        create_system_folder(conn, "Adventurous", Some(positive_id), None)?;
        create_system_folder(conn, "Celebratory", Some(positive_id), None)?;
        
        let neutral_id = create_system_folder(conn, "Neutral", Some(moods_id), Some("😐"))?;
        create_system_folder(conn, "Mysterious", Some(neutral_id), None)?;
        create_system_folder(conn, "Contemplative", Some(neutral_id), None)?;
        create_system_folder(conn, "Ethereal", Some(neutral_id), None)?;
        create_system_folder(conn, "Ceremonial", Some(neutral_id), None)?;
        
        let dark_id = create_system_folder(conn, "Dark", Some(moods_id), Some("😰"))?;
        create_system_folder(conn, "Ominous", Some(dark_id), None)?;
        create_system_folder(conn, "Tense", Some(dark_id), None)?;
        create_system_folder(conn, "Gothic", Some(dark_id), None)?;
        create_system_folder(conn, "Tragic", Some(dark_id), None)?;

        // === ACTIVITIES & CRAFTS ===
        let activities_id = create_system_folder(conn, "Activities & Crafts", None, Some("🔨"))?;
        
        // Artisan Crafts
        let artisan_id = create_system_folder(conn, "Artisan Crafts", Some(activities_id), Some("⚒️"))?;
        
        let blacksmithing_id = create_system_folder(conn, "Blacksmithing", Some(artisan_id), Some("🔥"))?;
        create_system_folder(conn, "Forges", Some(blacksmithing_id), None)?;
        create_system_folder(conn, "Anvil Work", Some(blacksmithing_id), None)?;
        create_system_folder(conn, "Weapon Making", Some(blacksmithing_id), None)?;
        
        let alchemy_id = create_system_folder(conn, "Alchemy", Some(artisan_id), Some("⚗️"))?;
        create_system_folder(conn, "Brewing", Some(alchemy_id), None)?;
        create_system_folder(conn, "Laboratories", Some(alchemy_id), None)?;
        create_system_folder(conn, "Experiments", Some(alchemy_id), None)?;
        
        let enchanting_id = create_system_folder(conn, "Enchanting", Some(artisan_id), Some("✨"))?;
        create_system_folder(conn, "Rituals", Some(enchanting_id), None)?;
        create_system_folder(conn, "Rune Carving", Some(enchanting_id), None)?;
        create_system_folder(conn, "Magical Infusion", Some(enchanting_id), None)?;
        
        // Daily Life
        let daily_life_id = create_system_folder(conn, "Daily Life", Some(activities_id), Some("🏠"))?;
        
        let cooking_id = create_system_folder(conn, "Cooking", Some(daily_life_id), Some("🍳"))?;
        create_system_folder(conn, "Kitchens", Some(cooking_id), None)?;
        create_system_folder(conn, "Hearths", Some(cooking_id), None)?;
        create_system_folder(conn, "Feasts", Some(cooking_id), None)?;
        
        let training_id = create_system_folder(conn, "Training", Some(daily_life_id), Some("🎯"))?;
        create_system_folder(conn, "Combat Practice", Some(training_id), None)?;
        create_system_folder(conn, "Magic Study", Some(training_id), None)?;
        create_system_folder(conn, "Skill Learning", Some(training_id), None)?;

        // === SFX & FOLEY ===
        let sfx_id = create_system_folder(conn, "SFX & Foley", None, Some("🔊"))?;
        
        // Combat Sounds
        let combat_sounds_id = create_system_folder(conn, "Combat Sounds", Some(sfx_id), Some("⚔️"))?;
        
        let weapon_impacts_id = create_system_folder(conn, "Weapon Impacts", Some(combat_sounds_id), Some("💥"))?;
        create_system_folder(conn, "Sword Clashing", Some(weapon_impacts_id), None)?;
        create_system_folder(conn, "Bow Releases", Some(weapon_impacts_id), None)?;
        create_system_folder(conn, "Gunshots", Some(weapon_impacts_id), None)?;
        create_system_folder(conn, "Explosions", Some(weapon_impacts_id), None)?;
        
        let armor_movement_id = create_system_folder(conn, "Armor & Movement", Some(combat_sounds_id), Some("👟"))?;
        create_system_folder(conn, "Armor Clanking", Some(armor_movement_id), None)?;
        create_system_folder(conn, "Footsteps", Some(armor_movement_id), None)?;
        create_system_folder(conn, "Running", Some(armor_movement_id), None)?;
        create_system_folder(conn, "Sneaking", Some(armor_movement_id), None)?;
        
        // Environment Foley
        let env_foley_id = create_system_folder(conn, "Environment Foley", Some(sfx_id), Some("🌿"))?;
        
        let natural_sounds_id = create_system_folder(conn, "Natural Sounds", Some(env_foley_id), Some("🌊"))?;
        create_system_folder(conn, "Water Dripping", Some(natural_sounds_id), None)?;
        create_system_folder(conn, "River Flowing", Some(natural_sounds_id), None)?;
        create_system_folder(conn, "Wind", Some(natural_sounds_id), None)?;
        create_system_folder(conn, "Thunder", Some(natural_sounds_id), None)?;
        
        let urban_sounds_id = create_system_folder(conn, "Urban Sounds", Some(env_foley_id), Some("🏘️"))?;
        create_system_folder(conn, "Market Crowds", Some(urban_sounds_id), None)?;
        create_system_folder(conn, "Tavern Murmurs", Some(urban_sounds_id), None)?;
        create_system_folder(conn, "Church Bells", Some(urban_sounds_id), None)?;
        create_system_folder(conn, "Door Creaking", Some(urban_sounds_id), None)?;

        // === MUSICAL INSTRUMENTS ===
        let instruments_id = create_system_folder(conn, "Musical Instruments", None, Some("🎵"))?;
        
        // String Instruments
        let strings_id = create_system_folder(conn, "String Instruments", Some(instruments_id), Some("🎻"))?;
        
        let orchestral_strings_id = create_system_folder(conn, "Orchestral", Some(strings_id), Some("🎼"))?;
        create_system_folder(conn, "Warm Strings", Some(orchestral_strings_id), None)?;
        create_system_folder(conn, "Dissonant Strings", Some(orchestral_strings_id), None)?;
        create_system_folder(conn, "Solo Violin", Some(orchestral_strings_id), None)?;
        create_system_folder(conn, "Solo Cello", Some(orchestral_strings_id), None)?;
        
        let folk_strings_id = create_system_folder(conn, "Folk Strings", Some(strings_id), Some("🪕"))?;
        create_system_folder(conn, "Harp", Some(folk_strings_id), None)?;
        create_system_folder(conn, "Lute", Some(folk_strings_id), None)?;
        create_system_folder(conn, "Hurdy-Gurdy", Some(folk_strings_id), None)?;
        
        // Wind Instruments
        let winds_id = create_system_folder(conn, "Wind Instruments", Some(instruments_id), Some("🎺"))?;
        
        let orchestral_winds_id = create_system_folder(conn, "Orchestral Winds", Some(winds_id), Some("🎷"))?;
        create_system_folder(conn, "Flute", Some(orchestral_winds_id), None)?;
        create_system_folder(conn, "Whistle", Some(orchestral_winds_id), None)?;
        create_system_folder(conn, "Low Brass", Some(orchestral_winds_id), None)?;
        
        let folk_winds_id = create_system_folder(conn, "Folk Winds", Some(winds_id), Some("🎵"))?;
        create_system_folder(conn, "Bagpipes", Some(folk_winds_id), None)?;
        create_system_folder(conn, "Recorder", Some(folk_winds_id), None)?;
        
        // Percussion
        let percussion_id = create_system_folder(conn, "Percussion", Some(instruments_id), Some("🥁"))?;
        
        let orchestral_perc_id = create_system_folder(conn, "Orchestral", Some(percussion_id), Some("🎶"))?;
        create_system_folder(conn, "Timpani", Some(orchestral_perc_id), None)?;
        create_system_folder(conn, "Metallic Hits", Some(orchestral_perc_id), None)?;
        
        let folk_perc_id = create_system_folder(conn, "Folk", Some(percussion_id), Some("🪘"))?;
        create_system_folder(conn, "Bodhran", Some(folk_perc_id), None)?;
        create_system_folder(conn, "Frame Drums", Some(folk_perc_id), None)?;

        // === ORGANIZATIONS ===
        let organizations_id = create_system_folder(conn, "Organizations", None, Some("🏛️"))?;
        
        let criminal_id = create_system_folder(conn, "Criminal", Some(organizations_id), Some("🗡️"))?;
        create_system_folder(conn, "Thieves Guilds", Some(criminal_id), None)?;
        create_system_folder(conn, "Cartels", Some(criminal_id), None)?;
        create_system_folder(conn, "Smuggler Networks", Some(criminal_id), None)?;
        create_system_folder(conn, "Pirate Crews", Some(criminal_id), None)?;
        
        let academic_id = create_system_folder(conn, "Academic", Some(organizations_id), Some("📚"))?;
        create_system_folder(conn, "Mages Guilds", Some(academic_id), None)?;
        create_system_folder(conn, "Universities", Some(academic_id), None)?;
        create_system_folder(conn, "Research Institutes", Some(academic_id), None)?;
        create_system_folder(conn, "Scholarly Orders", Some(academic_id), None)?;
        
        let religious_orgs_id = create_system_folder(conn, "Religious", Some(organizations_id), Some("⛪"))?;
        create_system_folder(conn, "Churches", Some(religious_orgs_id), None)?;
        create_system_folder(conn, "Cults", Some(religious_orgs_id), None)?;
        create_system_folder(conn, "Monastic Orders", Some(religious_orgs_id), None)?;
        create_system_folder(conn, "Divine Orders", Some(religious_orgs_id), None)?;
        
        let political_id = create_system_folder(conn, "Political", Some(organizations_id), Some("👑"))?;
        create_system_folder(conn, "Empires", Some(political_id), None)?;
        create_system_folder(conn, "Rebel Groups", Some(political_id), None)?;
        create_system_folder(conn, "Noble Houses", Some(political_id), None)?;
        create_system_folder(conn, "City States", Some(political_id), None)?;

        // === SESSION STRUCTURE ===
        let session_id = create_system_folder(conn, "Session Structure", None, Some("📋"))?;
        
        let opening_id = create_system_folder(conn, "Opening", Some(session_id), Some("🎬"))?;
        create_system_folder(conn, "Recap", Some(opening_id), None)?;
        create_system_folder(conn, "Setting Scene", Some(opening_id), None)?;
        create_system_folder(conn, "Call to Adventure", Some(opening_id), None)?;
        
        let exploration_id = create_system_folder(conn, "Exploration", Some(session_id), Some("🧭"))?;
        create_system_folder(conn, "Travel", Some(exploration_id), None)?;
        create_system_folder(conn, "Discovery", Some(exploration_id), None)?;
        create_system_folder(conn, "Mapping", Some(exploration_id), None)?;
        
        let challenges_id = create_system_folder(conn, "Challenges", Some(session_id), Some("🎯"))?;
        create_system_folder(conn, "Puzzles", Some(challenges_id), None)?;
        create_system_folder(conn, "Traps", Some(challenges_id), None)?;
        create_system_folder(conn, "Social", Some(challenges_id), None)?;
        create_system_folder(conn, "Physical", Some(challenges_id), None)?;
        
        let climax_id = create_system_folder(conn, "Climax", Some(session_id), Some("⚡"))?;
        create_system_folder(conn, "Boss Encounters", Some(climax_id), None)?;
        create_system_folder(conn, "Major Revelations", Some(climax_id), None)?;
        create_system_folder(conn, "Key Decisions", Some(climax_id), None)?;
        
        let resolution_id = create_system_folder(conn, "Resolution", Some(session_id), Some("🎊"))?;
        create_system_folder(conn, "Victory Celebration", Some(resolution_id), None)?;
        create_system_folder(conn, "Character Development", Some(resolution_id), None)?;
        create_system_folder(conn, "Next Steps", Some(resolution_id), None)?;

        // === CULTURAL STYLES ===
        let cultural_id = create_system_folder(conn, "Cultural Styles", None, Some("🌍"))?;
        
        // Ancient Civilizations
        let ancient_id = create_system_folder(conn, "Ancient Civilizations", Some(cultural_id), Some("🏛️"))?;
        
        let greek_id = create_system_folder(conn, "Ancient Greek", Some(ancient_id), Some("🏛️"))?;
        create_system_folder(conn, "Temples", Some(greek_id), None)?;
        create_system_folder(conn, "Agoras", Some(greek_id), None)?;
        create_system_folder(conn, "Battlefields", Some(greek_id), None)?;
        
        let roman_id = create_system_folder(conn, "Ancient Roman", Some(ancient_id), Some("🏟️"))?;
        create_system_folder(conn, "Forums", Some(roman_id), None)?;
        create_system_folder(conn, "Colosseums", Some(roman_id), None)?;
        create_system_folder(conn, "Legions", Some(roman_id), None)?;
        
        let egyptian_id = create_system_folder(conn, "Egyptian", Some(ancient_id), Some("🔺"))?;
        create_system_folder(conn, "Pyramids", Some(egyptian_id), None)?;
        create_system_folder(conn, "Temples", Some(egyptian_id), None)?;
        create_system_folder(conn, "Deserts", Some(egyptian_id), None)?;
        
        // Medieval & Renaissance
        let medieval_id = create_system_folder(conn, "Medieval & Renaissance", Some(cultural_id), Some("🏰"))?;
        
        let medieval_euro_id = create_system_folder(conn, "Medieval European", Some(medieval_id), Some("⚔️"))?;
        create_system_folder(conn, "Castles", Some(medieval_euro_id), None)?;
        create_system_folder(conn, "Villages", Some(medieval_euro_id), None)?;
        create_system_folder(conn, "Monasteries", Some(medieval_euro_id), None)?;
        
        let renaissance_id = create_system_folder(conn, "Renaissance", Some(medieval_id), Some("🎨"))?;
        create_system_folder(conn, "Courts", Some(renaissance_id), None)?;
        create_system_folder(conn, "Art Studios", Some(renaissance_id), None)?;
        create_system_folder(conn, "City States", Some(renaissance_id), None)?;
        
        // Eastern Traditions
        let eastern_id = create_system_folder(conn, "Eastern Traditions", Some(cultural_id), Some("🈴"))?;
        
        let japanese_id = create_system_folder(conn, "Japanese Traditional", Some(eastern_id), Some("⛩️"))?;
        create_system_folder(conn, "Temples", Some(japanese_id), None)?;
        create_system_folder(conn, "Dojos", Some(japanese_id), None)?;
        create_system_folder(conn, "Gardens", Some(japanese_id), None)?;
        
        let chinese_id = create_system_folder(conn, "Chinese Traditional", Some(eastern_id), Some("🏮"))?;
        create_system_folder(conn, "Palaces", Some(chinese_id), None)?;
        create_system_folder(conn, "Markets", Some(chinese_id), None)?;
        create_system_folder(conn, "Monasteries", Some(chinese_id), None)?;

        Ok(())
    }

    // Tag-based folder suggestion functions
    
    /// Get folder suggestions for a file based on its RPG tags
    pub fn suggest_folders_for_file(conn: &Connection, audio_file_id: i64, limit: Option<usize>) -> Result<Vec<(VirtualFolder, f64)>> {
        let limit = limit.unwrap_or(5);
        
        // Get all tags for the file
        let file_tags = Self::get_file_tags(conn, audio_file_id)?;
        
        if file_tags.is_empty() {
            return Ok(Vec::new());
        }
        
        // Get all folders and their common tags with scoring
        let mut folder_scores: Vec<(VirtualFolder, f64)> = Vec::new();
        let folders = Self::get_all_virtual_folders(conn)?;
        
        for folder in folders {
            let score = Self::calculate_folder_tag_score(conn, folder.id.unwrap(), &file_tags)?;
            if score > 0.0 {
                folder_scores.push((folder, score));
            }
        }
        
        // Sort by score descending and limit results
        folder_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        folder_scores.truncate(limit);
        
        Ok(folder_scores)
    }
    
    /// Calculate similarity score between a folder and a set of tags
    fn calculate_folder_tag_score(conn: &Connection, folder_id: i64, file_tags: &[String]) -> Result<f64> {
        // Get all tags from files currently in this folder
        let mut stmt = conn.prepare(
            "SELECT DISTINCT rt.tag_type, rt.tag_value
             FROM rpg_tags rt
             JOIN virtual_folder_contents vfc ON rt.audio_file_id = vfc.audio_file_id
             WHERE vfc.folder_id = ?"
        )?;
        
        let folder_tags: Vec<String> = stmt.query_map([folder_id], |row| {
            let tag_type: String = row.get(0)?;
            let tag_value: String = row.get(1)?;
            Ok(format!("{}:{}", tag_type, tag_value))
        })?.collect::<Result<Vec<_>, _>>()?;
        
        if folder_tags.is_empty() {
            return Ok(0.0);
        }
        
        // Calculate Jaccard similarity coefficient
        let file_tags_set: std::collections::HashSet<&String> = file_tags.iter().collect();
        let folder_tags_set: std::collections::HashSet<&String> = folder_tags.iter().collect();
        
        let intersection = file_tags_set.intersection(&folder_tags_set).count();
        let union = file_tags_set.union(&folder_tags_set).count();
        
        if union == 0 {
            Ok(0.0)
        } else {
            Ok(intersection as f64 / union as f64)
        }
    }
    
    /// Get all RPG tags for a file in "type:value" format
    fn get_file_tags(conn: &Connection, audio_file_id: i64) -> Result<Vec<String>> {
        let mut stmt = conn.prepare(
            "SELECT tag_type, tag_value FROM rpg_tags WHERE audio_file_id = ?"
        )?;
        
        let tags: Vec<String> = stmt.query_map([audio_file_id], |row| {
            let tag_type: String = row.get(0)?;
            let tag_value: String = row.get(1)?;
            Ok(format!("{}:{}", tag_type, tag_value))
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(tags)
    }
    
    /// Get auto-organization suggestions based on tag patterns
    pub fn get_auto_organization_suggestions(conn: &Connection, threshold: f64) -> Result<Vec<AutoOrganizationSuggestion>> {
        let mut suggestions = Vec::new();
        
        // Find files not in any folder
        let unorganized_files = Self::get_unorganized_files(conn)?;
        
        for file in unorganized_files {
            let folder_suggestions = Self::suggest_folders_for_file(conn, file.id.unwrap(), Some(3))?;
            
            // Only suggest if confidence is above threshold
            if let Some((folder, score)) = folder_suggestions.first() {
                if *score >= threshold {
                    suggestions.push(AutoOrganizationSuggestion {
                        audio_file_id: file.id.unwrap(),
                        audio_file_title: file.title.unwrap_or_else(|| "Unknown".to_string()),
                        suggested_folder_id: folder.id.unwrap(),
                        suggested_folder_name: folder.name.clone(),
                        confidence_score: *score,
                        matching_tags: Self::get_matching_tags(conn, file.id.unwrap(), folder.id.unwrap())?,
                    });
                }
            }
        }
        
        // Sort by confidence score descending
        suggestions.sort_by(|a, b| b.confidence_score.partial_cmp(&a.confidence_score).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(suggestions)
    }
    
    /// Get files that are not in any virtual folder
    fn get_unorganized_files(conn: &Connection) -> Result<Vec<AudioFile>> {
        let mut stmt = conn.prepare(
            "SELECT af.* FROM audio_files af
             LEFT JOIN virtual_folder_contents vfc ON af.id = vfc.audio_file_id
             WHERE vfc.audio_file_id IS NULL"
        )?;
        
        let files = stmt.query_map([], |row| {
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
                // Add other fields as needed
                ..Default::default()
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(files)
    }
    
    /// Get tags that match between a file and folder
    pub fn get_matching_tags(conn: &Connection, audio_file_id: i64, folder_id: i64) -> Result<Vec<String>> {
        let file_tags = Self::get_file_tags(conn, audio_file_id)?;
        
        let mut stmt = conn.prepare(
            "SELECT DISTINCT rt.tag_type, rt.tag_value
             FROM rpg_tags rt
             JOIN virtual_folder_contents vfc ON rt.audio_file_id = vfc.audio_file_id
             WHERE vfc.folder_id = ?"
        )?;
        
        let folder_tags: Vec<String> = stmt.query_map([folder_id], |row| {
            let tag_type: String = row.get(0)?;
            let tag_value: String = row.get(1)?;
            Ok(format!("{}:{}", tag_type, tag_value))
        })?.collect::<Result<Vec<_>, _>>()?;
        
        let file_tags_set: std::collections::HashSet<&String> = file_tags.iter().collect();
        let folder_tags_set: std::collections::HashSet<&String> = folder_tags.iter().collect();
        
        let matching: Vec<String> = file_tags_set.intersection(&folder_tags_set)
            .map(|s| (*s).clone())
            .collect();
        
        Ok(matching)
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