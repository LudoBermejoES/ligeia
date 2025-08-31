use serde::{Deserialize, Serialize, Deserializer};
use serde_json::Value;

// Custom deserializer to handle both string and array formats for genre/mood fields
fn deserialize_string_or_array<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<Value> = Option::deserialize(deserializer)?;
    match value {
        Some(Value::String(s)) => Ok(Some(s)),
        Some(Value::Array(arr)) => {
            let strings: Vec<String> = arr
                .into_iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            if strings.is_empty() {
                Ok(None)
            } else {
                Ok(Some(strings.join("; ")))
            }
        }
        Some(Value::Null) | None => Ok(None),
        _ => Ok(None), // Handle other types gracefully
    }
}

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

impl Default for AudioFile {
    fn default() -> Self {
        AudioFile {
            id: None,
            file_path: String::new(),
            title: None,
            artist: None,
            album: None,
            album_artist: None,
            genre: None,
            year: None,
            date: None,
            track_number: None,
            total_tracks: None,
            disc_number: None,
            total_discs: None,
            duration: None,
            composer: None,
            conductor: None,
            lyricist: None,
            original_artist: None,
            remixer: None,
            arranger: None,
            engineer: None,
            producer: None,
            dj_mixer: None,
            mixer: None,
            content_group: None,
            subtitle: None,
            initial_key: None,
            bpm: None,
            language: None,
            media_type: None,
            original_filename: None,
            original_lyricist: None,
            original_release_time: None,
            playlist_delay: None,
            recording_time: None,
            release_time: None,
            tagging_time: None,
            encoding_time: None,
            encoding_settings: None,
            encoded_by: None,
            copyright: None,
            file_owner: None,
            internet_radio_station_name: None,
            internet_radio_station_owner: None,
            isrc: None,
            publisher: None,
            mood: None,
            occasion: None,
            tempo: None,
            content_type: None,
            category: None,
        }
    }
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

// Export/Import structures with readable labels
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ExportData {
    pub version: u8,
    pub files: Vec<ExportAudioFile>,
    pub tags: Vec<ExportRpgTag>,
    // Enhanced vocabulary field (optional for backwards compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_vocabulary: Option<serde_json::Value>, // Store as JSON for flexibility
}

impl Default for ExportData {
    fn default() -> Self {
        ExportData {
            version: 1,
            files: Vec::new(),
            tags: Vec::new(),
            tag_vocabulary: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ExportAudioFile {
    pub id: Option<i64>,
    pub file_path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    #[serde(deserialize_with = "deserialize_string_or_array")]
    pub genre: Option<String>,
    pub year: Option<i32>,
    pub duration: Option<f64>,
    pub album_artist: Option<String>,
    pub track_number: Option<u32>,
    pub bpm: Option<u32>,
    pub initial_key: Option<String>,
    #[serde(deserialize_with = "deserialize_string_or_array")]
    pub mood: Option<String>,
    pub language: Option<String>,
    // Enhanced RPG fields (optional for backwards compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpg_occasion: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpg_keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpg_quality: Option<String>,
}

impl Default for ExportAudioFile {
    fn default() -> Self {
        ExportAudioFile {
            id: None,
            file_path: String::new(),
            title: None,
            artist: None,
            album: None,
            genre: None,
            year: None,
            duration: None,
            album_artist: None,
            track_number: None,
            bpm: None,
            initial_key: None,
            mood: None,
            language: None,
            rpg_occasion: None,
            rpg_keywords: None,
            rpg_quality: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportRpgTag {
    pub audio_file_id: i64,
    pub tag_type: String,
    pub tag_value: String,
}

// Atmosphere models
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Atmosphere {
    pub id: Option<i64>,
    pub name: String,
    pub title: String,
    pub description: String,
    pub category: String,              // e.g., "Unreal atmospheres"
    pub subcategory: String,           // e.g., "Horror"
    pub subsubcategory: Option<String>, // e.g., specific theme
    pub keywords: Vec<String>,         // Tags separated by comma
    pub background_image: Option<String>, // Path or URL to background image
    pub author_image: Option<String>,  // Author of the image
    pub is_public: bool,               // Whether atmosphere is public
    pub theme: Option<String>,         // Theme slug for this atmosphere
    pub default_crossfade_ms: i64,     // Preferred crossfade duration
    pub fade_curve: String,            // linear | equal_power | exp
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AtmosphereSoundMapping {
    pub id: Option<i64>,
    pub atmosphere_id: i64,
    pub audio_file_id: i64,
    pub volume: f32,        // Individual volume for this sound in atmosphere
    pub is_looping: bool,   // Whether this sound loops in atmosphere
    pub is_muted: bool,     // Whether this sound is muted in atmosphere
    pub min_seconds: i32,   // Minimum random delay in seconds (0 = disabled)
    pub max_seconds: i32,   // Maximum random delay in seconds (0 = disabled)
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AtmosphereWithSounds {
    pub atmosphere: Atmosphere,
    pub sounds: Vec<AtmosphereSoundMapping>,
    pub audio_files: Vec<AudioFile>, // The actual audio file data
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AtmosphereIntegrity {
    pub atmosphere_id: i64,
    pub missing_ids: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AtmosphereIntegrityBatchEntry {
    pub atmosphere_id: i64,
    pub missing_ids: Vec<i64>,
}

// Categories for dropdowns - these could be loaded from a file or database
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AtmosphereCategory {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AtmosphereSavePayload {
    #[serde(flatten)]
    pub atmosphere: Atmosphere,
    pub sounds: Option<Vec<AtmosphereSoundMapping>>,
}

impl Default for Atmosphere {
    fn default() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Atmosphere {
            id: None,
            name: String::new(),
            title: String::new(),
            description: String::new(),
            category: String::new(),
            subcategory: String::new(),
            subsubcategory: None,
            keywords: Vec::new(),
            background_image: None,
            author_image: None,
            is_public: false,
            theme: Some("default".to_string()),
            default_crossfade_ms: 2500,
            fade_curve: "linear".to_string(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

// Store Tags functionality models
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreTagsResult {
    pub total_files: usize,
    pub updated_files: usize,
    pub skipped_files: usize,
    pub failed_files: usize,
    pub errors: Vec<String>,
    pub duration_seconds: f64,
}

#[derive(Debug, Clone)]
pub struct FileTagComparison {
    pub file_path: String,
    pub needs_update: bool,
    pub missing_tags: Vec<String>,
    pub different_values: Vec<TagDifference>,
}

#[derive(Debug, Clone)]
pub struct TagDifference {
    pub field_name: String,
    pub current_value: String,
    pub new_value: String,
}

// Virtual Folders models
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VirtualFolder {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub parent_folder_id: Option<i64>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub created_by: Option<String>,
    pub folder_order: i32,
    pub is_system_folder: bool,
    pub metadata: Option<String>, // JSON string
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VirtualFolderContent {
    pub id: Option<i64>,
    pub folder_id: i64,
    pub audio_file_id: i64,
    pub added_at: String,
    pub added_by: Option<String>,
    pub file_order: i32,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VirtualFolderTree {
    pub folder: VirtualFolder,
    pub children: Vec<VirtualFolderTree>,
    pub file_count: i64,
    pub total_file_count: i64, // Including subfolders
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VirtualFolderWithContents {
    pub folder: VirtualFolder,
    pub audio_files: Vec<AudioFile>,
    pub subfolders: Vec<VirtualFolderTree>,  // Changed to include file counts
    pub breadcrumb: Vec<VirtualFolder>, // Path from root
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderTemplate {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub template_data: String, // JSON structure
    pub category: String,
    pub is_public: bool,
    pub created_at: String,
    pub created_by: Option<String>,
}

impl Default for VirtualFolder {
    fn default() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        VirtualFolder {
            id: None,
            name: String::new(),
            description: None,
            parent_folder_id: None,
            color: None,
            icon: None,
            created_at: now.clone(),
            updated_at: now,
            created_by: None,
            folder_order: 0,
            is_system_folder: false,
            metadata: None,
        }
    }
}

// Auto-organization suggestion model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutoOrganizationSuggestion {
    pub audio_file_id: i64,
    pub audio_file_title: String,
    pub suggested_folder_id: i64,
    pub suggested_folder_name: String,
    pub confidence_score: f64,
    pub matching_tags: Vec<String>,
}

// Folder suggestion with score
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderSuggestion {
    pub folder: VirtualFolder,
    pub confidence_score: f64,
    pub matching_tags: Vec<String>,
}