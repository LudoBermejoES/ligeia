# Tag-Folder Mapping Approach for Ligeia

## Analysis Summary

After analyzing the current data files, I've identified the need to leverage our **virtual folder system** to support **many-to-many relationships** between our **simplified 6-category folder structure** (247 folders) and our **comprehensive tag vocabulary** (875+ tags across genres, moods, occasions, keywords).

### Current Data Structure

**Virtual Folders**: 6 main categories → 3-level hierarchy → 247 total folders
- 🎵 Music (6 subcategories, 47 specific folders)
- 🎬 SFX (6 subcategories, 90 specific folders) 
- 🌍 Environments (6 subcategories, 44 specific folders)
- ⚔️ Combat (4 subcategories, 32 specific folders)
- 🗣️ Social (5 subcategories, 30 specific folders)
- ✨ Magic (6 subcategories, 38 specific folders)

**Tags**: 875+ specific tags across 4 dimensions
- 105 Genre tags (orchestral:cinematic, horror:psychological, etc.)
- 99 Mood tags (heroic, mysterious, aggressive, etc.)
- 297 Occasion tags (combat-skirmish, tavern, ritual, etc.)
- 374 Keywords (biome:forest, creature:dragon, sfx:sword-clash, etc.)

## The Solution: Comprehensive Tag-to-Folder Mapping

**MANDATORY REQUIREMENT**: Every single tag (all 875+ tags) must be explicitly mapped to at least one virtual folder from the folder_structure. No fallbacks, no guessing - every tag gets specific folder assignments.

**Core Principle**: Audio files are **automatically placed in ALL relevant virtual folders** based on their tags, with each tag having predetermined folder assignments.

**Example**: An epic orchestral boss battle track with tags:
- Genre: `orchestral:cinematic` → `Music/Orchestral/Epic Orchestral`
- Mood: `heroic` → `Combat/Combat Phases/Victory` + `Social/Entertainment/Theater`
- Mood: `triumphant` → `Combat/Combat Phases/Victory` + `Social/Ceremonies/Coronations`
- Occasion: `boss-intro` → `Combat/Combat Phases/Pre-Battle`
- Occasion: `combat-skirmish` → `Combat/Battle Ambience/Battlefield`
- Keywords: `creature:dragon` → `Combat/Monster Combat/Dragon Fights` + `Magic/Magical Creatures/Dragons`
- Keywords: `loc:castle` → `Environments/Settlements/Castles`

**Result**: This file appears in 8 different virtual folders automatically, discoverable from multiple organizational perspectives.

## Proposed Solution: Comprehensive Multi-Folder Assignment

### 1. **Multi-Folder Membership** (Many-to-Many mapping)
Each audio file gets placed in **ALL matching folders** based on comprehensive tag analysis:

```rust
fn determine_all_matching_folders(
    genre: &str, 
    mood: &[&str],
    occasion: &[&str], 
    keywords: &[&str]
) -> Vec<FolderAssignment> {
    let mut folders = Vec::new();
    
    // Genre-based folders
    folders.extend(map_genre_to_folders(genre));
    
    // Mood-based folders (when mood strongly suggests context)
    folders.extend(map_mood_to_folders(mood));
    
    // Occasion-based folders  
    folders.extend(map_occasion_to_folders(occasion));
    
    // Keyword-based folders
    folders.extend(map_keywords_to_folders(keywords));
    
    folders
}
```

### 2. **Smart Mapping Rules with Confidence Levels**

#### **Music Folder Mapping**
```rust
Genre Tag → Music Subfolder
orchestral:cinematic → Music/Orchestral/Epic Orchestral
orchestral:dark → Music/Orchestral/Dark Orchestral
electronic:synthwave → Music/Electronic/Synthwave  
horror:psychological → Music/Horror & Tension/Psychological
jazz:noir → Music/Jazz & Blues/Noir Jazz
folk:celtic → Music/Folk & World/Celtic
```

#### **SFX Folder Mapping**  
```rust
Genre/Keyword → SFX Subfolder
sound-design:weapons → SFX/Weapons/[weapon-type from keywords]
sfx:sword-clash → SFX/Weapons/Melee Weapons
sfx:magic-whoosh → SFX/Magical Effects/Spell Casting
sfx:footsteps → SFX/Movement/Footsteps
sound-design:impacts → SFX/Impacts & Crashes/[material from keywords]
```

#### **Context Folder Mapping**
```rust
Occasion + Keywords → Context Folder
combat-* occasions → Combat/[appropriate subcategory]
tavern occasion → Social/Conversations/Tavern Chatter
biome:forest keywords → Environments/Natural Landscapes/Forests
element:fire keywords → Magic/Elemental Magic/Fire Magic
```

### 3. **Comprehensive Virtual Folder Assignment**

**Multi-Folder Membership**: Files are automatically placed in ALL relevant virtual folders:

```rust
pub struct TagFolderMapping {
    pub folder_assignments: Vec<FolderAssignment>,
}

pub struct FolderAssignment {
    pub folder_path: String,
    pub assignment_reason: String,     // "Genre: orchestral:cinematic"
    pub confidence: f32,               // 0.0-1.0 relevance score
    pub assignment_type: AssignmentType,
}

pub enum AssignmentType {
    Genre,        // Based on musical/audio genre
    Occasion,     // Based on RPG use case
    Keyword,      // Based on specific descriptors
    Mood,         // Based on emotional context (when strongly contextual)
    Contextual,   // Multiple tags combine for assignment
}
```

**Example**:
```rust
// Epic orchestral boss music with heroic mood, dragon fight
TagFolderMapping {
    folder_assignments: vec![
        FolderAssignment {
            folder_path: "Music/Orchestral/Epic Orchestral",
            assignment_reason: "Genre: orchestral:cinematic",
            confidence: 1.0,
            assignment_type: AssignmentType::Genre,
        },
        FolderAssignment {
            folder_path: "Combat/Combat Phases/Pre-Battle", 
            assignment_reason: "Occasion: boss-intro",
            confidence: 0.9,
            assignment_type: AssignmentType::Occasion,
        },
        FolderAssignment {
            folder_path: "Combat/Monster Combat/Dragon Fights",
            assignment_reason: "Creature: dragon",
            confidence: 0.8,
            assignment_type: AssignmentType::Keyword,
        },
        FolderAssignment {
            folder_path: "Environments/Settlements/Castles",
            assignment_reason: "Location: castle",
            confidence: 0.7,
            assignment_type: AssignmentType::Keyword,
        },
    ]
}
```

### 4. **Implementation Strategy**

#### **Phase 1: Comprehensive Auto-Assignment**
Files are automatically added to ALL matching virtual folders based on their tags:

```rust
// tag_mappings.rs structure
pub const TAG_FOLDER_RULES: &[FolderMappingRule] = &[
    // Every tag can map to multiple folders with different confidence levels
    FolderMappingRule { 
        tag_pattern: "orchestral:cinematic", 
        folder_pattern: "Music/Orchestral/Epic Orchestral", 
        confidence: 1.0, assignment_type: AssignmentType::Genre 
    },
    FolderMappingRule { 
        tag_pattern: "boss-intro", 
        folder_pattern: "Combat/Combat Phases/Pre-Battle", 
        confidence: 0.9, assignment_type: AssignmentType::Occasion 
    },
    FolderMappingRule { 
        tag_pattern: "creature:dragon", 
        folder_pattern: "Combat/Monster Combat/Dragon Fights", 
        confidence: 0.8, assignment_type: AssignmentType::Keyword 
    },
];
```

#### **Phase 2: Confidence-Based Filtering** 
Allow users to set minimum confidence thresholds:
- "Show files in folders with >0.7 confidence"
- "Include all assignments >0.5 confidence"

#### **Phase 3: User Override System**
- Users can manually add/remove files from specific folders
- System learns from manual assignments
- Override preferences are preserved

## Benefits

1. **🎯 Complete Discoverability**: Files appear in ALL relevant folders - find by genre, occasion, creature, location, etc.
2. **🔍 Multiple Discovery Paths**: Same file accessible from different organizational perspectives  
3. **🔄 True Many-to-Many**: Leverages virtual folder system for natural cross-referencing
4. **📈 Scalable**: Adding new folders/tags creates more connections automatically
5. **🎮 RPG-Optimized**: Multi-dimensional organization matches complex RPG audio needs
6. **💡 Contextual Organization**: Files grouped by use case, not just content type
7. **🎚️ Confidence-Based**: Users can filter by relevance threshold

## Mandatory Implementation: Complete Tag Mappings

### REQUIREMENT: 100% Tag Coverage
- **105 Genre tags** → Each mapped to 1-3 specific folders
- **99 Mood tags** → Each mapped to 1-4 relevant folders  
- **297 Occasion tags** → Each mapped to 1-2 context folders
- **374 Keyword tags** → Each mapped to 1-3 descriptive folders
- **Total: 875+ explicit tag-to-folder mappings**

### File Structure: `src-tauri/src/data/tag_mappings/`

```
tag_mappings/
├── mod.rs                    // Public API and aggregation
├── genre_mappings.rs         // All 105 genre tags → folders
├── mood_mappings.rs          // All 99 mood tags → folders  
├── occasion_mappings.rs      // All 297 occasion tags → folders
└── keyword_mappings.rs       // All 374 keyword tags → folders
```

### Data Structure Per File:
```rust
// genre_mappings.rs example
pub const GENRE_FOLDER_MAPPINGS: &[(&str, &[&str])] = &[
    ("orchestral:cinematic", &["Music/Orchestral/Epic Orchestral"]),
    ("orchestral:dark", &["Music/Orchestral/Dark Orchestral", "Music/Horror & Tension/Gothic"]),
    ("horror:psychological", &["Music/Horror & Tension/Psychological"]),
    ("sound-design:weapons", &["SFX/Weapons"]),
    // ... ALL 105 genre tags explicitly mapped
];

// mood_mappings.rs example  
pub const MOOD_FOLDER_MAPPINGS: &[(&str, &[&str])] = &[
    ("heroic", &["Combat/Combat Phases/Victory", "Social/Entertainment/Theater"]),
    ("mysterious", &["Environments/Dungeons & Ruins", "Magic/Magical Environments"]),
    ("aggressive", &["Combat/Battle Ambience/Battlefield", "SFX/Weapons"]),
    // ... ALL 99 mood tags explicitly mapped
];

// occasion_mappings.rs example
pub const OCCASION_FOLDER_MAPPINGS: &[(&str, &[&str])] = &[
    ("boss-intro", &["Combat/Combat Phases/Pre-Battle"]),
    ("tavern", &["Social/Conversations/Tavern Chatter", "Environments/Settlements/Taverns"]),
    ("combat-skirmish", &["Combat/Battle Ambience/Battlefield"]),
    // ... ALL 297 occasion tags explicitly mapped
];

// keyword_mappings.rs example
pub const KEYWORD_FOLDER_MAPPINGS: &[(&str, &[&str])] = &[
    ("creature:dragon", &["Combat/Monster Combat/Dragon Fights", "Magic/Magical Creatures/Dragons"]),
    ("biome:forest", &["Environments/Natural Landscapes/Forests"]),
    ("sfx:sword-clash", &["SFX/Weapons/Melee Weapons"]),
    ("loc:castle", &["Environments/Settlements/Castles"]),
    // ... ALL 374 keyword tags explicitly mapped
];
```

### Aggregation API (`mod.rs`):
```rust
use std::collections::HashMap;

pub mod genre_mappings;
pub mod mood_mappings; 
pub mod occasion_mappings;
pub mod keyword_mappings;

pub struct TagFolderMapping {
    pub folder_assignments: Vec<String>,
}

// Build complete mapping lookup
pub fn get_all_folders_for_tags(
    genre: Option<&str>,
    mood: &[&str],
    occasion: &[&str], 
    keywords: &[&str]
) -> TagFolderMapping {
    let mut folders = std::collections::HashSet::new();
    
    // Genre folders
    if let Some(g) = genre {
        if let Some(genre_folders) = lookup_genre_folders(g) {
            folders.extend(genre_folders.iter().map(|s| s.to_string()));
        }
    }
    
    // Mood folders  
    for m in mood {
        if let Some(mood_folders) = lookup_mood_folders(m) {
            folders.extend(mood_folders.iter().map(|s| s.to_string()));
        }
    }
    
    // Occasion folders
    for o in occasion {
        if let Some(occasion_folders) = lookup_occasion_folders(o) {
            folders.extend(occasion_folders.iter().map(|s| s.to_string()));
        }
    }
    
    // Keyword folders
    for k in keywords {
        if let Some(keyword_folders) = lookup_keyword_folders(k) {
            folders.extend(keyword_folders.iter().map(|s| s.to_string()));
        }
    }
    
    TagFolderMapping {
        folder_assignments: folders.into_iter().collect()
    }
}
```

## Migration Strategy

1. **Create new tag_mappings.rs** with rule-based system
2. **Test with existing audio files** to validate mappings
3. **Generate reports** showing mapping distribution
4. **Fine-tune rules** based on results
5. **Add suggestion system** for cross-references

This approach balances **browsability** (simple folder structure) with **precision** (comprehensive tagging) while supporting the many-to-many relationships you requested.