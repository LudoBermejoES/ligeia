# Gemini Auto-Tagging Implementation Plan for Ligeia

## Executive Summary
Implement an automated RPG tagging system within Ligeia that uses Google's Gemini AI to enrich untagged audio files with genre, mood, occasion, and keyword tags. This will be a Rust implementation integrated directly into the Tauri backend with a new frontend action to trigger the process.

## Architecture Overview

### Backend (Rust/Tauri)
- **New Module**: `src-tauri/src/gemini_tagger.rs` - Core tagging logic
- **New Handler**: `src-tauri/src/gemini_handler.rs` - Tauri command handlers
- **Database Integration**: Extend existing tag system to store Gemini-generated tags
- **Configuration**: Read Gemini API key from .env file

### Frontend (JavaScript)
- **New Action**: "Auto-Tag with AI" in Actions dropdown menu
- **Progress UI**: Modal showing tagging progress with batch updates

## Detailed Implementation Steps

### Phase 1: Backend Infrastructure

#### 1.1 Create Gemini Tagger Module (`gemini_tagger.rs`)
```rust
use dotenv::dotenv;
use std::env;

pub struct GeminiTagger {
    api_key: String,
    client: GeminiClient,
    batch_size: usize,
    max_parallel: usize,
}

impl GeminiTagger {
    // Initialize with API key from .env
    pub fn new() -> Result<Self> {
        dotenv().ok();
        let api_key = env::var("GEMINI_API_KEY")
            .map_err(|_| "GEMINI_API_KEY not found in .env file")?;
    
    // Process untagged files
    pub async fn process_untagged_files(&self, files: Vec<AudioFile>) -> Result<Vec<TaggedFile>>
    
    // Create prompt with AUTOTAG.md and TAGS.md content
    fn create_prompt(&self, file_paths: Vec<String>) -> String
    
    // Parse Gemini response
    fn parse_response(&self, response: String) -> Result<Vec<TaggedFile>>
    
    // Apply tags to database
    async fn apply_tags(&self, tagged_files: Vec<TaggedFile>) -> Result<()>
}
```

#### 1.2 Create Gemini Handler (`gemini_handler.rs`)
```rust
// Tauri commands
#[tauri::command]
pub async fn get_untagged_files(db: State<Database>) -> Result<Vec<AudioFile>>

#[tauri::command]
pub async fn auto_tag_files(
    file_ids: Vec<i32>,
    batch_size: Option<usize>,
    db: State<Database>
) -> Result<TaggingProgress>

#[tauri::command]
pub async fn check_gemini_api_key() -> Result<bool> {
    dotenv().ok();
    Ok(env::var("GEMINI_API_KEY").is_ok())
}
```

#### 1.3 Database Schema Updates
```sql
-- Add new columns to audio_files table
ALTER TABLE audio_files ADD COLUMN auto_tagged BOOLEAN DEFAULT FALSE;
ALTER TABLE audio_files ADD COLUMN auto_tag_date TEXT;
ALTER TABLE audio_files ADD COLUMN auto_tag_version TEXT;

-- Create auto_tag_history table for tracking
CREATE TABLE auto_tag_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_id INTEGER NOT NULL,
    tagged_at TEXT NOT NULL,
    tags_applied TEXT NOT NULL,
    api_version TEXT,
    FOREIGN KEY (file_id) REFERENCES audio_files(id)
);
```

#### 1.4 Configuration Management
```rust
// Add to models.rs
#[derive(Serialize, Deserialize)]
pub struct GeminiConfig {
    pub batch_size: usize,  // Default: 50
    pub max_parallel: usize, // Default: 3
    pub model: String,       // Default: "gemini-1.5-flash-002"
}

// API key loaded from .env file in project root
// Create a .env file with:
// GEMINI_API_KEY=your_api_key_here
```

### Phase 2: Core Tagging Logic

#### 2.1 File Selection Logic
```rust
// Query for untagged files
// Files are considered untagged if they lack ALL of:
// - genre (or genre is empty/null)
// - mood (or mood is empty/null) 
// - rpg_occasion tags (count = 0)
// - rpg_keywords tags (count = 0)

SELECT * FROM audio_files 
WHERE (genre IS NULL OR genre = '')
   OR (mood IS NULL OR mood = '')
   OR id NOT IN (
       SELECT DISTINCT file_id FROM rpg_tags 
       WHERE category IN ('occasion', 'keyword')
   )
   AND auto_tagged = FALSE
```

#### 2.2 Prompt Generation
```rust
impl GeminiTagger {
    fn create_prompt(&self, file_paths: Vec<String>) -> String {
        // Load AUTOTAG.md from embedded resources
        let autotag_content = include_str!("../resources/AUTOTAG.md");
        
        // Load TAGS.md from embedded resources
        let tags_content = include_str!("../resources/TAGS.md");
        
        // Format file paths as JSON array
        let files_json = serde_json::to_string_pretty(&file_paths)?;
        
        format!(r#"
{}

TAGS.md Content:
{}

File Paths to Process:
{}

Please analyze these file paths and return enriched RPG tags as a JSON array.
Each object must have: file_path, genre, mood, rpg_occasion, rpg_keywords.
Return ONLY the JSON array, no explanations or markdown. Use only the tags I'm sharing with you, don't invent new ones.
"#, autotag_content, tags_content, files_json)
    }
}
```

#### 2.3 API Integration
```rust
use gemini_client_api::{GeminiClient, GenerateContentRequest};

impl GeminiTagger {
    async fn call_gemini(&self, prompt: String) -> Result<String> {
        let request = GenerateContentRequest {
            contents: vec![Content {
                parts: vec![Part::Text(prompt)],
                role: Some("user".to_string()),
            }],
            generation_config: Some(GenerationConfig {
                temperature: Some(0.3),  // Low temperature for consistency
                max_output_tokens: Some(8192),
                response_mime_type: Some("application/json".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        
        let response = self.client.generate_content(request).await?;
        Ok(response.text())
    }
}
```

#### 2.4 Response Parsing and Validation
```rust
#[derive(Deserialize)]
struct GeminiTagResponse {
    file_path: String,
    genre: String,
    mood: String,
    rpg_occasion: Vec<String>,
    rpg_keywords: Vec<String>,
}

impl GeminiTagger {
    fn parse_response(&self, response: String) -> Result<Vec<GeminiTagResponse>> {
        // Remove markdown code blocks if present
        let cleaned = response
            .trim_start_matches("```json")
            .trim_end_matches("```")
            .trim();
        
        // Parse JSON array
        let tagged_files: Vec<GeminiTagResponse> = serde_json::from_str(cleaned)?;
        
        // Validate against TAGS.md vocabulary
        for file in &tagged_files {
            self.validate_tags(file)?;
        }
        
        Ok(tagged_files)
    }
    
    fn validate_tags(&self, response: &GeminiTagResponse) -> Result<()> {
        // Check genre exists in vocabulary
        // Check mood tags exist in vocabulary
        // Check occasions exist in vocabulary
        // Check keywords follow faceted format
        Ok(())
    }
}
```

#### 2.5 Parallel Processing
```rust
use tokio::sync::Semaphore;
use futures::stream::{self, StreamExt};

impl GeminiTagger {
    async fn process_in_parallel(&self, batches: Vec<Vec<String>>) -> Vec<Result<Vec<GeminiTagResponse>>> {
        let semaphore = Arc::new(Semaphore::new(self.max_parallel));
        
        stream::iter(batches)
            .map(|batch| {
                let sem = semaphore.clone();
                async move {
                    let _permit = sem.acquire().await?;
                    self.process_batch(batch).await
                }
            })
            .buffer_unordered(self.max_parallel)
            .collect()
            .await
    }
}
```

### Phase 3: Frontend Integration

#### 3.1 Add Action to Actions Dropdown
```javascript
// In src-fe/src/managers/ActionsManager.js or equivalent

class ActionsManager {
    constructor() {
        this.actions = [
            // ... existing actions
            {
                id: 'auto-tag-ai',
                label: 'Auto-Tag with AI',
                icon: '🤖',
                handler: () => this.handleAutoTag(),
                requiresSelection: false,  // Process all untagged
                requiresApiKey: true
            }
        ];
    }
    
    async handleAutoTag() {
        // Check for API key in .env
        const hasApiKey = await window.__TAURI__.invoke('check_gemini_api_key');
        
        if (!hasApiKey) {
            this.showNotification('Please add GEMINI_API_KEY to your .env file');
            return;
        }
        
        // Get untagged files count
        const untaggedFiles = await window.__TAURI__.invoke('get_untagged_files');
        
        if (untaggedFiles.length === 0) {
            this.showNotification('All files are already tagged!');
            return;
        }
        
        // Show confirmation dialog
        const confirmed = await this.showConfirmation(
            `Found ${untaggedFiles.length} untagged files. Process with AI tagging?`
        );
        
        if (!confirmed) return;
        
        // Show progress modal
        this.showProgressModal(untaggedFiles.length);
        
        // Start tagging process
        this.startTagging(untaggedFiles);
    }
}
```

#### 3.2 Progress Modal UI
```javascript
class AutoTagProgressModal {
    constructor(totalFiles) {
        this.totalFiles = totalFiles;
        this.processedFiles = 0;
        this.failedFiles = 0;
        this.currentBatch = 0;
        this.totalBatches = Math.ceil(totalFiles / 50);
    }
    
    render() {
        return `
            <div class="modal">
                <h2>AI Auto-Tagging in Progress</h2>
                <div class="progress-stats">
                    <p>Total Files: ${this.totalFiles}</p>
                    <p>Processed: ${this.processedFiles}</p>
                    <p>Failed: ${this.failedFiles}</p>
                    <p>Batch: ${this.currentBatch} / ${this.totalBatches}</p>
                </div>
                <div class="progress-bar">
                    <div class="progress-fill" style="width: ${this.getProgress()}%"></div>
                </div>
                <div class="current-status">
                    ${this.currentStatus}
                </div>
                <button onclick="this.cancel()">Cancel</button>
            </div>
        `;
    }
    
    updateProgress(processed, failed, batch) {
        this.processedFiles = processed;
        this.failedFiles = failed;
        this.currentBatch = batch;
        this.render();
    }
}
```


### Phase 4: Resource Files

#### 4.1 Embed AUTOTAG.md and TAGS.md
```rust
// In src-tauri/src/resources/
// Copy AUTOTAG.md from enrich_tags project
// Use existing TAGS.md from Ligeia project

// In gemini_tagger.rs
const AUTOTAG_PROMPT: &str = include_str!("../resources/AUTOTAG.md");
const TAGS_VOCABULARY: &str = include_str!("../../../TAGS.md");
```

#### 4.2 Migration Scripts
```rust
// In src-tauri/src/database/migrations.rs
pub fn migrate_for_gemini(conn: &Connection) -> Result<()> {
    conn.execute(
        "ALTER TABLE audio_files ADD COLUMN auto_tagged BOOLEAN DEFAULT FALSE",
        [],
    )?;
    
    conn.execute(
        "ALTER TABLE audio_files ADD COLUMN auto_tag_date TEXT",
        [],
    )?;
    
    conn.execute(
        "ALTER TABLE audio_files ADD COLUMN auto_tag_version TEXT",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS auto_tag_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            file_id INTEGER NOT NULL,
            tagged_at TEXT NOT NULL,
            tags_applied TEXT NOT NULL,
            api_version TEXT,
            FOREIGN KEY (file_id) REFERENCES audio_files(id)
        )",
        [],
    )?;
    
    Ok(())
}
```

### Phase 5: Error Handling and Recovery

#### 5.1 Retry Logic
```rust
impl GeminiTagger {
    async fn process_batch_with_retry(&self, batch: Vec<String>) -> Result<Vec<GeminiTagResponse>> {
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 3;
        
        loop {
            attempts += 1;
            
            match self.process_batch(batch.clone()).await {
                Ok(response) => return Ok(response),
                Err(e) if attempts < MAX_ATTEMPTS => {
                    log::warn!("Batch processing failed (attempt {}): {}", attempts, e);
                    tokio::time::sleep(Duration::from_secs(2_u64.pow(attempts))).await;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
```

#### 5.2 Partial Progress Saving
```rust
impl GeminiTagger {
    async fn save_progress(&self, tagged_files: Vec<TaggedFile>, db: &Database) -> Result<()> {
        // Start transaction
        let tx = db.begin_transaction()?;
        
        for file in tagged_files {
            // Update audio_files table
            tx.execute(
                "UPDATE audio_files SET 
                    genre = ?, 
                    mood = ?, 
                    auto_tagged = TRUE, 
                    auto_tag_date = datetime('now'),
                    auto_tag_version = ?
                WHERE id = ?",
                params![file.genre, file.mood, "1.0.0", file.id],
            )?;
            
            // Insert occasion tags
            for occasion in file.rpg_occasion {
                tx.execute(
                    "INSERT INTO rpg_tags (file_id, category, tag) VALUES (?, 'occasion', ?)",
                    params![file.id, occasion],
                )?;
            }
            
            // Insert keyword tags
            for keyword in file.rpg_keywords {
                tx.execute(
                    "INSERT INTO rpg_tags (file_id, category, tag) VALUES (?, 'keyword', ?)",
                    params![file.id, keyword],
                )?;
            }
            
            // Log to history
            tx.execute(
                "INSERT INTO auto_tag_history (file_id, tagged_at, tags_applied, api_version) 
                VALUES (?, datetime('now'), ?, ?)",
                params![file.id, serde_json::to_string(&file)?, "gemini-1.5-flash-002"],
            )?;
        }
        
        tx.commit()?;
        Ok(())
    }
}
```

#### 5.3 Debug Logging
```rust
impl GeminiTagger {
    fn save_debug_response(&self, batch_num: usize, response: &str) -> Result<()> {
        let debug_dir = app_handle.path_resolver()
            .app_data_dir()
            .unwrap()
            .join("debug");
        
        fs::create_dir_all(&debug_dir)?;
        
        let filename = format!("gemini_response_batch_{}.txt", batch_num);
        let path = debug_dir.join(filename);
        
        fs::write(path, response)?;
        Ok(())
    }
}
```

### Phase 6: Testing Strategy

#### 6.1 Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prompt_generation() {
        let tagger = GeminiTagger::new("test_key".to_string()).unwrap();
        let files = vec!["test.mp3".to_string()];
        let prompt = tagger.create_prompt(files);
        
        assert!(prompt.contains("AUTOTAG"));
        assert!(prompt.contains("TAGS.md"));
        assert!(prompt.contains("test.mp3"));
    }
    
    #[test]
    fn test_response_parsing() {
        let tagger = GeminiTagger::new("test_key".to_string()).unwrap();
        let json = r#"[{"file_path": "test.mp3", "genre": "ambient", "mood": "mysterious", "rpg_occasion": ["dungeon-crawl"], "rpg_keywords": ["biome:cave"]}]"#;
        
        let result = tagger.parse_response(json.to_string());
        assert!(result.is_ok());
        
        let tagged = result.unwrap();
        assert_eq!(tagged.len(), 1);
        assert_eq!(tagged[0].file_path, "test.mp3");
    }
    
    #[tokio::test]
    async fn test_batch_processing() {
        // Test with mock API responses
    }
}
```

#### 6.2 Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    #[tokio::test]
    async fn test_full_tagging_workflow() {
        // 1. Create test database with untagged files
        // 2. Run tagging process with test API key
        // 3. Verify tags were applied correctly
        // 4. Verify auto_tagged flag is set
        // 5. Verify history is recorded
    }
}
```

### Phase 7: Deployment Considerations

#### 7.1 API Key Security
- Read API key from .env file (not tracked in git)
- Never log or expose API key in debug output
- Validate API key exists on startup

#### 7.2 Rate Limiting
- Implement exponential backoff for rate limit errors
- Track API usage to stay within quotas
- Allow user to configure request throttling

#### 7.3 Performance Optimization
- Cache AUTOTAG.md and TAGS.md content in memory
- Use connection pooling for database operations
- Implement progress streaming to frontend via Tauri events

#### 7.4 User Experience
- Show estimated time remaining based on batch progress
- Allow pausing/resuming of tagging process
- Provide detailed error messages for troubleshooting
- Save progress after each successful batch

## Implementation Timeline

### Week 1: Backend Foundation
- Day 1-2: Set up gemini-client-api integration and basic module structure
- Day 3-4: Implement core tagging logic and prompt generation
- Day 5: Database schema updates and migration scripts

### Week 2: Core Features
- Day 1-2: Response parsing and validation
- Day 3-4: Parallel processing and batch management
- Day 5: Error handling and retry logic

### Week 3: Frontend Integration
- Day 1-2: Add action to UI and create progress modal
- Day 3-4: Progress tracking and real-time updates
- Day 5: Error handling and notifications

### Week 4: Testing and Polish
- Day 1-2: Unit and integration tests
- Day 3-4: Performance optimization and rate limiting
- Day 5: Documentation and deployment preparation

## Success Metrics

1. **Accuracy**: 95%+ of generated tags match vocabulary specifications
2. **Performance**: Process 1000 files in under 5 minutes
3. **Reliability**: Graceful handling of API failures with resume capability
4. **User Experience**: Clear progress indication and error reporting
5. **Coverage**: Successfully tag 90%+ of untagged files

## Risk Mitigation

1. **API Cost**: Implement batch size limits and user warnings for large libraries
2. **Rate Limiting**: Configurable parallelism and automatic backoff
3. **Tag Quality**: Validation against vocabulary with fallback to generic tags
4. **Network Failures**: Robust retry logic and progress persistence
5. **Data Loss**: Transaction-based updates with rollback capability

## Future Enhancements

1. **Custom Prompts**: Allow users to customize tagging instructions
2. **Tag Review UI**: Interface to review and modify AI-generated tags
3. **Batch Undo**: Ability to revert auto-tagging for specific batches
4. **Alternative Models**: Support for other AI providers (OpenAI, Anthropic)
5. **Smart Retagging**: Only retag files when vocabulary updates
6. **Tag Confidence**: Show confidence scores for generated tags
7. **Learning Mode**: Improve tagging based on user corrections

## Conclusion

This implementation plan provides a comprehensive roadmap for integrating Gemini-based auto-tagging into Ligeia. The phased approach ensures systematic development with clear milestones and success metrics. The Rust implementation will provide performance benefits while maintaining the robustness expected of the Ligeia application.