use rusqlite::{Connection, params, Result, Row};
use crate::models::{Atmosphere, AudioFile};

/// Helper functions for atmosphere database operations
pub fn row_to_atmosphere(row: &Row) -> Result<Atmosphere> {
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
        theme: row.get(11)?,
        default_crossfade_ms: row.get(12).unwrap_or(2500),
        fade_curve: row.get(13).unwrap_or(String::from("linear")),
        created_at: row.get(14)?,
        updated_at: row.get(15)?,
    })
}

pub fn get_audio_file_by_id(conn: &Connection, id: i64) -> Result<AudioFile> {
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

pub fn name_exists(conn: &Connection, name: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT 1 FROM atmospheres WHERE name = ?1 LIMIT 1")?;
    Ok(stmt.exists([name])?)
}

pub fn ensure_columns(conn: &Connection) -> Result<()> {
    // SQLite doesn't support ADD COLUMN IF NOT EXISTS directly for older versions, so probe pragma
    let mut stmt = conn.prepare("PRAGMA table_info(atmospheres)")?;
    let mut has_crossfade = false;
    let mut has_curve = false;
    let mut has_theme = false;
    let mut rows = stmt.query([])?;
    
    while let Some(row) = rows.next()? {
        let col_name: String = row.get(1)?; // 1 = name
        if col_name == "default_crossfade_ms" { has_crossfade = true; }
        if col_name == "fade_curve" { has_curve = true; }
        if col_name == "theme" { has_theme = true; }
    }
    
    if !has_crossfade {
        conn.execute("ALTER TABLE atmospheres ADD COLUMN default_crossfade_ms INTEGER DEFAULT 2500", [])?;
    }
    if !has_theme {
        conn.execute("ALTER TABLE atmospheres ADD COLUMN theme TEXT DEFAULT 'default'", [])?;
    }
    if !has_curve {
        conn.execute("ALTER TABLE atmospheres ADD COLUMN fade_curve TEXT NOT NULL DEFAULT 'linear'", [])?;
    }
    
    Ok(())
}

pub fn ensure_atmosphere_sounds_columns(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("PRAGMA table_info(atmosphere_sounds)")?;
    let mut has_min_seconds = false;
    let mut has_max_seconds = false;
    let mut rows = stmt.query([])?;
    
    while let Some(row) = rows.next()? {
        let col_name: String = row.get(1)?; // 1 = name
        if col_name == "min_seconds" { has_min_seconds = true; }
        if col_name == "max_seconds" { has_max_seconds = true; }
    }
    
    if !has_min_seconds {
        conn.execute("ALTER TABLE atmosphere_sounds ADD COLUMN min_seconds INTEGER DEFAULT 0", [])?;
    }
    if !has_max_seconds {
        conn.execute("ALTER TABLE atmosphere_sounds ADD COLUMN max_seconds INTEGER DEFAULT 0", [])?;
    }
    
    Ok(())
}

pub fn initialize_default_categories(conn: &Connection) -> Result<()> {
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