use std::collections::HashSet;

pub mod genre_mappings;
pub mod mood_mappings; 
pub mod occasion_mappings;
pub mod keyword_mappings;

#[derive(Debug, Clone)]
pub struct TagFolderMapping {
    pub folder_assignments: Vec<String>,
}

/// Get all folder assignments for a set of tags
pub fn get_all_folders_for_tags(
    genre: Option<&str>,
    mood: &[&str],
    occasion: &[&str], 
    keywords: &[&str]
) -> TagFolderMapping {
    let mut folders = HashSet::new();
    
    // Genre folders
    if let Some(g) = genre {
        if let Some(genre_folders) = genre_mappings::lookup_genre_folders(g) {
            folders.extend(genre_folders.iter().map(|s| s.to_string()));
        }
    }
    
    // Mood folders  
    for m in mood {
        if let Some(mood_folders) = mood_mappings::lookup_mood_folders(m) {
            folders.extend(mood_folders.iter().map(|s| s.to_string()));
        }
    }
    
    // Occasion folders
    for o in occasion {
        if let Some(occasion_folders) = occasion_mappings::lookup_occasion_folders(o) {
            folders.extend(occasion_folders.iter().map(|s| s.to_string()));
        }
    }
    
    // Keyword folders
    for k in keywords {
        if let Some(keyword_folders) = keyword_mappings::lookup_keyword_folders(k) {
            folders.extend(keyword_folders.iter().map(|s| s.to_string()));
        }
    }
    
    TagFolderMapping {
        folder_assignments: folders.into_iter().collect()
    }
}

/// Get detailed folder assignments with metadata
#[derive(Debug, Clone)]
pub struct FolderAssignment {
    pub folder_path: String,
    pub assignment_reason: String,
    pub confidence: f32,
    pub assignment_type: AssignmentType,
}

#[derive(Debug, Clone)]
pub enum AssignmentType {
    Genre,
    Occasion,
    Keyword,
    Mood,
}

#[derive(Debug, Clone)]
pub struct DetailedTagFolderMapping {
    pub folder_assignments: Vec<FolderAssignment>,
}

/// Get detailed folder assignments with confidence and reasoning
pub fn get_detailed_folders_for_tags(
    genre: Option<&str>,
    mood: &[&str],
    occasion: &[&str], 
    keywords: &[&str]
) -> DetailedTagFolderMapping {
    let mut assignments = Vec::new();
    
    // Genre folders
    if let Some(g) = genre {
        if let Some(genre_folders) = genre_mappings::lookup_genre_folders(g) {
            for folder in genre_folders {
                assignments.push(FolderAssignment {
                    folder_path: folder.to_string(),
                    assignment_reason: format!("Genre: {}", g),
                    confidence: 1.0,
                    assignment_type: AssignmentType::Genre,
                });
            }
        }
    }
    
    // Mood folders  
    for m in mood {
        if let Some(mood_folders) = mood_mappings::lookup_mood_folders(m) {
            for folder in mood_folders {
                assignments.push(FolderAssignment {
                    folder_path: folder.to_string(),
                    assignment_reason: format!("Mood: {}", m),
                    confidence: 0.8,
                    assignment_type: AssignmentType::Mood,
                });
            }
        }
    }
    
    // Occasion folders
    for o in occasion {
        if let Some(occasion_folders) = occasion_mappings::lookup_occasion_folders(o) {
            for folder in occasion_folders {
                assignments.push(FolderAssignment {
                    folder_path: folder.to_string(),
                    assignment_reason: format!("Occasion: {}", o),
                    confidence: 0.9,
                    assignment_type: AssignmentType::Occasion,
                });
            }
        }
    }
    
    // Keyword folders
    for k in keywords {
        if let Some(keyword_folders) = keyword_mappings::lookup_keyword_folders(k) {
            for folder in keyword_folders {
                assignments.push(FolderAssignment {
                    folder_path: folder.to_string(),
                    assignment_reason: format!("Keyword: {}", k),
                    confidence: 0.8,
                    assignment_type: AssignmentType::Keyword,
                });
            }
        }
    }
    
    DetailedTagFolderMapping {
        folder_assignments: assignments
    }
}