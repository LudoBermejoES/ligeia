use rusqlite::{Connection, Result};
use crate::models::{AudioFile, RpgTag, TagVocabulary, AudioFileWithTags, Atmosphere, AtmosphereWithSounds, AtmosphereSoundMapping, AtmosphereCategory};

pub mod schema;
pub mod audio_files;
pub mod rpg_tags;
pub mod vocabulary;
pub mod search;
pub mod atmospheres;

pub use schema::SchemaManager;
pub use audio_files::AudioFileRepository;
pub use rpg_tags::RpgTagRepository;
pub use vocabulary::VocabularyRepository;
pub use search::SearchRepository;
pub use atmospheres::AtmosphereRepository;

/// Main database struct that coordinates all repositories
pub struct Database {
    conn: Connection,
    pub schema: SchemaManager,
    pub audio_files: AudioFileRepository,
    pub rpg_tags: RpgTagRepository,
    pub vocabulary: VocabularyRepository,
    pub search: SearchRepository,
    pub atmospheres: AtmosphereRepository,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("../db/audio_player.db")?;
        
        let schema = SchemaManager::new(&conn);
        let audio_files = AudioFileRepository::new();
        let rpg_tags = RpgTagRepository::new();
        let vocabulary = VocabularyRepository::new();
        let search = SearchRepository::new();
        let atmospheres = AtmosphereRepository::new();
        
        let db = Database {
            conn,
            schema,
            audio_files,
            rpg_tags,
            vocabulary,
            search,
            atmospheres,
        };
        
        // Initialize schema and vocabulary
        db.schema.create_tables(&db.conn)?;
        db.vocabulary.initialize_tag_vocabulary(&db.conn)?;
        db.atmospheres.create_tables(&db.conn)?;
        
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

    pub fn get_all_rpg_tags(&self) -> Result<Vec<RpgTag>> {
        self.rpg_tags.get_all(&self.conn)
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

    // Atmosphere methods
    
    pub fn save_atmosphere(&self, atmosphere: &Atmosphere) -> Result<i64> {
        self.atmospheres.save(&self.conn, atmosphere)
    }

    pub fn save_atmosphere_with_sounds(&self, atmosphere: &Atmosphere, sounds: &[AtmosphereSoundMapping]) -> Result<i64> {
        self.atmospheres.save_with_sounds(&self.conn, atmosphere, sounds)
    }

    pub fn get_all_atmospheres(&self) -> Result<Vec<Atmosphere>> {
        self.atmospheres.get_all(&self.conn)
    }

    pub fn get_atmosphere_by_id(&self, id: i64) -> Result<Atmosphere> {
        self.atmospheres.get_by_id(&self.conn, id)
    }

    pub fn delete_atmosphere(&self, id: i64) -> Result<()> {
        self.atmospheres.delete(&self.conn, id)
    }

    pub fn add_sound_to_atmosphere(&self, atmosphere_id: i64, audio_file_id: i64, volume: f32, is_looping: bool) -> Result<i64> {
        self.atmospheres.add_sound(&self.conn, atmosphere_id, audio_file_id, volume, is_looping)
    }

    pub fn remove_sound_from_atmosphere(&self, atmosphere_id: i64, audio_file_id: i64) -> Result<()> {
        self.atmospheres.remove_sound(&self.conn, atmosphere_id, audio_file_id)
    }

    pub fn update_atmosphere_sound(&self, atmosphere_id: i64, audio_file_id: i64, volume: f32, is_looping: bool, is_muted: bool) -> Result<()> {
        self.atmospheres.update_sound(&self.conn, atmosphere_id, audio_file_id, volume, is_looping, is_muted)
    }

    pub fn get_atmosphere_with_sounds(&self, atmosphere_id: i64) -> Result<AtmosphereWithSounds> {
        self.atmospheres.get_with_sounds(&self.conn, atmosphere_id)
    }

    pub fn get_atmosphere_categories(&self) -> Result<Vec<AtmosphereCategory>> {
        self.atmospheres.get_categories(&self.conn)
    }
}