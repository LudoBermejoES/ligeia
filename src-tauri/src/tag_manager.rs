use std::sync::Mutex;
use crate::database::Database;
use crate::models::{RpgTag, TagVocabulary, BulkTagRequest, TagSearchRequest, AudioFileWithTags};

pub struct TagManager {
    db: Mutex<Database>,
}

impl TagManager {
    pub fn new() -> Result<Self, String> {
        let db = Database::new().map_err(|e| e.to_string())?;
        Ok(TagManager {
            db: Mutex::new(db),
        })
    }

    /// For tests: build a TagManager with a prebuilt Database
    #[cfg(test)]
    pub fn with_database(db: Database) -> Self {
        TagManager { db: Mutex::new(db) }
    }

    pub fn get_tag_vocabulary(&self, tag_type: Option<&str>) -> Result<Vec<TagVocabulary>, String> {
        let db = self.db.lock().unwrap();
        db.get_tag_vocabulary(tag_type).map_err(|e| e.to_string())
    }

    pub fn add_rpg_tag(&self, audio_file_id: i64, tag_type: &str, tag_value: &str) -> Result<i64, String> {
        let db = self.db.lock().unwrap();
        
        // Auto-add tag to vocabulary if it doesn't exist (for import compatibility)
        if !self.is_valid_tag(&db, tag_type, tag_value)? {
            // Add the tag type and value to vocabulary
            self.auto_add_tag_to_vocabulary(&db, tag_type, tag_value)?;
        }
        
        db.add_rpg_tag(audio_file_id, tag_type, tag_value).map_err(|e| e.to_string())
    }

    pub fn remove_rpg_tag(&self, audio_file_id: i64, tag_type: &str, tag_value: &str) -> Result<(), String> {
        let db = self.db.lock().unwrap();
        db.remove_rpg_tag(audio_file_id, tag_type, tag_value).map_err(|e| e.to_string())
    }

    pub fn get_rpg_tags_for_file(&self, audio_file_id: i64) -> Result<Vec<RpgTag>, String> {
        let db = self.db.lock().unwrap();
        db.get_rpg_tags_for_file(audio_file_id).map_err(|e| e.to_string())
    }

    pub fn bulk_tag_files(&self, request: BulkTagRequest) -> Result<(), String> {
        let db = self.db.lock().unwrap();
        
        // First, get the audio file IDs for the given file paths
        let audio_file_ids = Vec::new();
        for _file_path in &request.file_paths {
            // This would require adding a method to get audio file by path
            // For now, we'll skip this implementation detail
        }

        // Add tags
        for tag in &request.tags_to_add {
            for &audio_file_id in &audio_file_ids {
                // Validate tag
                if !self.is_valid_tag(&db, &tag.tag_type, &tag.tag_value)? {
                    return Err(format!("Invalid tag: {} = {}", tag.tag_type, tag.tag_value));
                }
                
                db.add_rpg_tag(audio_file_id, &tag.tag_type, &tag.tag_value)
                    .map_err(|e| e.to_string())?;
            }
        }

        // Remove tags
        for tag in &request.tags_to_remove {
            for &audio_file_id in &audio_file_ids {
                db.remove_rpg_tag(audio_file_id, &tag.tag_type, &tag.tag_value)
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    pub fn search_files_by_tags(&self, request: TagSearchRequest) -> Result<Vec<AudioFileWithTags>, String> {
        let db = self.db.lock().unwrap();
        
        let tag_types = request.tag_types.as_ref().map(|v| v.as_slice());
        let tag_values = request.tag_values.as_ref().map(|v| v.as_slice());
        
        db.search_files_by_tags(tag_types, tag_values, request.match_all)
            .map_err(|e| e.to_string())
    }

    pub fn get_all_audio_files_with_tags(&self) -> Result<Vec<AudioFileWithTags>, String> {
        let db = self.db.lock().unwrap();
        db.get_audio_files_with_tags().map_err(|e| e.to_string())
    }

    pub fn get_tag_statistics(&self) -> Result<TagStatistics, String> {
        let db = self.db.lock().unwrap();
        
        // Get count of files per tag type
        let genre_count = self.count_files_with_tag_type(&db, "genre")?;
        let mood_count = self.count_files_with_tag_type(&db, "mood")?;
        let occasion_count = self.count_files_with_tag_type(&db, "occasion")?;
        let keyword_count = self.count_files_with_tag_type(&db, "keyword")?;
        
        // Get most common tags
        let most_common_genres = self.get_most_common_tags(&db, "genre", 10)?;
        let most_common_moods = self.get_most_common_tags(&db, "mood", 10)?;
        let most_common_occasions = self.get_most_common_tags(&db, "occasion", 10)?;
        let most_common_keywords = self.get_most_common_tags(&db, "keyword", 10)?;

        Ok(TagStatistics {
            genre_count,
            mood_count,
            occasion_count,
            keyword_count,
            most_common_genres,
            most_common_moods,
            most_common_occasions,
            most_common_keywords,
        })
    }

    fn is_valid_tag(&self, db: &Database, tag_type: &str, tag_value: &str) -> Result<bool, String> {
        let vocabulary = db.get_tag_vocabulary(Some(tag_type)).map_err(|e| e.to_string())?;
        Ok(vocabulary.iter().any(|v| v.tag_value == tag_value && v.is_active))
    }

    fn auto_add_tag_to_vocabulary(&self, db: &Database, tag_type: &str, tag_value: &str) -> Result<(), String> {
        // Add the tag value to vocabulary if it doesn't exist
        db.add_tag_vocabulary(tag_type, tag_value, Some(&format!("Auto-added {} tag", tag_type)), None, true)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_existing_tags(&self) -> Result<std::collections::HashMap<String, std::collections::HashSet<String>>, String> {
        let db = self.db.lock().unwrap();
        let mut existing_tags = std::collections::HashMap::new();
        
        // Initialize sets for each tag type
        existing_tags.insert("genre".to_string(), std::collections::HashSet::new());
        existing_tags.insert("mood".to_string(), std::collections::HashSet::new());
        existing_tags.insert("occasion".to_string(), std::collections::HashSet::new());
        existing_tags.insert("keyword".to_string(), std::collections::HashSet::new());
        
        // Get tags from rpg_tags table
        let rpg_tags = db.get_all_rpg_tags().map_err(|e| e.to_string())?;
        for tag in rpg_tags {
            if let Some(tag_set) = existing_tags.get_mut(&tag.tag_type) {
                tag_set.insert(tag.tag_value);
            }
        }
        
        // Get tags from audio file metadata fields
        let audio_files = db.get_all_audio_files().map_err(|e| e.to_string())?;
        for file in audio_files {
            // Parse genre field
            if let Some(genre) = file.genre {
                for tag in genre.split("; ").filter(|s| !s.trim().is_empty()) {
                    existing_tags.get_mut("genre").unwrap().insert(tag.trim().to_string());
                }
            }
            
            // Parse mood field  
            if let Some(mood) = file.mood {
                for tag in mood.split("; ").filter(|s| !s.trim().is_empty()) {
                    existing_tags.get_mut("mood").unwrap().insert(tag.trim().to_string());
                }
            }
            
            // Parse occasion field
            if let Some(occasion) = file.occasion {
                for tag in occasion.split("; ").filter(|s| !s.trim().is_empty()) {
                    existing_tags.get_mut("occasion").unwrap().insert(tag.trim().to_string());
                }
            }
            
            // Parse category field as keywords
            if let Some(category) = file.category {
                for tag in category.split("; ").filter(|s| !s.trim().is_empty()) {
                    existing_tags.get_mut("keyword").unwrap().insert(tag.trim().to_string());
                }
            }
        }
        
        Ok(existing_tags)
    }

    fn count_files_with_tag_type(&self, _db: &Database, _tag_type: &str) -> Result<u32, String> {
        // This would require a specific query to count distinct files with a tag type
        // For now, return 0 as placeholder
        Ok(0)
    }

    fn get_most_common_tags(&self, _db: &Database, _tag_type: &str, _limit: u32) -> Result<Vec<TagUsage>, String> {
        // This would require a query to get tag usage statistics
        // For now, return empty vector as placeholder
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;
    use rusqlite::Connection;
    use crate::models::AudioFile;

    fn setup_manager() -> TagManager {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("PRAGMA foreign_keys = ON", []).ok();
        let db = Database::with_connection(conn).unwrap();
        TagManager::with_database(db)
    }

    #[test]
    fn auto_adds_unknown_tag_and_lists_existing() {
        let mgr = setup_manager();
        // Insert a file
        let file = AudioFile { id: None, file_path: "/tmp/auto.mp3".into(), title: None, artist: None, album: None,
            album_artist: None, genre: Some("ambient".into()), year: None, date: None, track_number: None,
            total_tracks: None, disc_number: None, total_discs: None, duration: None, composer: None,
            conductor: None, lyricist: None, original_artist: None, remixer: None, arranger: None, engineer: None,
            producer: None, dj_mixer: None, mixer: None, content_group: None, subtitle: None, initial_key: None,
            bpm: None, language: None, media_type: None, original_filename: None, original_lyricist: None,
            original_release_time: None, playlist_delay: None, recording_time: None, release_time: None,
            tagging_time: None, encoding_time: None, encoding_settings: None, encoded_by: None, copyright: None,
            file_owner: None, internet_radio_station_name: None, internet_radio_station_owner: None, isrc: None,
            publisher: None, mood: Some("calm".into()), occasion: None, tempo: None, content_type: None, category: None };
        let db = mgr.db.lock().unwrap();
        let id = db.save_audio_file(&file).unwrap();
        drop(db);

        // Add a new, not-in-vocabulary keyword -> auto-add to vocabulary
        let _ = mgr.add_rpg_tag(id, "keyword", "rare-key").unwrap();

        // Verify it shows up in vocabulary and existing tags
        let vocab_keywords = mgr.get_tag_vocabulary(Some("keyword")).unwrap();
        assert!(vocab_keywords.iter().any(|v| v.tag_value == "rare-key"));

        let existing = mgr.get_existing_tags().unwrap();
        assert!(existing.get("genre").unwrap().contains("ambient"));
    }
}

#[derive(Debug, serde::Serialize)]
pub struct TagStatistics {
    pub genre_count: u32,
    pub mood_count: u32,
    pub occasion_count: u32,
    pub keyword_count: u32,
    pub most_common_genres: Vec<TagUsage>,
    pub most_common_moods: Vec<TagUsage>,
    pub most_common_occasions: Vec<TagUsage>,
    pub most_common_keywords: Vec<TagUsage>,
}

#[derive(Debug, serde::Serialize)]
pub struct TagUsage {
    pub tag_value: String,
    pub usage_count: u32,
}