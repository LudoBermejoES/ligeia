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
}