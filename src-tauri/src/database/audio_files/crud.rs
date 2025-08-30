use rusqlite::{Connection, params, Result};
use crate::models::AudioFile;
use super::helpers;
use super::AudioFileOps;

impl AudioFileOps {
    /// Save a new audio file to the database
    pub fn save(conn: &Connection, audio_file: &AudioFile) -> Result<i64> {
        conn.execute(
            "INSERT INTO audio_files (
                file_path, title, artist, album, duration, genre, year, track_number,
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
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20,
                ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30,
                ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38, ?39, ?40,
                ?41, ?42, ?43, ?44, ?45, ?46, ?47, ?48, ?49, ?50
            )",
            params![
                audio_file.file_path, audio_file.title, audio_file.artist,
                audio_file.album, audio_file.duration, audio_file.genre,
                audio_file.year, audio_file.track_number, audio_file.album_artist,
                audio_file.date, audio_file.total_tracks, audio_file.disc_number,
                audio_file.total_discs, audio_file.composer, audio_file.conductor,
                audio_file.lyricist, audio_file.original_artist, audio_file.remixer,
                audio_file.arranger, audio_file.engineer, audio_file.producer,
                audio_file.dj_mixer, audio_file.mixer, audio_file.content_group,
                audio_file.subtitle, audio_file.initial_key, audio_file.bpm,
                audio_file.language, audio_file.media_type, audio_file.original_filename,
                audio_file.original_lyricist, audio_file.original_release_time,
                audio_file.playlist_delay, audio_file.recording_time,
                audio_file.release_time, audio_file.tagging_time,
                audio_file.encoding_time, audio_file.encoding_settings,
                audio_file.encoded_by, audio_file.copyright, audio_file.file_owner,
                audio_file.internet_radio_station_name, audio_file.internet_radio_station_owner,
                audio_file.isrc, audio_file.publisher, audio_file.mood,
                audio_file.occasion, audio_file.tempo, audio_file.content_type,
                audio_file.category
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Get all audio files from the database
    pub fn get_all(conn: &Connection) -> Result<Vec<AudioFile>> {
        // First, check which columns exist in the table
        let existing_columns = helpers::get_existing_columns(conn)?;

        // Build the SELECT query with only existing columns
        let (query, column_order) = helpers::build_select_query(&existing_columns);

        let mut stmt = conn.prepare(&query)?;
        let rows = stmt.query_map([], |row| {
            helpers::map_row_to_audio_file(row, &existing_columns, &column_order)
        })?;

        rows.collect()
    }

    /// Get audio file by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<AudioFile> {
        let existing_columns = helpers::get_existing_columns(conn)?;
        let (mut query, column_order) = helpers::build_select_query(&existing_columns);
        
        // Add WHERE clause
        query.push_str(" WHERE id = ?1");
        
        let mut stmt = conn.prepare(&query)?;
        stmt.query_row([id], |row| {
            helpers::map_row_to_audio_file(row, &existing_columns, &column_order)
        })
    }

    /// Get audio file by path
    pub fn get_by_path(conn: &Connection, file_path: &str) -> Result<AudioFile> {
        let existing_columns = helpers::get_existing_columns(conn)?;
        let (mut query, column_order) = helpers::build_select_query(&existing_columns);
        
        // Add WHERE clause
        query.push_str(" WHERE file_path = ?1");
        
        let mut stmt = conn.prepare(&query)?;
        stmt.query_row([file_path], |row| {
            helpers::map_row_to_audio_file(row, &existing_columns, &column_order)
        })
    }

    /// Update an existing audio file
    pub fn update(conn: &Connection, audio_file: &AudioFile) -> Result<()> {
        if let Some(id) = audio_file.id {
            conn.execute(
                "UPDATE audio_files SET
                    file_path = ?1, title = ?2, artist = ?3, album = ?4, duration = ?5,
                    genre = ?6, year = ?7, track_number = ?8, album_artist = ?9, date = ?10,
                    total_tracks = ?11, disc_number = ?12, total_discs = ?13, composer = ?14,
                    conductor = ?15, lyricist = ?16, original_artist = ?17, remixer = ?18,
                    arranger = ?19, engineer = ?20, producer = ?21, dj_mixer = ?22, mixer = ?23,
                    content_group = ?24, subtitle = ?25, initial_key = ?26, bpm = ?27,
                    language = ?28, media_type = ?29, original_filename = ?30,
                    original_lyricist = ?31, original_release_time = ?32, playlist_delay = ?33,
                    recording_time = ?34, release_time = ?35, tagging_time = ?36,
                    encoding_time = ?37, encoding_settings = ?38, encoded_by = ?39,
                    copyright = ?40, file_owner = ?41, internet_radio_station_name = ?42,
                    internet_radio_station_owner = ?43, isrc = ?44, publisher = ?45,
                    mood = ?46, occasion = ?47, tempo = ?48, content_type = ?49, category = ?50
                WHERE id = ?51",
                params![
                    audio_file.file_path, audio_file.title, audio_file.artist,
                    audio_file.album, audio_file.duration, audio_file.genre,
                    audio_file.year, audio_file.track_number, audio_file.album_artist,
                    audio_file.date, audio_file.total_tracks, audio_file.disc_number,
                    audio_file.total_discs, audio_file.composer, audio_file.conductor,
                    audio_file.lyricist, audio_file.original_artist, audio_file.remixer,
                    audio_file.arranger, audio_file.engineer, audio_file.producer,
                    audio_file.dj_mixer, audio_file.mixer, audio_file.content_group,
                    audio_file.subtitle, audio_file.initial_key, audio_file.bpm,
                    audio_file.language, audio_file.media_type, audio_file.original_filename,
                    audio_file.original_lyricist, audio_file.original_release_time,
                    audio_file.playlist_delay, audio_file.recording_time,
                    audio_file.release_time, audio_file.tagging_time,
                    audio_file.encoding_time, audio_file.encoding_settings,
                    audio_file.encoded_by, audio_file.copyright, audio_file.file_owner,
                    audio_file.internet_radio_station_name, audio_file.internet_radio_station_owner,
                    audio_file.isrc, audio_file.publisher, audio_file.mood,
                    audio_file.occasion, audio_file.tempo, audio_file.content_type,
                    audio_file.category, id
                ],
            )?;
        }
        Ok(())
    }

    /// Delete an audio file from the database
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM audio_files WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    /// Check if file path already exists
    pub fn file_exists(conn: &Connection, file_path: &str) -> Result<bool> {
        let mut stmt = conn.prepare("SELECT 1 FROM audio_files WHERE file_path = ?1 LIMIT 1")?;
        Ok(stmt.exists([file_path])?)
    }
}