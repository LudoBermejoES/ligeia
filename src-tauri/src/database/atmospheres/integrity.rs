use rusqlite::{Connection, params, Result};
use crate::models::{AtmosphereIntegrity, AtmosphereIntegrityBatchEntry};
use super::AtmosphereOps;

impl AtmosphereOps {
    /// Compute integrity: which mapped audio_file_ids are missing in audio_files table
    pub fn compute_integrity(conn: &Connection, atmosphere_id: i64) -> Result<AtmosphereIntegrity> {
        // Collect mapped ids
        let mut stmt = conn.prepare("SELECT audio_file_id FROM atmosphere_sounds WHERE atmosphere_id = ?1")?;
        let mapped: Result<Vec<i64>> = stmt.query_map([atmosphere_id], |row| row.get(0))?.collect();
        let mapped = mapped?;
        
        if mapped.is_empty() {
            return Ok(AtmosphereIntegrity { atmosphere_id, missing_ids: Vec::new() });
        }
        
        // Build placeholders
        let placeholders = mapped.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let sql = format!("SELECT id FROM audio_files WHERE id IN ({})", placeholders);
        let mut stmt2 = conn.prepare(&sql)?;
        let existing: Result<Vec<i64>> = stmt2.query_map(
            rusqlite::params_from_iter(mapped.iter()), 
            |row| row.get(0)
        )?.collect();
        
        let existing_set: std::collections::HashSet<i64> = existing?.into_iter().collect();
        let missing_ids: Vec<i64> = mapped.into_iter()
            .filter(|id| !existing_set.contains(id))
            .collect();
        
        Ok(AtmosphereIntegrity { atmosphere_id, missing_ids })
    }

    /// Batch compute integrity for all atmospheres (more efficient than per-atmosphere calls)
    pub fn compute_all_integrities(conn: &Connection) -> Result<Vec<AtmosphereIntegrityBatchEntry>> {
        let sql = "SELECT a.id as atmosphere_id,
                          GROUP_CONCAT(CASE WHEN af.id IS NULL THEN s.audio_file_id END) AS missing_csv
                   FROM atmospheres a
                   LEFT JOIN atmosphere_sounds s ON s.atmosphere_id = a.id
                   LEFT JOIN audio_files af ON af.id = s.audio_file_id
                   GROUP BY a.id";
        
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            let atmosphere_id: i64 = row.get(0)?;
            let missing_csv: Option<String> = row.get(1)?;
            let missing_ids: Vec<i64> = missing_csv
                .unwrap_or_default()
                .split(',')
                .filter_map(|s| s.trim().parse::<i64>().ok())
                .collect();
            Ok(AtmosphereIntegrityBatchEntry { atmosphere_id, missing_ids })
        })?;
        
        rows.collect()
    }
}