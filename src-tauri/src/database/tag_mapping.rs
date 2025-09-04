use rusqlite::{Connection, Result, params};
use std::collections::HashMap;
use chrono::Utc;

pub struct TagMappingCache;

impl TagMappingCache {
    /// Get cached mapping for an invalid tag
    pub fn get_cached_mapping(conn: &Connection, invalid_tag: &str, tag_type: &str) -> Result<Option<String>> {
        let mut stmt = conn.prepare(
            "SELECT valid_tag FROM tag_mapping_cache 
             WHERE invalid_tag = ?1 AND tag_type = ?2"
        )?;
        
        let result = stmt.query_row(params![invalid_tag, tag_type], |row| {
            Ok(row.get::<_, String>(0)?)
        });
        
        match result {
            Ok(valid_tag) => {
                // Update usage count
                Self::increment_usage_count(conn, invalid_tag, tag_type)?;
                Ok(Some(valid_tag))
            },
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
    
    /// Store a new tag mapping in the cache
    pub fn store_mapping(conn: &Connection, invalid_tag: &str, valid_tag: &str, tag_type: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT OR REPLACE INTO tag_mapping_cache 
             (invalid_tag, valid_tag, tag_type, created_at, usage_count) 
             VALUES (?1, ?2, ?3, ?4, 1)",
            params![invalid_tag, valid_tag, tag_type, now],
        )?;
        
        Ok(())
    }
    
    /// Get all cached mappings for a specific tag type
    pub fn get_cached_mappings_by_type(conn: &Connection, tag_type: &str) -> Result<HashMap<String, String>> {
        let mut stmt = conn.prepare(
            "SELECT invalid_tag, valid_tag FROM tag_mapping_cache 
             WHERE tag_type = ?1"
        )?;
        
        let mapping_iter = stmt.query_map(params![tag_type], |row| {
            Ok((
                row.get::<_, String>(0)?,  // invalid_tag
                row.get::<_, String>(1)?   // valid_tag
            ))
        })?;
        
        let mut mappings = HashMap::new();
        for mapping in mapping_iter {
            let (invalid_tag, valid_tag) = mapping?;
            mappings.insert(invalid_tag, valid_tag);
        }
        
        Ok(mappings)
    }
    
    /// Get all cached mappings across all tag types
    pub fn get_all_cached_mappings(conn: &Connection) -> Result<(HashMap<String, String>, HashMap<String, String>, HashMap<String, String>, HashMap<String, String>)> {
        let genre_mappings = Self::get_cached_mappings_by_type(conn, "genre")?;
        let mood_mappings = Self::get_cached_mappings_by_type(conn, "mood")?;
        let occasion_mappings = Self::get_cached_mappings_by_type(conn, "occasion")?;
        let keyword_mappings = Self::get_cached_mappings_by_type(conn, "keyword")?;
        
        Ok((genre_mappings, mood_mappings, occasion_mappings, keyword_mappings))
    }
    
    /// Store multiple mappings at once
    pub fn store_mappings(conn: &Connection, genre_mappings: &HashMap<String, String>, mood_mappings: &HashMap<String, String>, occasion_mappings: &HashMap<String, String>, keyword_mappings: &HashMap<String, String>) -> Result<()> {
        let tx = conn.unchecked_transaction()?;
        
        // Store genre mappings
        for (invalid_tag, valid_tag) in genre_mappings {
            if valid_tag != "REMOVE" {
                Self::store_mapping(&tx, invalid_tag, valid_tag, "genre")?;
            }
        }
        
        // Store mood mappings
        for (invalid_tag, valid_tag) in mood_mappings {
            if valid_tag != "REMOVE" {
                Self::store_mapping(&tx, invalid_tag, valid_tag, "mood")?;
            }
        }
        
        // Store occasion mappings
        for (invalid_tag, valid_tag) in occasion_mappings {
            if valid_tag != "REMOVE" {
                Self::store_mapping(&tx, invalid_tag, valid_tag, "occasion")?;
            }
        }
        
        // Store keyword mappings
        for (invalid_tag, valid_tag) in keyword_mappings {
            if valid_tag != "REMOVE" {
                Self::store_mapping(&tx, invalid_tag, valid_tag, "keyword")?;
            }
        }
        
        tx.commit()?;
        Ok(())
    }
    
    /// Increment usage count for a cached mapping
    fn increment_usage_count(conn: &Connection, invalid_tag: &str, tag_type: &str) -> Result<()> {
        conn.execute(
            "UPDATE tag_mapping_cache 
             SET usage_count = usage_count + 1 
             WHERE invalid_tag = ?1 AND tag_type = ?2",
            params![invalid_tag, tag_type],
        )?;
        
        Ok(())
    }
    
    /// Get mapping statistics
    pub fn get_cache_stats(conn: &Connection) -> Result<(usize, usize, usize, usize)> {
        let mut stmt = conn.prepare(
            "SELECT tag_type, COUNT(*) FROM tag_mapping_cache GROUP BY tag_type"
        )?;
        
        let results = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,  // tag_type
                row.get::<_, usize>(1)?    // count
            ))
        })?;
        
        let mut genre_count = 0;
        let mut mood_count = 0;
        let mut occasion_count = 0;
        let mut keyword_count = 0;
        
        for result in results {
            let (tag_type, count) = result?;
            match tag_type.as_str() {
                "genre" => genre_count = count,
                "mood" => mood_count = count,
                "occasion" => occasion_count = count,
                "keyword" => keyword_count = count,
                _ => {}
            }
        }
        
        Ok((genre_count, mood_count, occasion_count, keyword_count))
    }
}