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