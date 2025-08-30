use rusqlite::{Connection, Result};

/// Apply database migrations for audio_files table
pub fn apply_migrations(conn: &Connection) -> Result<()> {
    // Get existing columns
    let mut stmt = conn.prepare("PRAGMA table_info(audio_files)")?;
    let mut existing_columns = std::collections::HashSet::new();
    let mut rows = stmt.query([])?;
    
    while let Some(row) = rows.next()? {
        let col_name: String = row.get(1)?;
        existing_columns.insert(col_name);
    }
    drop(rows);
    drop(stmt);
    
    // Add new columns if they don't exist
    let columns_to_add = vec![
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
    ];
    
    for (column_name, column_type) in columns_to_add {
        if !existing_columns.contains(column_name) {
            let sql = format!("ALTER TABLE audio_files ADD COLUMN {} {}", column_name, column_type);
            match conn.execute(&sql, []) {
                Ok(_) => log::info!("Added column {} to audio_files table", column_name),
                Err(e) => log::warn!("Failed to add column {}: {}", column_name, e),
            }
        }
    }
    
    Ok(())
}