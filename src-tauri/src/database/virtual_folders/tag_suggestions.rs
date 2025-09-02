use rusqlite::{Connection, Result, params};
use crate::models::{VirtualFolder, AudioFile, AutoOrganizationSuggestion};

/// Tag-based folder suggestion operations
/// 
/// CONFIDENCE SCORING SYSTEM (Updated):
/// - Individual tag mappings use 5-10 confidence scale (10 = perfect match)
/// - Multiple tags can contribute to the same folder, scores are summed
/// - THRESHOLD: Only folders with cumulative confidence >= 8 are included
/// 
/// Examples:
/// - Single tag "portal-opening" -> Magic/Magical Environments/Portals (confidence=10) ✓ INCLUDED (10 >= 8)
/// - Two tags both mapping to same folder with confidence=7 each -> Total=14 ✓ INCLUDED (14 >= 8)  
/// - Single tag with confidence=7 -> Total=7 ✗ EXCLUDED (7 < 8)
/// 
pub struct VirtualFolderTagSuggestions;

impl VirtualFolderTagSuggestions {
    /// Get folder suggestions for a file based on its RPG tags using enhanced mapping system
    pub fn suggest_folders_for_file(conn: &Connection, audio_file_id: i64, limit: Option<usize>) -> Result<Vec<(VirtualFolder, f64)>> {
        use crate::data::tag_mappings;
        
        let limit = limit.unwrap_or(5);
        
        // Get all tags for the file
        let file_tags = Self::get_file_tags(conn, audio_file_id)?;
        
        if file_tags.is_empty() {
            return Ok(Vec::new());
        }
        
        // Parse tags into categories
        let mut genre_tag: Option<String> = None;
        let mut mood_tags: Vec<String> = Vec::new();
        let mut occasion_tags: Vec<String> = Vec::new();
        let mut keyword_tags: Vec<String> = Vec::new();
        
        for tag in &file_tags {
            // Tags come in format "tag_type:tag_value" from database
            // We need to handle both simple tags (mood:dark) and prefixed tags (creature:dragon)
            if let Some(colon_pos) = tag.find(':') {
                let tag_type = &tag[..colon_pos];
                let tag_value = &tag[colon_pos + 1..];
                
                match tag_type {
                    "genre" => {
                        if genre_tag.is_none() {
                            genre_tag = Some(tag_value.to_string());
                        }
                    },
                    "mood" => mood_tags.push(tag_value.to_string()),
                    "occasion" => occasion_tags.push(tag_value.to_string()),
                    // Handle both "keyword" and "keywords" for compatibility
                    "keyword" | "keywords" => {
                        // For keyword tags, we want to keep the full prefixed value
                        // e.g., "creature:dragon" not just "dragon"
                        keyword_tags.push(tag_value.to_string());
                    },
                    _ => {}
                }
            }
        }
        
        // Get folder mappings using the new system
        let genre_ref = genre_tag.as_deref();
        let mood_refs: Vec<&str> = mood_tags.iter().map(|s| s.as_str()).collect();
        let occasion_refs: Vec<&str> = occasion_tags.iter().map(|s| s.as_str()).collect();
        let keyword_refs: Vec<&str> = keyword_tags.iter().map(|s| s.as_str()).collect();
        
        let detailed_mappings = tag_mappings::get_detailed_folders_for_tags(
            genre_ref,
            &mood_refs,
            &occasion_refs,
            &keyword_refs
        );
        
        let mut folder_suggestions: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
        
        // Process folder assignments with NEW CONFIDENCE SCORING SYSTEM:
        // - Each tag mapping has confidence 5-10 (where 10 = perfect match)
        // - Multiple tags can map to same folder, scores are SUMMED
        // - Only folders with cumulative score >= 8 are included
        // 
        // Examples:
        // - "portal-opening" -> Magic/Magical Environments/Portals (conf=10) -> TOTAL=10 ✓ INCLUDED
        // - "creature:dragon" + "mood:fierce" both -> Combat/Dragon (conf=7+7) -> TOTAL=14 ✓ INCLUDED  
        // - "ambient" -> Music/Ambient (conf=7) -> TOTAL=7 ✗ EXCLUDED
        
        for assignment in detailed_mappings.folder_assignments {
            // Convert normalized confidence (0.5-1.0) back to 5-10 scale for threshold logic
            let raw_confidence = assignment.confidence * 10.0;
            
            folder_suggestions.entry(assignment.folder_path.clone())
                .and_modify(|total_score| *total_score += raw_confidence as f64)
                .or_insert(raw_confidence as f64);
        }
        
        // Apply 8+ threshold: only keep folders with cumulative confidence >= 8
        folder_suggestions.retain(|_, &mut score| score >= 8.0);
        
        // Convert folder paths to actual folder objects and scores
        let mut folder_scores: Vec<(VirtualFolder, f64)> = Vec::new();
        
        for (folder_path, raw_score) in folder_suggestions {
            if let Some(folder) = Self::find_folder_by_path(conn, &folder_path)? {
                // Only include leaf folders (folders without children)
                if let Some(folder_id) = folder.id {
                    use crate::database::virtual_folders::hierarchy_ops::VirtualFolderHierarchy;
                    let children = VirtualFolderHierarchy::get_folder_children(conn, Some(folder_id))?;
                    if children.is_empty() {
                        // Convert raw confidence score (8-20+ range) to normalized score (0.8-1.0+ range)
                        // Cap at 1.0 for display purposes
                        let normalized_score = (raw_score / 10.0).min(1.0);
                        folder_scores.push((folder, normalized_score));
                    }
                }
            }
        }
        
        // Fallback to original algorithm for folders not covered by mappings
        if folder_scores.len() < limit {
            let additional_suggestions = Self::calculate_fallback_suggestions(conn, &file_tags, limit * 2)?; // Get more to filter
            for (folder, score) in additional_suggestions {
                // Only add if not already in suggestions, score is decent, and it's a leaf folder
                if score > 0.3 && !folder_scores.iter().any(|(f, _)| f.id == folder.id) {
                    // Check if it's a leaf folder
                    if let Some(folder_id) = folder.id {
                        use crate::database::virtual_folders::hierarchy_ops::VirtualFolderHierarchy;
                        let children = VirtualFolderHierarchy::get_folder_children(conn, Some(folder_id))?;
                        if children.is_empty() {
                            folder_scores.push((folder, score * 0.8)); // Reduce fallback scores slightly
                        }
                    }
                }
                
                // Stop if we have enough suggestions
                if folder_scores.len() >= limit {
                    break;
                }
            }
        }
        
        // Sort by score descending and limit results
        folder_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        folder_scores.truncate(limit);
        
        Ok(folder_scores)
    }
    
    /// Check if a file tag matches a pattern (supports exact match or prefix match)
    fn tag_matches(file_tag: &str, pattern: &str) -> bool {
        if pattern.contains(':') {
            // Exact tag match
            file_tag == pattern
        } else {
            // Check if file tag starts with pattern (for category matching)
            file_tag.starts_with(&format!("{}:", pattern))
        }
    }
    
    /// Get weight for tag type from TAG_WEIGHTS
    fn get_tag_type_weight(tag: &str, weights: &[(&str, u8)]) -> u8 {
        if let Some(colon_pos) = tag.find(':') {
            let tag_type = &tag[..colon_pos];
            
            // Check for specific prefixes first (like keyword:loc)
            for (weight_pattern, weight) in weights {
                if tag.starts_with(weight_pattern) {
                    return *weight;
                }
            }
            
            // Check for general category match
            for (weight_pattern, weight) in weights {
                if tag_type == *weight_pattern {
                    return *weight;
                }
            }
        }
        3 // Default weight for unrecognized tags
    }
    
    /// Find folder by hierarchical path (e.g., "Combat/Weapons/Melee/Swords")
    fn find_folder_by_path(conn: &Connection, path: &str) -> Result<Option<VirtualFolder>> {
        let parts: Vec<&str> = path.split('/').collect();
        if parts.is_empty() {
            return Ok(None);
        }
        
        let mut current_parent: Option<i64> = None;
        
        // Walk through each level of the hierarchy
        for (i, part) in parts.iter().enumerate() {
            let mut stmt = if current_parent.is_some() {
                conn.prepare(
                    "SELECT id, name, description, parent_folder_id, color, icon, 
                            created_at, updated_at, created_by, folder_order, is_system_folder, metadata
                     FROM virtual_folders 
                     WHERE name = ? AND parent_folder_id = ?
                     LIMIT 1"
                )?
            } else {
                conn.prepare(
                    "SELECT id, name, description, parent_folder_id, color, icon, 
                            created_at, updated_at, created_by, folder_order, is_system_folder, metadata
                     FROM virtual_folders 
                     WHERE name = ? AND parent_folder_id IS NULL
                     LIMIT 1"
                )?
            };
            
            let folder_result = if current_parent.is_some() {
                stmt.query_row(params![part, current_parent], |row| {
                    Ok(VirtualFolder {
                        id: Some(row.get(0)?),
                        name: row.get(1)?,
                        description: row.get(2)?,
                        parent_folder_id: row.get(3)?,
                        color: row.get(4)?,
                        icon: row.get(5)?,
                        created_at: row.get(6)?,
                        updated_at: row.get(7)?,
                        created_by: row.get(8)?,
                        folder_order: row.get(9)?,
                        is_system_folder: row.get(10)?,
                        metadata: row.get(11)?,
                    })
                })
            } else {
                stmt.query_row(params![part], |row| {
                    Ok(VirtualFolder {
                        id: Some(row.get(0)?),
                        name: row.get(1)?,
                        description: row.get(2)?,
                        parent_folder_id: row.get(3)?,
                        color: row.get(4)?,
                        icon: row.get(5)?,
                        created_at: row.get(6)?,
                        updated_at: row.get(7)?,
                        created_by: row.get(8)?,
                        folder_order: row.get(9)?,
                        is_system_folder: row.get(10)?,
                        metadata: row.get(11)?,
                    })
                })
            };
            
            match folder_result {
                Ok(folder) => {
                    if i == parts.len() - 1 {
                        // This is the final folder we're looking for
                        return Ok(Some(folder));
                    } else {
                        // Continue to next level
                        current_parent = folder.id;
                    }
                }
                Err(_e) => {
                    // Folder not found at this level - this is normal, not an error
                    return Ok(None);
                }
            }
        }
        
        Ok(None)
    }
    
    /// Fallback suggestion algorithm (original Jaccard similarity)
    fn calculate_fallback_suggestions(conn: &Connection, file_tags: &[String], limit: usize) -> Result<Vec<(VirtualFolder, f64)>> {
        let mut folder_scores: Vec<(VirtualFolder, f64)> = Vec::new();
        use crate::database::virtual_folders::hierarchy_ops::VirtualFolderHierarchy;
        let folders = VirtualFolderHierarchy::get_all_virtual_folders(conn)?;
        
        for folder in folders {
            let score = Self::calculate_folder_tag_score(conn, folder.id.unwrap(), file_tags)?;
            if score > 0.0 {
                folder_scores.push((folder, score));
            }
        }
        
        // Sort by score descending and limit results
        folder_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        folder_scores.truncate(limit);
        
        Ok(folder_scores)
    }
    
    /// Calculate similarity score between a folder and a set of tags (original algorithm)
    fn calculate_folder_tag_score(conn: &Connection, folder_id: i64, file_tags: &[String]) -> Result<f64> {
        // Get all tags from files currently in this folder
        let mut stmt = conn.prepare(
            "SELECT DISTINCT rt.tag_type, rt.tag_value
             FROM rpg_tags rt
             JOIN virtual_folder_contents vfc ON rt.audio_file_id = vfc.audio_file_id
             WHERE vfc.folder_id = ?"
        )?;
        
        let folder_tags: Vec<String> = stmt.query_map([folder_id], |row| {
            let tag_type: String = row.get(0)?;
            let tag_value: String = row.get(1)?;
            Ok(format!("{}:{}", tag_type, tag_value))
        })?.collect::<Result<Vec<_>, _>>()?;
        
        if folder_tags.is_empty() {
            return Ok(0.0);
        }
        
        // Calculate Jaccard similarity coefficient
        let file_tags_set: std::collections::HashSet<&String> = file_tags.iter().collect();
        let folder_tags_set: std::collections::HashSet<&String> = folder_tags.iter().collect();
        
        let intersection = file_tags_set.intersection(&folder_tags_set).count();
        let union = file_tags_set.union(&folder_tags_set).count();
        
        if union == 0 {
            Ok(0.0)
        } else {
            Ok(intersection as f64 / union as f64)
        }
    }
    
    /// Get all RPG tags for a file in "type:value" format
    fn get_file_tags(conn: &Connection, audio_file_id: i64) -> Result<Vec<String>> {
        let mut stmt = conn.prepare(
            "SELECT tag_type, tag_value FROM rpg_tags WHERE audio_file_id = ?"
        )?;
        
        let tags: Vec<String> = stmt.query_map([audio_file_id], |row| {
            let tag_type: String = row.get(0)?;
            let tag_value: String = row.get(1)?;
            Ok(format!("{}:{}", tag_type, tag_value))
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(tags)
    }
    
    /// Get auto-organization suggestions based on tag patterns
    pub fn get_auto_organization_suggestions(conn: &Connection, threshold: f64) -> Result<Vec<AutoOrganizationSuggestion>> {
        let mut suggestions = Vec::new();
        
        // Find files not in any folder
        let unorganized_files = Self::get_unorganized_files(conn)?;
        
        for file in unorganized_files {
            let folder_suggestions = Self::suggest_folders_for_file(conn, file.id.unwrap(), Some(3))?;
            
            // Only suggest if confidence is above threshold (converting back to normalized scale)
            if let Some((folder, score)) = folder_suggestions.first() {
                let normalized_score = (*score / 10.0).min(1.0); // Convert back to 0-1 scale, cap at 1.0
                if normalized_score >= threshold {
                    suggestions.push(AutoOrganizationSuggestion {
                        audio_file_id: file.id.unwrap(),
                        audio_file_title: file.title.unwrap_or_else(|| "Unknown".to_string()),
                        suggested_folder_id: folder.id.unwrap(),
                        suggested_folder_name: folder.name.clone(),
                        confidence_score: normalized_score,
                        matching_tags: Self::get_matching_tags(conn, file.id.unwrap(), folder.id.unwrap())?,
                    });
                }
            }
        }
        
        // Sort by confidence score descending
        suggestions.sort_by(|a, b| b.confidence_score.partial_cmp(&a.confidence_score).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(suggestions)
    }
    
    /// Get files that are not in any virtual folder
    fn get_unorganized_files(conn: &Connection) -> Result<Vec<AudioFile>> {
        let mut stmt = conn.prepare(
            "SELECT af.* FROM audio_files af
             LEFT JOIN virtual_folder_contents vfc ON af.id = vfc.audio_file_id
             WHERE vfc.audio_file_id IS NULL"
        )?;
        
        let files = stmt.query_map([], |row| {
            Ok(AudioFile {
                id: Some(row.get(0)?),
                file_path: row.get(1)?,
                title: row.get(2)?,
                artist: row.get(3)?,
                album: row.get(4)?,
                duration: row.get(5)?,
                genre: row.get(6)?,
                year: row.get(7)?,
                track_number: row.get(8)?,
                // Add other fields as needed
                ..Default::default()
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(files)
    }
    
    /// Get tags that match between a file and folder
    pub fn get_matching_tags(conn: &Connection, audio_file_id: i64, folder_id: i64) -> Result<Vec<String>> {
        let file_tags = Self::get_file_tags(conn, audio_file_id)?;
        
        let mut stmt = conn.prepare(
            "SELECT DISTINCT rt.tag_type, rt.tag_value
             FROM rpg_tags rt
             JOIN virtual_folder_contents vfc ON rt.audio_file_id = vfc.audio_file_id
             WHERE vfc.folder_id = ?"
        )?;
        
        let folder_tags: Vec<String> = stmt.query_map([folder_id], |row| {
            let tag_type: String = row.get(0)?;
            let tag_value: String = row.get(1)?;
            Ok(format!("{}:{}", tag_type, tag_value))
        })?.collect::<Result<Vec<_>, _>>()?;
        
        let file_tags_set: std::collections::HashSet<&String> = file_tags.iter().collect();
        let folder_tags_set: std::collections::HashSet<&String> = folder_tags.iter().collect();
        
        let matching: Vec<String> = file_tags_set.intersection(&folder_tags_set)
            .map(|s| (*s).clone())
            .collect();
        
        Ok(matching)
    }
}