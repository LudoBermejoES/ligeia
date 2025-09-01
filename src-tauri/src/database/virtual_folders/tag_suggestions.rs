use rusqlite::{Connection, Result, params};
use crate::models::{VirtualFolder, AudioFile, AutoOrganizationSuggestion};

/// Tag-based folder suggestion operations
pub struct VirtualFolderTagSuggestions;

impl VirtualFolderTagSuggestions {
    /// Get folder suggestions for a file based on its RPG tags using enhanced mapping system
    pub fn suggest_folders_for_file(conn: &Connection, audio_file_id: i64, limit: Option<usize>) -> Result<Vec<(VirtualFolder, f64)>> {
        // Load folder mappings from individual category files
        let mut folder_tag_mappings = Vec::new();
        
        // Include all individual category mapping files
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_combat.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_environments.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_creatures.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_magic.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_social.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_horror.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_superhero.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_moods.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_sfx.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_activities.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_cultural.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_fantasy.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_session.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_scifi.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_instruments.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_mental_states.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_audio_structure.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_organizations.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_temporal_events.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_vehicles.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_ui.rs"));
        folder_tag_mappings.extend_from_slice(&include!("../../data/tag_mappings/folder_tag_domestic.rs"));
        
        // Define tag weights for scoring algorithm
        let tag_weights = &[
            ("occasion", 10u8),
            ("keyword:loc", 9),
            ("keyword:creature", 8),
            ("keyword:sfx", 8),
            ("mood", 7),
            ("genre", 6),
            ("keyword", 5),
        ];
        
        let limit = limit.unwrap_or(5);
        
        // Get all tags for the file
        let file_tags = Self::get_file_tags(conn, audio_file_id)?;
        
        if file_tags.is_empty() {
            return Ok(Vec::new());
        }
        
        let mut folder_suggestions: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
        
        // Process tag mappings
        for (tag_pattern, folder_path, weight, _description) in folder_tag_mappings {
            for file_tag in &file_tags {
                if Self::tag_matches(file_tag, tag_pattern) {
                    // Calculate confidence based on tag type and weight
                    let tag_type_weight = Self::get_tag_type_weight(file_tag, tag_weights);
                    let confidence = (weight as f64 * tag_type_weight as f64) / 100.0;
                    
                    folder_suggestions.entry(folder_path.to_string())
                        .and_modify(|score| *score = (*score + confidence).min(1.0))
                        .or_insert(confidence);
                }
            }
        }
        
        // Convert folder paths to actual folder objects and scores
        let mut folder_scores: Vec<(VirtualFolder, f64)> = Vec::new();
        
        for (folder_path, score) in folder_suggestions {
            if let Some(folder) = Self::find_folder_by_path(conn, &folder_path)? {
                // Only include leaf folders (folders without children)
                if let Some(folder_id) = folder.id {
                    use crate::database::virtual_folders::hierarchy_ops::VirtualFolderHierarchy;
                    let children = VirtualFolderHierarchy::get_folder_children(conn, Some(folder_id))?;
                    if children.is_empty() {
                        folder_scores.push((folder, score));
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
            
            // Only suggest if confidence is above threshold
            if let Some((folder, score)) = folder_suggestions.first() {
                if *score >= threshold {
                    suggestions.push(AutoOrganizationSuggestion {
                        audio_file_id: file.id.unwrap(),
                        audio_file_title: file.title.unwrap_or_else(|| "Unknown".to_string()),
                        suggested_folder_id: folder.id.unwrap(),
                        suggested_folder_name: folder.name.clone(),
                        confidence_score: *score,
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