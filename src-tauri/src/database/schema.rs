use rusqlite::{Connection, Result};

/// Handles database schema creation and migration
pub struct SchemaManager;

impl SchemaManager {
    pub fn new(_conn: &Connection) -> Self {
        SchemaManager
    }

    pub fn create_tables(&self, conn: &Connection) -> Result<()> {
        self.create_audio_files_table(conn)?;
        self.add_extended_columns(conn)?;
        self.create_rpg_tags_table(conn)?;
        self.create_vocabulary_table(conn)?;
        self.create_atmospheres_tables(conn)?;
        self.create_virtual_folders_tables(conn)?;
        self.create_indexes(conn)?;
        Ok(())
    }

    fn create_audio_files_table(&self, conn: &Connection) -> Result<()> {
        // Create basic audio_files table first (for backward compatibility)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS audio_files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                file_path TEXT UNIQUE NOT NULL,
                title TEXT,
                artist TEXT,
                album TEXT,
                duration REAL,
                genre TEXT,
                year INTEGER,
                track_number INTEGER
            )",
            [],
        )?;
        Ok(())
    }

    fn add_extended_columns(&self, conn: &Connection) -> Result<()> {
        // Add extended columns if they don't exist (migration)
        let extended_columns = vec![
            ("album_artist", "TEXT"),
            ("date", "TEXT"),
            ("total_tracks", "INTEGER"),
            ("disc_number", "INTEGER"),
            ("total_discs", "INTEGER"),
            ("composer", "TEXT"),
            ("conductor", "TEXT"),
            ("lyricist", "TEXT"),
            ("original_artist", "TEXT"),
            ("remixer", "TEXT"),
            ("arranger", "TEXT"),
            ("engineer", "TEXT"),
            ("producer", "TEXT"),
            ("dj_mixer", "TEXT"),
            ("mixer", "TEXT"),
            ("content_group", "TEXT"),
            ("subtitle", "TEXT"),
            ("initial_key", "TEXT"),
            ("bpm", "INTEGER"),
            ("language", "TEXT"),
            ("media_type", "TEXT"),
            ("original_filename", "TEXT"),
            ("original_lyricist", "TEXT"),
            ("original_release_time", "TEXT"),
            ("playlist_delay", "INTEGER"),
            ("recording_time", "TEXT"),
            ("release_time", "TEXT"),
            ("tagging_time", "TEXT"),
            ("encoding_time", "TEXT"),
            ("encoding_settings", "TEXT"),
            ("encoded_by", "TEXT"),
            ("copyright", "TEXT"),
            ("file_owner", "TEXT"),
            ("internet_radio_station_name", "TEXT"),
            ("internet_radio_station_owner", "TEXT"),
            ("isrc", "TEXT"),
            ("publisher", "TEXT"),
            ("mood", "TEXT"),
            ("occasion", "TEXT"),
            ("tempo", "TEXT"),
            ("content_type", "TEXT"),
            ("category", "TEXT"),
            ("created_at", "DATETIME DEFAULT CURRENT_TIMESTAMP"),
            ("updated_at", "DATETIME DEFAULT CURRENT_TIMESTAMP"),
        ];

        // Add each column if it doesn't exist
        for (column_name, column_type) in extended_columns {
            let alter_sql = format!("ALTER TABLE audio_files ADD COLUMN {} {}", column_name, column_type);
            // Ignore errors for columns that already exist
            let _ = conn.execute(&alter_sql, []);
        }

        Ok(())
    }

    fn create_rpg_tags_table(&self, conn: &Connection) -> Result<()> {
        // New RPG tags table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS rpg_tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                audio_file_id INTEGER NOT NULL,
                tag_type TEXT NOT NULL,
                tag_value TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (audio_file_id) REFERENCES audio_files (id) ON DELETE CASCADE,
                UNIQUE(audio_file_id, tag_type, tag_value)
            )",
            [],
        )?;
        Ok(())
    }

    fn create_vocabulary_table(&self, conn: &Connection) -> Result<()> {
        // Tag vocabulary table for controlled vocabularies
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tag_vocabulary (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                tag_type TEXT NOT NULL,
                tag_value TEXT NOT NULL,
                description TEXT,
                parent_tag TEXT,
                is_active BOOLEAN DEFAULT TRUE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(tag_type, tag_value)
            )",
            [],
        )?;
        Ok(())
    }

    fn create_atmospheres_tables(&self, conn: &Connection) -> Result<()> {
        // Atmospheres table (if not already exists from previous implementation)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS atmospheres (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                title TEXT,
                description TEXT,
                category TEXT,
                subcategory TEXT,
                keywords TEXT, -- JSON array
                default_crossfade_ms INTEGER DEFAULT 2500,
                fade_curve TEXT DEFAULT 'linear',
                theme TEXT DEFAULT 'default',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Atmosphere sounds table (if not already exists)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS atmosphere_sounds (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                atmosphere_id INTEGER NOT NULL,
                audio_file_id INTEGER NOT NULL,
                volume REAL DEFAULT 1.0,
                is_looping BOOLEAN DEFAULT FALSE,
                is_muted BOOLEAN DEFAULT FALSE,
                min_seconds INTEGER DEFAULT 0,
                max_seconds INTEGER DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (atmosphere_id) REFERENCES atmospheres (id) ON DELETE CASCADE,
                FOREIGN KEY (audio_file_id) REFERENCES audio_files (id) ON DELETE CASCADE,
                UNIQUE(atmosphere_id, audio_file_id)
            )",
            [],
        )?;

        Ok(())
    }

    fn create_virtual_folders_tables(&self, conn: &Connection) -> Result<()> {
        // Virtual folders table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS virtual_folders (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name VARCHAR(255) NOT NULL,
                description TEXT,
                parent_folder_id INTEGER,
                color VARCHAR(7), -- Hex color code for UI theming
                icon VARCHAR(50), -- Icon identifier for UI display
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                created_by VARCHAR(100), -- User identifier for multi-user support
                folder_order INTEGER DEFAULT 0, -- Manual ordering within parent
                is_system_folder BOOLEAN DEFAULT FALSE, -- System vs user-created folders
                metadata TEXT, -- JSON metadata storage
                FOREIGN KEY (parent_folder_id) REFERENCES virtual_folders (id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Virtual folder contents table (many-to-many relationship)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS virtual_folder_contents (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                folder_id INTEGER NOT NULL,
                audio_file_id INTEGER NOT NULL,
                added_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                added_by VARCHAR(100), -- User who added the file
                file_order INTEGER DEFAULT 0, -- Manual ordering within folder
                notes TEXT, -- User notes about why this file is in this folder
                FOREIGN KEY (folder_id) REFERENCES virtual_folders (id) ON DELETE CASCADE,
                FOREIGN KEY (audio_file_id) REFERENCES audio_files (id) ON DELETE CASCADE,
                UNIQUE(folder_id, audio_file_id) -- Prevent duplicate entries
            )",
            [],
        )?;

        // Folder templates table (optional: predefined folder structures)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS folder_templates (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name VARCHAR(255) NOT NULL,
                description TEXT,
                template_data TEXT, -- JSON hierarchical folder structure
                category VARCHAR(100), -- RPG, Campaign, Mood, etc.
                is_public BOOLEAN DEFAULT TRUE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                created_by VARCHAR(100)
            )",
            [],
        )?;

        Ok(())
    }

    fn create_indexes(&self, conn: &Connection) -> Result<()> {
        // Indexes for performance
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_rpg_tags_audio_file ON rpg_tags(audio_file_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_rpg_tags_type_value ON rpg_tags(tag_type, tag_value)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tag_vocabulary_type ON tag_vocabulary(tag_type)",
            [],
        )?;

        // Virtual folder indexes
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_virtual_folders_parent ON virtual_folders(parent_folder_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_virtual_folders_name ON virtual_folders(name)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_virtual_folders_order ON virtual_folders(folder_order)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_folder_contents_folder ON virtual_folder_contents(folder_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_folder_contents_audio ON virtual_folder_contents(audio_file_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_folder_contents_order ON virtual_folder_contents(file_order)",
            [],
        )?;

        // Atmosphere indexes (if not already created)
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_atmosphere_sounds_atmosphere ON atmosphere_sounds(atmosphere_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_atmosphere_sounds_audio ON atmosphere_sounds(audio_file_id)",
            [],
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_in_memory() -> Connection {
        let conn = Connection::open_in_memory().expect("open in-memory db");
        // Ensure foreign keys if needed later
        conn.execute("PRAGMA foreign_keys = ON", []).ok();
        let schema = SchemaManager::new(&conn);
        schema.create_tables(&conn).expect("create tables");
        conn
    }

    #[test]
    fn creates_audio_files_with_extended_columns() {
        let conn = setup_in_memory();
        let mut stmt = conn
            .prepare("PRAGMA table_info(audio_files)")
            .expect("pragma");
        let cols: Vec<String> = stmt
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        // Base
        assert!(cols.contains(&"file_path".to_string()));
        assert!(cols.contains(&"title".to_string()));
        // Extended examples
        assert!(cols.contains(&"album_artist".to_string()));
        assert!(cols.contains(&"bpm".to_string()));
        assert!(cols.contains(&"occasion".to_string()));
        assert!(cols.contains(&"category".to_string()));
    }

    #[test]
    fn creates_rpg_tags_and_vocabulary() {
        let conn = setup_in_memory();
        // rpg_tags exists
        conn.prepare("SELECT 1 FROM rpg_tags WHERE 1=0").unwrap();
        // tag_vocabulary exists
        conn.prepare("SELECT 1 FROM tag_vocabulary WHERE 1=0").unwrap();
    }

    #[test]
    fn creates_virtual_folders_tables() {
        let conn = setup_in_memory();
        
        // virtual_folders exists
        conn.prepare("SELECT 1 FROM virtual_folders WHERE 1=0").unwrap();
        
        // virtual_folder_contents exists
        conn.prepare("SELECT 1 FROM virtual_folder_contents WHERE 1=0").unwrap();
        
        // folder_templates exists
        conn.prepare("SELECT 1 FROM folder_templates WHERE 1=0").unwrap();
        
        // Check foreign key constraint exists
        let mut stmt = conn.prepare("PRAGMA foreign_key_list(virtual_folders)").unwrap();
        let foreign_keys: Vec<String> = stmt
            .query_map([], |row| row.get::<_, String>(2))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert!(foreign_keys.contains(&"virtual_folders".to_string()));
    }

    #[test]
    fn creates_virtual_folder_indexes() {
        let conn = setup_in_memory();
        
        // Check some key indexes exist
        let mut stmt = conn.prepare("PRAGMA index_list(virtual_folders)").unwrap();
        let index_names: Vec<String> = stmt
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        
        assert!(index_names.iter().any(|name| name.contains("parent")));
        assert!(index_names.iter().any(|name| name.contains("name")));
    }
}