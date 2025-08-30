use rusqlite::{Connection, params, Result};
use crate::models::{AtmosphereSoundMapping, AtmosphereWithSounds};
use super::helpers;
use super::AtmosphereOps;

impl AtmosphereOps {
    /// Add sound to atmosphere
    pub fn add_sound(conn: &Connection, atmosphere_id: i64, audio_file_id: i64, volume: f32, is_looping: bool) -> Result<i64> {
        conn.execute(
            "INSERT OR REPLACE INTO atmosphere_sounds 
             (atmosphere_id, audio_file_id, volume, is_looping, is_muted, min_seconds, max_seconds)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![atmosphere_id, audio_file_id, volume, is_looping, false, 0, 0],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Remove sound from atmosphere
    pub fn remove_sound(conn: &Connection, atmosphere_id: i64, audio_file_id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM atmosphere_sounds WHERE atmosphere_id = ?1 AND audio_file_id = ?2",
            params![atmosphere_id, audio_file_id],
        )?;
        Ok(())
    }

    /// Update sound settings in atmosphere
    pub fn update_sound(conn: &Connection, atmosphere_id: i64, audio_file_id: i64, 
                       volume: f32, is_looping: bool, is_muted: bool, 
                       min_seconds: i32, max_seconds: i32) -> Result<()> {
        conn.execute(
            "UPDATE atmosphere_sounds 
             SET volume = ?3, is_looping = ?4, is_muted = ?5, min_seconds = ?6, max_seconds = ?7
             WHERE atmosphere_id = ?1 AND audio_file_id = ?2",
            params![atmosphere_id, audio_file_id, volume, is_looping, is_muted, min_seconds, max_seconds],
        )?;
        Ok(())
    }

    /// Get atmosphere with all its sounds
    pub fn get_with_sounds(conn: &Connection, atmosphere_id: i64) -> Result<AtmosphereWithSounds> {
        let atmosphere = Self::get_by_id(conn, atmosphere_id)?;
        
        // Get sound mappings
        let mut stmt = conn.prepare(
            "SELECT id, atmosphere_id, audio_file_id, volume, is_looping, is_muted, 
                    min_seconds, max_seconds, created_at
             FROM atmosphere_sounds WHERE atmosphere_id = ?1 ORDER BY created_at"
        )?;

        let mapping_rows = stmt.query_map([atmosphere_id], |row| {
            Ok(AtmosphereSoundMapping {
                id: Some(row.get(0)?),
                atmosphere_id: row.get(1)?,
                audio_file_id: row.get(2)?,
                volume: row.get(3)?,
                is_looping: row.get(4)?,
                is_muted: row.get(5)?,
                min_seconds: row.get(6).unwrap_or(0),
                max_seconds: row.get(7).unwrap_or(0),
                created_at: row.get(8)?,
            })
        })?;

        let mut sounds = Vec::new();
        let mut audio_file_ids = Vec::new();
        
        for row in mapping_rows {
            let mapping = row?;
            audio_file_ids.push(mapping.audio_file_id);
            sounds.push(mapping);
        }

        // Get audio files
        let mut audio_files = Vec::new();
        for audio_file_id in audio_file_ids {
            if let Ok(audio_file) = helpers::get_audio_file_by_id(conn, audio_file_id) {
                audio_files.push(audio_file);
            }
        }

        Ok(AtmosphereWithSounds {
            atmosphere,
            sounds,
            audio_files,
        })
    }

    /// Save atmosphere with sounds (complete save operation)
    pub fn save_with_sounds(conn: &Connection, atmosphere: &crate::models::Atmosphere, 
                           sounds: &[AtmosphereSoundMapping]) -> Result<i64> {
        // Start transaction
        let tx = conn.unchecked_transaction()?;
        
        // Save or update atmosphere
        let atmosphere_id = Self::save(&tx, atmosphere)?;
        
        // Clear existing sound mappings for this atmosphere
        tx.execute(
            "DELETE FROM atmosphere_sounds WHERE atmosphere_id = ?1",
            params![atmosphere_id],
        )?;
        
        // Insert new sound mappings
        for sound in sounds {
            tx.execute(
                "INSERT INTO atmosphere_sounds 
                 (atmosphere_id, audio_file_id, volume, is_looping, is_muted, min_seconds, max_seconds)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![atmosphere_id, sound.audio_file_id, sound.volume, sound.is_looping, 
                       sound.is_muted, sound.min_seconds, sound.max_seconds],
            )?;
        }
        
        // Commit transaction
        tx.commit()?;
        
        log::info!("Saved atmosphere {} with {} sounds", atmosphere_id, sounds.len());
        Ok(atmosphere_id)
    }
}