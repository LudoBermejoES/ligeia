use rusqlite::{Connection, params, Result};
use crate::models::RpgTag;

/// Repository for RPG tag operations
pub struct RpgTagRepository;

impl RpgTagRepository {
    pub fn new() -> Self {
        RpgTagRepository
    }

    /// Add an RPG tag to an audio file
    pub fn add(&self, conn: &Connection, audio_file_id: i64, tag_type: &str, tag_value: &str) -> Result<i64> {
        let _id = conn.execute(
            "INSERT OR IGNORE INTO rpg_tags (audio_file_id, tag_type, tag_value)
             VALUES (?1, ?2, ?3)",
            params![audio_file_id, tag_type, tag_value],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Remove an RPG tag from an audio file
    pub fn remove(&self, conn: &Connection, audio_file_id: i64, tag_type: &str, tag_value: &str) -> Result<()> {
        conn.execute(
            "DELETE FROM rpg_tags WHERE audio_file_id = ?1 AND tag_type = ?2 AND tag_value = ?3",
            params![audio_file_id, tag_type, tag_value],
        )?;
        Ok(())
    }

    /// Get all RPG tags for a specific audio file
    pub fn get_for_file(&self, conn: &Connection, audio_file_id: i64) -> Result<Vec<RpgTag>> {
        let mut stmt = conn.prepare(
            "SELECT id, audio_file_id, tag_type, tag_value, created_at
             FROM rpg_tags WHERE audio_file_id = ?1 ORDER BY tag_type, tag_value"
        )?;

        let rows = stmt.query_map([audio_file_id], |row| {
            Ok(RpgTag {
                id: Some(row.get(0)?),
                audio_file_id: row.get(1)?,
                tag_type: row.get(2)?,
                tag_value: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;

        let mut tags = Vec::new();
        for row in rows {
            tags.push(row?);
        }
        Ok(tags)
    }

    /// Get all RPG tags grouped by audio file
    #[allow(dead_code)]
    pub fn get_all_grouped(&self, conn: &Connection) -> Result<Vec<(i64, Vec<RpgTag>)>> {
        let mut stmt = conn.prepare(
            "SELECT id, audio_file_id, tag_type, tag_value, created_at
             FROM rpg_tags ORDER BY audio_file_id, tag_type, tag_value"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(RpgTag {
                id: Some(row.get(0)?),
                audio_file_id: row.get(1)?,
                tag_type: row.get(2)?,
                tag_value: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;

        let mut grouped_tags: Vec<(i64, Vec<RpgTag>)> = Vec::new();
        let mut current_file_id: Option<i64> = None;
        let mut current_tags: Vec<RpgTag> = Vec::new();

        for row_result in rows {
            let tag = row_result?;
            let tag_audio_file_id = tag.audio_file_id; // Extract the ID before moving
            
            match current_file_id {
                Some(file_id) if file_id == tag_audio_file_id => {
                    // Same file, add to current group
                    current_tags.push(tag);
                }
                Some(file_id) => {
                    // Different file, save previous group and start new one
                    grouped_tags.push((file_id, current_tags));
                    current_tags = vec![tag];
                    current_file_id = Some(tag_audio_file_id);
                }
                None => {
                    // First file
                    current_file_id = Some(tag_audio_file_id);
                    current_tags = vec![tag];
                }
            }
        }

        // Don't forget the last group
        if let Some(file_id) = current_file_id {
            grouped_tags.push((file_id, current_tags));
        }

        Ok(grouped_tags)
    }

    /// Remove all tags for a specific audio file
    #[allow(dead_code)]
    pub fn remove_all_for_file(&self, conn: &Connection, audio_file_id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM rpg_tags WHERE audio_file_id = ?1",
            params![audio_file_id],
        )?;
        Ok(())
    }

    /// Get distinct tag values for a specific tag type
    #[allow(dead_code)]
    pub fn get_distinct_values_for_type(&self, conn: &Connection, tag_type: &str) -> Result<Vec<String>> {
        let mut stmt = conn.prepare(
            "SELECT DISTINCT tag_value FROM rpg_tags WHERE tag_type = ?1 ORDER BY tag_value"
        )?;

        let rows = stmt.query_map([tag_type], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;

        let mut values = Vec::new();
        for row in rows {
            values.push(row?);
        }
        Ok(values)
    }
}