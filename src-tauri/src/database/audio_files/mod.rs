mod crud;
mod metadata;
mod helpers;
mod migration;


use rusqlite::{Connection, Result};

/// Database operations for audio files
pub struct AudioFileOps;

impl AudioFileOps {
    /// Create audio files table with all metadata columns
    pub fn create_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS audio_files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                file_path TEXT NOT NULL UNIQUE,
                title TEXT,
                artist TEXT,
                album TEXT,
                duration REAL,
                genre TEXT,
                year INTEGER,
                track_number INTEGER,
                album_artist TEXT,
                date TEXT,
                total_tracks INTEGER,
                disc_number INTEGER,
                total_discs INTEGER,
                composer TEXT,
                conductor TEXT,
                lyricist TEXT,
                original_artist TEXT,
                remixer TEXT,
                arranger TEXT,
                engineer TEXT,
                producer TEXT,
                dj_mixer TEXT,
                mixer TEXT,
                content_group TEXT,
                subtitle TEXT,
                initial_key TEXT,
                bpm INTEGER,
                language TEXT,
                media_type TEXT,
                original_filename TEXT,
                original_lyricist TEXT,
                original_release_time TEXT,
                playlist_delay INTEGER,
                recording_time TEXT,
                release_time TEXT,
                tagging_time TEXT,
                encoding_time TEXT,
                encoding_settings TEXT,
                encoded_by TEXT,
                copyright TEXT,
                file_owner TEXT,
                internet_radio_station_name TEXT,
                internet_radio_station_owner TEXT,
                isrc TEXT,
                publisher TEXT,
                mood TEXT,
                occasion TEXT,
                tempo TEXT,
                content_type TEXT,
                category TEXT
            )",
            [],
        )?;

        // Create index for faster lookups
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_audio_files_path ON audio_files(file_path)",
            [],
        )?;

        // Apply migrations for existing databases
        migration::apply_migrations(conn)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::AudioFile;

    fn create_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        AudioFileOps::create_table(&conn).unwrap();
        conn
    }

    fn create_test_audio_file() -> AudioFile {
        AudioFile {
            id: None,
            file_path: "/test/path/song.mp3".to_string(),
            title: Some("Test Song".to_string()),
            artist: Some("Test Artist".to_string()),
            album: Some("Test Album".to_string()),
            duration: Some(180.5),
            genre: Some("Electronic".to_string()),
            year: Some(2024),
            track_number: Some(1),
            album_artist: None,
            date: None,
            total_tracks: None,
            disc_number: None,
            total_discs: None,
            composer: None,
            conductor: None,
            lyricist: None,
            original_artist: None,
            remixer: None,
            arranger: None,
            engineer: None,
            producer: None,
            dj_mixer: None,
            mixer: None,
            content_group: None,
            subtitle: None,
            initial_key: None,
            bpm: Some(120),
            language: None,
            media_type: None,
            original_filename: None,
            original_lyricist: None,
            original_release_time: None,
            playlist_delay: None,
            recording_time: None,
            release_time: None,
            tagging_time: None,
            encoding_time: None,
            encoding_settings: None,
            encoded_by: None,
            copyright: None,
            file_owner: None,
            internet_radio_station_name: None,
            internet_radio_station_owner: None,
            isrc: None,
            publisher: None,
            mood: None,
            occasion: None,
            tempo: None,
            content_type: None,
            category: None,
        }
    }

    #[test]
    fn test_create_table() {
        let conn = create_test_db();
        
        // Verify table exists
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='audio_files'",
            [],
            |row| row.get(0)
        ).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_save_and_get() {
        let conn = create_test_db();
        let audio_file = create_test_audio_file();
        
        // Save
        let id = AudioFileOps::save(&conn, &audio_file).unwrap();
        assert!(id > 0);
        
        // Get all
        let files = AudioFileOps::get_all(&conn).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].file_path, "/test/path/song.mp3");
    }

    #[test]
    fn test_update_metadata() {
        let conn = create_test_db();
        let audio_file = create_test_audio_file();
        
        let id = AudioFileOps::save(&conn, &audio_file).unwrap();
        
        // Update duration
        AudioFileOps::update_duration(&conn, id, 200.0).unwrap();
        
        // Update BPM
        AudioFileOps::update_bpm(&conn, id, 140).unwrap();
        
        let files = AudioFileOps::get_all(&conn).unwrap();
        assert_eq!(files[0].duration, Some(200.0));
        assert_eq!(files[0].bpm, Some(140));
    }

    #[test]
    fn test_delete() {
        let conn = create_test_db();
        let audio_file = create_test_audio_file();
        
        let id = AudioFileOps::save(&conn, &audio_file).unwrap();
        
        // Delete
        AudioFileOps::delete(&conn, id).unwrap();
        
        let files = AudioFileOps::get_all(&conn).unwrap();
        assert_eq!(files.len(), 0);
    }
}