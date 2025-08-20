use rusqlite::{Connection, params, Result};
use crate::models::AudioFile;
use std::collections::HashSet;

/// Repository for audio file operations
pub struct AudioFileRepository;

impl AudioFileRepository {
    pub fn new() -> Self {
        AudioFileRepository
    }

    pub fn save(&self, conn: &Connection, audio_file: &AudioFile) -> Result<i64> {
        let _id = conn.execute(
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

    pub fn get_all(&self, conn: &Connection) -> Result<Vec<AudioFile>> {
        // First, check which columns exist in the table
        let existing_columns = self.get_existing_columns(conn)?;

        // Build the SELECT query with only existing columns
        let (query, column_order) = self.build_select_query(&existing_columns);

        let mut stmt = conn.prepare(&query)?;
        let rows = stmt.query_map([], |row| {
            self.map_row_to_audio_file(row, &existing_columns, &column_order)
        })?;

        let mut audio_files = Vec::new();
        for row in rows {
            audio_files.push(row?);
        }
        Ok(audio_files)
    }

    pub fn delete(&self, conn: &Connection, id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM audio_files WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn update_duration(&self, conn: &Connection, id: i64, duration: f64) -> Result<()> {
        conn.execute(
            "UPDATE audio_files SET duration = ?1 WHERE id = ?2",
            params![duration, id],
        )?;
        Ok(())
    }

    pub fn update_bpm(&self, conn: &Connection, id: i64, bpm: u32) -> Result<()> {
        conn.execute(
            "UPDATE audio_files SET bpm = ?1 WHERE id = ?2",
            params![bpm, id],
        )?;
        Ok(())
    }

    pub fn update_duration_and_bpm(&self, conn: &Connection, id: i64, duration: Option<f64>, bpm: Option<u32>) -> Result<()> {
        match (duration, bpm) {
            (Some(d), Some(b)) => {
                conn.execute(
                    "UPDATE audio_files SET duration = ?1, bpm = ?2 WHERE id = ?3",
                    params![d, b, id],
                )?;
            }
            (Some(d), None) => {
                conn.execute(
                    "UPDATE audio_files SET duration = ?1 WHERE id = ?2",
                    params![d, id],
                )?;
            }
            (None, Some(b)) => {
                conn.execute(
                    "UPDATE audio_files SET bpm = ?1 WHERE id = ?2",
                    params![b, id],
                )?;
            }
            (None, None) => {
                // Nothing to update
            }
        }
        Ok(())
    }

    // Helper methods

    fn get_existing_columns(&self, conn: &Connection) -> Result<HashSet<String>> {
        let mut stmt = conn.prepare("PRAGMA table_info(audio_files)")?;
        let column_rows = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(1)?) // column name is at index 1
        })?;
        
        let mut existing_columns = HashSet::new();
        for column_result in column_rows {
            existing_columns.insert(column_result?);
        }
        Ok(existing_columns)
    }

    fn build_select_query(&self, existing_columns: &HashSet<String>) -> (String, Vec<&'static str>) {
        let base_columns = vec![
            "id", "file_path", "title", "artist", "album", 
            "duration", "genre", "year", "track_number"
        ];
        
        let extended_columns = vec![
            "album_artist", "date", "total_tracks", "disc_number", "total_discs",
            "composer", "conductor", "lyricist", "original_artist", "remixer", 
            "arranger", "engineer", "producer", "dj_mixer", "mixer", 
            "content_group", "subtitle", "initial_key", "bpm", "language", 
            "media_type", "original_filename", "original_lyricist", 
            "original_release_time", "playlist_delay", "recording_time", 
            "release_time", "tagging_time", "encoding_time", "encoding_settings", 
            "encoded_by", "copyright", "file_owner", "internet_radio_station_name", 
            "internet_radio_station_owner", "isrc", "publisher", "mood", 
            "occasion", "tempo", "content_type", "category"
        ];

        let mut all_columns = base_columns.clone();
        for col in extended_columns.iter() {
            if existing_columns.contains(*col) {
                all_columns.push(*col);
            }
        }

        let query = format!(
            "SELECT {} FROM audio_files ORDER BY artist, album, track_number",
            all_columns.join(", ")
        );

        (query, all_columns)
    }

    fn map_row_to_audio_file(&self, row: &rusqlite::Row, existing_columns: &HashSet<String>, _column_order: &[&str]) -> Result<AudioFile> {
        let mut index = 0;
        
        // Helper macro to safely get values from the row
        macro_rules! get_value {
            ($col:expr) => {{
                if existing_columns.contains($col) {
                    let val = row.get(index)?;
                    index += 1;
                    val
                } else {
                    None
                }
            }};
        }

        // Required base columns (always present)
        let id = Some(row.get(index)?);
        index += 1;
        let file_path = row.get(index)?;
        index += 1;
        let title = row.get(index)?;
        index += 1;
        let artist = row.get(index)?;
        index += 1;
        let album = row.get(index)?;
        index += 1;
        let duration = row.get(index)?;
        index += 1;
        let genre = row.get(index)?;
        index += 1;
        let year = row.get(index)?;
        index += 1;
        let track_number = row.get(index)?;
        index += 1;

        // Extended columns - only read if they exist
        let album_artist = get_value!("album_artist");
        let date = get_value!("date");
        let total_tracks = get_value!("total_tracks");
        let disc_number = get_value!("disc_number");
        let total_discs = get_value!("total_discs");
        let composer = get_value!("composer");
        let conductor = get_value!("conductor");
        let lyricist = get_value!("lyricist");
        let original_artist = get_value!("original_artist");
        let remixer = get_value!("remixer");
        let arranger = get_value!("arranger");
        let engineer = get_value!("engineer");
        let producer = get_value!("producer");
        let dj_mixer = get_value!("dj_mixer");
        let mixer = get_value!("mixer");
        let content_group = get_value!("content_group");
        let subtitle = get_value!("subtitle");
        let initial_key = get_value!("initial_key");
        let bpm = get_value!("bpm");
        let language = get_value!("language");
        let media_type = get_value!("media_type");
        let original_filename = get_value!("original_filename");
        let original_lyricist = get_value!("original_lyricist");
        let original_release_time = get_value!("original_release_time");
        let playlist_delay = get_value!("playlist_delay");
        let recording_time = get_value!("recording_time");
        let release_time = get_value!("release_time");
        let tagging_time = get_value!("tagging_time");
        let encoding_time = get_value!("encoding_time");
        let encoding_settings = get_value!("encoding_settings");
        let encoded_by = get_value!("encoded_by");
        let copyright = get_value!("copyright");
        let file_owner = get_value!("file_owner");
        let internet_radio_station_name = get_value!("internet_radio_station_name");
        let internet_radio_station_owner = get_value!("internet_radio_station_owner");
        let isrc = get_value!("isrc");
        let publisher = get_value!("publisher");
        let mood = get_value!("mood");
        let occasion = get_value!("occasion");
        let tempo = get_value!("tempo");
        let content_type = get_value!("content_type");
        let category = get_value!("category");

        Ok(AudioFile {
            id,
            file_path,
            title,
            artist,
            album,
            duration,
            genre,
            year,
            track_number,
            album_artist,
            date,
            total_tracks,
            disc_number,
            total_discs,
            composer,
            conductor,
            lyricist,
            original_artist,
            remixer,
            arranger,
            engineer,
            producer,
            dj_mixer,
            mixer,
            content_group,
            subtitle,
            initial_key,
            bpm,
            language,
            media_type,
            original_filename,
            original_lyricist,
            original_release_time,
            playlist_delay,
            recording_time,
            release_time,
            tagging_time,
            encoding_time,
            encoding_settings,
            encoded_by,
            copyright,
            file_owner,
            internet_radio_station_name,
            internet_radio_station_owner,
            isrc,
            publisher,
            mood,
            occasion,
            tempo,
            content_type,
            category,
        })
    }
}