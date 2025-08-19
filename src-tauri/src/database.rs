use rusqlite::{Connection, params, Result};
use crate::models::{AudioFile, RpgTag, TagVocabulary, AudioFileWithTags};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("audio_player.db")?;
        let db = Database { conn };
        db.create_tables()?;
        db.initialize_tag_vocabulary()?;
        Ok(db)
    }

    fn create_tables(&self) -> Result<()> {
        // Create basic audio_files table first (for backward compatibility)
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS audio_files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                file_path TEXT UNIQUE NOT NULL,
                title TEXT,
                artist TEXT,
                album TEXT,
                duration REAL,
                genre TEXT,
                year INTEGER,
                track_number INTEGER
            )",
            [],
        )?;

        // Add extended columns if they don't exist (migration)
        let extended_columns = vec![
            ("album_artist", "TEXT"),
            ("date", "TEXT"),
            ("total_tracks", "INTEGER"),
            ("disc_number", "INTEGER"),
            ("total_discs", "INTEGER"),
            ("composer", "TEXT"),
            ("conductor", "TEXT"),
            ("lyricist", "TEXT"),
            ("original_artist", "TEXT"),
            ("remixer", "TEXT"),
            ("arranger", "TEXT"),
            ("engineer", "TEXT"),
            ("producer", "TEXT"),
            ("dj_mixer", "TEXT"),
            ("mixer", "TEXT"),
            ("content_group", "TEXT"),
            ("subtitle", "TEXT"),
            ("initial_key", "TEXT"),
            ("bpm", "INTEGER"),
            ("language", "TEXT"),
            ("media_type", "TEXT"),
            ("original_filename", "TEXT"),
            ("original_lyricist", "TEXT"),
            ("original_release_time", "TEXT"),
            ("playlist_delay", "INTEGER"),
            ("recording_time", "TEXT"),
            ("release_time", "TEXT"),
            ("tagging_time", "TEXT"),
            ("encoding_time", "TEXT"),
            ("encoding_settings", "TEXT"),
            ("encoded_by", "TEXT"),
            ("copyright", "TEXT"),
            ("file_owner", "TEXT"),
            ("internet_radio_station_name", "TEXT"),
            ("internet_radio_station_owner", "TEXT"),
            ("isrc", "TEXT"),
            ("publisher", "TEXT"),
            ("mood", "TEXT"),
            ("occasion", "TEXT"),
            ("tempo", "TEXT"),
            ("content_type", "TEXT"),
            ("category", "TEXT"),
            ("created_at", "DATETIME DEFAULT CURRENT_TIMESTAMP"),
            ("updated_at", "DATETIME DEFAULT CURRENT_TIMESTAMP"),
        ];

        // Add each column if it doesn't exist
        for (column_name, column_type) in extended_columns {
            let alter_sql = format!("ALTER TABLE audio_files ADD COLUMN {} {}", column_name, column_type);
            // Ignore errors for columns that already exist
            let _ = self.conn.execute(&alter_sql, []);
        }

        // New RPG tags table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS rpg_tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                audio_file_id INTEGER NOT NULL,
                tag_type TEXT NOT NULL,
                tag_value TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (audio_file_id) REFERENCES audio_files (id) ON DELETE CASCADE,
                UNIQUE(audio_file_id, tag_type, tag_value)
            )",
            [],
        )?;

        // Tag vocabulary table for controlled vocabularies
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tag_vocabulary (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                tag_type TEXT NOT NULL,
                tag_value TEXT NOT NULL,
                description TEXT,
                parent_tag TEXT,
                is_active BOOLEAN DEFAULT TRUE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(tag_type, tag_value)
            )",
            [],
        )?;

        // Indexes for performance
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_rpg_tags_audio_file ON rpg_tags(audio_file_id)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_rpg_tags_type_value ON rpg_tags(tag_type, tag_value)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tag_vocabulary_type ON tag_vocabulary(tag_type)",
            [],
        )?;

        Ok(())
    }

    fn initialize_tag_vocabulary(&self) -> Result<()> {
        // Check if vocabulary is already initialized
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM tag_vocabulary",
            [],
            |row| row.get(0)
        )?;

        if count > 0 {
            return Ok(()); // Already initialized
        }

        // Insert RPG genre vocabulary
        let genres = vec![
            ("genre", "ambient", Some("Atmospheric background music"), None::<&str>),
            ("genre", "battle", Some("Combat and action music"), None::<&str>),
            ("genre", "exploration", Some("Music for discovery and wandering"), None::<&str>),
            ("genre", "tavern", Some("Social gathering music"), None::<&str>),
            ("genre", "dungeon", Some("Dark underground music"), None::<&str>),
            ("genre", "town", Some("Settlement and civilization music"), None::<&str>),
            ("genre", "nature", Some("Outdoor and wilderness music"), None::<&str>),
            ("genre", "magic", Some("Mystical and arcane music"), None::<&str>),
            ("genre", "horror", Some("Scary and unsettling music"), None::<&str>),
            ("genre", "epic", Some("Grand and heroic music"), None::<&str>),
        ];

        // Insert mood vocabulary
        let moods = vec![
            ("mood", "peaceful", Some("Calm and serene"), None::<&str>),
            ("mood", "tense", Some("Anxious and stressful"), None::<&str>),
            ("mood", "mysterious", Some("Enigmatic and unknown"), None::<&str>),
            ("mood", "heroic", Some("Noble and brave"), None::<&str>),
            ("mood", "dark", Some("Gloomy and ominous"), None::<&str>),
            ("mood", "joyful", Some("Happy and uplifting"), None::<&str>),
            ("mood", "melancholic", Some("Sad and reflective"), None::<&str>),
            ("mood", "intense", Some("High energy and dramatic"), None::<&str>),
            ("mood", "suspenseful", Some("Building tension"), None::<&str>),
            ("mood", "whimsical", Some("Playful and light-hearted"), None::<&str>),
        ];

        // Insert occasion vocabulary
        let occasions = vec![
            ("occasion", "combat", Some("Fighting and battles"), None::<&str>),
            ("occasion", "rest", Some("Downtime and recovery"), None::<&str>),
            ("occasion", "dialogue", Some("Conversations and roleplay"), None::<&str>),
            ("occasion", "exploration", Some("Discovering new areas"), None::<&str>),
            ("occasion", "stealth", Some("Sneaking and hiding"), None::<&str>),
            ("occasion", "puzzle", Some("Problem solving"), None::<&str>),
            ("occasion", "ceremony", Some("Rituals and special events"), None::<&str>),
            ("occasion", "travel", Some("Journey and movement"), None::<&str>),
            ("occasion", "shopping", Some("Commerce and trading"), None::<&str>),
            ("occasion", "finale", Some("Climactic moments"), None::<&str>),
        ];

        // Insert keyword vocabulary
        let keywords = vec![
            ("keyword", "forest", Some("Woodland environments"), None::<&str>),
            ("keyword", "castle", Some("Fortified structures"), None::<&str>),
            ("keyword", "dragon", Some("Draconic themes"), None::<&str>),
            ("keyword", "magic", Some("Supernatural elements"), None::<&str>),
            ("keyword", "medieval", Some("Historical period themes"), None::<&str>),
            ("keyword", "fantasy", Some("Fantastical elements"), None::<&str>),
            ("keyword", "orchestral", Some("Full orchestra arrangements"), None::<&str>),
            ("keyword", "acoustic", Some("Natural instruments"), None::<&str>),
            ("keyword", "electronic", Some("Synthesized sounds"), None::<&str>),
            ("keyword", "vocal", Some("Singing and chanting"), None::<&str>),
        ];

        // Combine all vocabularies
        let all_vocab: Vec<_> = genres.into_iter()
            .chain(moods.into_iter())
            .chain(occasions.into_iter())
            .chain(keywords.into_iter())
            .collect();

        for (tag_type, tag_value, description, parent_tag) in all_vocab {
            self.conn.execute(
                "INSERT OR IGNORE INTO tag_vocabulary (tag_type, tag_value, description, parent_tag, is_active)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![tag_type, tag_value, description, parent_tag, true],
            )?;
        }

        Ok(())
    }

    // Audio file operations
    pub fn save_audio_file(&self, audio_file: &AudioFile) -> Result<i64> {
        let _id = self.conn.execute(
            "INSERT INTO audio_files (
                file_path, title, artist, album, duration, genre, year, track_number,
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
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20,
                ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30,
                ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38, ?39, ?40,
                ?41, ?42, ?43, ?44, ?45, ?46, ?47, ?48, ?49
            )",
            params![
                audio_file.file_path, audio_file.title, audio_file.artist,
                audio_file.album, audio_file.duration, audio_file.genre,
                audio_file.year, audio_file.track_number, audio_file.album_artist,
                audio_file.date, audio_file.total_tracks, audio_file.disc_number,
                audio_file.total_discs, audio_file.composer, audio_file.conductor,
                audio_file.lyricist, audio_file.original_artist, audio_file.remixer,
                audio_file.arranger, audio_file.engineer, audio_file.producer,
                audio_file.dj_mixer, audio_file.mixer, audio_file.content_group,
                audio_file.subtitle, audio_file.initial_key, audio_file.bpm,
                audio_file.language, audio_file.media_type, audio_file.original_filename,
                audio_file.original_lyricist, audio_file.original_release_time,
                audio_file.playlist_delay, audio_file.recording_time,
                audio_file.release_time, audio_file.tagging_time,
                audio_file.encoding_time, audio_file.encoding_settings,
                audio_file.encoded_by, audio_file.copyright, audio_file.file_owner,
                audio_file.internet_radio_station_name, audio_file.internet_radio_station_owner,
                audio_file.isrc, audio_file.publisher, audio_file.mood,
                audio_file.occasion, audio_file.tempo, audio_file.content_type,
                audio_file.category
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_all_audio_files(&self) -> Result<Vec<AudioFile>> {
        // First, check which columns exist in the table
        let mut stmt = self.conn.prepare("PRAGMA table_info(audio_files)")?;
        let column_rows = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(1)?) // column name is at index 1
        })?;
        
        let mut existing_columns = std::collections::HashSet::new();
        for column_result in column_rows {
            existing_columns.insert(column_result?);
        }

        // Build the SELECT query with only existing columns
        let base_columns = vec![
            "id", "file_path", "title", "artist", "album", 
            "duration", "genre", "year", "track_number"
        ];
        
        let extended_columns = vec![
            "album_artist", "date", "total_tracks", "disc_number", "total_discs",
            "composer", "conductor", "lyricist", "original_artist", "remixer", 
            "arranger", "engineer", "producer", "dj_mixer", "mixer", 
            "content_group", "subtitle", "initial_key", "bpm", "language", 
            "media_type", "original_filename", "original_lyricist", 
            "original_release_time", "playlist_delay", "recording_time", 
            "release_time", "tagging_time", "encoding_time", "encoding_settings", 
            "encoded_by", "copyright", "file_owner", "internet_radio_station_name", 
            "internet_radio_station_owner", "isrc", "publisher", "mood", 
            "occasion", "tempo", "content_type", "category"
        ];

        let mut all_columns = base_columns;
        for col in extended_columns {
            if existing_columns.contains(col) {
                all_columns.push(col);
            }
        }

        let query = format!(
            "SELECT {} FROM audio_files ORDER BY artist, album, track_number",
            all_columns.join(", ")
        );

        let mut stmt = self.conn.prepare(&query)?;

        let rows = stmt.query_map([], |row| {
            let mut index = 0;
            
            let id = Some(row.get(index)?);
            index += 1;
            let file_path = row.get(index)?;
            index += 1;
            let title = row.get(index)?;
            index += 1;
            let artist = row.get(index)?;
            index += 1;
            let album = row.get(index)?;
            index += 1;
            let duration = row.get(index)?;
            index += 1;
            let genre = row.get(index)?;
            index += 1;
            let year = row.get(index)?;
            index += 1;
            let track_number = row.get(index)?;
            index += 1;

            // Extended columns - only read if they exist
            let album_artist = if existing_columns.contains("album_artist") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let date = if existing_columns.contains("date") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let total_tracks = if existing_columns.contains("total_tracks") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let disc_number = if existing_columns.contains("disc_number") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let total_discs = if existing_columns.contains("total_discs") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let composer = if existing_columns.contains("composer") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let conductor = if existing_columns.contains("conductor") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let lyricist = if existing_columns.contains("lyricist") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let original_artist = if existing_columns.contains("original_artist") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let remixer = if existing_columns.contains("remixer") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let arranger = if existing_columns.contains("arranger") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let engineer = if existing_columns.contains("engineer") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let producer = if existing_columns.contains("producer") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let dj_mixer = if existing_columns.contains("dj_mixer") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let mixer = if existing_columns.contains("mixer") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let content_group = if existing_columns.contains("content_group") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let subtitle = if existing_columns.contains("subtitle") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let initial_key = if existing_columns.contains("initial_key") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let bpm = if existing_columns.contains("bpm") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let language = if existing_columns.contains("language") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let media_type = if existing_columns.contains("media_type") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let original_filename = if existing_columns.contains("original_filename") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let original_lyricist = if existing_columns.contains("original_lyricist") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let original_release_time = if existing_columns.contains("original_release_time") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let playlist_delay = if existing_columns.contains("playlist_delay") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let recording_time = if existing_columns.contains("recording_time") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let release_time = if existing_columns.contains("release_time") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let tagging_time = if existing_columns.contains("tagging_time") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let encoding_time = if existing_columns.contains("encoding_time") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let encoding_settings = if existing_columns.contains("encoding_settings") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let encoded_by = if existing_columns.contains("encoded_by") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let copyright = if existing_columns.contains("copyright") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let file_owner = if existing_columns.contains("file_owner") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let internet_radio_station_name = if existing_columns.contains("internet_radio_station_name") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let internet_radio_station_owner = if existing_columns.contains("internet_radio_station_owner") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let isrc = if existing_columns.contains("isrc") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let publisher = if existing_columns.contains("publisher") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let mood = if existing_columns.contains("mood") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let occasion = if existing_columns.contains("occasion") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let tempo = if existing_columns.contains("tempo") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let content_type = if existing_columns.contains("content_type") { 
                let val = row.get(index)?; 
                index += 1; 
                val 
            } else { 
                None 
            };
            
            let category = if existing_columns.contains("category") { 
                let val = row.get(index)?; 
                val 
            } else { 
                None 
            };

            Ok(AudioFile {
                id,
                file_path,
                title,
                artist,
                album,
                duration,
                genre,
                year,
                track_number,
                album_artist,
                date,
                total_tracks,
                disc_number,
                total_discs,
                composer,
                conductor,
                lyricist,
                original_artist,
                remixer,
                arranger,
                engineer,
                producer,
                dj_mixer,
                mixer,
                content_group,
                subtitle,
                initial_key,
                bpm,
                language,
                media_type,
                original_filename,
                original_lyricist,
                original_release_time,
                playlist_delay,
                recording_time,
                release_time,
                tagging_time,
                encoding_time,
                encoding_settings,
                encoded_by,
                copyright,
                file_owner,
                internet_radio_station_name,
                internet_radio_station_owner,
                isrc,
                publisher,
                mood,
                occasion,
                tempo,
                content_type,
                category,
            })
        })?;

        let mut audio_files = Vec::new();
        for row in rows {
            audio_files.push(row?);
        }
        Ok(audio_files)
    }

    pub fn delete_audio_file(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "DELETE FROM audio_files WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    // RPG tag operations
    pub fn add_rpg_tag(&self, audio_file_id: i64, tag_type: &str, tag_value: &str) -> Result<i64> {
        let _id = self.conn.execute(
            "INSERT OR IGNORE INTO rpg_tags (audio_file_id, tag_type, tag_value)
             VALUES (?1, ?2, ?3)",
            params![audio_file_id, tag_type, tag_value],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn remove_rpg_tag(&self, audio_file_id: i64, tag_type: &str, tag_value: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM rpg_tags WHERE audio_file_id = ?1 AND tag_type = ?2 AND tag_value = ?3",
            params![audio_file_id, tag_type, tag_value],
        )?;
        Ok(())
    }

    pub fn get_rpg_tags_for_file(&self, audio_file_id: i64) -> Result<Vec<RpgTag>> {
        let mut stmt = self.conn.prepare(
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

    pub fn get_audio_files_with_tags(&self) -> Result<Vec<AudioFileWithTags>> {
        let audio_files = self.get_all_audio_files()?;
        let mut files_with_tags = Vec::new();

        for audio_file in audio_files {
            if let Some(id) = audio_file.id {
                let rpg_tags = self.get_rpg_tags_for_file(id)?;
                files_with_tags.push(AudioFileWithTags {
                    audio_file,
                    rpg_tags,
                });
            }
        }

        Ok(files_with_tags)
    }

    // Tag vocabulary operations
    pub fn get_tag_vocabulary(&self, tag_type: Option<&str>) -> Result<Vec<TagVocabulary>> {
        let (query, params): (String, Vec<&str>) = match tag_type {
            Some(t) => (
                "SELECT id, tag_type, tag_value, description, parent_tag, is_active
                 FROM tag_vocabulary WHERE tag_type = ?1 AND is_active = TRUE
                 ORDER BY tag_value".to_string(),
                vec![t]
            ),
            None => (
                "SELECT id, tag_type, tag_value, description, parent_tag, is_active
                 FROM tag_vocabulary WHERE is_active = TRUE
                 ORDER BY tag_type, tag_value".to_string(),
                vec![]
            )
        };

        let mut stmt = self.conn.prepare(&query)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params), |row| {
            Ok(TagVocabulary {
                id: Some(row.get(0)?),
                tag_type: row.get(1)?,
                tag_value: row.get(2)?,
                description: row.get(3)?,
                parent_tag: row.get(4)?,
                is_active: row.get(5)?,
            })
        })?;

        let mut vocab = Vec::new();
        for row in rows {
            vocab.push(row?);
        }
        Ok(vocab)
    }

    pub fn search_files_by_tags(&self, tag_types: Option<&[String]>, tag_values: Option<&[String]>, match_all: bool) -> Result<Vec<AudioFileWithTags>> {
        let mut query = "SELECT DISTINCT af.id FROM audio_files af JOIN rpg_tags rt ON af.id = rt.audio_file_id WHERE ".to_string();
        let mut conditions = Vec::new();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();

        if let Some(types) = tag_types {
            if !types.is_empty() {
                let placeholders: Vec<String> = (0..types.len()).map(|i| format!("?{}", params.len() + i + 1)).collect();
                conditions.push(format!("rt.tag_type IN ({})", placeholders.join(", ")));
                for t in types {
                    params.push(t);
                }
            }
        }

        if let Some(values) = tag_values {
            if !values.is_empty() {
                let placeholders: Vec<String> = (0..values.len()).map(|i| format!("?{}", params.len() + i + 1)).collect();
                conditions.push(format!("rt.tag_value IN ({})", placeholders.join(", ")));
                for v in values {
                    params.push(v);
                }
            }
        }

        if conditions.is_empty() {
            return self.get_audio_files_with_tags();
        }

        let operator = if match_all { " AND " } else { " OR " };
        query.push_str(&conditions.join(operator));

        let mut stmt = self.conn.prepare(&query)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params), |row| {
            Ok(row.get::<_, i64>(0)?)
        })?;

        let mut file_ids = Vec::new();
        for row in rows {
            file_ids.push(row?);
        }

        let mut results = Vec::new();
        for file_id in file_ids {
            if let Ok(mut audio_stmt) = self.conn.prepare(
                "SELECT id, file_path, title, artist, album, duration, genre, year, 
                        track_number, album_artist, date, total_tracks, disc_number, total_discs,
                        composer, conductor, lyricist, original_artist, remixer, arranger,
                        engineer, producer, dj_mixer, mixer, content_group, subtitle,
                        initial_key, bpm, language, media_type, original_filename,
                        original_lyricist, original_release_time, playlist_delay,
                        recording_time, release_time, tagging_time, encoding_time,
                        encoding_settings, encoded_by, copyright, file_owner,
                        internet_radio_station_name, internet_radio_station_owner,
                        isrc, publisher, mood, occasion, tempo, content_type, category
                 FROM audio_files WHERE id = ?1"
            ) {
                if let Ok(audio_file) = audio_stmt.query_row([file_id], |row| {
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
                }) {
                    if let Ok(rpg_tags) = self.get_rpg_tags_for_file(file_id) {
                        results.push(AudioFileWithTags {
                            audio_file,
                            rpg_tags,
                        });
                    }
                }
            }
        }

        Ok(results)
    }
}