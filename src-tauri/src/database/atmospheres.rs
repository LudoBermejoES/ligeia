use rusqlite::{Connection, params, Result};
use crate::models::{Atmosphere, AtmosphereSoundMapping, AtmosphereWithSounds, AudioFile, AtmosphereCategory};
use crate::models::{AtmosphereIntegrity, AtmosphereIntegrityBatchEntry};

/// Repository for atmosphere operations
pub struct AtmosphereRepository;

impl AtmosphereRepository {
    pub fn new() -> Self {
        AtmosphereRepository
    }

    /// Create atmosphere tables
    pub fn create_tables(&self, conn: &Connection) -> Result<()> {
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
                default_crossfade_ms INTEGER DEFAULT 2500,
                fade_curve TEXT NOT NULL DEFAULT 'linear',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Backfill new columns if database pre-existed without them
        // SQLite doesn't support ADD COLUMN IF NOT EXISTS directly for older versions, so probe pragma
        let mut stmt = conn.prepare("PRAGMA table_info(atmospheres)")?;
        let mut has_crossfade = false;
        let mut has_curve = false;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let col_name: String = row.get(1)?; // 1 = name
            if col_name == "default_crossfade_ms" { has_crossfade = true; }
            if col_name == "fade_curve" { has_curve = true; }
        }
        if !has_crossfade {
            conn.execute("ALTER TABLE atmospheres ADD COLUMN default_crossfade_ms INTEGER DEFAULT 2500", [])?;
        }
        if !has_curve {
            conn.execute("ALTER TABLE atmospheres ADD COLUMN fade_curve TEXT NOT NULL DEFAULT 'linear'", [])?;
        }

        // Create atmosphere_sounds mapping table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS atmosphere_sounds (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                atmosphere_id INTEGER NOT NULL,
                audio_file_id INTEGER NOT NULL,
                volume REAL DEFAULT 0.5,
                is_looping BOOLEAN DEFAULT FALSE,
                is_muted BOOLEAN DEFAULT FALSE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (atmosphere_id) REFERENCES atmospheres (id) ON DELETE CASCADE,
                FOREIGN KEY (audio_file_id) REFERENCES audio_files (id) ON DELETE CASCADE,
                UNIQUE(atmosphere_id, audio_file_id)
            )",
            [],
        )?;

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
        self.initialize_default_categories(conn)?;

        Ok(())
    }

    /// Save or update an atmosphere
    pub fn save(&self, conn: &Connection, atmosphere: &Atmosphere) -> Result<i64> {
        let keywords_json = serde_json::to_string(&atmosphere.keywords).unwrap_or_default();
        
        match atmosphere.id {
            Some(id) => {
                // Update existing
                conn.execute(
                    "UPDATE atmospheres SET 
                        name = ?1, title = ?2, description = ?3, category = ?4, 
                        subcategory = ?5, subsubcategory = ?6, keywords = ?7,
                        background_image = ?8, author_image = ?9, is_public = ?10,
                                default_crossfade_ms = ?11, fade_curve = ?12,
                        updated_at = CURRENT_TIMESTAMP 
                            WHERE id = ?13",
                    params![
                        atmosphere.name, atmosphere.title, atmosphere.description,
                        atmosphere.category, atmosphere.subcategory, atmosphere.subsubcategory,
                        keywords_json, atmosphere.background_image, atmosphere.author_image,
                                atmosphere.is_public, atmosphere.default_crossfade_ms, atmosphere.fade_curve,
                                id
                    ],
                )?;
                Ok(id)
            }
            None => {
                // Create new
                conn.execute(
                    "INSERT INTO atmospheres (
                        name, title, description, category, subcategory, subsubcategory,
                        keywords, background_image, author_image, is_public,
                        default_crossfade_ms, fade_curve
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                    params![
                        atmosphere.name, atmosphere.title, atmosphere.description,
                        atmosphere.category, atmosphere.subcategory, atmosphere.subsubcategory,
                        keywords_json, atmosphere.background_image, atmosphere.author_image,
                        atmosphere.is_public, atmosphere.default_crossfade_ms, atmosphere.fade_curve
                    ],
                )?;
                Ok(conn.last_insert_rowid())
            }
        }
    }

    /// Get all atmospheres
    pub fn get_all(&self, conn: &Connection) -> Result<Vec<Atmosphere>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, title, description, category, subcategory, subsubcategory,
                    keywords, background_image, author_image, is_public, default_crossfade_ms, fade_curve, created_at, updated_at
             FROM atmospheres ORDER BY updated_at DESC"
        )?;

        let rows = stmt.query_map([], |row| {
            let keywords_json: String = row.get(7)?;
            let keywords: Vec<String> = serde_json::from_str(&keywords_json).unwrap_or_default();
            
            Ok(Atmosphere {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                category: row.get(4)?,
                subcategory: row.get(5)?,
                subsubcategory: row.get(6)?,
                keywords,
                background_image: row.get(8)?,
                author_image: row.get(9)?,
                is_public: row.get(10)?,
                default_crossfade_ms: row.get(11).unwrap_or(2500),
                fade_curve: row.get(12).unwrap_or(String::from("linear")),
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
            })
        })?;

        let mut atmospheres = Vec::new();
        for row in rows {
            atmospheres.push(row?);
        }
        Ok(atmospheres)
    }

    /// Get atmosphere by ID
    pub fn get_by_id(&self, conn: &Connection, id: i64) -> Result<Atmosphere> {
        let mut stmt = conn.prepare(
            "SELECT id, name, title, description, category, subcategory, subsubcategory,
                    keywords, background_image, author_image, is_public, default_crossfade_ms, fade_curve, created_at, updated_at
             FROM atmospheres WHERE id = ?1"
        )?;

        stmt.query_row([id], |row| {
            let keywords_json: String = row.get(7)?;
            let keywords: Vec<String> = serde_json::from_str(&keywords_json).unwrap_or_default();
            
            Ok(Atmosphere {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                category: row.get(4)?,
                subcategory: row.get(5)?,
                subsubcategory: row.get(6)?,
                keywords,
                background_image: row.get(8)?,
                author_image: row.get(9)?,
                is_public: row.get(10)?,
                default_crossfade_ms: row.get(11).unwrap_or(2500),
                fade_curve: row.get(12).unwrap_or(String::from("linear")),
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
            })
        })
    }

    /// Delete atmosphere
    pub fn delete(&self, conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM atmospheres WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Add sound to atmosphere
    pub fn add_sound(&self, conn: &Connection, atmosphere_id: i64, audio_file_id: i64, volume: f32, is_looping: bool) -> Result<i64> {
        conn.execute(
            "INSERT OR REPLACE INTO atmosphere_sounds 
             (atmosphere_id, audio_file_id, volume, is_looping, is_muted)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![atmosphere_id, audio_file_id, volume, is_looping, false],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Remove sound from atmosphere
    pub fn remove_sound(&self, conn: &Connection, atmosphere_id: i64, audio_file_id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM atmosphere_sounds WHERE atmosphere_id = ?1 AND audio_file_id = ?2",
            params![atmosphere_id, audio_file_id],
        )?;
        Ok(())
    }

    /// Update sound settings in atmosphere
    pub fn update_sound(&self, conn: &Connection, atmosphere_id: i64, audio_file_id: i64, volume: f32, is_looping: bool, is_muted: bool) -> Result<()> {
        conn.execute(
            "UPDATE atmosphere_sounds SET volume = ?3, is_looping = ?4, is_muted = ?5
             WHERE atmosphere_id = ?1 AND audio_file_id = ?2",
            params![atmosphere_id, audio_file_id, volume, is_looping, is_muted],
        )?;
        Ok(())
    }

    /// Get atmosphere with all its sounds
    pub fn get_with_sounds(&self, conn: &Connection, atmosphere_id: i64) -> Result<AtmosphereWithSounds> {
        let atmosphere = self.get_by_id(conn, atmosphere_id)?;
        
        // Get sound mappings
        let mut stmt = conn.prepare(
            "SELECT id, atmosphere_id, audio_file_id, volume, is_looping, is_muted, created_at
             FROM atmosphere_sounds WHERE atmosphere_id = ?1 ORDER BY created_at"
        )?;

        let mapping_rows = stmt.query_map([atmosphere_id], |row| {
            Ok(AtmosphereSoundMapping {
                id: Some(row.get(0)?),
                atmosphere_id: row.get(1)?,
                audio_file_id: row.get(2)?,
                volume: row.get(3)?,
                is_looping: row.get(4)?,
                is_muted: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;

        let mut sounds = Vec::new();
        let mut audio_file_ids = Vec::new();
        
        for row in mapping_rows {
            let mapping = row?;
            audio_file_ids.push(mapping.audio_file_id);
            sounds.push(mapping);
        }

        // Get audio files
        let mut audio_files = Vec::new();
        for audio_file_id in audio_file_ids {
            if let Ok(audio_file) = self.get_audio_file_by_id(conn, audio_file_id) {
                audio_files.push(audio_file);
            }
        }

        Ok(AtmosphereWithSounds {
            atmosphere,
            sounds,
            audio_files,
        })
    }

    /// Get all categories
    pub fn get_categories(&self, conn: &Connection) -> Result<Vec<AtmosphereCategory>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, parent_id FROM atmosphere_categories ORDER BY parent_id, display_order, name"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(AtmosphereCategory {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get(2)?,
            })
        })?;

        let mut categories = Vec::new();
        for row in rows {
            categories.push(row?);
        }
        Ok(categories)
    }
    
    /// Duplicate an atmosphere (metadata + sound mappings). Optionally provide a new base name.
    pub fn duplicate(&self, conn: &Connection, id: i64, new_name: Option<&str>) -> Result<i64> {
        let mut stmt = conn.prepare(
            "SELECT name, title, description, category, subcategory, subsubcategory, keywords, background_image, author_image, is_public, default_crossfade_ms, fade_curve FROM atmospheres WHERE id = ?1"
        )?;
        let row = stmt.query_row([id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, Option<String>>(8)?,
                row.get::<_, bool>(9)?,
                row.get::<_, i64>(10)?,
                row.get::<_, String>(11)?,
            ))
        })?;
        let (orig_name, _orig_title, description, category, subcategory, subsubcategory, keywords_json, background_image, author_image, is_public, default_crossfade_ms, fade_curve) = row;
        let base = new_name.map(|s| s.trim()).filter(|s| !s.is_empty()).unwrap_or_else(|| orig_name.as_str());
        let mut candidate = format!("{} (Copy)", base);
        let mut counter = 2;
        while self.name_exists(conn, &candidate)? {
            candidate = format!("{} (Copy {})", base, counter);
            counter += 1;
        }
        let final_name = candidate;
        let final_title = final_name.clone();
        conn.execute(
            "INSERT INTO atmospheres (name, title, description, category, subcategory, subsubcategory, keywords, background_image, author_image, is_public, default_crossfade_ms, fade_curve) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![final_name, final_title, description, category, subcategory, subsubcategory, keywords_json, background_image, author_image, is_public, default_crossfade_ms, fade_curve]
        )?;
        let new_id = conn.last_insert_rowid();
        conn.execute(
            "INSERT INTO atmosphere_sounds (atmosphere_id, audio_file_id, volume, is_looping, is_muted) SELECT ?1, audio_file_id, volume, is_looping, is_muted FROM atmosphere_sounds WHERE atmosphere_id = ?2",
            params![new_id, id]
        )?;
        Ok(new_id)
    }

    fn name_exists(&self, conn: &Connection, name: &str) -> Result<bool> {
        let mut stmt = conn.prepare("SELECT 1 FROM atmospheres WHERE name = ?1 LIMIT 1")?;
        Ok(stmt.exists([name])?)
    }

    /// Compute integrity: which mapped audio_file_ids are missing in audio_files table
    pub fn compute_integrity(&self, conn: &Connection, atmosphere_id: i64) -> Result<AtmosphereIntegrity> {
        // Collect mapped ids
        let mut stmt = conn.prepare("SELECT audio_file_id FROM atmosphere_sounds WHERE atmosphere_id = ?1")?;
        let mapped: Result<Vec<i64>> = stmt.query_map([atmosphere_id], |row| row.get(0))?.collect();
        let mapped = mapped?;
        if mapped.is_empty() {
            return Ok(AtmosphereIntegrity { atmosphere_id, missing_ids: Vec::new() });
        }
        // Build placeholders
        let placeholders = mapped.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let sql = format!("SELECT id FROM audio_files WHERE id IN ({})", placeholders);
        let mut stmt2 = conn.prepare(&sql)?;
        let existing: Result<Vec<i64>> = stmt2.query_map(rusqlite::params_from_iter(mapped.iter()), |row| row.get(0))?.collect();
        let existing_set: std::collections::HashSet<i64> = existing?.into_iter().collect();
        let missing_ids: Vec<i64> = mapped.into_iter().filter(|id| !existing_set.contains(id)).collect();
        Ok(AtmosphereIntegrity { atmosphere_id, missing_ids })
    }

    /// Batch compute integrity for all atmospheres (more efficient than per-atmosphere calls)
    pub fn compute_all_integrities(&self, conn: &Connection) -> Result<Vec<AtmosphereIntegrityBatchEntry>> {
        let sql = "SELECT a.id as atmosphere_id,
                          GROUP_CONCAT(CASE WHEN af.id IS NULL THEN s.audio_file_id END) AS missing_csv
                   FROM atmospheres a
                   LEFT JOIN atmosphere_sounds s ON s.atmosphere_id = a.id
                   LEFT JOIN audio_files af ON af.id = s.audio_file_id
                   GROUP BY a.id";
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            let atmosphere_id: i64 = row.get(0)?;
            let missing_csv: Option<String> = row.get(1)?;
            let missing_ids: Vec<i64> = missing_csv
                .unwrap_or_default()
                .split(',')
                .filter_map(|s| s.trim().parse::<i64>().ok())
                .collect();
            Ok(AtmosphereIntegrityBatchEntry { atmosphere_id, missing_ids })
        })?;
        let mut out = Vec::new();
        for r in rows { out.push(r?); }
        Ok(out)
    }

    /// Search atmospheres by free text (name/title/description/category/subcategory) + optional category + keywords list
    pub fn search(&self, conn: &Connection, query: Option<&str>, category: Option<&str>, keywords: Option<&[String]>) -> Result<Vec<Atmosphere>> {
        // Base select
        let mut sql = String::from("SELECT id, name, title, description, category, subcategory, subsubcategory, keywords, background_image, author_image, is_public, default_crossfade_ms, fade_curve, created_at, updated_at FROM atmospheres");
        let mut clauses: Vec<String> = Vec::new();
        let mut params: Vec<(String)> = Vec::new();

        if let Some(cat) = category { if !cat.is_empty() { clauses.push("category = ?".into()); params.push(cat.to_string()); } }
        if let Some(q) = query { if !q.trim().is_empty() { clauses.push("LOWER(name || ' ' || title || ' ' || description || ' ' || category || ' ' || subcategory) LIKE ?".into()); params.push(format!("%{}%", q.to_lowercase())); } }
        if let Some(kws) = keywords { for kw in kws { if !kw.is_empty() { clauses.push("keywords LIKE ?".into()); // naive JSON text search
            // match JSON array element containing kw (quoted)
            params.push(format!("%\"{}\"%", kw)); }} }
        if !clauses.is_empty() { sql.push_str(" WHERE "); sql.push_str(&clauses.join(" AND ")); }
        sql.push_str(" ORDER BY updated_at DESC");

        let mut stmt = conn.prepare(&sql)?;
        // Convert params to &dyn ToSql
        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
        let rows = stmt.query_map(rusqlite::params_from_iter(param_refs), |row| {
            let keywords_json: String = row.get(7)?;
            let keywords: Vec<String> = serde_json::from_str(&keywords_json).unwrap_or_default();
            Ok(Atmosphere {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                category: row.get(4)?,
                subcategory: row.get(5)?,
                subsubcategory: row.get(6)?,
                keywords,
                background_image: row.get(8)?,
                author_image: row.get(9)?,
                is_public: row.get(10)?,
                default_crossfade_ms: row.get(11).unwrap_or(2500),
                fade_curve: row.get(12).unwrap_or(String::from("linear")),
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
            })
        })?;
        let mut out = Vec::new();
        for r in rows { out.push(r?); }
        Ok(out)
    }

    // Helper methods

    fn get_audio_file_by_id(&self, conn: &Connection, id: i64) -> Result<AudioFile> {
        let mut stmt = conn.prepare(
            "SELECT id, file_path, title, artist, album, duration, genre, year, track_number,
                    album_artist, date, total_tracks, disc_number, total_discs,
                    composer, conductor, lyricist, original_artist, remixer,
                    arranger, engineer, producer, dj_mixer, mixer,
                    content_group, subtitle, initial_key, bpm, language,
                    media_type, original_filename, original_lyricist,
                    original_release_time, playlist_delay, recording_time,
                    release_time, tagging_time, encoding_time, encoding_settings,
                    encoded_by, copyright, file_owner, internet_radio_station_name,
                    internet_radio_station_owner, isrc, publisher, mood,
                    occasion, tempo, content_type, category
             FROM audio_files WHERE id = ?1"
        )?;

        stmt.query_row([id], |row| {
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
                album_artist: row.get(9)?,
                date: row.get(10)?,
                total_tracks: row.get(11)?,
                disc_number: row.get(12)?,
                total_discs: row.get(13)?,
                composer: row.get(14)?,
                conductor: row.get(15)?,
                lyricist: row.get(16)?,
                original_artist: row.get(17)?,
                remixer: row.get(18)?,
                arranger: row.get(19)?,
                engineer: row.get(20)?,
                producer: row.get(21)?,
                dj_mixer: row.get(22)?,
                mixer: row.get(23)?,
                content_group: row.get(24)?,
                subtitle: row.get(25)?,
                initial_key: row.get(26)?,
                bpm: row.get(27)?,
                language: row.get(28)?,
                media_type: row.get(29)?,
                original_filename: row.get(30)?,
                original_lyricist: row.get(31)?,
                original_release_time: row.get(32)?,
                playlist_delay: row.get(33)?,
                recording_time: row.get(34)?,
                release_time: row.get(35)?,
                tagging_time: row.get(36)?,
                encoding_time: row.get(37)?,
                encoding_settings: row.get(38)?,
                encoded_by: row.get(39)?,
                copyright: row.get(40)?,
                file_owner: row.get(41)?,
                internet_radio_station_name: row.get(42)?,
                internet_radio_station_owner: row.get(43)?,
                isrc: row.get(44)?,
                publisher: row.get(45)?,
                mood: row.get(46)?,
                occasion: row.get(47)?,
                tempo: row.get(48)?,
                content_type: row.get(49)?,
                category: row.get(50)?,
            })
        })
    }

    fn initialize_default_categories(&self, conn: &Connection) -> Result<()> {
        // Check if categories already exist
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM atmosphere_categories",
            [],
            |row| row.get(0)
        )?;

        if count > 0 {
            return Ok(()); // Already initialized
        }

        // Insert default categories based on the demo data
        let categories = vec![
            (1, "At Home", None, 1),
            (2, "Books", None, 2),
            (3, "Environmental", None, 3),
            (4, "Games", None, 4),
            (5, "Holidays", None, 5),
            (6, "Human", None, 6),
            (7, "Movies and Series", None, 7),
            (8, "Music", None, 8),
            (9, "Nature", None, 9),
            (10, "Other atmospheres", None, 10),
            (11, "Relaxing atmospheres", None, 11),
            (12, "Technical", None, 12),
            (13, "Unreal atmospheres", None, 13),
            
            // Subcategories for Unreal atmospheres
            (14, "Fantasy", Some(13), 1),
            (15, "Horror", Some(13), 2),
            (16, "Other", Some(13), 3),
            (17, "Science Fiction", Some(13), 4),
            (18, "Steampunk", Some(13), 5),

            // Environmental subcategories
            (19, "Beach", Some(3), 1),
            (20, "Cafe", Some(3), 2),
            (21, "Cave", Some(3), 3),
            (22, "City", Some(3), 4),
            (23, "Countryside", Some(3), 5),
            (24, "Desert", Some(3), 6),
            (25, "Underwater", Some(3), 7),
            (26, "Warfare", Some(3), 8),
        ];

        for (id, name, parent_id, display_order) in categories {
            conn.execute(
                "INSERT OR IGNORE INTO atmosphere_categories (id, name, parent_id, display_order)
                 VALUES (?1, ?2, ?3, ?4)",
                params![id, name, parent_id, display_order],
            )?;
        }

        log::info!("Default atmosphere categories initialized");
        Ok(())
    }
}