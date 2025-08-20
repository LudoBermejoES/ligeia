use rusqlite::{Connection, Result};
use crate::models::{AudioFile, RpgTag, AudioFileWithTags};

/// Repository for search operations
pub struct SearchRepository;

impl SearchRepository {
    pub fn new() -> Self {
        SearchRepository
    }

    /// Get all audio files with their RPG tags
    pub fn get_all_files_with_tags(&self, conn: &Connection) -> Result<Vec<AudioFileWithTags>> {
        let mut stmt = conn.prepare(
            "SELECT af.id, af.file_path, af.title, af.artist, af.album, af.duration, af.genre, af.year, af.track_number,
                    af.album_artist, af.date, af.total_tracks, af.disc_number, af.total_discs,
                    af.composer, af.conductor, af.lyricist, af.original_artist, af.remixer,
                    af.arranger, af.engineer, af.producer, af.dj_mixer, af.mixer,
                    af.content_group, af.subtitle, af.initial_key, af.bpm, af.language,
                    af.media_type, af.original_filename, af.original_lyricist,
                    af.original_release_time, af.playlist_delay, af.recording_time,
                    af.release_time, af.tagging_time, af.encoding_time, af.encoding_settings,
                    af.encoded_by, af.copyright, af.file_owner, af.internet_radio_station_name,
                    af.internet_radio_station_owner, af.isrc, af.publisher, af.mood,
                    af.occasion, af.tempo, af.content_type, af.category
             FROM audio_files af
             ORDER BY af.artist, af.album, af.track_number"
        )?;

        let audio_file_rows = stmt.query_map([], |row| {
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
                album_artist: row.get(9)?,
                date: row.get(10)?,
                total_tracks: row.get(11)?,
                disc_number: row.get(12)?,
                total_discs: row.get(13)?,
                composer: row.get(14)?,
                conductor: row.get(15)?,
                lyricist: row.get(16)?,
                original_artist: row.get(17)?,
                remixer: row.get(18)?,
                arranger: row.get(19)?,
                engineer: row.get(20)?,
                producer: row.get(21)?,
                dj_mixer: row.get(22)?,
                mixer: row.get(23)?,
                content_group: row.get(24)?,
                subtitle: row.get(25)?,
                initial_key: row.get(26)?,
                bpm: row.get(27)?,
                language: row.get(28)?,
                media_type: row.get(29)?,
                original_filename: row.get(30)?,
                original_lyricist: row.get(31)?,
                original_release_time: row.get(32)?,
                playlist_delay: row.get(33)?,
                recording_time: row.get(34)?,
                release_time: row.get(35)?,
                tagging_time: row.get(36)?,
                encoding_time: row.get(37)?,
                encoding_settings: row.get(38)?,
                encoded_by: row.get(39)?,
                copyright: row.get(40)?,
                file_owner: row.get(41)?,
                internet_radio_station_name: row.get(42)?,
                internet_radio_station_owner: row.get(43)?,
                isrc: row.get(44)?,
                publisher: row.get(45)?,
                mood: row.get(46)?,
                occasion: row.get(47)?,
                tempo: row.get(48)?,
                content_type: row.get(49)?,
                category: row.get(50)?,
            })
        })?;

        let mut files_with_tags = Vec::new();

        for audio_file_result in audio_file_rows {
            let audio_file = audio_file_result?;
            if let Some(id) = audio_file.id {
                let rpg_tags = self.get_rpg_tags_for_file(conn, id)?;
                files_with_tags.push(AudioFileWithTags {
                    audio_file,
                    rpg_tags,
                });
            }
        }

        Ok(files_with_tags)
    }

    /// Search files by tags with various matching options
    pub fn search_by_tags(&self, conn: &Connection, tag_types: Option<&[String]>, tag_values: Option<&[String]>, match_all: bool) -> Result<Vec<AudioFileWithTags>> {
        let mut query = "SELECT DISTINCT af.id FROM audio_files af JOIN rpg_tags rt ON af.id = rt.audio_file_id WHERE ".to_string();
        let mut conditions = Vec::new();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();

        if let Some(types) = tag_types {
            if !types.is_empty() {
                let placeholders: Vec<String> = (0..types.len()).map(|i| format!("?{}", params.len() + i + 1)).collect();
                conditions.push(format!("rt.tag_type IN ({})", placeholders.join(", ")));
                for tag_type in types {
                    params.push(tag_type);
                }
            }
        }

        if let Some(values) = tag_values {
            if !values.is_empty() {
                let placeholders: Vec<String> = (0..values.len()).map(|i| format!("?{}", params.len() + i + 1)).collect();
                conditions.push(format!("rt.tag_value IN ({})", placeholders.join(", ")));
                for tag_value in values {
                    params.push(tag_value);
                }
            }
        }

        if conditions.is_empty() {
            // No search criteria provided, return empty result
            return Ok(Vec::new());
        }

        query.push_str(&conditions.join(" AND "));

        if match_all && tag_values.is_some() {
            let tag_count = tag_values.unwrap().len();
            query.push_str(&format!(" GROUP BY af.id HAVING COUNT(DISTINCT rt.tag_value) = {}", tag_count));
        }

        // Get matching file IDs
        let mut stmt = conn.prepare(&query)?;
        let id_rows = stmt.query_map(rusqlite::params_from_iter(params), |row| {
            Ok(row.get::<_, i64>(0)?)
        })?;

        let mut file_ids = Vec::new();
        for id_result in id_rows {
            file_ids.push(id_result?);
        }

        // Get full file information for matching IDs
        let mut results = Vec::new();
        for file_id in file_ids {
            if let Ok(audio_file) = self.get_audio_file_by_id(conn, file_id) {
                let rpg_tags = self.get_rpg_tags_for_file(conn, file_id)?;
                results.push(AudioFileWithTags {
                    audio_file,
                    rpg_tags,
                });
            }
        }

        Ok(results)
    }

    // Helper methods

    fn get_rpg_tags_for_file(&self, conn: &Connection, audio_file_id: i64) -> Result<Vec<RpgTag>> {
        let mut stmt = conn.prepare(
            "SELECT id, audio_file_id, tag_type, tag_value, created_at
             FROM rpg_tags WHERE audio_file_id = ?1 ORDER BY tag_type, tag_value"
        )?;

        let rows = stmt.query_map([audio_file_id], |row| {
            Ok(RpgTag {
                id: Some(row.get(0)?),
                audio_file_id: row.get(1)?,
                tag_type: row.get(2)?,
                tag_value: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;

        let mut tags = Vec::new();
        for row in rows {
            tags.push(row?);
        }
        Ok(tags)
    }

    fn get_audio_file_by_id(&self, conn: &Connection, id: i64) -> Result<AudioFile> {
        let mut stmt = conn.prepare(
            "SELECT id, file_path, title, artist, album, duration, genre, year, track_number,
                    album_artist, date, total_tracks, disc_number, total_discs,
                    composer, conductor, lyricist, original_artist, remixer,
                    arranger, engineer, producer, dj_mixer, mixer,
                    content_group, subtitle, initial_key, bpm, language,
                    media_type, original_filename, original_lyricist,
                    original_release_time, playlist_delay, recording_time,
                    release_time, tagging_time, encoding_time, encoding_settings,
                    encoded_by, copyright, file_owner, internet_radio_station_name,
                    internet_radio_station_owner, isrc, publisher, mood,
                    occasion, tempo, content_type, category
             FROM audio_files WHERE id = ?1"
        )?;

        stmt.query_row([id], |row| {
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
                album_artist: row.get(9)?,
                date: row.get(10)?,
                total_tracks: row.get(11)?,
                disc_number: row.get(12)?,
                total_discs: row.get(13)?,
                composer: row.get(14)?,
                conductor: row.get(15)?,
                lyricist: row.get(16)?,
                original_artist: row.get(17)?,
                remixer: row.get(18)?,
                arranger: row.get(19)?,
                engineer: row.get(20)?,
                producer: row.get(21)?,
                dj_mixer: row.get(22)?,
                mixer: row.get(23)?,
                content_group: row.get(24)?,
                subtitle: row.get(25)?,
                initial_key: row.get(26)?,
                bpm: row.get(27)?,
                language: row.get(28)?,
                media_type: row.get(29)?,
                original_filename: row.get(30)?,
                original_lyricist: row.get(31)?,
                original_release_time: row.get(32)?,
                playlist_delay: row.get(33)?,
                recording_time: row.get(34)?,
                release_time: row.get(35)?,
                tagging_time: row.get(36)?,
                encoding_time: row.get(37)?,
                encoding_settings: row.get(38)?,
                encoded_by: row.get(39)?,
                copyright: row.get(40)?,
                file_owner: row.get(41)?,
                internet_radio_station_name: row.get(42)?,
                internet_radio_station_owner: row.get(43)?,
                isrc: row.get(44)?,
                publisher: row.get(45)?,
                mood: row.get(46)?,
                occasion: row.get(47)?,
                tempo: row.get(48)?,
                content_type: row.get(49)?,
                category: row.get(50)?,
            })
        })
    }
}