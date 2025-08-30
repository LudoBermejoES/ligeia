use rusqlite::{Connection, Result};
use crate::models::{Atmosphere, AtmosphereCategory};
use super::helpers;
use super::AtmosphereOps;

impl AtmosphereOps {
    /// Get all categories
    pub fn get_categories(conn: &Connection) -> Result<Vec<AtmosphereCategory>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, parent_id FROM atmosphere_categories 
             ORDER BY parent_id, display_order, name"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(AtmosphereCategory {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get(2)?,
            })
        })?;

        rows.collect()
    }
    
    /// Search atmospheres by free text (name/title/description/category/subcategory) + optional category + keywords list
    pub fn search(conn: &Connection, query: Option<&str>, category: Option<&str>, 
                 keywords: Option<&[String]>) -> Result<Vec<Atmosphere>> {
        // Base select
        let mut sql = String::from(
            "SELECT id, name, title, description, category, subcategory, subsubcategory, 
                    keywords, background_image, author_image, is_public, theme, 
                    default_crossfade_ms, fade_curve, created_at, updated_at 
             FROM atmospheres"
        );
        
        let mut clauses: Vec<String> = Vec::new();
        let mut params: Vec<String> = Vec::new();

        if let Some(cat) = category { 
            if !cat.is_empty() { 
                clauses.push("category = ?".into()); 
                params.push(cat.to_string()); 
            } 
        }
        
        if let Some(q) = query { 
            if !q.trim().is_empty() { 
                clauses.push(
                    "LOWER(name || ' ' || title || ' ' || description || ' ' || category || ' ' || subcategory) LIKE ?".into()
                ); 
                params.push(format!("%{}%", q.to_lowercase())); 
            } 
        }
        
        if let Some(kws) = keywords { 
            for kw in kws { 
                if !kw.is_empty() { 
                    clauses.push("keywords LIKE ?".into()); // naive JSON text search
                    // match JSON array element containing kw (quoted)
                    params.push(format!("%\"{}\"%", kw)); 
                }
            } 
        }
        
        if !clauses.is_empty() { 
            sql.push_str(" WHERE "); 
            sql.push_str(&clauses.join(" AND ")); 
        }
        
        sql.push_str(" ORDER BY updated_at DESC");

        let mut stmt = conn.prepare(&sql)?;
        
        // Convert params to &dyn ToSql
        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter()
            .map(|s| s as &dyn rusqlite::ToSql)
            .collect();
        
        let rows = stmt.query_map(
            rusqlite::params_from_iter(param_refs), 
            |row| helpers::row_to_atmosphere(row)
        )?;
        
        rows.collect()
    }
}