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
    
    // Genre folders (now returns tuples (folder_path, confidence))
    if let Some(g) = genre {
        if let Some(genre_folders) = genre_mappings::lookup_genre_folders(g) {
            folders.extend(genre_folders.iter().map(|(folder_path, _confidence)| folder_path.to_string()));
        }
    }
    
    // Mood folders  
    for m in mood {
        if let Some(mood_folders) = mood_mappings::lookup_mood_folders(m) {
            folders.extend(mood_folders.iter().map(|(folder_path, _confidence)| folder_path.to_string()));
        }
    }
    
    // Occasion folders
    for o in occasion {
        if let Some(occasion_folders) = occasion_mappings::lookup_occasion_folders(o) {
            folders.extend(occasion_folders.iter().map(|(folder_path, _confidence)| folder_path.to_string()));
        }
    }
    
    // Keyword folders
    for k in keywords {
        if let Some(keyword_folders) = keyword_mappings::lookup_keyword_folders(k) {
            folders.extend(keyword_folders.iter().map(|(folder_path, _confidence)| folder_path.to_string()));
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
    
    // Genre folders with actual confidence scores
    if let Some(g) = genre {
        if let Some(genre_folders) = genre_mappings::lookup_genre_folders(g) {
            for (folder_path, confidence) in genre_folders {
                assignments.push(FolderAssignment {
                    folder_path: folder_path.to_string(),
                    assignment_reason: format!("Genre: {}", g),
                    confidence: (*confidence as f32) / 10.0, // Convert 5-10 scale to 0.5-1.0
                    assignment_type: AssignmentType::Genre,
                });
            }
        }
    }
    
    // Mood folders with actual confidence scores
    for m in mood {
        if let Some(mood_folders) = mood_mappings::lookup_mood_folders(m) {
            for (folder_path, confidence) in mood_folders {
                assignments.push(FolderAssignment {
                    folder_path: folder_path.to_string(),
                    assignment_reason: format!("Mood: {}", m),
                    confidence: (*confidence as f32) / 10.0, // Convert 5-10 scale to 0.5-1.0
                    assignment_type: AssignmentType::Mood,
                });
            }
        }
    }
    
    // Occasion folders with actual confidence scores
    for o in occasion {
        if let Some(occasion_folders) = occasion_mappings::lookup_occasion_folders(o) {
            for (folder_path, confidence) in occasion_folders {
                assignments.push(FolderAssignment {
                    folder_path: folder_path.to_string(),
                    assignment_reason: format!("Occasion: {}", o),
                    confidence: (*confidence as f32) / 10.0, // Convert 5-10 scale to 0.5-1.0
                    assignment_type: AssignmentType::Occasion,
                });
            }
        }
    }
    
    // Keyword folders with actual confidence scores
    for k in keywords {
        if let Some(keyword_folders) = keyword_mappings::lookup_keyword_folders(k) {
            for (folder_path, confidence) in keyword_folders {
                assignments.push(FolderAssignment {
                    folder_path: folder_path.to_string(),
                    assignment_reason: format!("Keyword: {}", k),
                    confidence: (*confidence as f32) / 10.0, // Convert 5-10 scale to 0.5-1.0
                    assignment_type: AssignmentType::Keyword,
                });
            }
        }
    }
    
    DetailedTagFolderMapping {
        folder_assignments: assignments
    }
}