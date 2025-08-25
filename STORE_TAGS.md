# Store Tags Strategy

## Overview
This document outlines the strategy for writing all database-stored metadata and RPG tags into the actual audio files using ID3v2.4 tags. This creates a backup of metadata within the files themselves and ensures portability.

**🎯 Implementation Status: COMPLETED** ✅
- Backend: Fully implemented in `src-tauri/src/store_tags_handler.rs`
- Frontend: Complete UI with confirmation, progress, and results dialogs
- Integration: Button added to header, all event handlers connected
- Testing: Ready for user testing

## Current Database Fields to Store

### Core Metadata Fields
- **title** → `TIT2` (Title/Song name)
- **artist** → `TPE1` (Lead performer/Artist)
- **album** → `TALB` (Album/Movie/Show title)
- **album_artist** → `TPE2` (Band/Orchestra/Accompaniment)
- **year** → `TYER` (Year) or `TDRC` (Recording time)
- **track_number** → `TRCK` (Track number/Position in set)
- **genre** → `TCON` (Content type/Genre)

### Extended Metadata Fields
- **composer** → `TCOM` (Composer)
- **conductor** → `TPE3` (Conductor/Performer refinement)
- **producer** → `TPRO` (Produced notice)
- **publisher** → `TPUB` (Publisher)
- **copyright** → `TCOP` (Copyright message)
- **language** → `TLAN` (Language)
- **initial_key** → `TKEY` (Initial key)

### Technical Fields
- **duration** → `TLEN` (Length in milliseconds)
- **bpm** → `TBPM` (BPM - Beats per minute)
- **encoding_info** → `TENC` (Encoded by)

## RPG Tags Strategy

### Primary RPG Tags (Standard ID3 Fields)
- **Primary Genre** → `TCON` (Content type) - Use main RPG genre (e.g., "orchestral:cinematic")
- **Mood** → `TMOO` (Mood) - Primary mood tag
- **BPM** → `TBPM` (BPM) - Calculated BPM value

### Extended RPG Tags (Custom TXXX Fields)
For RPG-specific tags that don't map to standard ID3 fields, use `TXXX` (User defined text information) frames with descriptive names:

#### RPG Genre Tags
- **TXXX:RPG_GENRE** → All genre tags joined with semicolons
  - Example: `"orchestral:cinematic;hybrid:orchestral-electronic"`

#### RPG Mood Tags  
- **TXXX:RPG_MOOD** → All mood tags joined with semicolons
  - Example: `"mysterious;tense;heroic"`

#### RPG Occasion Tags
- **TXXX:RPG_OCCASION** → All occasion tags joined with semicolons
  - Example: `"dungeon-crawl;boss-loop;combat-ambush"`

#### RPG Keyword Tags
- **TXXX:RPG_KEYWORDS** → All keyword tags joined with semicolons
  - Example: `"biome:forest;creature:dragon;loc:castle;sfx:sword-clash"`

#### Combined Tags for Compatibility
- **TXXX:RPG_ALL_TAGS** → Complete list of all RPG tags for maximum compatibility
  - Example: `"orchestral:cinematic;mysterious;dungeon-crawl;biome:forest;creature:dragon"`

### File Path Information
- **TXXX:ORIGINAL_PATH** → Store original file path for reference
  - Example: `"I:\Musica\RPG\Dungeon\dark_ambient_01.wav"`

### Ligeia-Specific Metadata
- **TXXX:LIGEIA_VERSION** → Version of Ligeia that wrote the tags
- **TXXX:LIGEIA_TIMESTAMP** → When tags were written (ISO format)
- **TXXX:LIGEIA_DATABASE_ID** → Original database ID for cross-reference

## Implementation Strategy

### Phase 1: Tag Reading and Comparison
1. **Read Current File Tags**: Use Rust `id3` crate to read existing ID3v2.4 tags
2. **Compare with Database**: Check each field to see if file tags match database values
3. **Identify Differences**: Create a diff report showing what needs to be updated
4. **User Confirmation**: Show preview of changes before writing

### Phase 2: Tag Writing Process
1. **Backup Strategy**: Consider creating backup copies (optional user setting)
2. **Batch Processing**: Process files in batches with progress indication
3. **Error Handling**: Skip problematic files, log errors, continue with others
4. **Verification**: Re-read tags after writing to confirm success

### Phase 3: UI Integration
1. **Button Location**: Add "📝 Store Tags in Files" button after Export Library button
2. **Progress Dialog**: Show progress with file count, current file, errors
3. **Results Summary**: Display how many files were updated, skipped, or failed
4. **Error Report**: Show detailed list of any files that couldn't be processed

## Technical Implementation ✅ IMPLEMENTED

### Rust Backend (src-tauri/src/store_tags_handler.rs) ✅
```rust
// ✅ IMPLEMENTED: New Tauri command
#[tauri::command]
pub async fn store_all_tags_in_files(app_handle: AppHandle) -> Result<StoreTagsResult, String>

// ✅ IMPLEMENTED: Data structures with additional fields
pub struct StoreTagsResult {
    pub total_files: usize,
    pub updated_files: usize,
    pub skipped_files: usize,
    pub failed_files: usize,
    pub errors: Vec<String>,
    pub duration_seconds: f64,  // Added for performance tracking
}

pub struct FileTagComparison {
    pub file_path: String,
    pub needs_update: bool,
    pub missing_tags: Vec<String>,
    pub different_values: Vec<TagDifference>,  // Enhanced with structured type
}

pub struct TagDifference {
    pub field_name: String,
    pub current_value: String,
    pub new_value: String,
}
```

### Key Functions ✅ IMPLEMENTED
1. ✅ **`compare_file_tags_with_database()`** - Compares current file tags with DB values
2. ✅ **`write_metadata_to_tag()`** - Writes all metadata fields to ID3 tag
3. ✅ **`write_rpg_tags_to_txxx()`** - Converts RPG tags to TXXX format
4. ✅ **`process_single_file()`** - Processes individual files with error handling
5. ✅ **`compare_standard_fields()`** - Compares standard ID3 metadata
6. ✅ **`compare_rpg_tags()`** - Compares custom RPG TXXX fields

### Frontend JavaScript ✅ IMPLEMENTED
```javascript
// ✅ IMPLEMENTED: StoreTagsManager class with full UI
class StoreTagsManager {
    async storeAllTagsInFiles() {
        // ✅ Confirmation dialog with detailed explanation
        // ✅ Progress modal with animated spinner  
        // ✅ Results dialog with statistics and error reporting
        const result = await invoke('store_all_tags_in_files');
    }
}
```

### UI Integration ✅ IMPLEMENTED
- ✅ **Button Added**: "📝 Store Tags" button in header after Import
- ✅ **Event Handler**: Connected to AmbientMixerApp.js event system
- ✅ **CSS Styling**: Complete modal styles with professional appearance

## Tag Format Specifications

### Semicolon-Separated Lists
For multi-value fields, use semicolons as separators (ID3v2.4 standard):
```
"orchestral:cinematic;hybrid:orchestral-electronic;world:celtic"
```

### Hierarchical Tags
Preserve hierarchical structure with colons:
```
"orchestral:cinematic" (parent:child)
"biome:forest" (category:value)
```

### Special Characters
- Escape special characters if needed
- Use UTF-8 encoding for international characters
- Handle empty/null values gracefully

## Quality Assurance

### Validation Steps
1. **Pre-write Validation**: Verify all data types and formats
2. **File Permission Check**: Ensure files are writable
3. **Backup Verification**: If backup enabled, verify backup creation
4. **Post-write Verification**: Read back written tags to confirm

### Error Scenarios to Handle
- **Read-only files**: Skip with clear error message
- **Corrupted files**: Skip and log error
- **Unsupported formats**: Skip non-audio files
- **Permission denied**: Log and continue with other files
- **Disk space**: Check available space before writing

### Recovery Strategy
- **Atomic operations**: Don't partially update files
- **Error isolation**: One file failure doesn't stop batch
- **Rollback capability**: Option to restore from backups if available

## User Experience

### Progress Feedback
```
📝 Storing Tags in Files...
Progress: 1,247 / 2,856 files (43%)
Current: /path/to/current/file.mp3
Updated: 1,198 | Skipped: 45 | Failed: 4
```

### Completion Summary
```
✅ Tag Storage Complete!
📊 Results:
  • 2,856 files processed
  • 2,807 files updated
  • 45 files skipped (already current)
  • 4 files failed (see error log)
⏱️ Completed in 2m 34s
```

### Error Handling
- **Non-blocking errors**: Continue processing other files
- **Clear error messages**: Explain why each file failed
- **Recovery suggestions**: Provide actionable solutions

## Benefits

### Data Portability
- Tags travel with the files
- No dependency on Ligeia database
- Works with other audio software

### Backup and Recovery
- Metadata embedded in files serves as backup
- Can rebuild database from file tags if needed
- Preserves work even if database is lost

### Interoperability  
- Standard ID3v2.4 tags work with most audio software
- RPG-specific tags available via TXXX fields
- Maintains both standard and custom metadata

## Future Enhancements

### Advanced Features
- **Selective tag writing**: Choose which tag types to write
- **Template-based tagging**: Custom tag format templates
- **Batch tag verification**: Verify all files have correct tags
- **Tag synchronization**: Keep file tags in sync with database changes

### Integration Options
- **Auto-sync mode**: Automatically update file tags when database changes
- **Import from tags**: Read tags from files to populate database
- **Conflict resolution**: Handle cases where file tags differ from database

This comprehensive strategy ensures that all valuable RPG metadata is preserved within the audio files themselves, creating a robust and portable audio library system.