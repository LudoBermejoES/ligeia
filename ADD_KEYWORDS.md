# Adding Keywords to Tag Mappings - Process Documentation

## Overview
This document describes the systematic process of adding unmapped tags from the Ligeia vocabulary to the tag mapping files for auto-organization functionality.

## Background
- **Goal**: Map vocabulary tags to virtual folder hierarchies for automatic file organization
- **Current Status**: 1,231 unmapped tags remaining (as of batch 6)
- **Mapping Files**: 19 files in `src-tauri/src/data/tag_mappings/`
- **Constraint**: Only use existing folder paths from `folder_structure.rs` - never create new ones

## Process Steps

### 1. Generate Unmapped Tags Report
```bash
node find-unmapped-tags.js
```
This script:
- Loads the audio library JSON file (`ligeia-library-2025-08-31.json`)
- Extracts all vocabulary tags from the database
- Reads all 19 mapping files in `tag_mappings/` directory
- Uses regex to parse Rust tuple format: `("tag", "folder_path", priority, "description")`
- Generates report showing unmapped tags by category

### 2. Batch Planning (100 tags per batch)
For each batch, select tags from these categories based on strategic importance:
1. **Genres** (COMPLETE - 100% coverage achieved)
2. **Moods** (~35% complete - 72 remaining)
3. **Occasions** (~20% complete - 132 remaining) 
4. **Keywords** (~17% complete - 1,027 remaining)
   - SFX keywords (sfx:*)
   - Location keywords (loc:*)
   - Creature keywords (creature:*)
   - Biome keywords (biome:*)
   - Tech keywords (tech:*)
   - Magic keywords (magic:*)
   - Timbre keywords (timbre:*)
   - Vehicle keywords (vehicle:*)
   - Weather keywords (weather:*)
   - Style keywords (style:*)
   - Utility keywords (util:*)

### 3. Mapping File Selection
Choose appropriate mapping files based on tag semantics:
- **folder_tag_environments.rs**: biome:*, loc:*, weather:* keywords
- **folder_tag_sfx.rs**: sfx:* keywords and sound-related tags
- **folder_tag_creatures.rs**: creature:* keywords
- **folder_tag_scifi.rs**: tech:* keywords, electronic/sci-fi genres
- **folder_tag_magic.rs**: magic:* keywords
- **folder_tag_instruments.rs**: timbre:* keywords, instrument-related tags
- **folder_tag_moods.rs**: mood tags and emotional descriptors
- **folder_tag_session.rs**: occasion tags, util:* keywords
- **folder_tag_combat.rs**: combat-related genres and occasions
- **folder_tag_cultural.rs**: style:* keywords, historical/cultural tags
- **folder_tag_fantasy.rs**: fantasy-related tags
- **folder_tag_horror.rs**: horror/dark themes

### 4. Folder Path Verification
**CRITICAL**: Always use existing paths from `folder_structure.rs`. Never create new folder hierarchies.

Common existing paths by category:
- **Environments**: `Environments/Natural/Forest/Ancient Forest`, `Environments/Urban/Cities/Markets`
- **SFX**: `SFX & Foley/Combat Sounds/Weapon Impacts/Sword Clashing`, `SFX & Foley/Environmental Sounds/Water`
- **Creatures**: `Creatures/Beasts/Predators/Big Cats`, `Creatures/Undead/Lesser/Zombies`
- **Combat**: `Combat/Victory & Defeat/Last Stand`, `Combat/Combat Phases/Siege`
- **Magic**: `Magic & Powers/Schools of Magic/Evocation/Fire`
- **Session**: `Session Structure/Adventure Sequences/Travel`

### 5. Tag Addition Format
Add tags in Rust tuple format before the closing `]`:
```rust
("tag-name", "Existing/Folder/Path", priority, "Description"),
```

**Priority Guidelines**:
- 10: Primary/exact match for folder
- 9: Strong secondary match
- 8: Good contextual fit
- 7: Reasonable alternative placement

### 6. Multi-Contextual Placement
**Important**: Tags can and should appear in multiple mapping files when contextually relevant.

Example: `"sfx:sword-clash"` could appear in:
- `folder_tag_sfx.rs` (primary SFX context)
- `folder_tag_combat.rs` (combat context)
- `folder_tag_fantasy.rs` (fantasy context)

### 7. Update and Verify
After each batch:
1. Run `node find-unmapped-tags.js` to generate updated report
2. Verify tags were correctly added (should see reduction in unmapped count)
3. Update progress tracking

## Tools and Scripts

### find-unmapped-tags.js
- **Location**: `C:\code\ligeia\find-unmapped-tags.js`
- **Purpose**: Compare vocabulary tags against mapping files
- **Output**: Console report + `unmapped-tags-report.json`

### Key Files
- **Vocabulary Source**: `ligeia-library-2025-08-31.json`
- **Folder Structure**: `src-tauri/src/data/folder_structure.rs`
- **Mapping Files**: `src-tauri/src/data/tag_mappings/*.rs`

## Progress Tracking

### Completed Batches
1. **Batch 1**: Genre tags (ambient, electronic, sound-design, orchestral, horror, folk)
2. **Batch 2**: More genre variants (sci-fi, fantasy, jazz, metal, western, mythic)
3. **Batch 3**: Completed remaining genres + started moods and occasions
4. **Batch 4**: More moods, occasions, and keyword categories (creatures, locations, SFX)
5. **Batch 5**: Biomes, timbres, tech, magic keywords + more moods (100 tags)
6. **Batch 6**: Essential SFX, locations, creatures, utilities (73 tags)

### Current Status (After Batch 6)
- **Total Mapped**: 2,185 tags
- **Total Unmapped**: 1,231 tags
- **Genres**: 100% Complete ✅
- **Moods**: ~35% Complete (72 remaining)
- **Occasions**: ~20% Complete (132 remaining)
- **Keywords**: ~17% Complete (1,027 remaining)

## Next Steps Suggestions

### Batch 7 Priority
1. **More SFX Keywords** (~30 tags): Focus on common sound effects still unmapped
2. **Vehicle Keywords** (~25 tags): Add vehicle:* tags to appropriate contexts
3. **Style Keywords** (~20 tags): Add style:* tags to cultural mapping
4. **Weather Keywords** (~15 tags): Add weather:* tags to environments
5. **More Location Keywords** (~10 tags): Continue with loc:* tags

### Long-term Strategy
1. **Keywords Category**: Largest remaining category (1,027 tags)
   - Focus on high-frequency keyword prefixes (sfx:, loc:, creature:)
   - Systematic coverage of all keyword subcategories
2. **Occasions Category**: 132 remaining RPG scenario tags
   - Add combat-related occasions to combat mapping
   - Add social occasions to session mapping
3. **Moods Category**: 72 remaining emotional descriptors
   - Distribute across thematic mapping files based on context

## Important Constraints

### DO NOT:
- Create new folder paths not in `folder_structure.rs`
- Ignore multi-contextual placement opportunities
- Batch more than ~100 tags at once (manageable chunks)
- Skip verification steps after each batch

### DO:
- Always verify folder paths exist in `folder_structure.rs`
- Place tags in multiple relevant mapping files
- Use appropriate priority levels (7-10)
- Write descriptive comments for tag groups
- Track progress systematically
- Run verification script after each batch

## Recovery Instructions

If the process is interrupted:
1. Run `node find-unmapped-tags.js` to see current status
2. Check the last completed batch in this documentation
3. Continue with the next logical batch based on remaining categories
4. Prioritize high-impact categories (SFX, locations, creatures) for keywords
5. Maintain the 70-100 tags per batch target for manageable progress

## File Locations Summary
- **Process Doc**: `C:\code\ligeia\ADD_KEYWORDS.md` (this file)
- **Analysis Script**: `C:\code\ligeia\find-unmapped-tags.js`
- **Report Output**: `C:\code\ligeia\unmapped-tags-report.json`
- **Mapping Files**: `C:\code\ligeia\src-tauri\src\data\tag_mappings\*.rs`
- **Folder Structure**: `C:\code\ligeia\src-tauri\src\data\folder_structure.rs`
- **Library Data**: `C:\code\ligeia\ligeia-library-2025-08-31.json`