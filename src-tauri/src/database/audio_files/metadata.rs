use rusqlite::{Connection, params, Result};
use super::AudioFileOps;

impl AudioFileOps {
    /// Update the duration of an audio file
    pub fn update_duration(conn: &Connection, id: i64, duration: f64) -> Result<()> {
        conn.execute(
            "UPDATE audio_files SET duration = ?1 WHERE id = ?2",
            params![duration, id],
        )?;
        Ok(())
    }

    /// Update the BPM of an audio file
    pub fn update_bpm(conn: &Connection, id: i64, bpm: u32) -> Result<()> {
        conn.execute(
            "UPDATE audio_files SET bpm = ?1 WHERE id = ?2",
            params![bpm, id],
        )?;
        Ok(())
    }

    /// Update multiple metadata fields at once
    pub fn update_metadata(conn: &Connection, id: i64, field: &str, value: Option<&str>) -> Result<()> {
        // Validate field name to prevent SQL injection
        let valid_fields = [
            "title", "artist", "album", "genre", "year", "track_number",
            "album_artist", "date", "total_tracks", "disc_number", "total_discs",
            "composer", "conductor", "lyricist", "original_artist", "remixer",
            "arranger", "engineer", "producer", "dj_mixer", "mixer",
            "content_group", "subtitle", "initial_key", "language",
            "media_type", "original_filename", "original_lyricist",
            "original_release_time", "playlist_delay", "recording_time",
            "release_time", "tagging_time", "encoding_time", "encoding_settings",
            "encoded_by", "copyright", "file_owner", "internet_radio_station_name",
            "internet_radio_station_owner", "isrc", "publisher", "mood",
            "occasion", "tempo", "content_type", "category"
        ];
        
        if !valid_fields.contains(&field) {
            return Err(rusqlite::Error::InvalidParameterName(field.to_string()));
        }
        
        let sql = format!("UPDATE audio_files SET {} = ?1 WHERE id = ?2", field);
        conn.execute(&sql, params![value, id])?;
        Ok(())
    }

    /// Get files without duration
    pub fn get_files_without_duration(conn: &Connection) -> Result<Vec<(i64, String)>> {
        let mut stmt = conn.prepare(
            "SELECT id, file_path FROM audio_files WHERE duration IS NULL OR duration = 0"
        )?;
        
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;
        
        rows.collect()
    }

    /// Get files without BPM
    pub fn get_files_without_bpm(conn: &Connection) -> Result<Vec<(i64, String)>> {
        let mut stmt = conn.prepare(
            "SELECT id, file_path FROM audio_files WHERE bpm IS NULL OR bpm = 0"
        )?;
        
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;
        
        rows.collect()
    }

    /// Batch update durations
    pub fn batch_update_durations(conn: &Connection, updates: &[(i64, f64)]) -> Result<()> {
        let tx = conn.unchecked_transaction()?;
        
        for (id, duration) in updates {
            tx.execute(
                "UPDATE audio_files SET duration = ?1 WHERE id = ?2",
                params![duration, id],
            )?;
        }
        
        tx.commit()?;
        Ok(())
    }

    /// Batch update BPMs
    pub fn batch_update_bpms(conn: &Connection, updates: &[(i64, u32)]) -> Result<()> {
        let tx = conn.unchecked_transaction()?;
        
        for (id, bpm) in updates {
            tx.execute(
                "UPDATE audio_files SET bpm = ?1 WHERE id = ?2",
                params![bpm, id],
            )?;
        }
        
        tx.commit()?;
        Ok(())
    }

    /// Get metadata statistics
    pub fn get_metadata_stats(conn: &Connection) -> Result<MetadataStats> {
        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM audio_files",
            [],
            |row| row.get(0)
        )?;
        
        let with_duration: i64 = conn.query_row(
            "SELECT COUNT(*) FROM audio_files WHERE duration IS NOT NULL AND duration > 0",
            [],
            |row| row.get(0)
        )?;
        
        let with_bpm: i64 = conn.query_row(
            "SELECT COUNT(*) FROM audio_files WHERE bpm IS NOT NULL AND bpm > 0",
            [],
            |row| row.get(0)
        )?;
        
        let with_artist: i64 = conn.query_row(
            "SELECT COUNT(*) FROM audio_files WHERE artist IS NOT NULL AND artist != ''",
            [],
            |row| row.get(0)
        )?;
        
        let with_album: i64 = conn.query_row(
            "SELECT COUNT(*) FROM audio_files WHERE album IS NOT NULL AND album != ''",
            [],
            |row| row.get(0)
        )?;
        
        Ok(MetadataStats {
            total_files: total,
            files_with_duration: with_duration,
            files_with_bpm: with_bpm,
            files_with_artist: with_artist,
            files_with_album: with_album,
        })
    }
}

#[derive(Debug)]
pub struct MetadataStats {
    pub total_files: i64,
    pub files_with_duration: i64,
    pub files_with_bpm: i64,
    pub files_with_artist: i64,
    pub files_with_album: i64,
}