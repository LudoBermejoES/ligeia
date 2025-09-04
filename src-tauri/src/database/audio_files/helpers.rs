use rusqlite::{Connection, Result, Row};
use crate::models::AudioFile;
use std::collections::HashSet;

/// Get the list of existing columns in the audio_files table
pub fn get_existing_columns(conn: &Connection) -> Result<HashSet<String>> {
    let mut stmt = conn.prepare("PRAGMA table_info(audio_files)")?;
    let mut columns = HashSet::new();
    let mut rows = stmt.query([])?;
    
    while let Some(row) = rows.next()? {
        let col_name: String = row.get(1)?; // Column 1 is the name
        columns.insert(col_name);
    }
    
    Ok(columns)
}

/// Build a SELECT query with only existing columns
pub fn build_select_query(existing_columns: &HashSet<String>) -> (String, Vec<String>) {
    let all_columns = vec![
        "id", "file_path", "title", "artist", "album", "duration", "genre", "year", 
        "track_number", "album_artist", "date", "total_tracks", "disc_number", 
        "total_discs", "composer", "conductor", "lyricist", "original_artist", 
        "remixer", "arranger", "engineer", "producer", "dj_mixer", "mixer", 
        "content_group", "subtitle", "initial_key", "bpm", "language", "media_type", 
        "original_filename", "original_lyricist", "original_release_time", 
        "playlist_delay", "recording_time", "release_time", "tagging_time", 
        "encoding_time", "encoding_settings", "encoded_by", "copyright", 
        "file_owner", "internet_radio_station_name", "internet_radio_station_owner", 
        "isrc", "publisher", "mood", "occasion", "tempo", "content_type", "category",
        "auto_tagged", "auto_tag_date", "auto_tag_version"
    ];
    
    let mut selected_columns = Vec::new();
    let mut column_order = Vec::new();
    
    for col in &all_columns {
        if existing_columns.contains(*col) {
            selected_columns.push(*col);
            column_order.push(col.to_string());
        }
    }
    
    let query = format!("SELECT {} FROM audio_files", selected_columns.join(", "));
    (query, column_order)
}

/// Map a database row to an AudioFile struct
pub fn map_row_to_audio_file(
    row: &Row, 
    existing_columns: &HashSet<String>, 
    column_order: &[String]
) -> Result<AudioFile> {
    // Helper function to get value by column name
    let get_value_by_name = |col_name: &str| -> Option<usize> {
        if existing_columns.contains(col_name) {
            column_order.iter().position(|c| c == col_name)
        } else {
            None
        }
    };
    
    // Helper to get optional string value
    let get_optional = |col_name: &str| -> Result<Option<String>> {
        if let Some(idx) = get_value_by_name(col_name) {
            row.get(idx)
        } else {
            Ok(None)
        }
    };
    
    // Helper to get optional i32 value
    let get_optional_i32 = |col_name: &str| -> Result<Option<i32>> {
        if let Some(idx) = get_value_by_name(col_name) {
            row.get(idx)
        } else {
            Ok(None)
        }
    };
    
    // Helper to get optional f64 value
    let get_optional_f64 = |col_name: &str| -> Result<Option<f64>> {
        if let Some(idx) = get_value_by_name(col_name) {
            row.get(idx)
        } else {
            Ok(None)
        }
    };
    
    // Helper to get optional bool value
    let get_optional_bool = |col_name: &str| -> Result<Option<bool>> {
        if let Some(idx) = get_value_by_name(col_name) {
            row.get(idx)
        } else {
            Ok(None)
        }
    };
    
    // Get ID (always exists)
    let id: Option<i64> = if let Some(idx) = get_value_by_name("id") {
        Some(row.get(idx)?)
    } else {
        None
    };
    
    // Get file_path (always exists)
    let file_path: String = if let Some(idx) = get_value_by_name("file_path") {
        row.get(idx)?
    } else {
        return Err(rusqlite::Error::InvalidColumnName("file_path".to_string()));
    };
    
    // Build the AudioFile struct with all optional fields
    Ok(AudioFile {
        id,
        file_path,
        title: get_optional("title")?,
        artist: get_optional("artist")?,
        album: get_optional("album")?,
        duration: get_optional_f64("duration")?,
        genre: get_optional("genre")?,
        year: get_optional_i32("year")?,
        track_number: get_optional_i32("track_number")?.map(|v| v as u32),
        album_artist: get_optional("album_artist")?,
        date: get_optional("date")?,
        total_tracks: get_optional_i32("total_tracks")?.map(|v| v as u32),
        disc_number: get_optional_i32("disc_number")?.map(|v| v as u32),
        total_discs: get_optional_i32("total_discs")?.map(|v| v as u32),
        composer: get_optional("composer")?,
        conductor: get_optional("conductor")?,
        lyricist: get_optional("lyricist")?,
        original_artist: get_optional("original_artist")?,
        remixer: get_optional("remixer")?,
        arranger: get_optional("arranger")?,
        engineer: get_optional("engineer")?,
        producer: get_optional("producer")?,
        dj_mixer: get_optional("dj_mixer")?,
        mixer: get_optional("mixer")?,
        content_group: get_optional("content_group")?,
        subtitle: get_optional("subtitle")?,
        initial_key: get_optional("initial_key")?,
        bpm: get_optional_i32("bpm")?.map(|v| v as u32),
        language: get_optional("language")?,
        media_type: get_optional("media_type")?,
        original_filename: get_optional("original_filename")?,
        original_lyricist: get_optional("original_lyricist")?,
        original_release_time: get_optional("original_release_time")?,
        playlist_delay: get_optional_i32("playlist_delay")?.map(|v| v as u32),
        recording_time: get_optional("recording_time")?,
        release_time: get_optional("release_time")?,
        tagging_time: get_optional("tagging_time")?,
        encoding_time: get_optional("encoding_time")?,
        encoding_settings: get_optional("encoding_settings")?,
        encoded_by: get_optional("encoded_by")?,
        copyright: get_optional("copyright")?,
        file_owner: get_optional("file_owner")?,
        internet_radio_station_name: get_optional("internet_radio_station_name")?,
        internet_radio_station_owner: get_optional("internet_radio_station_owner")?,
        isrc: get_optional("isrc")?,
        publisher: get_optional("publisher")?,
        mood: get_optional("mood")?,
        occasion: get_optional("occasion")?,
        tempo: get_optional("tempo")?,
        content_type: get_optional("content_type")?,
        category: get_optional("category")?,
        auto_tagged: get_optional_bool("auto_tagged")?,
        auto_tag_date: get_optional("auto_tag_date")?,
        auto_tag_version: get_optional("auto_tag_version")?,
    })
}