use rusqlite::{Connection, params, Result};
use crate::models::TagVocabulary;

/// Repository for tag vocabulary operations
pub struct VocabularyRepository;

impl VocabularyRepository {
    pub fn new() -> Self {
        VocabularyRepository
    }

    /// Initialize the tag vocabulary with all RPG tags
    pub fn initialize_tag_vocabulary(&self, conn: &Connection) -> Result<()> {
        // Check if vocabulary is already initialized
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM tag_vocabulary",
            [],
            |row| row.get(0)
        )?;

        if count > 0 {
            return Ok(()); // Already initialized
        }

        // Initialize vocabulary data
        self.insert_genre_vocabulary(conn)?;
        self.insert_mood_vocabulary(conn)?;
        self.insert_occasion_vocabulary(conn)?;
        self.insert_keyword_vocabulary(conn)?;

        log::info!("Tag vocabulary initialized successfully with {} entries", self.get_total_count(conn)?);
        Ok(())
    }

    /// Add a tag to the vocabulary
    pub fn add(&self, conn: &Connection, tag_type: &str, tag_value: &str, description: Option<&str>, parent_tag: Option<&str>, is_active: bool) -> Result<()> {
        conn.execute(
            "INSERT OR IGNORE INTO tag_vocabulary (tag_type, tag_value, description, parent_tag, is_active)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![tag_type, tag_value, description, parent_tag, is_active],
        )?;
        Ok(())
    }

    /// Get vocabulary entries, optionally filtered by tag type
    pub fn get(&self, conn: &Connection, tag_type: Option<&str>) -> Result<Vec<TagVocabulary>> {
        let (query, params): (String, Vec<&str>) = match tag_type {
            Some(t) => (
                "SELECT id, tag_type, tag_value, description, parent_tag, is_active
                 FROM tag_vocabulary WHERE tag_type = ?1 AND is_active = TRUE
                 ORDER BY tag_value".to_string(),
                vec![t]
            ),
            None => (
                "SELECT id, tag_type, tag_value, description, parent_tag, is_active
                 FROM tag_vocabulary WHERE is_active = TRUE
                 ORDER BY tag_type, tag_value".to_string(),
                vec![]
            )
        };

        let mut stmt = conn.prepare(&query)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params), |row| {
            Ok(TagVocabulary {
                id: Some(row.get(0)?),
                tag_type: row.get(1)?,
                tag_value: row.get(2)?,
                description: row.get(3)?,
                parent_tag: row.get(4)?,
                is_active: row.get(5)?,
            })
        })?;

        let mut vocab = Vec::new();
        for row in rows {
            vocab.push(row?);
        }
        Ok(vocab)
    }

    fn get_total_count(&self, conn: &Connection) -> Result<i64> {
        conn.query_row(
            "SELECT COUNT(*) FROM tag_vocabulary",
            [],
            |row| row.get(0)
        )
    }

    /// Insert a batch of vocabulary entries
    fn insert_batch(&self, conn: &Connection, entries: &[(&str, &str, Option<&str>, Option<&str>)]) -> Result<()> {
        let mut stmt = conn.prepare(
            "INSERT OR IGNORE INTO tag_vocabulary (tag_type, tag_value, description, parent_tag, is_active)
             VALUES (?1, ?2, ?3, ?4, TRUE)"
        )?;

        for (tag_type, tag_value, description, parent_tag) in entries {
            stmt.execute(params![tag_type, tag_value, description, parent_tag])?;
        }

        Ok(())
    }

    fn insert_genre_vocabulary(&self, conn: &Connection) -> Result<()> {
        let genres = include!("../data/genre_vocabulary.rs");
        self.insert_batch(conn, &genres)
    }

    fn insert_mood_vocabulary(&self, conn: &Connection) -> Result<()> {
        let moods = include!("../data/mood_vocabulary.rs");
        self.insert_batch(conn, &moods)
    }

    fn insert_occasion_vocabulary(&self, conn: &Connection) -> Result<()> {
        let occasions = include!("../data/occasion_vocabulary.rs");
        self.insert_batch(conn, &occasions)
    }

    fn insert_keyword_vocabulary(&self, conn: &Connection) -> Result<()> {
        let keywords = include!("../data/keyword_vocabulary.rs");
        self.insert_batch(conn, &keywords)
    }

    /// Static method for pool initialization - initializes vocabulary without instance
    pub fn initialize_tag_vocabulary_static(conn: &Connection) -> Result<()> {
        let vocab = VocabularyRepository;
        vocab.initialize_tag_vocabulary(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::schema::SchemaManager;

    fn setup() -> (Connection, VocabularyRepository) {
        let conn = Connection::open_in_memory().expect("mem db");
        conn.execute("PRAGMA foreign_keys = ON", []).ok();
        let schema = SchemaManager::new(&conn);
        schema.create_tables(&conn).expect("schema");
        (conn, VocabularyRepository::new())
    }

    #[test]
    fn initialize_and_get() {
        let (conn, repo) = setup();
        repo.initialize_tag_vocabulary(&conn).expect("init vocab");
        // Should have at least some entries after init
        let all = repo.get(&conn, None).expect("get all");
        assert!(!all.is_empty());
        // Filter by type
        let genres = repo.get(&conn, Some("genre")).expect("get genres");
        assert!(!genres.is_empty());
    }

    #[test]
    fn add_custom_tag_and_fetch() {
        let (conn, repo) = setup();
        repo.initialize_tag_vocabulary(&conn).unwrap();
        repo.add(&conn, "keyword", "custom", Some("Custom tag"), None, true).unwrap();
        let keywords = repo.get(&conn, Some("keyword")).unwrap();
        assert!(keywords.iter().any(|v| v.tag_value == "custom"));
    }
}