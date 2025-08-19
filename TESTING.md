# RPG Audio Tagging System - Testing Guide

## Overview

The RPG Audio Tagging System allows users to categorize their audio files using a controlled vocabulary specifically designed for tabletop RPG sessions. This testing guide covers all implemented features.

## Features Implemented

### 1. **Modular Rust Backend**
- ✅ Refactored monolithic `main.rs` into separate modules
- ✅ `models.rs` - Data structures for AudioFile, RpgTag, TagVocabulary
- ✅ `database.rs` - Database operations and schema management
- ✅ `audio_handler.rs` - Audio file metadata processing
- ✅ `tag_manager.rs` - RPG tag management logic
- ✅ `file_scanner.rs` - Recursive directory scanning

### 2. **Database Schema**
- ✅ Enhanced `audio_files` table with all ID3v2.4 fields
- ✅ New `rpg_tags` table for RPG-specific tags
- ✅ New `tag_vocabulary` table with controlled vocabularies
- ✅ Proper indexes for performance
- ✅ Foreign key constraints and data integrity

### 3. **Tag Vocabulary System**
- ✅ Pre-loaded controlled vocabularies for:
  - **Genre**: ambient, battle, exploration, tavern, dungeon, town, nature, magic, horror, epic
  - **Mood**: peaceful, tense, mysterious, heroic, dark, joyful, melancholic, intense, suspenseful, whimsical
  - **Occasion**: combat, rest, dialogue, exploration, stealth, puzzle, ceremony, travel, shopping, finale
  - **Keywords**: forest, castle, dragon, magic, medieval, fantasy, orchestral, acoustic, electronic, vocal

### 4. **Bulk Tag Editor**
- ✅ Modal interface for selecting multiple audio files
- ✅ Visual tag vocabulary with clickable chips
- ✅ File selection with checkboxes and visual feedback
- ✅ Bulk application of multiple tags to multiple files
- ✅ Real-time tag summary display for each file

### 5. **Tag Search & Filtering**
- ✅ Tag-based search interface in sidebar
- ✅ Filter by any combination of genres, moods, occasions, keywords
- ✅ AND/OR logic for tag matching
- ✅ Real-time search results count
- ✅ Visual active filter indicators

### 6. **Frontend Services**
- ✅ `TagService.js` - Handles all RPG tag operations
- ✅ `BulkTagEditorController.js` - Manages bulk tagging UI
- ✅ `TagSearchController.js` - Handles search and filtering

## Testing Procedures

### Phase 1: Basic Functionality Test

1. **Start the Application**
   ```bash
   npm run dev
   ```

2. **Load Audio Files**
   - Click "📂 Load Sounds" or "📁 Load Directory (Recursive)"
   - Verify files are loaded and displayed in the sound pads grid
   - Check that basic metadata (title, artist) is shown

3. **Verify Tag Service Initialization**
   - Check browser console for "TagService initialized successfully"
   - Verify no errors during tag vocabulary loading

### Phase 2: Bulk Tag Editor Test

1. **Open Bulk Tag Editor**
   - Click "🏷️ Bulk Tag Editor" button in the header
   - Verify modal opens with two panels: file selection and tag editor

2. **File Selection Test**
   - Verify all loaded audio files appear in the left panel
   - Test individual file selection with checkboxes
   - Test "Select All Files" and "Clear Selection" buttons
   - Verify selection count updates correctly

3. **Tag Vocabulary Test**
   - Verify all 4 tag categories are displayed (Genre, Mood, Occasion, Keywords)
   - Verify correct tag chips are shown for each category
   - Test clicking tag chips to select/deselect them
   - Verify visual feedback (active state) when tags are selected

4. **Bulk Tagging Test**
   - Select multiple files (e.g., 2-3 files)
   - Select tags from different categories (e.g., "ambient" from Genre, "peaceful" from Mood)
   - Click "Apply Tags" button
   - Verify success in console logs
   - Verify file list refreshes showing updated tag summaries

### Phase 3: Tag Search & Filtering Test

1. **Locate Search Interface**
   - Verify "🏷️ RPG Tag Filters" section appears in the sidebar
   - Check that search mode toggle is present (Any tags OR / All tags AND)

2. **Basic Filtering Test**
   - Click on a genre tag chip (e.g., "ambient")
   - Verify the chip becomes active (green background)
   - Verify sound pads grid updates to show only files with that tag
   - Verify search results count updates

3. **Multi-Tag Filtering Test**
   - Select multiple tags from different categories
   - Test both "Any tags (OR)" and "All tags (AND)" modes
   - Verify different results for OR vs AND logic
   - Test "Clear All" button to reset filters

### Phase 4: Integration Test

1. **End-to-End Workflow**
   - Load audio files → Tag them with bulk editor → Search for them with filters
   - Verify the complete workflow works seamlessly
   - Test with different combinations of tags and search criteria

2. **Data Persistence Test**
   - Tag some files, close application, restart
   - Verify tags are persisted and search still works
   - Verify tag vocabulary is maintained

### Phase 5: Error Handling Test

1. **Invalid Operations**
   - Try to apply tags without selecting files
   - Try to apply tags without selecting any tags
   - Verify appropriate console warnings/errors

2. **Database Errors**
   - Check console for any database-related errors
   - Verify graceful degradation if tag service fails to initialize

## Expected Behavior

### Successful Operations Should Show:
- ✅ Clean console logs with operation confirmations
- ✅ Visual feedback in UI (active states, selection counts)
- ✅ Real-time updates to file lists and search results
- ✅ Smooth modal interactions without errors

### File List Should Display:
- ✅ File name (from title or filename)
- ✅ Artist information
- ✅ Tag summary (e.g., "genre: ambient | mood: peaceful")

### Search Results Should:
- ✅ Update immediately when filters change
- ✅ Show accurate count of matching files
- ✅ Hide non-matching files from the grid
- ✅ Preserve sound playback state during filtering

## Known Limitations

1. **Browser Performance**: With very large audio libraries (>1000 files), the UI may become slower
2. **Tag Vocabulary**: Currently uses predefined vocabulary; custom tags not supported in this version
3. **File Format Support**: Limited to common audio formats (MP3, WAV, OGG, FLAC, AAC, M4A, WMA, M4P)

## Troubleshooting

### Common Issues:

1. **Tags Not Appearing**
   - Check console for TagService initialization errors
   - Verify Rust backend compiled successfully
   - Check that database was created properly

2. **Bulk Tagging Fails**
   - Verify file selection count is > 0
   - Verify at least one tag is selected
   - Check console for backend error messages

3. **Search Not Working**
   - Verify files have been tagged first
   - Check that search interface loaded properly
   - Try clearing all filters and reselecting

### Console Commands for Debugging:

```javascript
// Check if TagService is loaded
window.app?.tagService?.loadedVocabulary

// Check current tag vocabulary
window.app?.tagService?.getAllVocabulary()

// Check selected files in bulk editor
window.app?.bulkTagEditorController?.selectedFiles

// Check active search filters
window.app?.tagSearchController?.getActiveFilters()
```

## Success Criteria

The RPG Audio Tagging System is working correctly if:

1. ✅ All audio files can be loaded and displayed
2. ✅ Bulk tag editor opens and allows file/tag selection
3. ✅ Tags can be applied to multiple files simultaneously
4. ✅ Tag search interface allows filtering by applied tags
5. ✅ Search results update in real-time and are accurate
6. ✅ All operations complete without errors in console
7. ✅ Tag data persists between application restarts

This system significantly enhances Ligeia's utility for RPG audio management by providing organized, searchable categorization of audio content.