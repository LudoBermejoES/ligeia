use rusqlite::{Connection, params, Result};
use crate::models::Atmosphere;
use super::helpers;
use super::AtmosphereOps;

impl AtmosphereOps {
    /// Save or update an atmosphere
    pub fn save(conn: &Connection, atmosphere: &Atmosphere) -> Result<i64> {
        let keywords_json = serde_json::to_string(&atmosphere.keywords).unwrap_or_default();
        
        match atmosphere.id {
            Some(id) => {
                // Update existing
                conn.execute(
                    "UPDATE atmospheres SET 
                        name = ?1, title = ?2, description = ?3, category = ?4, 
                        subcategory = ?5, subsubcategory = ?6, keywords = ?7,
                        background_image = ?8, author_image = ?9, is_public = ?10,
                        theme = ?11, default_crossfade_ms = ?12, fade_curve = ?13,
                        updated_at = CURRENT_TIMESTAMP 
                    WHERE id = ?14",
                    params![
                        atmosphere.name, atmosphere.title, atmosphere.description,
                        atmosphere.category, atmosphere.subcategory, atmosphere.subsubcategory,
                        keywords_json, atmosphere.background_image, atmosphere.author_image,
                        atmosphere.is_public, atmosphere.theme, atmosphere.default_crossfade_ms, atmosphere.fade_curve,
                        id
                    ],
                )?;
                Ok(id)
            }
            None => {
                // Create new
                conn.execute(
                    "INSERT INTO atmospheres (
                        name, title, description, category, subcategory, subsubcategory,
                        keywords, background_image, author_image, is_public, theme,
                        default_crossfade_ms, fade_curve
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                    params![
                        atmosphere.name, atmosphere.title, atmosphere.description,
                        atmosphere.category, atmosphere.subcategory, atmosphere.subsubcategory,
                        keywords_json, atmosphere.background_image, atmosphere.author_image,
                        atmosphere.is_public, atmosphere.theme, atmosphere.default_crossfade_ms, 
                        atmosphere.fade_curve
                    ],
                )?;
                Ok(conn.last_insert_rowid())
            }
        }
    }

    /// Get all atmospheres
    pub fn get_all(conn: &Connection) -> Result<Vec<Atmosphere>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, title, description, category, subcategory, subsubcategory,
                    keywords, background_image, author_image, is_public, theme, default_crossfade_ms, 
                    fade_curve, created_at, updated_at
             FROM atmospheres ORDER BY updated_at DESC"
        )?;

        let rows = stmt.query_map([], |row| helpers::row_to_atmosphere(row))?;
        rows.collect()
    }

    /// Get atmosphere by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Atmosphere> {
        let mut stmt = conn.prepare(
            "SELECT id, name, title, description, category, subcategory, subsubcategory,
                    keywords, background_image, author_image, is_public, theme, default_crossfade_ms, 
                    fade_curve, created_at, updated_at
             FROM atmospheres WHERE id = ?1"
        )?;

        stmt.query_row([id], |row| helpers::row_to_atmosphere(row))
    }

    /// Delete atmosphere
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM atmospheres WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Duplicate an atmosphere (metadata + sound mappings)
    pub fn duplicate(conn: &Connection, id: i64, new_name: Option<&str>) -> Result<i64> {
        let mut stmt = conn.prepare(
            "SELECT name, title, description, category, subcategory, subsubcategory, keywords, 
                    background_image, author_image, is_public, theme, default_crossfade_ms, fade_curve 
             FROM atmospheres WHERE id = ?1"
        )?;
        
        let row = stmt.query_row([id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, Option<String>>(8)?,
                row.get::<_, bool>(9)?,
                row.get::<_, Option<String>>(10)?,
                row.get::<_, i64>(11)?,
                row.get::<_, String>(12)?,
            ))
        })?;
        
        let (orig_name, _orig_title, description, category, subcategory, subsubcategory, 
             keywords_json, background_image, author_image, is_public, theme, 
             default_crossfade_ms, fade_curve) = row;
        
        let base = new_name.map(|s| s.trim()).filter(|s| !s.is_empty()).unwrap_or_else(|| orig_name.as_str());
        let mut candidate = format!("{} (Copy)", base);
        let mut counter = 2;
        
        while helpers::name_exists(conn, &candidate)? {
            candidate = format!("{} (Copy {})", base, counter);
            counter += 1;
        }
        
        let final_name = candidate;
        let final_title = final_name.clone();
        
        conn.execute(
            "INSERT INTO atmospheres (name, title, description, category, subcategory, subsubcategory, 
                                     keywords, background_image, author_image, is_public, theme, 
                                     default_crossfade_ms, fade_curve) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![final_name, final_title, description, category, subcategory, subsubcategory, 
                   keywords_json, background_image, author_image, is_public, theme, 
                   default_crossfade_ms, fade_curve]
        )?;
        
        let new_id = conn.last_insert_rowid();
        
        // Copy sound mappings
        conn.execute(
            "INSERT INTO atmosphere_sounds (atmosphere_id, audio_file_id, volume, is_looping, is_muted, min_seconds, max_seconds) 
             SELECT ?1, audio_file_id, volume, is_looping, is_muted, min_seconds, max_seconds 
             FROM atmosphere_sounds WHERE atmosphere_id = ?2",
            params![new_id, id]
        )?;
        
        Ok(new_id)
    }
}