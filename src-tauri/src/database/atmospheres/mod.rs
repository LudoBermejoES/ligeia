mod crud;
mod sounds;
mod integrity;
mod search;
mod helpers;


use rusqlite::{Connection, Result};

/// Database operations for atmospheres
pub struct AtmosphereOps;

impl AtmosphereOps {
    /// Create atmosphere tables
    pub fn create_tables(conn: &Connection) -> Result<()> {
        // Create atmospheres table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS atmospheres (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                category TEXT NOT NULL DEFAULT '',
                subcategory TEXT NOT NULL DEFAULT '',
                subsubcategory TEXT,
                keywords TEXT NOT NULL DEFAULT '', -- JSON array as text
                background_image TEXT,
                author_image TEXT,
                is_public BOOLEAN DEFAULT FALSE,
                theme TEXT DEFAULT 'default',
                default_crossfade_ms INTEGER DEFAULT 2500,
                fade_curve TEXT NOT NULL DEFAULT 'linear',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Backfill new columns if database pre-existed without them
        helpers::ensure_columns(conn)?;

        // Create atmosphere_sounds mapping table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS atmosphere_sounds (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                atmosphere_id INTEGER NOT NULL,
                audio_file_id INTEGER NOT NULL,
                volume REAL DEFAULT 0.5,
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

        // Add columns if they don't exist (for existing databases)
        helpers::ensure_atmosphere_sounds_columns(conn)?;

        // Create atmosphere categories table (predefined categories)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS atmosphere_categories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                parent_id INTEGER,
                display_order INTEGER DEFAULT 0,
                FOREIGN KEY (parent_id) REFERENCES atmosphere_categories (id)
            )",
            [],
        )?;

        // Create indexes for performance
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_atmosphere_sounds_atmosphere ON atmosphere_sounds(atmosphere_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_atmosphere_sounds_audio_file ON atmosphere_sounds(audio_file_id)",
            [],
        )?;

        // Initialize default categories if empty
        helpers::initialize_default_categories(conn)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use crate::models::Atmosphere;

    fn create_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        // Create audio_files table first (dependency)
        conn.execute(
            "CREATE TABLE audio_files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                file_path TEXT NOT NULL UNIQUE,
                title TEXT,
                artist TEXT,
                album TEXT,
                duration REAL
            )",
            [],
        ).unwrap();
        AtmosphereOps::create_tables(&conn).unwrap();
        conn
    }

    #[test]
    fn test_create_tables() {
        let conn = create_test_db();
        
        // Verify tables exist
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='atmospheres'",
            [],
            |row| row.get(0)
        ).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_crud_operations() {
        let conn = create_test_db();
        
        // Create atmosphere
        let mut atmosphere = Atmosphere {
            id: None,
            name: "Test Atmosphere".to_string(),
            title: "Test Title".to_string(),
            description: "Test Description".to_string(),
            category: "Test".to_string(),
            subcategory: "".to_string(),
            subsubcategory: None,
            keywords: vec!["test".to_string(), "atmosphere".to_string()],
            background_image: None,
            author_image: None,
            is_public: false,
            theme: Some("default".to_string()),
            default_crossfade_ms: 2500,
            fade_curve: "linear".to_string(),
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };
        
        let id = AtmosphereOps::save(&conn, &atmosphere).unwrap();
        assert!(id > 0);
        
        // Get by ID
        let loaded = AtmosphereOps::get_by_id(&conn, id).unwrap();
        assert_eq!(loaded.name, "Test Atmosphere");
        
        // Update
        atmosphere.id = Some(id);
        atmosphere.description = "Updated Description".to_string();
        AtmosphereOps::save(&conn, &atmosphere).unwrap();
        
        let updated = AtmosphereOps::get_by_id(&conn, id).unwrap();
        assert_eq!(updated.description, "Updated Description");
        
        // Delete
        AtmosphereOps::delete(&conn, id).unwrap();
        assert!(AtmosphereOps::get_by_id(&conn, id).is_err());
    }
}