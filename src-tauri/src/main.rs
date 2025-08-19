#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use id3::{Tag, TagLike};
use rusqlite::{Connection, params, Result};
use tauri::{AppHandle, Manager};
use scan_dir::ScanDir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioFile {
    pub id: Option<i64>,
    pub file_path: String,
    // Basic tags
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub genre: Option<String>,
    pub year: Option<i32>,
    pub date: Option<String>,
    pub track_number: Option<u32>,
    pub total_tracks: Option<u32>,
    pub disc_number: Option<u32>,
    pub total_discs: Option<u32>,
    pub duration: Option<f64>,
    
    // Additional ID3v2 tags
    pub composer: Option<String>,
    pub conductor: Option<String>,
    pub lyricist: Option<String>,
    pub original_artist: Option<String>,
    pub remixer: Option<String>,
    pub arranger: Option<String>,
    pub engineer: Option<String>,
    pub producer: Option<String>,
    pub dj_mixer: Option<String>,
    pub mixer: Option<String>,
    
    // Content tags
    pub content_group: Option<String>,
    pub subtitle: Option<String>,
    pub initial_key: Option<String>,
    pub bpm: Option<u32>,
    pub language: Option<String>,
    pub media_type: Option<String>,
    pub original_filename: Option<String>,
    pub original_lyricist: Option<String>,
    pub original_release_time: Option<String>,
    pub playlist_delay: Option<u32>,
    
    // Recording info
    pub recording_time: Option<String>,
    pub release_time: Option<String>,
    pub tagging_time: Option<String>,
    pub encoding_time: Option<String>,
    pub encoding_settings: Option<String>,
    pub encoded_by: Option<String>,
    
    // Copyright and legal
    pub copyright: Option<String>,
    pub file_owner: Option<String>,
    pub internet_radio_station_name: Option<String>,
    pub internet_radio_station_owner: Option<String>,
    pub isrc: Option<String>,
    pub publisher: Option<String>,
    
    // Additional metadata
    pub mood: Option<String>,
    pub occasion: Option<String>,
    pub tempo: Option<String>,
    pub content_type: Option<String>,
    pub category: Option<String>,
}

struct AppState {
    db: Mutex<Connection>,
}

#[tauri::command]
async fn load_audio_file(file_path: String) -> Result<AudioFile, String> {
    let mut audio_file = AudioFile {
        id: None,
        file_path: file_path.clone(),
        // Initialize all fields as None
        title: None,
        artist: None,
        album: None,
        album_artist: None,
        genre: None,
        year: None,
        date: None,
        track_number: None,
        total_tracks: None,
        disc_number: None,
        total_discs: None,
        duration: None,
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
        bpm: None,
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
    };

    if let Ok(tag) = Tag::read_from_path(&file_path) {
        // Basic tags
        audio_file.title = tag.title().map(|s| s.to_string());
        audio_file.artist = tag.artist().map(|s| s.to_string());
        audio_file.album = tag.album().map(|s| s.to_string());
        audio_file.album_artist = tag.album_artist().map(|s| s.to_string());
        audio_file.genre = tag.genre().map(|s| s.to_string());
        audio_file.year = tag.year().map(|y| y as i32);
        audio_file.date = tag.date_recorded().map(|d| d.to_string());
        audio_file.track_number = tag.track();
        audio_file.total_tracks = tag.total_tracks();
        audio_file.disc_number = tag.disc();
        audio_file.total_discs = tag.total_discs();
        
        // Duration from file analysis (not from tag)  
        audio_file.duration = tag.duration().map(|d| d as f64);
        
        // Extended tags
        for frame in tag.frames() {
            match frame.id() {
                "TCOM" => audio_file.composer = frame.content().text().map(|s| s.to_string()),
                "TPE3" => audio_file.conductor = frame.content().text().map(|s| s.to_string()),
                "TEXT" => audio_file.lyricist = frame.content().text().map(|s| s.to_string()),
                "TOPE" => audio_file.original_artist = frame.content().text().map(|s| s.to_string()),
                "TPE4" => audio_file.remixer = frame.content().text().map(|s| s.to_string()),
                "TIPL" => {
                    // Involved people list - could contain arranger, engineer, producer, etc.
                    if let Some(text) = frame.content().text() {
                        if text.to_lowercase().contains("arranger") {
                            audio_file.arranger = Some(text.to_string());
                        } else if text.to_lowercase().contains("engineer") {
                            audio_file.engineer = Some(text.to_string());
                        } else if text.to_lowercase().contains("producer") {
                            audio_file.producer = Some(text.to_string());
                        } else if text.to_lowercase().contains("dj") {
                            audio_file.dj_mixer = Some(text.to_string());
                        } else if text.to_lowercase().contains("mix") {
                            audio_file.mixer = Some(text.to_string());
                        }
                    }
                },
                "TIT1" => audio_file.content_group = frame.content().text().map(|s| s.to_string()),
                "TIT3" => audio_file.subtitle = frame.content().text().map(|s| s.to_string()),
                "TKEY" => audio_file.initial_key = frame.content().text().map(|s| s.to_string()),
                "TBPM" => audio_file.bpm = frame.content().text().and_then(|s| s.parse().ok()),
                "TLAN" => audio_file.language = frame.content().text().map(|s| s.to_string()),
                "TMED" => audio_file.media_type = frame.content().text().map(|s| s.to_string()),
                "TOFN" => audio_file.original_filename = frame.content().text().map(|s| s.to_string()),
                "TOLY" => audio_file.original_lyricist = frame.content().text().map(|s| s.to_string()),
                "TORY" => audio_file.original_release_time = frame.content().text().map(|s| s.to_string()),
                "TDLY" => audio_file.playlist_delay = frame.content().text().and_then(|s| s.parse().ok()),
                "TDRC" => audio_file.recording_time = frame.content().text().map(|s| s.to_string()),
                "TDRL" => audio_file.release_time = frame.content().text().map(|s| s.to_string()),
                "TDTG" => audio_file.tagging_time = frame.content().text().map(|s| s.to_string()),
                "TDEN" => audio_file.encoding_time = frame.content().text().map(|s| s.to_string()),
                "TSSE" => audio_file.encoding_settings = frame.content().text().map(|s| s.to_string()),
                "TENC" => audio_file.encoded_by = frame.content().text().map(|s| s.to_string()),
                "TCOP" => audio_file.copyright = frame.content().text().map(|s| s.to_string()),
                "TOWN" => audio_file.file_owner = frame.content().text().map(|s| s.to_string()),
                "TRSN" => audio_file.internet_radio_station_name = frame.content().text().map(|s| s.to_string()),
                "TRSO" => audio_file.internet_radio_station_owner = frame.content().text().map(|s| s.to_string()),
                "TSRC" => audio_file.isrc = frame.content().text().map(|s| s.to_string()),
                "TPUB" => audio_file.publisher = frame.content().text().map(|s| s.to_string()),
                "TMOO" => audio_file.mood = frame.content().text().map(|s| s.to_string()),
                _ => {} // Ignore unknown frames
            }
        }
    }

    Ok(audio_file)
}

#[tauri::command]
async fn save_audio_file(app_handle: AppHandle, audio_file: AudioFile) -> Result<i64, String> {
    let state = app_handle.state::<AppState>();
    let conn = state.db.lock().unwrap();
    
    let id = conn.execute(
        "INSERT INTO audio_files (file_path, title, artist, album, duration, genre, year, track_number)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            audio_file.file_path,
            audio_file.title,
            audio_file.artist,
            audio_file.album,
            audio_file.duration,
            audio_file.genre,
            audio_file.year,
            audio_file.track_number,
        ],
    ).map_err(|e| e.to_string())?;
    
    Ok(id as i64)
}

#[tauri::command]
async fn get_all_audio_files(app_handle: AppHandle) -> Result<Vec<AudioFile>, String> {
    let state = app_handle.state::<AppState>();
    let conn = state.db.lock().unwrap();
    
    let mut stmt = conn.prepare(
        "SELECT id, file_path, title, artist, album, duration, genre, year, track_number 
         FROM audio_files ORDER BY artist, album, track_number"
    ).map_err(|e| e.to_string())?;
    
    let rows = stmt.query_map([], |row| {
        Ok(AudioFile {
            id: Some(row.get(0)?),
            file_path: row.get(1)?,
            title: row.get(2)?,
            artist: row.get(3)?,
            album: row.get(4)?,
            duration: row.get(5)?,
            genre: row.get(6)?,
            year: row.get(7)?,
            track_number: row.get(8).unwrap_or(None),
            // Initialize all new fields as None since they're not in the database yet
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
            bpm: None,
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
        })
    }).map_err(|e| e.to_string())?;
    
    let mut audio_files = Vec::new();
    for row in rows {
        audio_files.push(row.map_err(|e| e.to_string())?);
    }
    
    Ok(audio_files)
}

#[tauri::command]
async fn update_audio_file_tags(file_path: String, updates: AudioFile) -> Result<(), String> {
    let mut tag = Tag::read_from_path(&file_path).unwrap_or_else(|_| Tag::new());
    
    // Update basic tags
    if let Some(title) = &updates.title {
        tag.set_title(title);
    }
    if let Some(artist) = &updates.artist {
        tag.set_artist(artist);
    }
    if let Some(album) = &updates.album {
        tag.set_album(album);
    }
    if let Some(album_artist) = &updates.album_artist {
        tag.set_album_artist(album_artist);
    }
    if let Some(genre) = &updates.genre {
        tag.set_genre(genre);
    }
    if let Some(year) = updates.year {
        tag.set_year(year);
    }
    if let Some(track) = updates.track_number {
        tag.set_track(track);
    }
    if let Some(total_tracks) = updates.total_tracks {
        tag.set_total_tracks(total_tracks);
    }
    if let Some(disc) = updates.disc_number {
        tag.set_disc(disc);
    }
    if let Some(total_discs) = updates.total_discs {
        tag.set_total_discs(total_discs);
    }
    
    // Update extended tags using frame manipulation
    if let Some(composer) = &updates.composer {
        tag.set_text("TCOM", composer);
    }
    if let Some(conductor) = &updates.conductor {
        tag.set_text("TPE3", conductor);
    }
    if let Some(lyricist) = &updates.lyricist {
        tag.set_text("TEXT", lyricist);
    }
    if let Some(original_artist) = &updates.original_artist {
        tag.set_text("TOPE", original_artist);
    }
    if let Some(remixer) = &updates.remixer {
        tag.set_text("TPE4", remixer);
    }
    if let Some(content_group) = &updates.content_group {
        tag.set_text("TIT1", content_group);
    }
    if let Some(subtitle) = &updates.subtitle {
        tag.set_text("TIT3", subtitle);
    }
    if let Some(initial_key) = &updates.initial_key {
        tag.set_text("TKEY", initial_key);
    }
    if let Some(bpm) = updates.bpm {
        tag.set_text("TBPM", &bpm.to_string());
    }
    if let Some(language) = &updates.language {
        tag.set_text("TLAN", language);
    }
    if let Some(media_type) = &updates.media_type {
        tag.set_text("TMED", media_type);
    }
    if let Some(copyright) = &updates.copyright {
        tag.set_text("TCOP", copyright);
    }
    if let Some(encoded_by) = &updates.encoded_by {
        tag.set_text("TENC", encoded_by);
    }
    if let Some(publisher) = &updates.publisher {
        tag.set_text("TPUB", publisher);
    }
    if let Some(mood) = &updates.mood {
        tag.set_text("TMOO", mood);
    }
    
    // Write the updated tag back to the file
    tag.write_to_path(&file_path, id3::Version::Id3v24)
        .map_err(|e| format!("Failed to write tags: {}", e))?;
    
    Ok(())
}

#[tauri::command]
async fn scan_directory_recursive(dir_path: String) -> Result<Vec<String>, String> {
    println!("Scanning directory recursively: {}", dir_path);
    
    let audio_extensions = vec!["mp3", "wav", "ogg", "flac", "aac", "m4a", "wma", "m4p"];
    
    let audio_files = ScanDir::files().walk(&dir_path, |iter| {
        iter.filter(|&(_, ref name)| {
            audio_extensions.iter().any(|ext| {
                name.to_lowercase().ends_with(&format!(".{}", ext))
            })
        })
        .map(|(entry, _)| entry.path().to_string_lossy().to_string())
        .collect::<Vec<String>>()
    }).map_err(|e| format!("Failed to scan directory: {:?}", e))?;
    
    println!("Found {} audio files", audio_files.len());
    for file in &audio_files {
        println!("Audio file: {}", file);
    }
    
    Ok(audio_files)
}

#[tauri::command]
async fn delete_audio_file(app_handle: AppHandle, id: i64) -> Result<(), String> {
    let state = app_handle.state::<AppState>();
    let conn = state.db.lock().unwrap();
    
    conn.execute(
        "DELETE FROM audio_files WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}

fn create_database() -> Result<Connection> {
    let conn = Connection::open("audio_player.db")?;
    
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
    
    Ok(conn)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db = create_database().expect("Failed to create database");
    
    tauri::Builder::default()
        .manage(AppState {
            db: Mutex::new(db),
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            load_audio_file,
            save_audio_file,
            get_all_audio_files,
            delete_audio_file,
            update_audio_file_tags,
            scan_directory_recursive
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
