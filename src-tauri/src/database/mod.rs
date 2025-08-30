use rusqlite::{Connection, Result};
use crate::models::{AudioFile, RpgTag, TagVocabulary, AudioFileWithTags, Atmosphere, AtmosphereWithSounds, AtmosphereSoundMapping, AtmosphereCategory, VirtualFolder, VirtualFolderTree, VirtualFolderWithContents, FolderTemplate};

pub mod schema;
pub mod audio_files;
pub mod rpg_tags;
pub mod vocabulary;
pub mod search;
pub mod atmospheres;
pub mod virtual_folders;

pub use schema::SchemaManager;
pub use audio_files::AudioFileOps;
pub use rpg_tags::RpgTagRepository;
pub use vocabulary::VocabularyRepository;
pub use search::SearchRepository;
pub use atmospheres::AtmosphereOps;
pub use virtual_folders::VirtualFolderOps;

/// Main database struct that coordinates all database operations
pub struct Database {
    conn: Connection,
    pub schema: SchemaManager,
    pub rpg_tags: RpgTagRepository,
    pub vocabulary: VocabularyRepository,
    pub search: SearchRepository,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("../db/audio_player.db")?;
        
        Self::from_connection(conn)
    }

    /// Build a Database from an existing rusqlite Connection (e.g., in-memory for tests)
    pub fn with_connection(conn: Connection) -> Result<Self> {
        Self::from_connection(conn)
    }

    fn from_connection(conn: Connection) -> Result<Self> {
        
        let schema = SchemaManager::new(&conn);
        let rpg_tags = RpgTagRepository::new();
        let vocabulary = VocabularyRepository::new();
        let search = SearchRepository::new();
        
        let db = Database {
            conn,
            schema,
            rpg_tags,
            vocabulary,
            search,
        };
        
        // Initialize schema and vocabulary
        db.schema.create_tables(&db.conn)?;
        db.vocabulary.initialize_tag_vocabulary(&db.conn)?;
        AudioFileOps::create_table(&db.conn)?;
        AtmosphereOps::create_tables(&db.conn)?;
        
        // Initialize default virtual folders
        VirtualFolderOps::initialize_default_virtual_folders(&db.conn)?;
        
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
        AudioFileOps::save(&self.conn, audio_file)
    }

    pub fn get_all_audio_files(&self) -> Result<Vec<AudioFile>> {
        AudioFileOps::get_all(&self.conn)
    }

    pub fn get_audio_file_by_path(&self, file_path: &str) -> Result<AudioFile> {
        AudioFileOps::get_by_path(&self.conn, file_path)
    }

    pub fn update_audio_file(&self, audio_file: &AudioFile) -> Result<()> {
        AudioFileOps::update(&self.conn, audio_file)
    }

    pub fn delete_audio_file(&self, id: i64) -> Result<()> {
        AudioFileOps::delete(&self.conn, id)
    }

    pub fn update_audio_file_duration(&self, id: i64, duration: f64) -> Result<()> {
        AudioFileOps::update_duration(&self.conn, id, duration)
    }

    pub fn update_audio_file_bpm(&self, id: i64, bpm: u32) -> Result<()> {
        AudioFileOps::update_bpm(&self.conn, id, bpm)
    }

    pub fn update_audio_file_duration_and_bpm(&self, id: i64, duration: Option<f64>, bpm: Option<u32>) -> Result<()> {
        if let Some(dur) = duration {
            AudioFileOps::update_duration(&self.conn, id, dur)?;
        }
        if let Some(b) = bpm {
            AudioFileOps::update_bpm(&self.conn, id, b)?;
        }
        Ok(())
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
        AtmosphereOps::save(&self.conn, atmosphere)
    }

    pub fn save_atmosphere_with_sounds(&self, atmosphere: &Atmosphere, sounds: &[AtmosphereSoundMapping]) -> Result<i64> {
        AtmosphereOps::save_with_sounds(&self.conn, atmosphere, sounds)
    }

    pub fn get_all_atmospheres(&self) -> Result<Vec<Atmosphere>> {
        AtmosphereOps::get_all(&self.conn)
    }

    pub fn get_atmosphere_by_id(&self, id: i64) -> Result<Atmosphere> {
        AtmosphereOps::get_by_id(&self.conn, id)
    }

    pub fn delete_atmosphere(&self, id: i64) -> Result<()> {
        AtmosphereOps::delete(&self.conn, id)
    }

    pub fn add_sound_to_atmosphere(&self, atmosphere_id: i64, audio_file_id: i64, volume: f32, is_looping: bool) -> Result<i64> {
        AtmosphereOps::add_sound(&self.conn, atmosphere_id, audio_file_id, volume, is_looping)
    }

    pub fn remove_sound_from_atmosphere(&self, atmosphere_id: i64, audio_file_id: i64) -> Result<()> {
        AtmosphereOps::remove_sound(&self.conn, atmosphere_id, audio_file_id)
    }

    pub fn update_atmosphere_sound(&self, atmosphere_id: i64, audio_file_id: i64, volume: f32, is_looping: bool, is_muted: bool, min_seconds: i32, max_seconds: i32) -> Result<()> {
        AtmosphereOps::update_sound(&self.conn, atmosphere_id, audio_file_id, volume, is_looping, is_muted, min_seconds, max_seconds)
    }

    pub fn get_atmosphere_with_sounds(&self, atmosphere_id: i64) -> Result<AtmosphereWithSounds> {
        AtmosphereOps::get_with_sounds(&self.conn, atmosphere_id)
    }

    pub fn get_atmosphere_categories(&self) -> Result<Vec<AtmosphereCategory>> {
        AtmosphereOps::get_categories(&self.conn)
    }

    // Virtual Folders methods
    
    pub fn create_virtual_folder(&self, folder: &VirtualFolder) -> Result<i64> {
        VirtualFolderOps::create_virtual_folder(&self.conn, folder)
    }

    pub fn get_virtual_folder_by_id(&self, id: i64) -> Result<VirtualFolder> {
        VirtualFolderOps::get_virtual_folder_by_id(&self.conn, id)
    }

    pub fn update_virtual_folder(&self, folder: &VirtualFolder) -> Result<()> {
        VirtualFolderOps::update_virtual_folder(&self.conn, folder)
    }

    pub fn delete_virtual_folder(&self, id: i64) -> Result<()> {
        VirtualFolderOps::delete_virtual_folder(&self.conn, id)
    }

    pub fn get_folder_children(&self, parent_id: Option<i64>) -> Result<Vec<VirtualFolder>> {
        VirtualFolderOps::get_folder_children(&self.conn, parent_id)
    }

    pub fn get_virtual_folder_tree(&self) -> Result<Vec<VirtualFolderTree>> {
        VirtualFolderOps::get_folder_tree(&self.conn)
    }

    pub fn get_folder_path(&self, folder_id: i64) -> Result<Vec<VirtualFolder>> {
        VirtualFolderOps::get_folder_path(&self.conn, folder_id)
    }

    pub fn move_virtual_folder(&self, folder_id: i64, new_parent_id: Option<i64>) -> Result<()> {
        VirtualFolderOps::move_folder(&self.conn, folder_id, new_parent_id)
    }

    pub fn add_file_to_virtual_folder(&self, folder_id: i64, audio_file_id: i64) -> Result<()> {
        VirtualFolderOps::add_file_to_folder(&self.conn, folder_id, audio_file_id)
    }

    pub fn remove_file_from_virtual_folder(&self, folder_id: i64, audio_file_id: i64) -> Result<()> {
        VirtualFolderOps::remove_file_from_folder(&self.conn, folder_id, audio_file_id)
    }

    pub fn get_virtual_folder_contents(&self, folder_id: i64) -> Result<VirtualFolderWithContents> {
        VirtualFolderOps::get_folder_contents(&self.conn, folder_id)
    }

    pub fn get_file_virtual_folders(&self, audio_file_id: i64) -> Result<Vec<VirtualFolder>> {
        VirtualFolderOps::get_file_folders(&self.conn, audio_file_id)
    }

    pub fn search_virtual_folders(&self, query: &str) -> Result<Vec<VirtualFolder>> {
        VirtualFolderOps::search_folders(&self.conn, query)
    }

    pub fn get_folders_containing_files(&self, file_ids: &[i64]) -> Result<Vec<VirtualFolder>> {
        VirtualFolderOps::get_folders_containing_files(&self.conn, file_ids)
    }

    pub fn create_folder_template(&self, template: &FolderTemplate) -> Result<i64> {
        VirtualFolderOps::create_folder_template(&self.conn, template)
    }

    pub fn get_folder_templates(&self, category: Option<&str>) -> Result<Vec<FolderTemplate>> {
        VirtualFolderOps::get_folder_templates(&self.conn, category)
    }

    // Tag-based suggestion methods
    pub fn suggest_folders_for_file(&self, audio_file_id: i64, limit: Option<usize>) -> Result<Vec<(VirtualFolder, f64)>> {
        VirtualFolderOps::suggest_folders_for_file(&self.conn, audio_file_id, limit)
    }

    pub fn get_auto_organization_suggestions(&self, threshold: f64) -> Result<Vec<crate::models::AutoOrganizationSuggestion>> {
        VirtualFolderOps::get_auto_organization_suggestions(&self.conn, threshold)
    }

    pub fn get_matching_tags(&self, audio_file_id: i64, folder_id: i64) -> Result<Vec<String>> {
        VirtualFolderOps::get_matching_tags(&self.conn, audio_file_id, folder_id)
    }

    pub fn get_unorganized_tagged_files(&self) -> Result<Vec<i64>> {
        VirtualFolderOps::get_unorganized_tagged_files(&self.conn)
    }
}