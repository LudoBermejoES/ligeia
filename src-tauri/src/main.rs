#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use audiotags::Tag;
use rusqlite::{Connection, params, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioFile {
    pub id: Option<i64>,
    pub file_path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<f64>,
    pub genre: Option<String>,
    pub year: Option<i32>,
    pub track_number: Option<i32>,
}

struct AppState {
    db: Mutex<Connection>,
}

#[tauri::command]
async fn load_audio_file(file_path: String) -> Result<AudioFile, String> {
    let mut audio_file = AudioFile {
        id: None,
        file_path: file_path.clone(),
        title: None,
        artist: None,
        album: None,
        duration: None,
        genre: None,
        year: None,
        track_number: None,
    };

    if let Ok(tag) = Tag::new().read_from_path(&file_path) {
        audio_file.title = tag.title().map(|s| s.to_string());
        audio_file.artist = tag.artist().map(|s| s.to_string());
        audio_file.album = tag.album_title().map(|s| s.to_string());
        audio_file.duration = tag.duration().map(|d| d as f64);
        audio_file.genre = tag.genre().map(|s| s.to_string());
        audio_file.year = tag.year().map(|y| y as i32);
        audio_file.track_number = tag.track_number().map(|t| t as i32);
    }

    Ok(audio_file)
}

#[tauri::command]
async fn save_audio_file(app_handle: tauri::AppHandle, audio_file: AudioFile) -> Result<i64, String> {
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
async fn get_all_audio_files(app_handle: tauri::AppHandle) -> Result<Vec<AudioFile>, String> {
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
            track_number: row.get(8)?,
        })
    }).map_err(|e| e.to_string())?;
    
    let mut audio_files = Vec::new();
    for row in rows {
        audio_files.push(row.map_err(|e| e.to_string())?);
    }
    
    Ok(audio_files)
}

#[tauri::command]
async fn delete_audio_file(app_handle: tauri::AppHandle, id: i64) -> Result<(), String> {
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

fn main() {
    let db = create_database().expect("Failed to create database");
    
    tauri::Builder::default()
        .manage(AppState {
            db: Mutex::new(db),
        })
        .invoke_handler(tauri::generate_handler![
            load_audio_file,
            save_audio_file,
            get_all_audio_files,
            delete_audio_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
