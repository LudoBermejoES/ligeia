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
use log::{info, warn, error};

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

pub struct GeminiTagger {
    client: Gemini,
    session: Session,
    batch_size: usize,
    max_parallel: usize,
    autotag_prompt: String,
    tags_vocabulary: String,
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
        
        // Load prompts from embedded resources
        let autotag_prompt = include_str!("../resources/AUTOTAG.md").to_string();
        let tags_vocabulary = include_str!("../../TAGS.md").to_string();
        
        Ok(Self {
            client,
            session,
            batch_size: 50,
            max_parallel: 3,
            autotag_prompt,
            tags_vocabulary,
        })
    }
    
    pub fn with_config(batch_size: usize, max_parallel: usize) -> Result<Self> {
        let mut tagger = Self::new()?;
        tagger.batch_size = batch_size;
        tagger.max_parallel = max_parallel;
        Ok(tagger)
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
            autotag_prompt: self.autotag_prompt.clone(),
            tags_vocabulary: self.tags_vocabulary.clone(),
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
        
        // Match responses back to original files and create TaggedFile objects
        let mut tagged_files = Vec::new();
        for audio_file in batch {
            if let Some(gemini_response) = gemini_responses.iter()
                .find(|r| r.file_path == audio_file.file_path) {
                tagged_files.push(TaggedFile {
                    id: audio_file.id,
                    file_path: audio_file.file_path,
                    genre: gemini_response.genre.clone(),
                    mood: gemini_response.mood.clone(),
                    rpg_occasion: gemini_response.rpg_occasion.clone(),
                    rpg_keywords: gemini_response.rpg_keywords.clone(),
                });
            }
        }
        
        Ok(tagged_files)
    }
    
    fn create_prompt(&self, file_paths: Vec<String>) -> String {
        let files_json = serde_json::to_string_pretty(&file_paths)
            .unwrap_or_else(|_| "[]".to_string());
        
        format!(r#"{}

TAGS.md Content:
{}

File Paths to Process:
{}

Please analyze these file paths and return enriched RPG tags as a JSON array.
Each object must have: file_path, genre, mood, rpg_occasion, rpg_keywords.

CRITICAL: Return ONLY a valid JSON array starting with [ and ending with ]. No explanations, no markdown, no additional text.
Use only the tags I'm sharing with you, don't invent new ones.

Example format:
[
  {{
    "file_path": "path/to/file.wav",
    "genre": "ambient:dark-ambient", 
    "mood": "ominous; eerie",
    "rpg_occasion": ["dungeon-crawl", "night-watch"],
    "rpg_keywords": ["loc:cave", "timbre:drone"]
  }}
]"#,
            self.autotag_prompt,
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
        
        Ok(response.get_text(""))
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
            
            assert!(prompt.contains("AUTOTAG"));
            assert!(prompt.contains("TAGS.md"));
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