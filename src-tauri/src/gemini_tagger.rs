use anyhow::{anyhow, Result};
use dotenv::dotenv;
use gemini_client_api::gemini::{
    ask::Gemini,
    types::sessions::Session,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tokio::sync::Semaphore;
use futures::stream::{self, StreamExt};
use futures::TryStreamExt;
use log::{info, warn, error, debug};
use std::collections::{HashSet, HashMap};
use regex::Regex;
use crate::database::TagMappingCache;
use rusqlite::Connection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFile {
    pub id: i32,
    pub file_path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub mood: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiTagResponse {
    pub file_path: String,
    pub genre: String,
    pub mood: String,
    pub rpg_occasion: Vec<String>,
    pub rpg_keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaggedFile {
    pub id: i32,
    pub file_path: String,
    pub genre: String,
    pub mood: String,
    pub rpg_occasion: Vec<String>,
    pub rpg_keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaggingProgress {
    pub total_files: usize,
    pub processed_files: usize,
    pub failed_files: usize,
    pub current_batch: usize,
    pub total_batches: usize,
    pub status: String,
}

struct ValidationResult {
    is_valid: bool,
    errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TagMapping {
    pub genre_mappings: HashMap<String, String>,
    pub mood_mappings: HashMap<String, String>, 
    pub occasion_mappings: HashMap<String, String>,
    pub keyword_mappings: HashMap<String, String>,
}

pub struct GeminiTagger {
    client: Gemini,
    session: Session,
    batch_size: usize,
    max_parallel: usize,
    tags_vocabulary: String,
    valid_genres: HashSet<String>,
    valid_moods: HashSet<String>,
    valid_occasions: HashSet<String>,
    valid_keywords: HashSet<String>,
}

// Struct to hold cache mappings that can be shared across async boundaries
#[derive(Debug, Clone, Default)]
struct CachedMappings {
    genre_mappings: HashMap<String, String>,
    mood_mappings: HashMap<String, String>,
    occasion_mappings: HashMap<String, String>,
    keyword_mappings: HashMap<String, String>,
}

impl GeminiTagger {
    pub fn new() -> Result<Self> {
        // Load environment variables
        dotenv().ok();
        let api_key = env::var("GEMINI_API_KEY")
            .map_err(|_| anyhow!("GEMINI_API_KEY not found in .env file"))?;
        
        // Initialize Gemini client and session
        let client = Gemini::new(api_key, "gemini-1.5-flash", None);
        let session = Session::new(10); // Keep last 10 messages for context
        
        // No longer using AUTOTAG.md to avoid tag conflicts
        
        // Generate vocabulary from data files
        let tags_vocabulary = Self::generate_tags_vocabulary();
        
        // Parse valid tags from vocabulary data
        let (valid_genres, valid_moods, valid_occasions, valid_keywords) = 
            Self::parse_valid_tags_from_data()?;
        
        Ok(Self {
            client,
            session,
            batch_size: 50,
            max_parallel: 3,
            tags_vocabulary,
            valid_genres,
            valid_moods,
            valid_occasions,
            valid_keywords,
        })
    }
    
    pub fn with_config(batch_size: usize, max_parallel: usize) -> Result<Self> {
        let mut tagger = Self::new()?;
        tagger.batch_size = batch_size;
        tagger.max_parallel = max_parallel;
        Ok(tagger)
    }
    
    pub async fn process_untagged_files_with_cache(&self, files: Vec<AudioFile>, db_conn: &Connection) -> Result<Vec<TaggedFile>> {
        info!("Processing {} untagged files with caching enabled", files.len());
        
        // Load all existing mappings from cache synchronously before starting async processing
        let cached_mappings = self.load_cached_mappings(db_conn)?;
        info!("Loaded {} cached mappings from database", 
              cached_mappings.genre_mappings.len() + cached_mappings.mood_mappings.len() + 
              cached_mappings.occasion_mappings.len() + cached_mappings.keyword_mappings.len());
        
        // Create batches
        let batches: Vec<Vec<AudioFile>> = files
            .chunks(self.batch_size)
            .map(|chunk| chunk.to_vec())
            .collect();
        
        info!("Created {} batches for cached processing", batches.len());
        
        // Process batches with cache - collect new mappings as we go
        let mut all_tagged_files = Vec::new();
        let mut failed_count = 0;
        let mut new_mappings = TagMapping::default();
        
        for (batch_idx, batch) in batches.into_iter().enumerate() {
            match self.process_batch_with_cache_internal(batch, batch_idx, &cached_mappings).await {
                Ok((tagged_files, batch_new_mappings)) => {
                    info!("Batch {} processed successfully with {} files", batch_idx, tagged_files.len());
                    all_tagged_files.extend(tagged_files);
                    
                    // Accumulate new mappings
                    new_mappings.genre_mappings.extend(batch_new_mappings.genre_mappings);
                    new_mappings.mood_mappings.extend(batch_new_mappings.mood_mappings);
                    new_mappings.occasion_mappings.extend(batch_new_mappings.occasion_mappings);
                    new_mappings.keyword_mappings.extend(batch_new_mappings.keyword_mappings);
                }
                Err(e) => {
                    error!("Batch {} failed: {}", batch_idx, e);
                    failed_count += 1;
                }
            }
        }
        
        // Store all new mappings to database synchronously at the end
        if !new_mappings.genre_mappings.is_empty() || !new_mappings.mood_mappings.is_empty() ||
           !new_mappings.occasion_mappings.is_empty() || !new_mappings.keyword_mappings.is_empty() {
            
            if let Err(e) = TagMappingCache::store_mappings(
                db_conn,
                &new_mappings.genre_mappings,
                &new_mappings.mood_mappings,
                &new_mappings.occasion_mappings,
                &new_mappings.keyword_mappings,
            ) {
                warn!("Failed to store new mappings in cache: {}", e);
            } else {
                let stored_count = new_mappings.genre_mappings.len() + 
                                 new_mappings.mood_mappings.len() + 
                                 new_mappings.occasion_mappings.len() + 
                                 new_mappings.keyword_mappings.len();
                info!("Stored {} new mappings in cache", stored_count);
            }
        }
        
        if failed_count > 0 {
            warn!("{} batches failed to process", failed_count);
        }
        
        info!("Successfully tagged {} files with caching", all_tagged_files.len());
        Ok(all_tagged_files)
    }
    
    pub async fn process_untagged_files_with_loaded_cache(&self, files: Vec<AudioFile>, cached_mappings: (HashMap<String, String>, HashMap<String, String>, HashMap<String, String>, HashMap<String, String>)) -> Result<Vec<TaggedFile>> {
        info!("Processing {} untagged files with pre-loaded cache", files.len());
        
        let cached_mappings = CachedMappings {
            genre_mappings: cached_mappings.0,
            mood_mappings: cached_mappings.1,
            occasion_mappings: cached_mappings.2,
            keyword_mappings: cached_mappings.3,
        };
        
        info!("Using {} cached mappings from database", 
              cached_mappings.genre_mappings.len() + cached_mappings.mood_mappings.len() + 
              cached_mappings.occasion_mappings.len() + cached_mappings.keyword_mappings.len());
        
        // Create batches
        let batches: Vec<Vec<AudioFile>> = files
            .chunks(self.batch_size)
            .map(|chunk| chunk.to_vec())
            .collect();
        
        info!("Created {} batches for cached processing", batches.len());
        
        // Process batches with cache - collect new mappings as we go
        let mut all_tagged_files = Vec::new();
        let mut failed_count = 0;
        
        for (batch_idx, batch) in batches.into_iter().enumerate() {
            match self.process_batch_with_cache_internal(batch, batch_idx, &cached_mappings).await {
                Ok((tagged_files, _batch_new_mappings)) => {
                    info!("Batch {} processed successfully with {} files", batch_idx, tagged_files.len());
                    all_tagged_files.extend(tagged_files);
                    // Note: new mappings are not stored back to database in this version
                    // This avoids the Send/Sync issues with async database operations
                }
                Err(e) => {
                    error!("Batch {} failed: {}", batch_idx, e);
                    failed_count += 1;
                }
            }
        }
        
        if failed_count > 0 {
            warn!("{} batches failed to process", failed_count);
        }
        
        info!("Successfully tagged {} files with cached processing", all_tagged_files.len());
        Ok(all_tagged_files)
    }
    
    pub async fn process_untagged_files_with_loaded_cache_and_return_mappings(&self, files: Vec<AudioFile>, cached_mappings: (HashMap<String, String>, HashMap<String, String>, HashMap<String, String>, HashMap<String, String>)) -> Result<(Vec<TaggedFile>, TagMapping)> {
        info!("Processing {} untagged files with pre-loaded cache and returning mappings", files.len());
        
        let cached_mappings = CachedMappings {
            genre_mappings: cached_mappings.0,
            mood_mappings: cached_mappings.1,
            occasion_mappings: cached_mappings.2,
            keyword_mappings: cached_mappings.3,
        };
        
        info!("Using {} cached mappings from database", 
              cached_mappings.genre_mappings.len() + cached_mappings.mood_mappings.len() + 
              cached_mappings.occasion_mappings.len() + cached_mappings.keyword_mappings.len());
        
        // Create batches
        let batches: Vec<Vec<AudioFile>> = files
            .chunks(self.batch_size)
            .map(|chunk| chunk.to_vec())
            .collect();
        
        info!("Created {} batches for cached processing", batches.len());
        
        // Process batches with cache - collect new mappings as we go
        let mut all_tagged_files = Vec::new();
        let mut failed_count = 0;
        let mut all_new_mappings = TagMapping::default();
        
        for (batch_idx, batch) in batches.into_iter().enumerate() {
            match self.process_batch_with_cache_internal(batch, batch_idx, &cached_mappings).await {
                Ok((tagged_files, batch_new_mappings)) => {
                    info!("Batch {} processed successfully with {} files", batch_idx, tagged_files.len());
                    all_tagged_files.extend(tagged_files);
                    
                    // Accumulate new mappings from this batch
                    all_new_mappings.genre_mappings.extend(batch_new_mappings.genre_mappings);
                    all_new_mappings.mood_mappings.extend(batch_new_mappings.mood_mappings);
                    all_new_mappings.occasion_mappings.extend(batch_new_mappings.occasion_mappings);
                    all_new_mappings.keyword_mappings.extend(batch_new_mappings.keyword_mappings);
                }
                Err(e) => {
                    error!("Batch {} failed: {}", batch_idx, e);
                    failed_count += 1;
                }
            }
        }
        
        if failed_count > 0 {
            warn!("{} batches failed to process", failed_count);
        }
        
        let total_new_mappings = all_new_mappings.genre_mappings.len() + 
                               all_new_mappings.mood_mappings.len() + 
                               all_new_mappings.occasion_mappings.len() + 
                               all_new_mappings.keyword_mappings.len();
        
        info!("Successfully tagged {} files with cached processing, generated {} new mappings for storage", 
              all_tagged_files.len(), total_new_mappings);
        
        Ok((all_tagged_files, all_new_mappings))
    }
    
    fn load_cached_mappings(&self, db_conn: &Connection) -> Result<CachedMappings> {
        let (genre_mappings, mood_mappings, occasion_mappings, keyword_mappings) = 
            TagMappingCache::get_all_cached_mappings(db_conn)
            .map_err(|e| anyhow!("Failed to load cached mappings: {}", e))?;
        
        Ok(CachedMappings {
            genre_mappings,
            mood_mappings,
            occasion_mappings,
            keyword_mappings,
        })
    }
    
    pub async fn process_single_batch_with_cache(&self, batch: Vec<AudioFile>, batch_idx: usize, cached_mappings: &(HashMap<String, String>, HashMap<String, String>, HashMap<String, String>, HashMap<String, String>)) -> Result<(Vec<TaggedFile>, TagMapping)> {
        let cached_mappings = CachedMappings {
            genre_mappings: cached_mappings.0.clone(),
            mood_mappings: cached_mappings.1.clone(),
            occasion_mappings: cached_mappings.2.clone(),
            keyword_mappings: cached_mappings.3.clone(),
        };
        
        self.process_batch_with_cache_internal(batch, batch_idx, &cached_mappings).await
    }
    
    async fn process_batch_with_cache_internal(&self, batch: Vec<AudioFile>, batch_idx: usize, cached_mappings: &CachedMappings) -> Result<(Vec<TaggedFile>, TagMapping)> {
        info!("=== PROCESSING BATCH {} WITH CACHE ===", batch_idx);
        info!("Batch contains {} files", batch.len());
        
        // Call Gemini API for initial tagging
        let file_paths: Vec<String> = batch.iter().map(|f| f.file_path.clone()).collect();
        info!("Calling Gemini API for batch {} files", file_paths.len());
        let prompt = self.create_prompt(file_paths);
        let response = self.call_gemini(prompt).await?;
        
        // Parse response
        info!("Parsing Gemini response for batch {}", batch_idx);
        let gemini_responses = self.parse_response(response)?;
        info!("Parsed {} responses from Gemini", gemini_responses.len());
        
        // Validate tags and collect files with invalid tags
        info!("Validating tags for batch {}", batch_idx);
        let mut valid_files = Vec::new();
        let mut invalid_responses = Vec::new();
        let mut invalid_files = Vec::new();
        
        for gemini_response in gemini_responses {
            // First try strict validation
            let validation_result = self.validate_tags_detailed(&gemini_response);
            if validation_result.is_valid {
                // All tags are valid, proceed normally
                info!("VALID: {} -> Genre: '{}', Mood: '{}', Occasions: {:?}, Keywords: {:?}",
                      gemini_response.file_path, gemini_response.genre, gemini_response.mood,
                      gemini_response.rpg_occasion, gemini_response.rpg_keywords);
                if let Some(audio_file) = batch.iter().find(|f| f.file_path == gemini_response.file_path) {
                    valid_files.push(TaggedFile {
                        id: audio_file.id,
                        file_path: audio_file.file_path.clone(),
                        genre: gemini_response.genre,
                        mood: gemini_response.mood,
                        rpg_occasion: gemini_response.rpg_occasion,
                        rpg_keywords: gemini_response.rpg_keywords,
                    });
                }
            } else {
                // Some tags are invalid, try partial validation
                if let Some(cleaned_response) = self.validate_and_clean_tags(&gemini_response) {
                    // We have enough valid tags, use the cleaned version
                    info!("PARTIALLY VALID: {} -> Genre: '{}', Mood: '{}', Occasions: {:?}, Keywords: {:?}",
                          gemini_response.file_path, cleaned_response.genre, cleaned_response.mood,
                          cleaned_response.rpg_occasion, cleaned_response.rpg_keywords);
                    if let Some(audio_file) = batch.iter().find(|f| f.file_path == gemini_response.file_path) {
                        valid_files.push(TaggedFile {
                            id: audio_file.id,
                            file_path: audio_file.file_path.clone(),
                            genre: cleaned_response.genre,
                            mood: cleaned_response.mood,
                            rpg_occasion: cleaned_response.rpg_occasion,
                            rpg_keywords: cleaned_response.rpg_keywords,
                        });
                    }
                } else {
                    // Not enough valid tags, collect for tag mapping WITH CACHE
                    warn!("INVALID: {} -> Genre: '{}', Mood: '{}', Occasions: {:?}, Keywords: {:?} - needs mapping",
                          gemini_response.file_path, gemini_response.genre, gemini_response.mood,
                          gemini_response.rpg_occasion, gemini_response.rpg_keywords);
                    invalid_responses.push(gemini_response);
                    if let Some(audio_file) = batch.iter().find(|f| f.file_path == invalid_responses.last().unwrap().file_path) {
                        invalid_files.push(audio_file.clone());
                    }
                }
            }
        }
        
        info!("BATCH {} VALIDATION COMPLETE: {} valid files, {} invalid files need mapping",
              batch_idx, valid_files.len(), invalid_responses.len());
        
        // If there are invalid responses, try to map their tags to valid ones WITH CACHE
        let mut new_mappings = TagMapping::default();
        if !invalid_responses.is_empty() {
            info!("Attempting to map invalid tags for {} files using cache", invalid_responses.len());
            let (mapping_results, batch_mappings) = self.map_invalid_tags_with_cache(invalid_responses, invalid_files, cached_mappings).await?;
            info!("Cache mapping returned {} fixed files and {} new mappings", 
                  mapping_results.len(), 
                  batch_mappings.genre_mappings.len() + batch_mappings.mood_mappings.len() + 
                  batch_mappings.occasion_mappings.len() + batch_mappings.keyword_mappings.len());
            valid_files.extend(mapping_results);
            new_mappings = batch_mappings;
        } else {
            info!("All files in batch {} had valid tags - no cache mapping needed", batch_idx);
        }
        
        info!("=== BATCH {} COMPLETE ===: {} total files processed, {} new mappings generated", 
              batch_idx, valid_files.len(), 
              new_mappings.genre_mappings.len() + new_mappings.mood_mappings.len() + 
              new_mappings.occasion_mappings.len() + new_mappings.keyword_mappings.len());
        
        Ok((valid_files, new_mappings))
    }
    
    pub async fn process_untagged_files(&self, files: Vec<AudioFile>) -> Result<Vec<TaggedFile>> {
        info!("Processing {} untagged files", files.len());
        
        // Create batches
        let batches: Vec<Vec<AudioFile>> = files
            .chunks(self.batch_size)
            .map(|chunk| chunk.to_vec())
            .collect();
        
        info!("Created {} batches of size {}", batches.len(), self.batch_size);
        
        // Process batches in parallel
        let results = self.process_in_parallel(batches).await;
        
        // Collect successful results
        let mut all_tagged_files = Vec::new();
        let mut failed_count = 0;
        
        for (batch_idx, result) in results.into_iter().enumerate() {
            match result {
                Ok(tagged_files) => {
                    info!("Batch {} processed successfully with {} files", batch_idx, tagged_files.len());
                    all_tagged_files.extend(tagged_files);
                }
                Err(e) => {
                    error!("Batch {} failed: {}", batch_idx, e);
                    failed_count += 1;
                }
            }
        }
        
        if failed_count > 0 {
            warn!("{} batches failed to process", failed_count);
        }
        
        info!("Successfully tagged {} files", all_tagged_files.len());
        Ok(all_tagged_files)
    }
    
    async fn process_in_parallel(&self, batches: Vec<Vec<AudioFile>>) -> Vec<Result<Vec<TaggedFile>>> {
        let semaphore = Arc::new(Semaphore::new(self.max_parallel));
        
        stream::iter(batches.into_iter().enumerate())
            .map(|(idx, batch)| {
                let sem = semaphore.clone();
                let tagger = self.clone_for_async();
                async move {
                    let _permit = sem.acquire().await.unwrap();
                    tagger.process_batch_with_retry(batch, idx).await
                }
            })
            .buffer_unordered(self.max_parallel)
            .collect()
            .await
    }
    
    fn clone_for_async(&self) -> Self {
        Self {
            client: self.client.clone(),
            session: Session::new(10),
            batch_size: self.batch_size,
            max_parallel: self.max_parallel,
            tags_vocabulary: self.tags_vocabulary.clone(),
            valid_genres: self.valid_genres.clone(),
            valid_moods: self.valid_moods.clone(),
            valid_occasions: self.valid_occasions.clone(),
            valid_keywords: self.valid_keywords.clone(),
        }
    }
    
    async fn process_batch_with_retry(&self, batch: Vec<AudioFile>, batch_idx: usize) -> Result<Vec<TaggedFile>> {
        const MAX_ATTEMPTS: u32 = 3;
        let mut attempts = 0;
        
        loop {
            attempts += 1;
            info!("Processing batch {} (attempt {}/{})", batch_idx, attempts, MAX_ATTEMPTS);
            
            match self.process_batch(batch.clone(), batch_idx).await {
                Ok(response) => return Ok(response),
                Err(e) if attempts < MAX_ATTEMPTS => {
                    warn!("Batch {} processing failed (attempt {}): {}", batch_idx, attempts, e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(2_u64.pow(attempts))).await;
                }
                Err(e) => {
                    error!("Batch {} processing failed after {} attempts: {}", batch_idx, MAX_ATTEMPTS, e);
                    return Err(e);
                }
            }
        }
    }
    
    async fn process_batch(&self, batch: Vec<AudioFile>, batch_idx: usize) -> Result<Vec<TaggedFile>> {
        // Extract file paths for the prompt
        let file_paths: Vec<String> = batch.iter().map(|f| f.file_path.clone()).collect();
        
        // Create the prompt
        let prompt = self.create_prompt(file_paths);
        
        // Call Gemini API
        let response = self.call_gemini(prompt).await?;
        
        // Save debug response if needed
        #[cfg(debug_assertions)]
        self.save_debug_response(batch_idx, &response)?;
        
        // Parse response
        let gemini_responses = self.parse_response(response)?;
        
        // Validate tags and collect files with invalid tags
        let mut valid_files = Vec::new();
        let mut invalid_responses = Vec::new();
        let mut invalid_files = Vec::new();
        
        for gemini_response in gemini_responses {
            // First try strict validation
            let validation_result = self.validate_tags_detailed(&gemini_response);
            if validation_result.is_valid {
                // All tags are valid, proceed normally
                if let Some(audio_file) = batch.iter()
                    .find(|f| f.file_path == gemini_response.file_path) {
                    valid_files.push(TaggedFile {
                        id: audio_file.id,
                        file_path: audio_file.file_path.clone(),
                        genre: gemini_response.genre,
                        mood: gemini_response.mood,
                        rpg_occasion: gemini_response.rpg_occasion,
                        rpg_keywords: gemini_response.rpg_keywords,
                    });
                }
            } else {
                // Some tags are invalid, try partial validation
                if let Some(cleaned_response) = self.validate_and_clean_tags(&gemini_response) {
                    // We have enough valid tags, use the cleaned version
                    info!("Partial validation success for {}: keeping {} occasions, {} keywords", 
                        gemini_response.file_path, 
                        cleaned_response.rpg_occasion.len(), 
                        cleaned_response.rpg_keywords.len());
                    
                    if let Some(audio_file) = batch.iter()
                        .find(|f| f.file_path == gemini_response.file_path) {
                        valid_files.push(TaggedFile {
                            id: audio_file.id,
                            file_path: audio_file.file_path.clone(),
                            genre: cleaned_response.genre,
                            mood: cleaned_response.mood,
                            rpg_occasion: cleaned_response.rpg_occasion,
                            rpg_keywords: cleaned_response.rpg_keywords,
                        });
                    }
                } else {
                    // Not enough valid tags, collect for tag mapping
                    warn!("Insufficient valid tags for {}: {}", 
                        gemini_response.file_path, 
                        validation_result.errors.join(", "));
                    
                    invalid_responses.push(gemini_response);
                    if let Some(audio_file) = batch.iter()
                        .find(|f| f.file_path == invalid_responses.last().unwrap().file_path) {
                        invalid_files.push(audio_file.clone());
                    }
                }
            }
        }
        
        // If there are invalid responses, try to map their tags to valid ones
        if !invalid_responses.is_empty() {
            info!("Attempting to map invalid tags for {} files", invalid_responses.len());
            let mapping_results = self.map_invalid_tags_and_fix(invalid_responses, invalid_files, None).await?;
            valid_files.extend(mapping_results);
        }
        
        Ok(valid_files)
    }
    
    fn create_prompt(&self, file_paths: Vec<String>) -> String {
        let files_json = serde_json::to_string_pretty(&file_paths)
            .unwrap_or_else(|_| "[]".to_string());
        
        format!(r#"You are a strict audio file tagger that MUST ONLY use predefined tags from the provided vocabulary. 

=== STRICT TAG VOCABULARY (DO NOT DEVIATE FROM THESE) ===
{}

=== CRITICAL GENRE RULES ===
1. GENRE: Choose EXACTLY ONE from the GENRES section above. Use the exact spelling and casing shown.
2. FOR HIERARCHICAL GENRES: You MUST use the full hierarchical format (parent:child)
   - NEVER use just "sound-design" - use specific tags like "sound-design:impacts", "sound-design:vehicles", etc.
   - NEVER use just "orchestral" - use specific tags like "orchestral:cinematic", "orchestral:fantasy", etc.
   - NEVER use just "electronic" - use specific tags like "electronic:ambient", "electronic:techno", etc.
3. LOOK FOR THE MOST SPECIFIC genre tag that matches the audio content

=== OTHER TAG RULES ===
4. MOOD: Choose 2-3 from the MOODS section above. Separate multiple moods with semicolons. Use exact spelling.
5. RPG_OCCASION: Choose 2-5 from the OCCASIONS section above. Must be exact matches from the list.
6. RPG_KEYWORDS: Choose 4-8 from the KEYWORDS section above. Must be exact matches from the list.

=== FILE PATHS TO PROCESS ===
{}

=== CRITICAL CONSTRAINTS ===
- FORBIDDEN: Using parent categories like "sound-design", "orchestral", "electronic" without the specific child tag
- FORBIDDEN: Creating, modifying, or inventing any tags not in the vocabulary above
- FORBIDDEN: Using variations, synonyms, or similar words
- FORBIDDEN: Adding prefixes, suffixes, or modifications to existing tags  
- FORBIDDEN: Using "combat-encounter", "gunfight", "injury" - these are NOT in the vocabulary
- REQUIRED: Every tag MUST be copy-pasted exactly from the vocabulary sections above
- REQUIRED: For hierarchical genres, always use the full parent:child format
- REQUIRED: Cross-reference each tag against the vocabulary before including it
- REQUIRED: If unsure about a tag, DO NOT USE IT - choose a different one that definitely exists

=== EXAMPLES OF CORRECT GENRE USAGE ===
✓ CORRECT: "sound-design:impacts" (for impact/hit sounds)
✓ CORRECT: "sound-design:objects" (for object/vehicle sounds)  
✓ CORRECT: "sound-design:weapons" (for weapon sounds)
✓ CORRECT: "sound-design:movement" (for movement/motion sounds)
✓ CORRECT: "orchestral:cinematic" (for film score style)
✗ WRONG: "sound-design" (too broad, not specific enough)
✗ WRONG: "orchestral" (too broad, not specific enough)

=== OUTPUT REQUIREMENTS ===
Return ONLY a JSON array. No explanations, no markdown blocks, no additional text.
Each object must have exactly these fields: file_path, genre, mood, rpg_occasion, rpg_keywords

=== VALIDATION CHECKLIST ===
Before finalizing your response:
1. ✓ Is the genre tag spelled EXACTLY as shown in GENRES section with full hierarchy?
2. ✓ Did I avoid using broad parent categories like "sound-design" alone?
3. ✓ Are ALL mood tags spelled EXACTLY as shown in MOODS section?
4. ✓ Are ALL occasion tags spelled EXACTLY as shown in OCCASIONS section? 
5. ✓ Are ALL keyword tags spelled EXACTLY as shown in KEYWORDS section?
6. ✓ Did I avoid inventing any new tags?

Example output format (but verify all tags exist in vocabulary above):
[
  {{
    "file_path": "example.wav",
    "genre": "sound-design:impacts",
    "mood": "mysterious",
    "rpg_occasion": ["dungeon-crawl", "cave-exploration"],
    "rpg_keywords": ["biome:cave", "timbre:analog-synth", "util:loopable"]
  }}
]"#,
            self.tags_vocabulary,
            files_json
        )
    }
    
    async fn call_gemini(&self, prompt: String) -> Result<String> {
        // Create a mutable session clone for this request
        let mut session = Session::new(1); // Single-use session for this request
        
        let response = self.client
            .ask(session.ask_string(prompt))
            .await
            .map_err(|e| anyhow!("Gemini API error: {}", e))?;
        
        let response_text = response.get_text("");
        
        info!("=== GEMINI RESPONSE ===");
        info!("Response length: {} characters", response_text.len());
        info!("Full response:\n{}", response_text);
        info!("=== END RESPONSE ===");
        
        Ok(response_text)
    }
    
    fn parse_response(&self, response: String) -> Result<Vec<GeminiTagResponse>> {
        // Remove markdown code blocks if present
        let cleaned = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();
        
        // Try parsing as JSON array first
        let tagged_files: Vec<GeminiTagResponse> = match serde_json::from_str::<Vec<GeminiTagResponse>>(cleaned) {
            Ok(array) => array,
            Err(_) => {
                // If array parsing fails, try parsing as a single object
                match serde_json::from_str::<GeminiTagResponse>(cleaned) {
                    Ok(single_object) => vec![single_object],
                    Err(e) => {
                        // If both fail, try parsing as generic JSON value to provide better error info
                        match serde_json::from_str::<serde_json::Value>(cleaned) {
                            Ok(value) => {
                                error!("Unexpected JSON format received from Gemini: {}", serde_json::to_string_pretty(&value).unwrap_or_default());
                                let value_type = match value {
                                    serde_json::Value::Null => "null",
                                    serde_json::Value::Bool(_) => "boolean",
                                    serde_json::Value::Number(_) => "number",
                                    serde_json::Value::String(_) => "string",
                                    serde_json::Value::Array(_) => "array",
                                    serde_json::Value::Object(_) => "object",
                                };
                                return Err(anyhow!("Gemini returned unexpected JSON format. Expected array of objects or single object, got: {}", value_type));
                            }
                            Err(_) => {
                                error!("Invalid JSON received from Gemini: {}", cleaned);
                                return Err(anyhow!("Failed to parse Gemini response as JSON: {}", e));
                            }
                        }
                    }
                }
            }
        };
        
        // Basic validation (could be expanded)
        for file in &tagged_files {
            self.validate_tags(file)?;
        }
        
        Ok(tagged_files)
    }
    
    fn validate_tags(&self, response: &GeminiTagResponse) -> Result<()> {
        // Basic validation - could be expanded to check against vocabulary
        if response.genre.is_empty() {
            return Err(anyhow!("Genre is empty for file: {}", response.file_path));
        }
        if response.mood.is_empty() {
            return Err(anyhow!("Mood is empty for file: {}", response.file_path));
        }
        Ok(())
    }
    
    fn generate_tags_vocabulary() -> String {
        // Include vocabulary data from data files
        let genre_vocab = include!("data/genre_vocabulary.rs");
        let mood_vocab = include!("data/mood_vocabulary.rs");
        let occasion_vocab = include!("data/occasion_vocabulary.rs");
        let keyword_vocab = include!("data/keyword_vocabulary.rs");

        let mut vocabulary = String::new();
        
        // Generate genre section
        vocabulary.push_str("## GENRES\n\n");
        vocabulary.push_str("Use hierarchical genre tags with colon notation (parent:child).\n");
        vocabulary.push_str("Available genres:\n\n");
        
        let mut genre_map: std::collections::HashMap<Option<&str>, Vec<&str>> = std::collections::HashMap::new();
        for (_, tag, _label, parent) in genre_vocab.iter() {
            genre_map.entry(*parent).or_insert_with(Vec::new).push(tag);
        }
        
        // First add top-level genres (no parent)
        if let Some(root_genres) = genre_map.get(&None) {
            for genre in root_genres {
                vocabulary.push_str(&format!("- `{}`\n", genre));
            }
        }
        
        // Then add hierarchical genres grouped by parent
        for (parent, children) in &genre_map {
            if let Some(parent_name) = parent {
                vocabulary.push_str(&format!("\n### {}\n", parent_name));
                for child in children {
                    vocabulary.push_str(&format!("- `{}`\n", child));
                }
            }
        }
        
        // Generate mood section
        vocabulary.push_str("\n## MOODS\n\n");
        vocabulary.push_str("Emotional and psychological descriptors. Can be used individually or combined with semicolons.\n");
        vocabulary.push_str("Available moods:\n\n");
        
        for (_, tag, _label, _parent) in mood_vocab.iter() {
            vocabulary.push_str(&format!("- `{}`\n", tag));
        }
        
        // Generate occasion section
        vocabulary.push_str("\n## OCCASIONS\n\n");
        vocabulary.push_str("Scene and use case descriptors for RPG scenarios.\n");
        vocabulary.push_str("Available occasions:\n\n");
        
        for (_, tag, _label, _parent) in occasion_vocab.iter() {
            vocabulary.push_str(&format!("- `{}`\n", tag));
        }
        
        // Generate keywords section
        vocabulary.push_str("\n## KEYWORDS\n\n");
        vocabulary.push_str("Faceted keywords using prefix:value notation for detailed categorization.\n");
        vocabulary.push_str("Available keywords:\n\n");
        
        // Group keywords by facet (prefix) - filter by "keywords" tag type
        let mut keyword_map: std::collections::HashMap<String, Vec<&str>> = std::collections::HashMap::new();
        for (tag_type, tag, _label, _parent) in keyword_vocab.iter() {
            if tag_type == &"keywords" {
                if let Some(colon_pos) = tag.find(':') {
                    let prefix = &tag[..colon_pos];
                    keyword_map.entry(prefix.to_string()).or_insert_with(Vec::new).push(tag);
                }
            }
        }
        
        for (facet, keywords) in keyword_map.iter() {
            vocabulary.push_str(&format!("\n### {} Keywords\n", facet));
            for keyword in keywords {
                vocabulary.push_str(&format!("- `{}`\n", keyword));
            }
        }
        
        vocabulary
    }
    
    fn parse_valid_tags_from_data() -> Result<(HashSet<String>, HashSet<String>, HashSet<String>, HashSet<String>)> {
        // Include vocabulary data from data files
        let genre_vocab = include!("data/genre_vocabulary.rs");
        let mood_vocab = include!("data/mood_vocabulary.rs");
        let occasion_vocab = include!("data/occasion_vocabulary.rs");
        let keyword_vocab = include!("data/keyword_vocabulary.rs");
        
        let mut valid_genres = HashSet::new();
        let mut valid_moods = HashSet::new();
        let mut valid_occasions = HashSet::new();
        let mut valid_keywords = HashSet::new();
        
        // Extract genres
        for (_, tag, _label, _parent) in genre_vocab.iter() {
            valid_genres.insert(tag.to_string());
        }
        
        // Extract moods
        for (_, tag, _label, _parent) in mood_vocab.iter() {
            valid_moods.insert(tag.to_string());
        }
        
        // Extract occasions
        for (_, tag, _label, _parent) in occasion_vocab.iter() {
            valid_occasions.insert(tag.to_string());
        }
        
        // Extract keywords (filter by "keywords" tag type)
        for (tag_type, tag, _label, _parent) in keyword_vocab.iter() {
            if tag_type == &"keywords" {
                valid_keywords.insert(tag.to_string());
            }
        }
        
        info!("Loaded {} genres, {} moods, {} occasions, {} keywords from vocabulary data", 
            valid_genres.len(), valid_moods.len(), valid_occasions.len(), valid_keywords.len());
        
        Ok((valid_genres, valid_moods, valid_occasions, valid_keywords))
    }
    
    fn parse_valid_tags(tags_content: &str) -> Result<(HashSet<String>, HashSet<String>, HashSet<String>, HashSet<String>)> {
        let mut valid_genres = HashSet::new();
        let mut valid_moods = HashSet::new();
        let mut valid_occasions = HashSet::new();
        let mut valid_keywords = HashSet::new();
        
        // Parse genres (hierarchical tags like `orchestral:cinematic`)
        let genre_regex = Regex::new(r"`([^`]+)`").unwrap();
        let mut in_genre_section = false;
        let mut in_mood_section = false;
        let mut in_occasion_section = false;
        let mut in_keyword_section = false;
        
        for line in tags_content.lines() {
            // Section detection
            if line.starts_with("## 2) GENRE") || line.starts_with("### Orchestral") {
                in_genre_section = true;
                in_mood_section = false;
                in_occasion_section = false;
                in_keyword_section = false;
            } else if line.starts_with("## 3) MOOD") {
                in_genre_section = false;
                in_mood_section = true;
                in_occasion_section = false;
                in_keyword_section = false;
            } else if line.starts_with("## 4) OCCASION") {
                in_genre_section = false;
                in_mood_section = false;
                in_occasion_section = true;
                in_keyword_section = false;
            } else if line.starts_with("## 5) KEYWORDS") {
                in_genre_section = false;
                in_mood_section = false;
                in_occasion_section = false;
                in_keyword_section = true;
            } else if line.starts_with("## 6)") {
                // End of tags sections
                break;
            }
            
            // Extract tags from backticks
            for cap in genre_regex.captures_iter(line) {
                let tag = cap[1].to_string();
                
                if in_genre_section {
                    valid_genres.insert(tag);
                } else if in_mood_section {
                    // For mood section, tags are in a different format (comma-separated)
                    if line.starts_with("`") {
                        for mood_tag in line.split(", ") {
                            let cleaned = mood_tag.trim_matches('`').trim();
                            if !cleaned.is_empty() {
                                valid_moods.insert(cleaned.to_string());
                            }
                        }
                    }
                } else if in_occasion_section {
                    valid_occasions.insert(tag);
                } else if in_keyword_section {
                    valid_keywords.insert(tag);
                }
            }
        }
        
        info!("Parsed {} genres, {} moods, {} occasions, {} keywords", 
            valid_genres.len(), valid_moods.len(), valid_occasions.len(), valid_keywords.len());
        
        Ok((valid_genres, valid_moods, valid_occasions, valid_keywords))
    }
    
    fn validate_tags_detailed(&self, response: &GeminiTagResponse) -> ValidationResult {
        let mut errors = Vec::new();
        
        // Validate genre
        if !response.genre.is_empty() && !self.valid_genres.contains(&response.genre) {
            errors.push(format!("Invalid genre: {}", response.genre));
        }
        
        // Validate moods (semicolon-separated)
        for mood in response.mood.split(';') {
            let mood = mood.trim();
            if !mood.is_empty() && !self.valid_moods.contains(mood) {
                errors.push(format!("Invalid mood: {}", mood));
            }
        }
        
        // Validate occasions
        for occasion in &response.rpg_occasion {
            if !self.valid_occasions.contains(occasion) {
                errors.push(format!("Invalid occasion: {}", occasion));
            }
        }
        
        // Validate keywords
        for keyword in &response.rpg_keywords {
            if !self.valid_keywords.contains(keyword) {
                errors.push(format!("Invalid keyword: {}", keyword));
            }
        }
        
        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
        }
    }

    fn validate_and_clean_tags(&self, response: &GeminiTagResponse) -> Option<GeminiTagResponse> {
        debug!("=== VALIDATION START ===");
        debug!("Validating tags for file: {}", response.file_path);
        debug!("Input genre: '{}'", response.genre);
        debug!("Input mood: '{}'", response.mood);
        debug!("Input occasions: {:?}", response.rpg_occasion);
        debug!("Input keywords: {:?}", response.rpg_keywords);
        
        // Validate and keep valid genre (must be valid to proceed)
        let valid_genre = if !response.genre.is_empty() && self.valid_genres.contains(&response.genre) {
            debug!("✓ Genre '{}' is VALID", response.genre);
            response.genre.clone()
        } else {
            debug!("✗ Genre '{}' is INVALID - not found in vocabulary", response.genre);
            return None; // Genre is required and must be valid
        };

        // Validate and keep valid moods
        let input_moods: Vec<&str> = response.mood.split(';')
            .map(|mood| mood.trim())
            .collect();
        
        let valid_moods: Vec<&str> = input_moods.iter()
            .filter(|mood| !mood.is_empty() && self.valid_moods.contains(&mood.to_string()))
            .cloned()
            .collect();
        
        let invalid_moods: Vec<&str> = input_moods.iter()
            .filter(|mood| !mood.is_empty() && !self.valid_moods.contains(&mood.to_string()))
            .cloned()
            .collect();
        
        debug!("Valid moods: {:?}", valid_moods);
        debug!("Invalid moods: {:?}", invalid_moods);
        
        let cleaned_mood = if valid_moods.is_empty() {
            debug!("✗ No valid moods found");
            return None; // At least one mood is required
        } else {
            valid_moods.join("; ")
        };

        // Validate and keep valid occasions
        let valid_occasions: Vec<String> = response.rpg_occasion.iter()
            .filter(|occasion| self.valid_occasions.contains(*occasion))
            .cloned()
            .collect();
        
        let invalid_occasions: Vec<String> = response.rpg_occasion.iter()
            .filter(|occasion| !self.valid_occasions.contains(*occasion))
            .cloned()
            .collect();
        
        debug!("Valid occasions ({} found): {:?}", valid_occasions.len(), valid_occasions);
        debug!("Invalid occasions: {:?}", invalid_occasions);

        // Validate and keep valid keywords
        let valid_keywords: Vec<String> = response.rpg_keywords.iter()
            .filter(|keyword| self.valid_keywords.contains(*keyword))
            .cloned()
            .collect();
        
        let invalid_keywords: Vec<String> = response.rpg_keywords.iter()
            .filter(|keyword| !self.valid_keywords.contains(*keyword))
            .cloned()
            .collect();
        
        debug!("Valid keywords ({} found): {:?}", valid_keywords.len(), valid_keywords);
        debug!("Invalid keywords: {:?}", invalid_keywords);

        // Check if we have enough valid tags to proceed
        let has_enough = valid_occasions.len() >= 3 && valid_keywords.len() >= 4;
        debug!("Partial validation check: {} occasions >= 3? {}, {} keywords >= 4? {} => {}",
               valid_occasions.len(), valid_occasions.len() >= 3,
               valid_keywords.len(), valid_keywords.len() >= 4,
               if has_enough { "ACCEPT" } else { "REJECT" });
        
        if has_enough {
            let result = GeminiTagResponse {
                file_path: response.file_path.clone(),
                genre: valid_genre,
                mood: cleaned_mood,
                rpg_occasion: valid_occasions,
                rpg_keywords: valid_keywords,
            };
            debug!("✓ Partial validation PASSED - returning cleaned tags");
            debug!("=== VALIDATION END ===");
            Some(result)
        } else {
            debug!("✗ Partial validation FAILED - not enough valid tags for partial acceptance");
            debug!("=== VALIDATION END ===");
            None // Not enough valid tags, needs retry
        }
    }
    
    async fn map_invalid_tags_with_cache(&self, invalid_responses: Vec<GeminiTagResponse>, invalid_files: Vec<AudioFile>, cached_mappings: &CachedMappings) -> Result<(Vec<TaggedFile>, TagMapping)> {
        info!("=== MAPPING INVALID TAGS WITH CACHE ===");
        info!("Processing {} responses with invalid tags from {} files", invalid_responses.len(), invalid_files.len());
        info!("Available cache: {} genres, {} moods, {} occasions, {} keywords", 
              cached_mappings.genre_mappings.len(), cached_mappings.mood_mappings.len(),
              cached_mappings.occasion_mappings.len(), cached_mappings.keyword_mappings.len());
        
        // Collect all unique invalid tags
        let mut invalid_genres = HashSet::new();
        let mut invalid_moods = HashSet::new();
        let mut invalid_occasions = HashSet::new();
        let mut invalid_keywords = HashSet::new();
        
        for response in &invalid_responses {
            // Check genre
            if !response.genre.is_empty() && !self.valid_genres.contains(&response.genre) {
                invalid_genres.insert(response.genre.clone());
            }
            
            // Check moods
            for mood in response.mood.split(';') {
                let mood = mood.trim();
                if !mood.is_empty() && !self.valid_moods.contains(&mood.to_string()) {
                    invalid_moods.insert(mood.to_string());
                }
            }
            
            // Check occasions
            for occasion in &response.rpg_occasion {
                if !self.valid_occasions.contains(occasion) {
                    invalid_occasions.insert(occasion.clone());
                }
            }
            
            // Check keywords
            for keyword in &response.rpg_keywords {
                if !self.valid_keywords.contains(keyword) {
                    invalid_keywords.insert(keyword.clone());
                }
            }
        }
        
        info!("Found invalid tags - Genres: {} {:?}, Moods: {} {:?}, Occasions: {} {:?}, Keywords: {} {:?}",
              invalid_genres.len(), invalid_genres, invalid_moods.len(), invalid_moods, 
              invalid_occasions.len(), invalid_occasions, invalid_keywords.len(), invalid_keywords);
        
        // Apply cached mappings and collect uncached tags
        let mut final_mapping = TagMapping::default();
        let mut uncached_genres = HashSet::new();
        let mut uncached_moods = HashSet::new();
        let mut uncached_occasions = HashSet::new();
        let mut uncached_keywords = HashSet::new();
        
        // Track cache statistics
        let mut cache_hits = 0;
        let mut cache_misses = 0;
        
        // Check genres against cache
        for genre in &invalid_genres {
            if let Some(cached_mapping) = cached_mappings.genre_mappings.get(genre) {
                info!("CACHE HIT: Genre '{}' -> '{}'", genre, cached_mapping);
                final_mapping.genre_mappings.insert(genre.clone(), cached_mapping.clone());
                cache_hits += 1;
            } else {
                info!("CACHE MISS: Genre '{}' not found in cache", genre);
                uncached_genres.insert(genre.clone());
                cache_misses += 1;
            }
        }
        
        // Check moods against cache
        for mood in &invalid_moods {
            if let Some(cached_mapping) = cached_mappings.mood_mappings.get(mood) {
                info!("CACHE HIT: Mood '{}' -> '{}'", mood, cached_mapping);
                final_mapping.mood_mappings.insert(mood.clone(), cached_mapping.clone());
                cache_hits += 1;
            } else {
                info!("CACHE MISS: Mood '{}' not found in cache", mood);
                uncached_moods.insert(mood.clone());
                cache_misses += 1;
            }
        }
        
        // Check occasions against cache
        for occasion in &invalid_occasions {
            if let Some(cached_mapping) = cached_mappings.occasion_mappings.get(occasion) {
                info!("CACHE HIT: Occasion '{}' -> '{}'", occasion, cached_mapping);
                final_mapping.occasion_mappings.insert(occasion.clone(), cached_mapping.clone());
                cache_hits += 1;
            } else {
                info!("CACHE MISS: Occasion '{}' not found in cache", occasion);
                uncached_occasions.insert(occasion.clone());
                cache_misses += 1;
            }
        }
        
        // Check keywords against cache
        for keyword in &invalid_keywords {
            if let Some(cached_mapping) = cached_mappings.keyword_mappings.get(keyword) {
                info!("CACHE HIT: Keyword '{}' -> '{}'", keyword, cached_mapping);
                final_mapping.keyword_mappings.insert(keyword.clone(), cached_mapping.clone());
                cache_hits += 1;
            } else {
                info!("CACHE MISS: Keyword '{}' not found in cache", keyword);
                uncached_keywords.insert(keyword.clone());
                cache_misses += 1;
            }
        }
        
        // Report cache statistics
        let total_invalid = invalid_genres.len() + invalid_moods.len() + invalid_occasions.len() + invalid_keywords.len();
        info!("CACHE STATISTICS: {} hits, {} misses out of {} invalid tags ({:.1}% cache hit rate)", 
              cache_hits, cache_misses, total_invalid, 
              if total_invalid > 0 { (cache_hits as f32 / total_invalid as f32) * 100.0 } else { 0.0 });
        
        // If we have uncached tags, ask Gemini
        let mut new_mappings = TagMapping::default();
        let uncached_total = uncached_genres.len() + uncached_moods.len() + uncached_occasions.len() + uncached_keywords.len();
        if uncached_total > 0 {
            
            info!("REQUESTING GEMINI MAPPINGS for {} uncached tags:", uncached_total);
            info!("  - Genres ({}): {:?}", uncached_genres.len(), uncached_genres);
            info!("  - Moods ({}): {:?}", uncached_moods.len(), uncached_moods);
            info!("  - Occasions ({}): {:?}", uncached_occasions.len(), uncached_occasions);
            info!("  - Keywords ({}): {:?}", uncached_keywords.len(), uncached_keywords);
            
            let gemini_mapping = self.create_tag_mapping_request(
                uncached_genres, uncached_moods, uncached_occasions, uncached_keywords
            ).await?;
            
            info!("GEMINI RETURNED MAPPINGS: {} genres, {} moods, {} occasions, {} keywords",
                  gemini_mapping.genre_mappings.len(), gemini_mapping.mood_mappings.len(),
                  gemini_mapping.occasion_mappings.len(), gemini_mapping.keyword_mappings.len());
            
            // Log the actual mappings for debugging
            for (invalid, valid) in &gemini_mapping.genre_mappings {
                info!("  NEW Genre mapping: '{}' -> '{}'", invalid, valid);
            }
            for (invalid, valid) in &gemini_mapping.mood_mappings {
                info!("  NEW Mood mapping: '{}' -> '{}'", invalid, valid);
            }
            for (invalid, valid) in &gemini_mapping.occasion_mappings {
                info!("  NEW Occasion mapping: '{}' -> '{}'", invalid, valid);
            }
            for (invalid, valid) in &gemini_mapping.keyword_mappings {
                info!("  NEW Keyword mapping: '{}' -> '{}'", invalid, valid);
            }
            
            // Merge with final mapping
            final_mapping.genre_mappings.extend(gemini_mapping.genre_mappings.clone());
            final_mapping.mood_mappings.extend(gemini_mapping.mood_mappings.clone());
            final_mapping.occasion_mappings.extend(gemini_mapping.occasion_mappings.clone());
            final_mapping.keyword_mappings.extend(gemini_mapping.keyword_mappings.clone());
            
            // Track new mappings for later storage
            new_mappings = gemini_mapping;
        } else {
            info!("ALL MAPPINGS FOUND IN CACHE - no Gemini API call needed!");
        }
        
        info!("FINAL MAPPING SUMMARY: {} genre, {} mood, {} occasion, {} keyword mappings ready to apply",
              final_mapping.genre_mappings.len(), final_mapping.mood_mappings.len(),
              final_mapping.occasion_mappings.len(), final_mapping.keyword_mappings.len());
        
        // Apply mappings to original responses
        let mut tagged_files = Vec::new();
        let mut fixed_files = 0;
        let mut unfixable_files = 0;
        
        for response in &invalid_responses {
            if let Some(audio_file) = invalid_files.iter().find(|f| f.file_path == response.file_path) {
                let fixed_response = self.apply_tag_mappings(&response, &final_mapping);
                
                // Final validation of fixed response
                if self.is_response_valid(&fixed_response) {
                    info!("FIXED: {} -> Genre: '{}', Mood: '{}', Occasions: {:?}, Keywords: {:?}",
                          audio_file.file_path, fixed_response.genre, fixed_response.mood,
                          fixed_response.rpg_occasion, fixed_response.rpg_keywords);
                    tagged_files.push(TaggedFile {
                        id: audio_file.id,
                        file_path: audio_file.file_path.clone(),
                        genre: fixed_response.genre,
                        mood: fixed_response.mood,
                        rpg_occasion: fixed_response.rpg_occasion,
                        rpg_keywords: fixed_response.rpg_keywords,
                    });
                    fixed_files += 1;
                } else {
                    warn!("UNFIXABLE: Could not fix tags for file: {}", response.file_path);
                    unfixable_files += 1;
                }
            }
        }
        
        info!("=== CACHE MAPPING COMPLETE ===");
        info!("RESULTS: {} files fixed, {} unfixable, {} new mappings for storage", 
              fixed_files, unfixable_files, 
              new_mappings.genre_mappings.len() + new_mappings.mood_mappings.len() + 
              new_mappings.occasion_mappings.len() + new_mappings.keyword_mappings.len());
        
        Ok((tagged_files, new_mappings))
    }
    
    async fn map_invalid_tags_and_fix(&self, invalid_responses: Vec<GeminiTagResponse>, invalid_files: Vec<AudioFile>, db_conn: Option<&Connection>) -> Result<Vec<TaggedFile>> {
        debug!("=== MAPPING INVALID TAGS (FALLBACK WITHOUT CACHE) ===");
        debug!("Processing {} responses with invalid tags", invalid_responses.len());
        
        // Collect all unique invalid tags
        let mut invalid_genres = HashSet::new();
        let mut invalid_moods = HashSet::new();
        let mut invalid_occasions = HashSet::new();
        let mut invalid_keywords = HashSet::new();
        
        for response in &invalid_responses {
            // Check genre
            if !response.genre.is_empty() && !self.valid_genres.contains(&response.genre) {
                invalid_genres.insert(response.genre.clone());
            }
            
            // Check moods
            for mood in response.mood.split(';') {
                let mood = mood.trim();
                if !mood.is_empty() && !self.valid_moods.contains(&mood.to_string()) {
                    invalid_moods.insert(mood.to_string());
                }
            }
            
            // Check occasions
            for occasion in &response.rpg_occasion {
                if !self.valid_occasions.contains(occasion) {
                    invalid_occasions.insert(occasion.clone());
                }
            }
            
            // Check keywords
            for keyword in &response.rpg_keywords {
                if !self.valid_keywords.contains(keyword) {
                    invalid_keywords.insert(keyword.clone());
                }
            }
        }
        
        debug!("Found invalid tags - Genres: {:?}, Moods: {:?}, Occasions: {:?}, Keywords: {:?}",
               invalid_genres, invalid_moods, invalid_occasions, invalid_keywords);
        
        // Fallback to direct API call (no caching)
        let tag_mapping = self.create_tag_mapping_request(
            invalid_genres, invalid_moods, invalid_occasions, invalid_keywords
        ).await?;
        
        debug!("Final tag mappings: {:?}", tag_mapping);
        
        // Apply mappings to original responses
        let mut tagged_files = Vec::new();
        for response in &invalid_responses {
            if let Some(audio_file) = invalid_files.iter().find(|f| f.file_path == response.file_path) {
                let fixed_response = self.apply_tag_mappings(&response, &tag_mapping);
                
                // Final validation of fixed response
                if self.is_response_valid(&fixed_response) {
                    tagged_files.push(TaggedFile {
                        id: audio_file.id,
                        file_path: audio_file.file_path.clone(),
                        genre: fixed_response.genre,
                        mood: fixed_response.mood,
                        rpg_occasion: fixed_response.rpg_occasion,
                        rpg_keywords: fixed_response.rpg_keywords,
                    });
                } else {
                    warn!("Could not fix tags for file: {}", response.file_path);
                }
            }
        }
        
        debug!("Successfully fixed {} out of {} files", tagged_files.len(), invalid_responses.len());
        Ok(tagged_files)
    }
    
    
    async fn create_tag_mapping_request(
        &self,
        invalid_genres: HashSet<String>,
        invalid_moods: HashSet<String>,
        invalid_occasions: HashSet<String>,
        invalid_keywords: HashSet<String>,
    ) -> Result<TagMapping> {
        if invalid_genres.is_empty() && invalid_moods.is_empty() && 
           invalid_occasions.is_empty() && invalid_keywords.is_empty() {
            return Ok(TagMapping::default());
        }
        
        // Create a vocabulary containing only the actually valid tags
        let valid_vocab = self.create_mapping_vocabulary();
        
        let prompt = format!(r#"You are a tag mapping expert. I have some invalid tags that need to be mapped to valid tags from our vocabulary.

=== VALID TAGS ONLY (DO NOT USE PARENT CATEGORIES) ===
{}

=== INVALID TAGS TO MAP ===
Genres: {:?}
Moods: {:?}
Occasions: {:?}
Keywords: {:?}

=== CRITICAL MAPPING RULES ===
- ONLY use tags from the VALID TAGS list above
- DO NOT use parent categories like "sound-design" - use specific hierarchical tags like "sound-design:impacts", "sound-design:vehicles"
- For genres, you MUST use the full hierarchical tags (parent:child format)
- When mapping broad categories, pick the MOST APPROPRIATE specific subcategory based on context

=== SPECIAL MAPPING RULES FOR BROAD CATEGORIES ===
- "sound-design" → Choose the most appropriate: "sound-design:impacts", "sound-design:objects", "sound-design:weapons", "sound-design:movement", "sound-design:whooshes", "sound-design:booms", etc.
- "orchestral" → Choose the most appropriate: "orchestral:cinematic", "orchestral:fantasy", "orchestral:classical", etc.
- "electronic" → Choose the most appropriate: "electronic:cyberpunk", "electronic:techno", "electronic:industrial", etc.
- If context suggests vehicles/objects, use "sound-design:objects"
- If context suggests impacts/hits, use "sound-design:impacts"
- If context suggests weapons, use "sound-design:weapons"
- If context suggests movement/motion, use "sound-design:movement"

Return ONLY a JSON object in this format:
{{
    "genre_mappings": {{ "invalid_tag": "valid_hierarchical_tag" }},
    "mood_mappings": {{ "invalid_tag": "valid_tag" }},
    "occasion_mappings": {{ "invalid_tag": "valid_tag" }},
    "keyword_mappings": {{ "invalid_tag": "valid_tag" }}
}}

Examples:
- "sound-design" → "sound-design:objects" (for car/vehicle sounds context)
- "sound-design" → "sound-design:impacts" (for impact sounds context)
- "orchestral" → "orchestral:cinematic" (for film-like music)
- "sfx:car-engine" → "sfx:vehicle" (if exists) or "sfx:mechanical"
- "combat-encounter" → "combat-skirmish"
- "gunfight" → "combat-ranged"
- "tavern-music" → "tavern"
- "scary" → "eerie"

Map each invalid tag to the closest valid alternative from the list above:"#,
            valid_vocab,
            invalid_genres, invalid_moods, invalid_occasions, invalid_keywords
        );
        
        let response = self.call_gemini(prompt).await?;
        
        // Clean the response (remove markdown code blocks like the main parse_response does)
        let cleaned_response = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();
        
        let mapping: TagMapping = serde_json::from_str(&cleaned_response)
            .map_err(|e| anyhow!("Failed to parse tag mapping response: {}. Raw response: {}", e, cleaned_response))?;
        
        Ok(mapping)
    }
    
    fn create_mapping_vocabulary(&self) -> String {
        let mut vocab = String::new();
        
        vocab.push_str("## VALID GENRES\n");
        for genre in &self.valid_genres {
            vocab.push_str(&format!("- {}\n", genre));
        }
        
        vocab.push_str("\n## VALID MOODS\n");
        for mood in &self.valid_moods {
            vocab.push_str(&format!("- {}\n", mood));
        }
        
        vocab.push_str("\n## VALID OCCASIONS\n");
        for occasion in &self.valid_occasions {
            vocab.push_str(&format!("- {}\n", occasion));
        }
        
        vocab.push_str("\n## VALID KEYWORDS\n");
        for keyword in &self.valid_keywords {
            vocab.push_str(&format!("- {}\n", keyword));
        }
        
        vocab
    }
    
    fn apply_tag_mappings(&self, response: &GeminiTagResponse, mapping: &TagMapping) -> GeminiTagResponse {
        // Map genre
        let fixed_genre = mapping.genre_mappings.get(&response.genre)
            .unwrap_or(&response.genre)
            .clone();
        
        // Map moods
        let fixed_mood = response.mood.split(';')
            .map(|mood| {
                let mood = mood.trim();
                mapping.mood_mappings.get(mood)
                    .unwrap_or(&mood.to_string())
                    .clone()
            })
            .filter(|mood| mood != "REMOVE")
            .collect::<Vec<_>>()
            .join("; ");
        
        // Map occasions
        let fixed_occasions = response.rpg_occasion.iter()
            .map(|occasion| {
                mapping.occasion_mappings.get(occasion)
                    .unwrap_or(occasion)
                    .clone()
            })
            .filter(|occasion| occasion != "REMOVE")
            .collect();
        
        // Map keywords
        let fixed_keywords = response.rpg_keywords.iter()
            .map(|keyword| {
                mapping.keyword_mappings.get(keyword)
                    .unwrap_or(keyword)
                    .clone()
            })
            .filter(|keyword| keyword != "REMOVE")
            .collect();
        
        GeminiTagResponse {
            file_path: response.file_path.clone(),
            genre: fixed_genre,
            mood: fixed_mood,
            rpg_occasion: fixed_occasions,
            rpg_keywords: fixed_keywords,
        }
    }
    
    fn is_response_valid(&self, response: &GeminiTagResponse) -> bool {
        debug!("=== FINAL VALIDATION START ===");
        debug!("Validating tags for file: {}", response.file_path);
        debug!("Genre: '{}'", response.genre);
        debug!("Mood: '{}'", response.mood);
        debug!("Occasions: {:?}", response.rpg_occasion);
        debug!("Keywords: {:?}", response.rpg_keywords);
        
        // Check genre
        if response.genre.is_empty() || !self.valid_genres.contains(&response.genre) {
            debug!("✗ Genre '{}' is INVALID - empty or not found in vocabulary", response.genre);
            return false;
        }
        debug!("✓ Genre '{}' is valid", response.genre);
        
        // Check moods
        let moods: Vec<&str> = response.mood.split(';')
            .map(|mood| mood.trim())
            .collect();
        
        if moods.is_empty() {
            debug!("✗ No moods provided");
            return false;
        }
        
        for mood in &moods {
            if !self.valid_moods.contains(&mood.to_string()) {
                debug!("✗ Mood '{}' is INVALID - not found in vocabulary", mood);
                return false;
            }
            debug!("✓ Mood '{}' is valid", mood);
        }
        
        // Check occasions
        for occasion in &response.rpg_occasion {
            if !self.valid_occasions.contains(occasion) {
                debug!("✗ Occasion '{}' is INVALID - not found in vocabulary", occasion);
                return false;
            }
            debug!("✓ Occasion '{}' is valid", occasion);
        }
        
        // Check keywords
        for keyword in &response.rpg_keywords {
            if !self.valid_keywords.contains(keyword) {
                debug!("✗ Keyword '{}' is INVALID - not found in vocabulary", keyword);
                return false;
            }
            debug!("✓ Keyword '{}' is valid", keyword);
        }
        
        debug!("✓ All tags valid for file: {}", response.file_path);
        true
    }
    
    #[cfg(debug_assertions)]
    fn save_debug_response(&self, batch_num: usize, response: &str) -> Result<()> {
        use std::fs;
        use std::path::PathBuf;
        
        // Get the project root (parent of src-tauri)
        let current_exe = std::env::current_exe()?;
        let mut project_root = current_exe.parent()
            .and_then(|p| p.parent()) // Go up from target/debug or target/release
            .and_then(|p| p.parent()) // Go up from target
            .unwrap_or_else(|| std::path::Path::new("."))
            .to_path_buf();
        
        // If we're running from src-tauri directory, go up one level
        if project_root.file_name().and_then(|n| n.to_str()) == Some("src-tauri") {
            project_root = project_root.parent().unwrap_or(&project_root).to_path_buf();
        }
        
        let tmp_dir = project_root.join("tmp");
        fs::create_dir_all(&tmp_dir)?;
        
        let filename = format!("gemini_response_batch_{}.txt", batch_num);
        let path = tmp_dir.join(filename);
        
        fs::write(path, response)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prompt_generation() {
        // This test will fail without a valid .env file
        if let Ok(tagger) = GeminiTagger::new() {
            let files = vec!["test.mp3".to_string()];
            let prompt = tagger.create_prompt(files);
            
            assert!(prompt.contains("STRICT TAG VOCABULARY"));
            assert!(prompt.contains("GENRES"));
            assert!(prompt.contains("test.mp3"));
        }
    }
    
    #[test]
    fn test_response_parsing() {
        if let Ok(tagger) = GeminiTagger::new() {
            let json = r#"[{
                "file_path": "test.mp3",
                "genre": "ambient",
                "mood": "mysterious",
                "rpg_occasion": ["dungeon-crawl"],
                "rpg_keywords": ["biome:cave"]
            }]"#;
            
            let result = tagger.parse_response(json.to_string());
            assert!(result.is_ok());
            
            let tagged = result.unwrap();
            assert_eq!(tagged.len(), 1);
            assert_eq!(tagged[0].file_path, "test.mp3");
            assert_eq!(tagged[0].genre, "ambient");
        }
    }
}