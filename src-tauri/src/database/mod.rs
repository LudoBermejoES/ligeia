use rusqlite::{Connection, Result};
use crate::models::{AudioFile, RpgTag, TagVocabulary, AudioFileWithTags};

pub mod schema;
pub mod audio_files;
pub mod rpg_tags;
pub mod vocabulary;
pub mod search;

pub use schema::SchemaManager;
pub use audio_files::AudioFileRepository;
pub use rpg_tags::RpgTagRepository;
pub use vocabulary::VocabularyRepository;
pub use search::SearchRepository;

/// Main database struct that coordinates all repositories
pub struct Database {
    conn: Connection,
    pub schema: SchemaManager,
    pub audio_files: AudioFileRepository,
    pub rpg_tags: RpgTagRepository,
    pub vocabulary: VocabularyRepository,
    pub search: SearchRepository,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("audio_player.db")?;
        
        let schema = SchemaManager::new(&conn);
        let audio_files = AudioFileRepository::new();
        let rpg_tags = RpgTagRepository::new();
        let vocabulary = VocabularyRepository::new();
        let search = SearchRepository::new();
        
        let db = Database {
            conn,
            schema,
            audio_files,
            rpg_tags,
            vocabulary,
            search,
        };
        
        // Initialize schema and vocabulary
        db.schema.create_tables(&db.conn)?;
        db.vocabulary.initialize_tag_vocabulary(&db.conn)?;
        
        Ok(db)
    }

    /// Get a reference to the database connection for complex operations
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    /// Clear all data from the database (used for import operations)
    pub fn clear_all_data(&self) -> Result<()> {
        self.conn.execute("DELETE FROM rpg_tags", [])?;
        self.conn.execute("DELETE FROM audio_files", [])?;
        log::info!("Cleared all audio files and RPG tags from database");
        Ok(())
    }

    // Legacy API compatibility methods that delegate to repositories
    
    pub fn save_audio_file(&self, audio_file: &AudioFile) -> Result<i64> {
        self.audio_files.save(&self.conn, audio_file)
    }

    pub fn get_all_audio_files(&self) -> Result<Vec<AudioFile>> {
        self.audio_files.get_all(&self.conn)
    }

    pub fn delete_audio_file(&self, id: i64) -> Result<()> {
        self.audio_files.delete(&self.conn, id)
    }

    pub fn update_audio_file_duration(&self, id: i64, duration: f64) -> Result<()> {
        self.audio_files.update_duration(&self.conn, id, duration)
    }

    pub fn update_audio_file_bpm(&self, id: i64, bpm: u32) -> Result<()> {
        self.audio_files.update_bpm(&self.conn, id, bpm)
    }

    pub fn update_audio_file_duration_and_bpm(&self, id: i64, duration: Option<f64>, bpm: Option<u32>) -> Result<()> {
        self.audio_files.update_duration_and_bpm(&self.conn, id, duration, bpm)
    }

    pub fn add_rpg_tag(&self, audio_file_id: i64, tag_type: &str, tag_value: &str) -> Result<i64> {
        self.rpg_tags.add(&self.conn, audio_file_id, tag_type, tag_value)
    }

    pub fn remove_rpg_tag(&self, audio_file_id: i64, tag_type: &str, tag_value: &str) -> Result<()> {
        self.rpg_tags.remove(&self.conn, audio_file_id, tag_type, tag_value)
    }

    pub fn get_rpg_tags_for_file(&self, audio_file_id: i64) -> Result<Vec<RpgTag>> {
        self.rpg_tags.get_for_file(&self.conn, audio_file_id)
    }

    pub fn get_audio_files_with_tags(&self) -> Result<Vec<AudioFileWithTags>> {
        self.search.get_all_files_with_tags(&self.conn)
    }

    pub fn add_tag_vocabulary(&self, tag_type: &str, tag_value: &str, description: Option<&str>, parent_tag: Option<&str>, is_active: bool) -> Result<()> {
        self.vocabulary.add(&self.conn, tag_type, tag_value, description, parent_tag, is_active)
    }

    pub fn get_tag_vocabulary(&self, tag_type: Option<&str>) -> Result<Vec<TagVocabulary>> {
        self.vocabulary.get(&self.conn, tag_type)
    }

    pub fn search_files_by_tags(&self, tag_types: Option<&[String]>, tag_values: Option<&[String]>, match_all: bool) -> Result<Vec<AudioFileWithTags>> {
        self.search.search_by_tags(&self.conn, tag_types, tag_values, match_all)
    }
}