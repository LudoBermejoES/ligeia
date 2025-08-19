use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RpgTag {
    pub id: Option<i64>,
    pub audio_file_id: i64,
    pub tag_type: String, // "genre", "mood", "occasion", "keyword"
    pub tag_value: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagVocabulary {
    pub id: Option<i64>,
    pub tag_type: String,
    pub tag_value: String,
    pub description: Option<String>,
    pub parent_tag: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkTagRequest {
    pub file_paths: Vec<String>,
    pub tags_to_add: Vec<RpgTag>,
    pub tags_to_remove: Vec<RpgTag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagSearchRequest {
    pub tag_types: Option<Vec<String>>,
    pub tag_values: Option<Vec<String>>,
    pub match_all: bool, // true for AND, false for OR
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioFileWithTags {
    pub audio_file: AudioFile,
    pub rpg_tags: Vec<RpgTag>,
}