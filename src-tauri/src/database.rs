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

        // Insert RPG genre vocabulary (from TAGS.md specification)
        let genres = vec![
            // Orchestral
            ("genre", "orchestral", Some("Orchestral music"), None::<&str>),
            ("genre", "orchestral:cinematic", Some("Cinematic orchestral"), Some("orchestral")),
            ("genre", "orchestral:hybrid", Some("Hybrid orchestral"), Some("orchestral")),
            ("genre", "orchestral:heroic", Some("Heroic orchestral"), Some("orchestral")),
            ("genre", "orchestral:dark", Some("Dark orchestral"), Some("orchestral")),
            ("genre", "orchestral:minimal", Some("Minimal orchestral"), Some("orchestral")),
            ("genre", "orchestral:romantic", Some("Romantic orchestral"), Some("orchestral")),
            ("genre", "orchestral:baroque", Some("Baroque orchestral"), Some("orchestral")),
            ("genre", "orchestral:renaissance", Some("Renaissance orchestral"), Some("orchestral")),
            ("genre", "orchestral:medieval", Some("Medieval orchestral"), Some("orchestral")),
            ("genre", "orchestral:choral", Some("Choral orchestral"), Some("orchestral")),
            ("genre", "orchestral:percussive", Some("Percussive orchestral"), Some("orchestral")),
            ("genre", "orchestral:trailer", Some("Trailer music"), Some("orchestral")),
            
            // Ambient & Drones
            ("genre", "ambient", Some("Ambient music"), None::<&str>),
            ("genre", "ambient:dark-ambient", Some("Dark ambient"), Some("ambient")),
            ("genre", "ambient:space-ambient", Some("Space ambient"), Some("ambient")),
            ("genre", "ambient:nature-ambient", Some("Nature ambient"), Some("ambient")),
            ("genre", "ambient:ritual", Some("Ritual ambient"), Some("ambient")),
            ("genre", "ambient:drone", Some("Drone ambient"), Some("ambient")),
            ("genre", "ambient:textural", Some("Textural ambient"), Some("ambient")),
            ("genre", "ambient:new-age", Some("New age ambient"), Some("ambient")),
            ("genre", "ambient:lofi-ambient", Some("Lo-fi ambient"), Some("ambient")),
            
            // Electronic
            ("genre", "electronic", Some("Electronic music"), None::<&str>),
            ("genre", "electronic:synthwave", Some("Synthwave"), Some("electronic")),
            ("genre", "electronic:cyberpunk", Some("Cyberpunk electronic"), Some("electronic")),
            ("genre", "electronic:idm", Some("IDM"), Some("electronic")),
            ("genre", "electronic:glitch", Some("Glitch"), Some("electronic")),
            ("genre", "electronic:industrial", Some("Industrial electronic"), Some("electronic")),
            ("genre", "electronic:ebm", Some("EBM"), Some("electronic")),
            ("genre", "electronic:techno", Some("Techno"), Some("electronic")),
            ("genre", "electronic:trance", Some("Trance"), Some("electronic")),
            ("genre", "electronic:dnb", Some("Drum & Bass"), Some("electronic")),
            ("genre", "electronic:downtempo", Some("Downtempo"), Some("electronic")),
            ("genre", "electronic:chiptune", Some("Chiptune"), Some("electronic")),
            
            // Rock / Metal / Post-
            ("genre", "rock", Some("Rock music"), None::<&str>),
            ("genre", "rock:post-rock", Some("Post-rock"), Some("rock")),
            ("genre", "rock:gothic-rock", Some("Gothic rock"), Some("rock")),
            ("genre", "rock:progressive", Some("Progressive rock"), Some("rock")),
            ("genre", "metal", Some("Metal music"), None::<&str>),
            ("genre", "metal:power", Some("Power metal"), Some("metal")),
            ("genre", "metal:symphonic", Some("Symphonic metal"), Some("metal")),
            ("genre", "metal:black", Some("Black metal"), Some("metal")),
            ("genre", "metal:doom", Some("Doom metal"), Some("metal")),
            ("genre", "metal:folk-metal", Some("Folk metal"), Some("metal")),
            ("genre", "metal:industrial-metal", Some("Industrial metal"), Some("metal")),
            ("genre", "post-metal", Some("Post-metal"), None::<&str>),
            
            // Folk / World / Traditional
            ("genre", "folk", Some("Folk music"), None::<&str>),
            ("genre", "folk:celtic", Some("Celtic folk"), Some("folk")),
            ("genre", "folk:nordic", Some("Nordic folk"), Some("folk")),
            ("genre", "folk:middle-eastern", Some("Middle Eastern folk"), Some("folk")),
            ("genre", "folk:mediterranean", Some("Mediterranean folk"), Some("folk")),
            ("genre", "folk:asian-east", Some("East Asian folk"), Some("folk")),
            ("genre", "folk:asian-south", Some("South Asian folk"), Some("folk")),
            ("genre", "folk:african", Some("African folk"), Some("folk")),
            ("genre", "folk:andino", Some("Andean folk"), Some("folk")),
            ("genre", "folk:balkan", Some("Balkan folk"), Some("folk")),
            ("genre", "folk:sea-shanty", Some("Sea shanty"), Some("folk")),
            ("genre", "folk:wild-west-folk", Some("Wild West folk"), Some("folk")),
            
            // Jazz / Blues / Noir / Lounge
            ("genre", "jazz", Some("Jazz music"), None::<&str>),
            ("genre", "jazz:noir", Some("Noir jazz"), Some("jazz")),
            ("genre", "jazz:swing", Some("Swing jazz"), Some("jazz")),
            ("genre", "jazz:cool", Some("Cool jazz"), Some("jazz")),
            ("genre", "jazz:latin", Some("Latin jazz"), Some("jazz")),
            ("genre", "blues", Some("Blues music"), None::<&str>),
            ("genre", "lounge", Some("Lounge music"), None::<&str>),
            
            // Horror / Sound-Design Forward
            ("genre", "horror", Some("Horror music"), None::<&str>),
            ("genre", "horror:atonal", Some("Atonal horror"), Some("horror")),
            ("genre", "horror:dissonant-strings", Some("Dissonant strings"), Some("horror")),
            ("genre", "horror:sound-design", Some("Horror sound design"), Some("horror")),
            ("genre", "sound-design", Some("Sound design"), None::<&str>),
            ("genre", "sound-design:risers", Some("Risers"), Some("sound-design")),
            ("genre", "sound-design:impacts", Some("Impacts"), Some("sound-design")),
            ("genre", "sound-design:whooshes", Some("Whooshes"), Some("sound-design")),
            ("genre", "sound-design:stingers", Some("Stingers"), Some("sound-design")),
            ("genre", "sound-design:booms", Some("Booms"), Some("sound-design")),
            ("genre", "sound-design:braams", Some("Braams"), Some("sound-design")),
            
            // Setting-Driven Blends
            ("genre", "fantasy", Some("Fantasy music"), None::<&str>),
            ("genre", "fantasy:high-fantasy", Some("High fantasy"), Some("fantasy")),
            ("genre", "fantasy:grimdark", Some("Grimdark fantasy"), Some("fantasy")),
            ("genre", "fantasy:fairy", Some("Fairy fantasy"), Some("fantasy")),
            ("genre", "sci-fi", Some("Science fiction music"), None::<&str>),
            ("genre", "sci-fi:space-opera", Some("Space opera"), Some("sci-fi")),
            ("genre", "sci-fi:hard-sci-fi", Some("Hard science fiction"), Some("sci-fi")),
            ("genre", "sci-fi:cyberpunk", Some("Cyberpunk"), Some("sci-fi")),
            ("genre", "sci-fi:biopunk", Some("Biopunk"), Some("sci-fi")),
            ("genre", "sci-fi:post-human", Some("Post-human"), Some("sci-fi")),
            ("genre", "dieselpunk", Some("Dieselpunk"), None::<&str>),
            ("genre", "steampunk", Some("Steampunk"), None::<&str>),
            ("genre", "atompunk", Some("Atompunk"), None::<&str>),
            ("genre", "solarpunk", Some("Solarpunk"), None::<&str>),
            ("genre", "post-apocalyptic", Some("Post-apocalyptic"), None::<&str>),
            ("genre", "western", Some("Western"), None::<&str>),
            ("genre", "mystery-noir", Some("Mystery noir"), None::<&str>),
            ("genre", "modern-urban", Some("Modern urban"), None::<&str>),
            ("genre", "superhero", Some("Superhero"), None::<&str>),
            ("genre", "mythic", Some("Mythic"), None::<&str>),
            ("genre", "mythic:norse", Some("Norse mythic"), Some("mythic")),
            ("genre", "mythic:greco-roman", Some("Greco-Roman mythic"), Some("mythic")),
            ("genre", "mythic:egyptian", Some("Egyptian mythic"), Some("mythic")),
            ("genre", "mythic:celtic", Some("Celtic mythic"), Some("mythic")),
            ("genre", "mythic:japanese", Some("Japanese mythic"), Some("mythic")),
            ("genre", "mythic:mesoamerican", Some("Mesoamerican mythic"), Some("mythic")),
        ];

        // Insert mood vocabulary (from TAGS.md specification)
        let moods = vec![
            // Positive / Uplifting
            ("mood", "heroic", Some("Noble and brave"), None::<&str>),
            ("mood", "triumphant", Some("Victorious and celebratory"), None::<&str>),
            ("mood", "noble", Some("Dignified and honorable"), None::<&str>),
            ("mood", "uplifting", Some("Inspiring and encouraging"), None::<&str>),
            ("mood", "hopeful", Some("Optimistic and positive"), None::<&str>),
            ("mood", "inspiring", Some("Motivating and empowering"), None::<&str>),
            ("mood", "adventurous", Some("Bold and exploratory"), None::<&str>),
            ("mood", "confident", Some("Self-assured and strong"), None::<&str>),
            ("mood", "victorious", Some("Winning and successful"), None::<&str>),
            ("mood", "festive", Some("Celebratory and joyous"), None::<&str>),
            ("mood", "playful", Some("Fun and lighthearted"), None::<&str>),
            ("mood", "whimsical", Some("Fanciful and imaginative"), None::<&str>),
            ("mood", "merry", Some("Cheerful and happy"), None::<&str>),
            ("mood", "lighthearted", Some("Carefree and jovial"), None::<&str>),
            ("mood", "tender", Some("Gentle and caring"), None::<&str>),
            ("mood", "romantic", Some("Loving and passionate"), None::<&str>),
            ("mood", "serene", Some("Peaceful and calm"), None::<&str>),
            ("mood", "pastoral", Some("Rural and idyllic"), None::<&str>),
            ("mood", "warm", Some("Comforting and cozy"), None::<&str>),
            ("mood", "comforting", Some("Soothing and reassuring"), None::<&str>),
            ("mood", "nostalgic", Some("Wistful and reminiscent"), None::<&str>),
            ("mood", "bittersweet", Some("Both happy and sad"), None::<&str>),
            
            // Neutral / Reflective
            ("mood", "mysterious", Some("Enigmatic and unknown"), None::<&str>),
            ("mood", "enigmatic", Some("Puzzling and cryptic"), None::<&str>),
            ("mood", "curious", Some("Inquisitive and wondering"), None::<&str>),
            ("mood", "contemplative", Some("Thoughtful and reflective"), None::<&str>),
            ("mood", "dreamlike", Some("Surreal and ethereal"), None::<&str>),
            ("mood", "ethereal", Some("Otherworldly and delicate"), None::<&str>),
            ("mood", "mythic", Some("Legendary and ancient"), None::<&str>),
            ("mood", "arcane", Some("Mystical and secret"), None::<&str>),
            ("mood", "otherworldly", Some("Beyond normal reality"), None::<&str>),
            ("mood", "solemn", Some("Serious and dignified"), None::<&str>),
            ("mood", "ceremonial", Some("Ritual and formal"), None::<&str>),
            ("mood", "sacred", Some("Holy and reverent"), None::<&str>),
            ("mood", "ritualistic", Some("Ceremonial and traditional"), None::<&str>),
            ("mood", "austere", Some("Severe and simple"), None::<&str>),
            ("mood", "stoic", Some("Enduring and unemotional"), None::<&str>),
            
            // Dark / Tense / Negative
            ("mood", "ominous", Some("Threatening and foreboding"), None::<&str>),
            ("mood", "foreboding", Some("Portending doom"), None::<&str>),
            ("mood", "tense", Some("Anxious and stressed"), None::<&str>),
            ("mood", "suspenseful", Some("Building tension"), None::<&str>),
            ("mood", "uneasy", Some("Uncomfortable and worried"), None::<&str>),
            ("mood", "eerie", Some("Strange and unsettling"), None::<&str>),
            ("mood", "creepy", Some("Frightening and disturbing"), None::<&str>),
            ("mood", "unsettling", Some("Disturbing and unnerving"), None::<&str>),
            ("mood", "menacing", Some("Threatening and hostile"), None::<&str>),
            ("mood", "sinister", Some("Evil and malevolent"), None::<&str>),
            ("mood", "gothic", Some("Dark and brooding"), None::<&str>),
            ("mood", "dread", Some("Deep fear and anxiety"), None::<&str>),
            ("mood", "grim", Some("Harsh and forbidding"), None::<&str>),
            ("mood", "bleak", Some("Hopeless and desolate"), None::<&str>),
            ("mood", "oppressive", Some("Crushing and overwhelming"), None::<&str>),
            ("mood", "claustrophobic", Some("Confining and suffocating"), None::<&str>),
            ("mood", "tragic", Some("Sorrowful and devastating"), None::<&str>),
            ("mood", "melancholic", Some("Sad and pensive"), None::<&str>),
            ("mood", "sorrowful", Some("Full of grief"), None::<&str>),
            ("mood", "desolate", Some("Empty and abandoned"), None::<&str>),
            ("mood", "lonely", Some("Isolated and alone"), None::<&str>),
            ("mood", "fatalistic", Some("Accepting inevitable doom"), None::<&str>),
            ("mood", "nihilistic", Some("Rejecting moral principles"), None::<&str>),
            
            // Action Energy / Aggression
            ("mood", "driving", Some("Propelling forward"), None::<&str>),
            ("mood", "relentless", Some("Unforgiving and persistent"), None::<&str>),
            ("mood", "frenetic", Some("Fast and energetic"), None::<&str>),
            ("mood", "furious", Some("Extremely angry"), None::<&str>),
            ("mood", "aggressive", Some("Forceful and attacking"), None::<&str>),
            ("mood", "percussive", Some("Sharp and rhythmic"), None::<&str>),
            ("mood", "charged", Some("Energetic and electric"), None::<&str>),
            ("mood", "urgent", Some("Requiring immediate action"), None::<&str>),
            ("mood", "high-stakes", Some("Critical and important"), None::<&str>),
            ("mood", "chaotic", Some("Disordered and frantic"), None::<&str>),
            ("mood", "volatile", Some("Unpredictable and explosive"), None::<&str>),
            ("mood", "explosive", Some("Sudden and powerful"), None::<&str>),
            ("mood", "brooding-intensity", Some("Dark building energy"), None::<&str>),
            ("mood", "building", Some("Gradually increasing"), None::<&str>),
            ("mood", "rising-tension", Some("Escalating suspense"), None::<&str>),
            ("mood", "calm-before-storm", Some("Quiet before chaos"), None::<&str>),
            
            // Subtle Texture / Atmospherics
            ("mood", "airy", Some("Light and spacious"), None::<&str>),
            ("mood", "hazy", Some("Unclear and dreamlike"), None::<&str>),
            ("mood", "glacial", Some("Slow and cold"), None::<&str>),
            ("mood", "glitchy", Some("Digital and fragmented"), None::<&str>),
            ("mood", "noisy", Some("Rough and distorted"), None::<&str>),
            ("mood", "grainy", Some("Textured and rough"), None::<&str>),
            ("mood", "organic", Some("Natural and flowing"), None::<&str>),
            ("mood", "mechanical", Some("Robotic and precise"), None::<&str>),
            ("mood", "rusted", Some("Aged and deteriorated"), None::<&str>),
            ("mood", "industrial", Some("Machine-like and harsh"), None::<&str>),
            ("mood", "neon", Some("Electric and synthetic"), None::<&str>),
            ("mood", "digital-cold", Some("Electronic and sterile"), None::<&str>),
            ("mood", "bio-organic", Some("Living and breathing"), None::<&str>),
            ("mood", "wet", Some("Fluid and moist"), None::<&str>),
            ("mood", "dry", Some("Arid and stark"), None::<&str>),
            
            // Horror-Specific
            ("mood", "eldritch", Some("Cosmic horror and madness"), None::<&str>),
            ("mood", "uncanny", Some("Familiar yet strange"), None::<&str>),
            ("mood", "body-horror", Some("Physical transformation fear"), None::<&str>),
            ("mood", "liminal", Some("Threshold and in-between"), None::<&str>),
            ("mood", "dissonant", Some("Harsh and clashing"), None::<&str>),
            ("mood", "abhorrent", Some("Disgusting and repulsive"), None::<&str>),
            ("mood", "ritual-fear", Some("Ceremonial dread"), None::<&str>),
            ("mood", "cosmic-dread", Some("Universal existential fear"), None::<&str>),
        ];

        // Insert occasion vocabulary (comprehensive selection from TAGS.md)
        let occasions = vec![
            // Meta / Session Flow
            ("occasion", "session-start", Some("Beginning of game session"), None::<&str>),
            ("occasion", "recap", Some("Story summary"), None::<&str>),
            ("occasion", "session-end", Some("End of game session"), None::<&str>),
            ("occasion", "level-up", Some("Character advancement"), None::<&str>),
            ("occasion", "quest-complete", Some("Mission accomplished"), None::<&str>),
            ("occasion", "character-death", Some("PC or important NPC death"), None::<&str>),
            
            // Exploration & Travel
            ("occasion", "overworld-travel", Some("Long distance journeys"), None::<&str>),
            ("occasion", "wilderness-exploration", Some("Exploring wild areas"), None::<&str>),
            ("occasion", "urban-exploration", Some("City and town exploration"), None::<&str>),
            ("occasion", "dungeon-crawl", Some("Underground exploration"), None::<&str>),
            ("occasion", "ruin-delving", Some("Exploring ancient ruins"), None::<&str>),
            ("occasion", "cave-exploration", Some("Natural cave systems"), None::<&str>),
            ("occasion", "mountain-pass", Some("Traveling through mountains"), None::<&str>),
            ("occasion", "sea-voyage", Some("Ocean travel"), None::<&str>),
            ("occasion", "airship-voyage", Some("Flying travel"), None::<&str>),
            ("occasion", "space-cruise", Some("Space travel"), None::<&str>),
            
            // Social / Roleplay
            ("occasion", "tavern", Some("Tavern and inn scenes"), None::<&str>),
            ("occasion", "market", Some("Shopping and trading"), None::<&str>),
            ("occasion", "noble-court", Some("Royal court interactions"), None::<&str>),
            ("occasion", "negotiation", Some("Deal making and diplomacy"), None::<&str>),
            ("occasion", "interrogation", Some("Questioning and investigation"), None::<&str>),
            ("occasion", "trial", Some("Legal proceedings"), None::<&str>),
            ("occasion", "festival", Some("Celebrations and festivities"), None::<&str>),
            ("occasion", "wedding", Some("Marriage ceremonies"), None::<&str>),
            ("occasion", "funeral", Some("Death rites and mourning"), None::<&str>),
            ("occasion", "ceremony", Some("Rituals and formal events"), None::<&str>),
            
            // Investigation / Heist / Stealth
            ("occasion", "crime-scene", Some("Investigation scenes"), None::<&str>),
            ("occasion", "library-research", Some("Knowledge gathering"), None::<&str>),
            ("occasion", "surveillance", Some("Watching and monitoring"), None::<&str>),
            ("occasion", "infiltration", Some("Sneaking into places"), None::<&str>),
            ("occasion", "lockpicking", Some("Breaking and entering"), None::<&str>),
            ("occasion", "hacking", Some("Digital infiltration"), None::<&str>),
            ("occasion", "escape", Some("Getting away"), None::<&str>),
            
            // Combat (phase-aware)
            ("occasion", "combat-ambush", Some("Surprise attacks"), None::<&str>),
            ("occasion", "combat-skirmish", Some("Small battles"), None::<&str>),
            ("occasion", "combat-duel", Some("One-on-one fights"), None::<&str>),
            ("occasion", "combat-horde", Some("Fighting many enemies"), None::<&str>),
            ("occasion", "combat-siege", Some("Large scale warfare"), None::<&str>),
            ("occasion", "boss-intro", Some("Major enemy introduction"), None::<&str>),
            ("occasion", "boss-loop", Some("Main boss battle"), None::<&str>),
            ("occasion", "boss-final-phase", Some("Climactic boss moments"), None::<&str>),
            ("occasion", "victory-fanfare", Some("Winning celebration"), None::<&str>),
            ("occasion", "defeat-lament", Some("Loss and failure"), None::<&str>),
            ("occasion", "chase", Some("Pursuit scenes"), None::<&str>),
            
            // Horror / Supernatural
            ("occasion", "haunting", Some("Ghost encounters"), None::<&str>),
            ("occasion", "ritual", Some("Magical ceremonies"), None::<&str>),
            ("occasion", "summoning", Some("Calling forth entities"), None::<&str>),
            ("occasion", "eldritch-reveal", Some("Cosmic horror discovery"), None::<&str>),
            ("occasion", "ghost-encounter", Some("Spirit interactions"), None::<&str>),
            ("occasion", "vampire-lair", Some("Undead strongholds"), None::<&str>),
            ("occasion", "zombie-siege", Some("Undead attacks"), None::<&str>),
            ("occasion", "cult-gathering", Some("Dark worship"), None::<&str>),
            
            // Magic / Psionics / Sci-Tech
            ("occasion", "spellcasting-prep", Some("Magic preparation"), None::<&str>),
            ("occasion", "battle-magic", Some("Combat spellcasting"), None::<&str>),
            ("occasion", "divination", Some("Fortune telling and prophecy"), None::<&str>),
            ("occasion", "portal-crossing", Some("Dimensional travel"), None::<&str>),
            ("occasion", "teleportation", Some("Instant transport"), None::<&str>),
            ("occasion", "lab-experiment", Some("Scientific research"), None::<&str>),
            ("occasion", "ai-core", Some("Artificial intelligence"), None::<&str>),
            ("occasion", "cyberdeck-dive", Some("Virtual reality hacking"), None::<&str>),
            ("occasion", "warp-jump", Some("FTL travel"), None::<&str>),
            
            // Survival / Downtime / Crafting
            ("occasion", "campfire", Some("Rest around fire"), None::<&str>),
            ("occasion", "short-rest", Some("Brief recovery"), None::<&str>),
            ("occasion", "long-rest", Some("Extended recovery"), None::<&str>),
            ("occasion", "hunting", Some("Tracking and killing prey"), None::<&str>),
            ("occasion", "blacksmithing", Some("Metalworking"), None::<&str>),
            ("occasion", "alchemy", Some("Potion making"), None::<&str>),
            ("occasion", "enchanting", Some("Magic item creation"), None::<&str>),
            ("occasion", "training", Some("Skill development"), None::<&str>),
            ("occasion", "shopping", Some("Commerce and trading"), None::<&str>),
            
            // Environment / Events
            ("occasion", "sunrise", Some("Dawn breaking"), None::<&str>),
            ("occasion", "sunset", Some("Evening twilight"), None::<&str>),
            ("occasion", "night-watch", Some("Nighttime vigilance"), None::<&str>),
            ("occasion", "storm", Some("Severe weather"), None::<&str>),
            ("occasion", "earthquake", Some("Ground shaking"), None::<&str>),
            ("occasion", "eclipse", Some("Celestial events"), None::<&str>),
            
            // Add remaining occasions from TAGS.md
            ("occasion", "table-chatter", Some("Player conversation"), None::<&str>),
            ("occasion", "break", Some("Game break"), None::<&str>),
            ("occasion", "credits", Some("End credits"), None::<&str>),
            ("occasion", "achievement", Some("Achievement unlocked"), None::<&str>),
            ("occasion", "loot-found", Some("Treasure discovered"), None::<&str>),
            ("occasion", "epilogue", Some("Story epilogue"), None::<&str>),
            ("occasion", "flashback", Some("Flashback sequence"), None::<&str>),
            ("occasion", "montage", Some("Montage sequence"), None::<&str>),
            
            // More exploration
            ("occasion", "underdark-journey", Some("Underground realm travel"), None::<&str>),
            ("occasion", "sewers", Some("Sewer exploration"), None::<&str>),
            ("occasion", "desert-crossing", Some("Desert travel"), None::<&str>),
            ("occasion", "jungle-trek", Some("Jungle exploration"), None::<&str>),
            ("occasion", "swamp-march", Some("Swamp traversal"), None::<&str>),
            ("occasion", "arctic-trek", Some("Arctic exploration"), None::<&str>),
            ("occasion", "river-journey", Some("River travel"), None::<&str>),
            ("occasion", "hyperspace-transit", Some("FTL travel"), None::<&str>),
            ("occasion", "derelict-ship-exploration", Some("Abandoned ship"), None::<&str>),
            ("occasion", "space-station-walk", Some("Space station"), None::<&str>),
            
            // More social
            ("occasion", "inn", Some("Inn and lodging"), None::<&str>),
            ("occasion", "black-market", Some("Illegal trading"), None::<&str>),
            ("occasion", "audience-with-ruler", Some("Meeting royalty"), None::<&str>),
            ("occasion", "council-debate", Some("Political meetings"), None::<&str>),
            ("occasion", "religious-service", Some("Religious ceremonies"), None::<&str>),
            ("occasion", "gambling-den", Some("Gambling establishments"), None::<&str>),
            ("occasion", "speakeasy", Some("Hidden bars"), None::<&str>),
            ("occasion", "noir-club", Some("Jazz clubs"), None::<&str>),
            ("occasion", "tea-house", Some("Tea ceremonies"), None::<&str>),
            
            // More investigation
            ("occasion", "occult-research", Some("Supernatural investigation"), None::<&str>),
            ("occasion", "stakeout", Some("Surveillance operation"), None::<&str>),
            ("occasion", "tailing", Some("Following suspects"), None::<&str>),
            ("occasion", "safecracking", Some("Breaking safes"), None::<&str>),
            ("occasion", "netrun", Some("Cyberpunk hacking"), None::<&str>),
            ("occasion", "vault-breach", Some("Breaking into vaults"), None::<&str>),
            ("occasion", "disguise", Some("Undercover operations"), None::<&str>),
            ("occasion", "extraction", Some("Rescue operations"), None::<&str>),
            ("occasion", "clean-getaway", Some("Successful escape"), None::<&str>),
            
            // Puzzles & traps
            ("occasion", "riddle-solving", Some("Solving riddles"), None::<&str>),
            ("occasion", "mechanism-puzzle", Some("Mechanical puzzles"), None::<&str>),
            ("occasion", "arcane-puzzle", Some("Magic puzzles"), None::<&str>),
            ("occasion", "trap-primed", Some("Trap detection"), None::<&str>),
            ("occasion", "trap-triggered", Some("Trap activation"), None::<&str>),
            ("occasion", "chase-timer", Some("Timed pursuits"), None::<&str>),
            ("occasion", "bomb-timer", Some("Explosive countdowns"), None::<&str>),
            ("occasion", "reactor-meltdown", Some("Nuclear emergencies"), None::<&str>),
            ("occasion", "airlock-timer", Some("Space emergencies"), None::<&str>),
            
            // More combat
            ("occasion", "combat-naval", Some("Naval warfare"), None::<&str>),
            ("occasion", "combat-aerial", Some("Air combat"), None::<&str>),
            ("occasion", "combat-vehicular", Some("Vehicle combat"), None::<&str>),
            ("occasion", "combat-mecha", Some("Mecha battles"), None::<&str>),
            ("occasion", "combat-space-battle", Some("Space warfare"), None::<&str>),
            ("occasion", "car-chase", Some("Vehicle pursuit"), None::<&str>),
            ("occasion", "foot-chase", Some("Running pursuit"), None::<&str>),
            ("occasion", "dogfight", Some("Aerial combat"), None::<&str>),
            ("occasion", "boarding-action", Some("Ship boarding"), None::<&str>),
            
            // More horror
            ("occasion", "possession", Some("Demonic possession"), None::<&str>),
            ("occasion", "banishment", Some("Exorcism rituals"), None::<&str>),
            ("occasion", "sanity-slip", Some("Mental breakdown"), None::<&str>),
            ("occasion", "werewolf-hunt", Some("Lycanthrope encounters"), None::<&str>),
            ("occasion", "sacrificial-altar", Some("Ritual sacrifice"), None::<&str>),
            
            // More magic/sci-tech
            ("occasion", "telepathy", Some("Mind reading"), None::<&str>),
            ("occasion", "dream-walk", Some("Dream exploration"), None::<&str>),
            ("occasion", "astral-travel", Some("Astral projection"), None::<&str>),
            ("occasion", "time-warp", Some("Time manipulation"), None::<&str>),
            ("occasion", "biotech-lab", Some("Biological research"), None::<&str>),
            ("occasion", "nanotech-swarm", Some("Nanotechnology"), None::<&str>),
            ("occasion", "cyber-combat", Some("Digital warfare"), None::<&str>),
            ("occasion", "drone-control", Some("Drone operations"), None::<&str>),
            ("occasion", "mech-dock", Some("Mecha docking"), None::<&str>),
            ("occasion", "tractor-beam", Some("Gravity manipulation"), None::<&str>),
            ("occasion", "ship-docking", Some("Spacecraft docking"), None::<&str>),
            ("occasion", "eva-walk", Some("Spacewalk"), None::<&str>),
            
            // More crafting/survival
            ("occasion", "foraging", Some("Gathering resources"), None::<&str>),
            ("occasion", "tracking", Some("Following trails"), None::<&str>),
            ("occasion", "fletching", Some("Arrow making"), None::<&str>),
            ("occasion", "cooking", Some("Food preparation"), None::<&str>),
            ("occasion", "base-building", Some("Construction"), None::<&str>),
            ("occasion", "bargain", Some("Price negotiation"), None::<&str>),
            ("occasion", "healing", Some("Medical treatment"), None::<&str>),
            ("occasion", "hospital-ward", Some("Medical facilities"), None::<&str>),
            
            // More environment
            ("occasion", "rain", Some("Rainy weather"), None::<&str>),
            ("occasion", "blizzard", Some("Snow storms"), None::<&str>),
            ("occasion", "sandstorm", Some("Desert storms"), None::<&str>),
            ("occasion", "flood", Some("Flooding events"), None::<&str>),
            ("occasion", "meteor-shower", Some("Celestial events"), None::<&str>),
            ("occasion", "radiation-storm", Some("Radioactive weather"), None::<&str>),
            ("occasion", "anomaly-event", Some("Strange phenomena"), None::<&str>),
            ("occasion", "void-rift", Some("Dimensional tears"), None::<&str>),
            ("occasion", "volcanic-eruption", Some("Volcanic activity"), None::<&str>),
            
            // Transitions & UI
            ("occasion", "scene-transition", Some("Changing scenes"), None::<&str>),
            ("occasion", "reveal-stinger", Some("Dramatic reveals"), None::<&str>),
            ("occasion", "mystery-sting", Some("Clue discoveries"), None::<&str>),
            ("occasion", "map-open", Some("Opening maps"), None::<&str>),
            ("occasion", "map-close", Some("Closing maps"), None::<&str>),
            ("occasion", "quest-accepted", Some("Mission accepted"), None::<&str>),
            ("occasion", "quest-failed", Some("Mission failed"), None::<&str>),
            ("occasion", "dice-roll", Some("Random outcomes"), None::<&str>),
            ("occasion", "success-cue", Some("Positive results"), None::<&str>),
            ("occasion", "failure-cue", Some("Negative results"), None::<&str>),
        ];

        // Insert keyword vocabulary (COMPLETE from TAGS.md specification)
        let keywords = vec![
            // Biomes & Terrain
            ("keyword", "biome:forest", Some("Forest environments"), None::<&str>),
            ("keyword", "biome:ancient-forest", Some("Old growth forests"), None::<&str>),
            ("keyword", "biome:rainforest", Some("Tropical rainforests"), None::<&str>),
            ("keyword", "biome:swamp", Some("Wetland swamps"), None::<&str>),
            ("keyword", "biome:bog", Some("Peat bogs"), None::<&str>),
            ("keyword", "biome:marsh", Some("Marshy wetlands"), None::<&str>),
            ("keyword", "biome:desert", Some("Arid desert"), None::<&str>),
            ("keyword", "biome:dunes", Some("Sand dunes"), None::<&str>),
            ("keyword", "biome:oasis", Some("Desert oasis"), None::<&str>),
            ("keyword", "biome:arctic", Some("Arctic regions"), None::<&str>),
            ("keyword", "biome:tundra", Some("Frozen tundra"), None::<&str>),
            ("keyword", "biome:glacier", Some("Glacial ice"), None::<&str>),
            ("keyword", "biome:mountain", Some("Mountain ranges"), None::<&str>),
            ("keyword", "biome:canyon", Some("Deep canyons"), None::<&str>),
            ("keyword", "biome:steppe", Some("Grassland steppes"), None::<&str>),
            ("keyword", "biome:plains", Some("Open plains"), None::<&str>),
            ("keyword", "biome:grassland", Some("Grassy areas"), None::<&str>),
            ("keyword", "biome:jungle", Some("Dense jungle"), None::<&str>),
            ("keyword", "biome:savanna", Some("African savanna"), None::<&str>),
            ("keyword", "biome:volcanic", Some("Volcanic regions"), None::<&str>),
            ("keyword", "biome:underdark", Some("Underground realms"), None::<&str>),
            ("keyword", "biome:cave", Some("Natural caves"), None::<&str>),
            ("keyword", "biome:sewers", Some("Underground sewers"), None::<&str>),
            ("keyword", "biome:coast", Some("Coastal areas"), None::<&str>),
            ("keyword", "biome:open-sea", Some("Open ocean"), None::<&str>),
            ("keyword", "biome:river", Some("Rivers and streams"), None::<&str>),
            ("keyword", "biome:lake", Some("Lakes and ponds"), None::<&str>),
            ("keyword", "biome:sky", Some("Aerial environments"), None::<&str>),
            ("keyword", "biome:astral", Some("Astral plane"), None::<&str>),
            ("keyword", "biome:void", Some("Empty void"), None::<&str>),
            ("keyword", "biome:otherworld", Some("Other dimensions"), None::<&str>),

            // Locations & Structures
            ("keyword", "loc:castle", Some("Medieval castles"), None::<&str>),
            ("keyword", "loc:keep", Some("Fortified keeps"), None::<&str>),
            ("keyword", "loc:fortress", Some("Military fortresses"), None::<&str>),
            ("keyword", "loc:watchtower", Some("Guard towers"), None::<&str>),
            ("keyword", "loc:dungeon", Some("Underground dungeons"), None::<&str>),
            ("keyword", "loc:catacombs", Some("Underground burial chambers"), None::<&str>),
            ("keyword", "loc:crypt", Some("Burial crypts"), None::<&str>),
            ("keyword", "loc:temple", Some("Religious temples"), None::<&str>),
            ("keyword", "loc:shrine", Some("Sacred shrines"), None::<&str>),
            ("keyword", "loc:monastery", Some("Religious monasteries"), None::<&str>),
            ("keyword", "loc:library", Some("Knowledge repositories"), None::<&str>),
            ("keyword", "loc:academy", Some("Learning institutions"), None::<&str>),
            ("keyword", "loc:mage-tower", Some("Wizard towers"), None::<&str>),
            ("keyword", "loc:throne-room", Some("Royal chambers"), None::<&str>),
            ("keyword", "loc:market", Some("Trading markets"), None::<&str>),
            ("keyword", "loc:slums", Some("Poor districts"), None::<&str>),
            ("keyword", "loc:harbor", Some("Ship harbors"), None::<&str>),
            ("keyword", "loc:mine", Some("Mining operations"), None::<&str>),
            ("keyword", "loc:smithy", Some("Blacksmith workshops"), None::<&str>),
            ("keyword", "loc:inn", Some("Traveler inns"), None::<&str>),
            ("keyword", "loc:tavern", Some("Drinking establishments"), None::<&str>),
            ("keyword", "loc:prison", Some("Detention facilities"), None::<&str>),
            ("keyword", "loc:arena", Some("Combat arenas"), None::<&str>),
            ("keyword", "loc:laboratory", Some("Research labs"), None::<&str>),
            ("keyword", "loc:biolab", Some("Biological labs"), None::<&str>),
            ("keyword", "loc:reactor", Some("Nuclear reactors"), None::<&str>),
            ("keyword", "loc:spaceport", Some("Space travel hubs"), None::<&str>),
            ("keyword", "loc:hangar", Some("Vehicle storage"), None::<&str>),
            ("keyword", "loc:bridge-deck", Some("Ship bridges"), None::<&str>),
            ("keyword", "loc:engineering", Some("Engineering sections"), None::<&str>),
            ("keyword", "loc:cryosleep-bay", Some("Hibernation chambers"), None::<&str>),
            ("keyword", "loc:cargo-bay", Some("Storage compartments"), None::<&str>),
            ("keyword", "loc:derelict", Some("Abandoned structures"), None::<&str>),
            ("keyword", "loc:ruins", Some("Ancient ruins"), None::<&str>),
            ("keyword", "loc:ancient-city", Some("Lost civilizations"), None::<&str>),
            ("keyword", "loc:fairy-glen", Some("Magical clearings"), None::<&str>),

            // Cultures, Eras & Styles
            ("keyword", "style:medieval-european", Some("Medieval European"), None::<&str>),
            ("keyword", "style:renaissance", Some("Renaissance period"), None::<&str>),
            ("keyword", "style:baroque", Some("Baroque era"), None::<&str>),
            ("keyword", "style:romantic-era", Some("Romantic period"), None::<&str>),
            ("keyword", "style:ancient-greek", Some("Ancient Greek"), None::<&str>),
            ("keyword", "style:ancient-roman", Some("Ancient Roman"), None::<&str>),
            ("keyword", "style:egyptian", Some("Ancient Egyptian"), None::<&str>),
            ("keyword", "style:norse", Some("Norse/Viking"), None::<&str>),
            ("keyword", "style:celtic", Some("Celtic traditions"), None::<&str>),
            ("keyword", "style:arabesque", Some("Middle Eastern"), None::<&str>),
            ("keyword", "style:persian", Some("Persian culture"), None::<&str>),
            ("keyword", "style:ottoman", Some("Ottoman Empire"), None::<&str>),
            ("keyword", "style:indian-classical", Some("Indian classical"), None::<&str>),
            ("keyword", "style:japanese-traditional", Some("Traditional Japanese"), None::<&str>),
            ("keyword", "style:chinese-traditional", Some("Traditional Chinese"), None::<&str>),
            ("keyword", "style:korean-traditional", Some("Traditional Korean"), None::<&str>),
            ("keyword", "style:balinese-gamelan", Some("Balinese gamelan"), None::<&str>),
            ("keyword", "style:andino", Some("Andean culture"), None::<&str>),
            ("keyword", "style:west-african", Some("West African"), None::<&str>),
            ("keyword", "style:native-north-american", Some("Native North American"), None::<&str>),
            ("keyword", "style:mexican-folk", Some("Mexican folk"), None::<&str>),
            ("keyword", "style:iberian-folk", Some("Iberian folk"), None::<&str>),
            ("keyword", "style:byzantine-chant", Some("Byzantine chant"), None::<&str>),
            ("keyword", "style:gothic", Some("Gothic period"), None::<&str>),
            ("keyword", "style:western-frontier", Some("American frontier"), None::<&str>),
            ("keyword", "style:1920s", Some("1920s era"), None::<&str>),
            ("keyword", "style:noir-1940s", Some("1940s film noir"), None::<&str>),
            ("keyword", "style:cold-war", Some("Cold War era"), None::<&str>),
            ("keyword", "style:cyberpunk-neon", Some("Cyberpunk aesthetic"), None::<&str>),
            ("keyword", "style:dieselpunk", Some("Dieselpunk aesthetic"), None::<&str>),
            ("keyword", "style:steampunk", Some("Steampunk aesthetic"), None::<&str>),
            ("keyword", "style:post-apocalyptic", Some("Post-apocalyptic"), None::<&str>),

            // Creatures & Foes
            ("keyword", "creature:dragon", Some("Dragons"), None::<&str>),
            ("keyword", "creature:drake", Some("Drakes"), None::<&str>),
            ("keyword", "creature:wyvern", Some("Wyverns"), None::<&str>),
            ("keyword", "creature:giant", Some("Giants"), None::<&str>),
            ("keyword", "creature:troll", Some("Trolls"), None::<&str>),
            ("keyword", "creature:ogre", Some("Ogres"), None::<&str>),
            ("keyword", "creature:goblin", Some("Goblins"), None::<&str>),
            ("keyword", "creature:orc", Some("Orcs"), None::<&str>),
            ("keyword", "creature:kobold", Some("Kobolds"), None::<&str>),
            ("keyword", "creature:gnoll", Some("Gnolls"), None::<&str>),
            ("keyword", "creature:undead", Some("Undead creatures"), None::<&str>),
            ("keyword", "creature:zombie", Some("Zombies"), None::<&str>),
            ("keyword", "creature:ghoul", Some("Ghouls"), None::<&str>),
            ("keyword", "creature:skeleton", Some("Skeletons"), None::<&str>),
            ("keyword", "creature:lich", Some("Liches"), None::<&str>),
            ("keyword", "creature:vampire", Some("Vampires"), None::<&str>),
            ("keyword", "creature:werewolf", Some("Werewolves"), None::<&str>),
            ("keyword", "creature:ghost", Some("Ghosts"), None::<&str>),
            ("keyword", "creature:demon", Some("Demons"), None::<&str>),
            ("keyword", "creature:devil", Some("Devils"), None::<&str>),
            ("keyword", "creature:angel", Some("Angels"), None::<&str>),
            ("keyword", "creature:fae", Some("Fae creatures"), None::<&str>),
            ("keyword", "creature:dryad", Some("Dryads"), None::<&str>),
            ("keyword", "creature:elemental", Some("Elementals"), None::<&str>),
            ("keyword", "creature:construct", Some("Constructs"), None::<&str>),
            ("keyword", "creature:golem", Some("Golems"), None::<&str>),
            ("keyword", "creature:slime", Some("Slimes"), None::<&str>),
            ("keyword", "creature:beast", Some("Wild beasts"), None::<&str>),
            ("keyword", "creature:wolf", Some("Wolves"), None::<&str>),
            ("keyword", "creature:bear", Some("Bears"), None::<&str>),
            ("keyword", "creature:spider", Some("Spiders"), None::<&str>),
            ("keyword", "creature:kraken", Some("Krakens"), None::<&str>),
            ("keyword", "creature:siren", Some("Sirens"), None::<&str>),
            ("keyword", "creature:aberration", Some("Aberrations"), None::<&str>),
            ("keyword", "creature:eldritch-entity", Some("Cosmic horrors"), None::<&str>),
            ("keyword", "creature:alien", Some("Extraterrestrials"), None::<&str>),
            ("keyword", "creature:parasite", Some("Parasitic organisms"), None::<&str>),
            ("keyword", "creature:machine-swarm", Some("Robot swarms"), None::<&str>),

            // Factions & NPC Types
            ("keyword", "faction:empire", Some("Imperial forces"), None::<&str>),
            ("keyword", "faction:rebels", Some("Rebel groups"), None::<&str>),
            ("keyword", "faction:mercenary-band", Some("Mercenary companies"), None::<&str>),
            ("keyword", "faction:thieves-guild", Some("Criminal organizations"), None::<&str>),
            ("keyword", "faction:mages-guild", Some("Wizard organizations"), None::<&str>),
            ("keyword", "faction:knightly-order", Some("Knight orders"), None::<&str>),
            ("keyword", "faction:church", Some("Religious institutions"), None::<&str>),
            ("keyword", "faction:cult", Some("Cult groups"), None::<&str>),
            ("keyword", "faction:cartel", Some("Criminal cartels"), None::<&str>),
            ("keyword", "faction:megacorp", Some("Mega corporations"), None::<&str>),
            ("keyword", "faction:syndicate", Some("Crime syndicates"), None::<&str>),
            ("keyword", "faction:raiders", Some("Raider groups"), None::<&str>),
            ("keyword", "faction:mutants", Some("Mutant factions"), None::<&str>),
            ("keyword", "faction:androids", Some("Android collectives"), None::<&str>),
            ("keyword", "npc:noble", Some("Nobility"), None::<&str>),
            ("keyword", "npc:merchant", Some("Traders"), None::<&str>),
            ("keyword", "npc:smuggler", Some("Smugglers"), None::<&str>),
            ("keyword", "npc:guard", Some("Guards"), None::<&str>),
            ("keyword", "npc:assassin", Some("Assassins"), None::<&str>),
            ("keyword", "npc:priest", Some("Religious figures"), None::<&str>),
            ("keyword", "npc:witch", Some("Witches"), None::<&str>),
            ("keyword", "npc:warlock", Some("Warlocks"), None::<&str>),
            ("keyword", "npc:necromancer", Some("Necromancers"), None::<&str>),
            ("keyword", "npc:ranger", Some("Rangers"), None::<&str>),
            ("keyword", "npc:alchemist", Some("Alchemists"), None::<&str>),
            ("keyword", "npc:scientist", Some("Scientists"), None::<&str>),
            ("keyword", "npc:hacker", Some("Hackers"), None::<&str>),
            ("keyword", "npc:pilot", Some("Pilots"), None::<&str>),
            ("keyword", "npc:ai", Some("Artificial intelligences"), None::<&str>),

            // Magic, Powers & Elements
            ("keyword", "magic:abjuration", Some("Protective magic"), None::<&str>),
            ("keyword", "magic:conjuration", Some("Summoning magic"), None::<&str>),
            ("keyword", "magic:divination", Some("Knowledge magic"), None::<&str>),
            ("keyword", "magic:enchantment", Some("Mind magic"), None::<&str>),
            ("keyword", "magic:evocation", Some("Energy magic"), None::<&str>),
            ("keyword", "magic:illusion", Some("Deception magic"), None::<&str>),
            ("keyword", "magic:necromancy", Some("Death magic"), None::<&str>),
            ("keyword", "magic:transmutation", Some("Transformation magic"), None::<&str>),
            ("keyword", "element:fire", Some("Fire element"), None::<&str>),
            ("keyword", "element:ice", Some("Ice element"), None::<&str>),
            ("keyword", "element:lightning", Some("Lightning element"), None::<&str>),
            ("keyword", "element:wind", Some("Wind element"), None::<&str>),
            ("keyword", "element:earth", Some("Earth element"), None::<&str>),
            ("keyword", "element:water", Some("Water element"), None::<&str>),
            ("keyword", "element:poison", Some("Poison element"), None::<&str>),
            ("keyword", "element:acid", Some("Acid element"), None::<&str>),
            ("keyword", "element:shadow", Some("Shadow element"), None::<&str>),
            ("keyword", "element:light", Some("Light element"), None::<&str>),
            ("keyword", "element:void", Some("Void element"), None::<&str>),
            ("keyword", "element:metal", Some("Metal element"), None::<&str>),
            ("keyword", "element:wood", Some("Wood element"), None::<&str>),
            ("keyword", "ritual:blood-magic", Some("Blood rituals"), None::<&str>),
            ("keyword", "ritual:summoning", Some("Summoning rituals"), None::<&str>),
            ("keyword", "ritual:sacrifice", Some("Sacrificial rituals"), None::<&str>),
            ("keyword", "ritual:binding", Some("Binding rituals"), None::<&str>),
            ("keyword", "ritual:banishment", Some("Banishment rituals"), None::<&str>),

            // Technology & Vehicles
            ("keyword", "tech:medieval", Some("Medieval technology"), None::<&str>),
            ("keyword", "tech:clockwork", Some("Clockwork mechanisms"), None::<&str>),
            ("keyword", "tech:steam", Some("Steam technology"), None::<&str>),
            ("keyword", "tech:diesel", Some("Diesel technology"), None::<&str>),
            ("keyword", "tech:nuclear", Some("Nuclear technology"), None::<&str>),
            ("keyword", "tech:fusion", Some("Fusion technology"), None::<&str>),
            ("keyword", "tech:antimatter", Some("Antimatter technology"), None::<&str>),
            ("keyword", "tech:biotech", Some("Biotechnology"), None::<&str>),
            ("keyword", "tech:nanotech", Some("Nanotechnology"), None::<&str>),
            ("keyword", "tech:ai", Some("Artificial intelligence"), None::<&str>),
            ("keyword", "tech:cybernetics", Some("Cybernetic technology"), None::<&str>),
            ("keyword", "tech:synthetic", Some("Synthetic technology"), None::<&str>),
            ("keyword", "vehicle:horse", Some("Horses"), None::<&str>),
            ("keyword", "vehicle:carriage", Some("Carriages"), None::<&str>),
            ("keyword", "vehicle:war-wagon", Some("War wagons"), None::<&str>),
            ("keyword", "vehicle:train", Some("Trains"), None::<&str>),
            ("keyword", "vehicle:subway", Some("Subway trains"), None::<&str>),
            ("keyword", "vehicle:motorbike", Some("Motorcycles"), None::<&str>),
            ("keyword", "vehicle:armored-vehicle", Some("Armored vehicles"), None::<&str>),
            ("keyword", "vehicle:airship", Some("Airships"), None::<&str>),
            ("keyword", "vehicle:zeppelin", Some("Zeppelins"), None::<&str>),
            ("keyword", "vehicle:airplane", Some("Airplanes"), None::<&str>),
            ("keyword", "vehicle:helicopter", Some("Helicopters"), None::<&str>),
            ("keyword", "vehicle:mech", Some("Mecha suits"), None::<&str>),
            ("keyword", "vehicle:tank", Some("Tanks"), None::<&str>),
            ("keyword", "vehicle:starfighter", Some("Space fighters"), None::<&str>),
            ("keyword", "vehicle:shuttle", Some("Space shuttles"), None::<&str>),
            ("keyword", "vehicle:frigate", Some("Space frigates"), None::<&str>),
            ("keyword", "vehicle:battleship", Some("Battleships"), None::<&str>),
            ("keyword", "vehicle:freighter", Some("Cargo ships"), None::<&str>),

            // Weather & Natural Phenomena
            ("keyword", "weather:clear", Some("Clear weather"), None::<&str>),
            ("keyword", "weather:wind", Some("Windy conditions"), None::<&str>),
            ("keyword", "weather:rain", Some("Rainfall"), None::<&str>),
            ("keyword", "weather:thunderstorm", Some("Thunderstorms"), None::<&str>),
            ("keyword", "weather:snow", Some("Snowfall"), None::<&str>),
            ("keyword", "weather:blizzard", Some("Blizzards"), None::<&str>),
            ("keyword", "weather:heatwave", Some("Extreme heat"), None::<&str>),
            ("keyword", "weather:sandstorm", Some("Sandstorms"), None::<&str>),
            ("keyword", "weather:fog", Some("Fog"), None::<&str>),
            ("keyword", "weather:mist", Some("Mist"), None::<&str>),
            ("keyword", "weather:hail", Some("Hailstorms"), None::<&str>),
            ("keyword", "weather:aurora", Some("Aurora phenomena"), None::<&str>),
            ("keyword", "weather:eclipse", Some("Eclipses"), None::<&str>),
            ("keyword", "weather:meteor-shower", Some("Meteor showers"), None::<&str>),
            ("keyword", "weather:solar-storm", Some("Solar storms"), None::<&str>),
            ("keyword", "weather:radiation-storm", Some("Radiation storms"), None::<&str>),
            ("keyword", "weather:anomaly", Some("Weather anomalies"), None::<&str>),

            // Instruments & Timbres
            ("keyword", "timbre:strings-warm", Some("Warm strings"), None::<&str>),
            ("keyword", "timbre:strings-dissonant", Some("Dissonant strings"), None::<&str>),
            ("keyword", "timbre:low-brass", Some("Low brass"), None::<&str>),
            ("keyword", "timbre:braams", Some("Braams"), None::<&str>),
            ("keyword", "timbre:solo-violin", Some("Solo violin"), None::<&str>),
            ("keyword", "timbre:solo-cello", Some("Solo cello"), None::<&str>),
            ("keyword", "timbre:harp", Some("Harp"), None::<&str>),
            ("keyword", "timbre:flute", Some("Flute"), None::<&str>),
            ("keyword", "timbre:whistle", Some("Whistle"), None::<&str>),
            ("keyword", "timbre:bagpipes", Some("Bagpipes"), None::<&str>),
            ("keyword", "timbre:nyckelharpa", Some("Nyckelharpa"), None::<&str>),
            ("keyword", "timbre:hurdy-gurdy", Some("Hurdy-gurdy"), None::<&str>),
            ("keyword", "timbre:oud", Some("Oud"), None::<&str>),
            ("keyword", "timbre:sitar", Some("Sitar"), None::<&str>),
            ("keyword", "timbre:shakuhachi", Some("Shakuhachi flute"), None::<&str>),
            ("keyword", "timbre:erhu", Some("Erhu"), None::<&str>),
            ("keyword", "timbre:guzheng", Some("Guzheng"), None::<&str>),
            ("keyword", "timbre:koto", Some("Koto"), None::<&str>),
            ("keyword", "timbre:gamelan", Some("Gamelan orchestra"), None::<&str>),
            ("keyword", "timbre:frame-drum", Some("Frame drum"), None::<&str>),
            ("keyword", "timbre:taiko", Some("Taiko drums"), None::<&str>),
            ("keyword", "timbre:bodhran", Some("Bodhran drum"), None::<&str>),
            ("keyword", "timbre:dulcimer", Some("Dulcimer"), None::<&str>),
            ("keyword", "timbre:lute", Some("Lute"), None::<&str>),
            ("keyword", "timbre:organ", Some("Organ"), None::<&str>),
            ("keyword", "timbre:church-choir", Some("Church choir"), None::<&str>),
            ("keyword", "timbre:male-chant", Some("Male chanting"), None::<&str>),
            ("keyword", "timbre:female-vocalise", Some("Female vocals"), None::<&str>),
            ("keyword", "timbre:child-choir", Some("Children's choir"), None::<&str>),
            ("keyword", "timbre:synthetic-pad", Some("Synthesizer pad"), None::<&str>),
            ("keyword", "timbre:analog-synth", Some("Analog synthesizer"), None::<&str>),
            ("keyword", "timbre:fm-synth", Some("FM synthesis"), None::<&str>),
            ("keyword", "timbre:granular", Some("Granular synthesis"), None::<&str>),
            ("keyword", "timbre:noise-texture", Some("Noise textures"), None::<&str>),
            ("keyword", "timbre:clockwork", Some("Clockwork sounds"), None::<&str>),
            ("keyword", "timbre:metallic-hits", Some("Metallic percussion"), None::<&str>),
            ("keyword", "timbre:sub-boom", Some("Sub bass"), None::<&str>),

            // SFX & Foley
            ("keyword", "sfx:footsteps", Some("Footstep sounds"), None::<&str>),
            ("keyword", "sfx:armor-clank", Some("Armor sounds"), None::<&str>),
            ("keyword", "sfx:sword-clash", Some("Sword fighting"), None::<&str>),
            ("keyword", "sfx:bow-release", Some("Bow and arrow"), None::<&str>),
            ("keyword", "sfx:gunshot", Some("Gunfire"), None::<&str>),
            ("keyword", "sfx:reload", Some("Weapon reloading"), None::<&str>),
            ("keyword", "sfx:ricochet", Some("Bullet ricochet"), None::<&str>),
            ("keyword", "sfx:explosion", Some("Explosions"), None::<&str>),
            ("keyword", "sfx:door-creak", Some("Creaking doors"), None::<&str>),
            ("keyword", "sfx:gate-bang", Some("Slamming gates"), None::<&str>),
            ("keyword", "sfx:chains", Some("Chain sounds"), None::<&str>),
            ("keyword", "sfx:coins", Some("Coin sounds"), None::<&str>),
            ("keyword", "sfx:pages-turning", Some("Book pages"), None::<&str>),
            ("keyword", "sfx:quill-scratch", Some("Writing sounds"), None::<&str>),
            ("keyword", "sfx:campfire", Some("Fire crackling"), None::<&str>),
            ("keyword", "sfx:wood-crackle", Some("Wood burning"), None::<&str>),
            ("keyword", "sfx:water-drip", Some("Dripping water"), None::<&str>),
            ("keyword", "sfx:river", Some("Flowing water"), None::<&str>),
            ("keyword", "sfx:ocean-surf", Some("Ocean waves"), None::<&str>),
            ("keyword", "sfx:thunder", Some("Thunder sounds"), None::<&str>),
            ("keyword", "sfx:wind", Some("Wind sounds"), None::<&str>),
            ("keyword", "sfx:rain", Some("Rain sounds"), None::<&str>),
            ("keyword", "sfx:market-crowd", Some("Market crowds"), None::<&str>),
            ("keyword", "sfx:tavern-murmur", Some("Tavern chatter"), None::<&str>),
            ("keyword", "sfx:church-bells", Some("Church bells"), None::<&str>),
            ("keyword", "sfx:chant", Some("Chanting voices"), None::<&str>),
            ("keyword", "sfx:chant-latin", Some("Latin chanting"), None::<&str>),
            ("keyword", "sfx:monster-roar", Some("Monster sounds"), None::<&str>),
            ("keyword", "sfx:dragon-breath", Some("Dragon breathing"), None::<&str>),
            ("keyword", "sfx:zombie-moans", Some("Zombie sounds"), None::<&str>),
            ("keyword", "sfx:ghost-wail", Some("Ghost sounds"), None::<&str>),
            ("keyword", "sfx:space-engine-hum", Some("Spaceship engines"), None::<&str>),
            ("keyword", "sfx:alarm", Some("Alarm sounds"), None::<&str>),
            ("keyword", "sfx:scanner-beeps", Some("Scanner beeps"), None::<&str>),
            ("keyword", "sfx:keyboard", Some("Typing sounds"), None::<&str>),
            ("keyword", "sfx:hacking-glitches", Some("Digital glitches"), None::<&str>),
            ("keyword", "sfx:warp", Some("Warp drive"), None::<&str>),
            ("keyword", "sfx:teleport", Some("Teleportation"), None::<&str>),
            ("keyword", "sfx:magic-whoosh", Some("Magic effects"), None::<&str>),
            ("keyword", "sfx:spell-impact", Some("Spell impacts"), None::<&str>),
            ("keyword", "sfx:shield", Some("Shield effects"), None::<&str>),
            ("keyword", "sfx:portal-open", Some("Portal opening"), None::<&str>),
            ("keyword", "sfx:portal-close", Some("Portal closing"), None::<&str>),

            // Audio Structure & Utility
            ("keyword", "util:loopable", Some("Loopable track"), None::<&str>),
            ("keyword", "util:stinger", Some("Short stinger"), None::<&str>),
            ("keyword", "util:intro", Some("Introduction section"), None::<&str>),
            ("keyword", "util:outro", Some("Ending section"), None::<&str>),
            ("keyword", "util:transition", Some("Transition piece"), None::<&str>),
            ("keyword", "util:bed", Some("Background bed"), None::<&str>),
            ("keyword", "util:drone", Some("Sustained drone"), None::<&str>),
            ("keyword", "util:motif", Some("Musical motif"), None::<&str>),
            ("keyword", "util:theme", Some("Thematic music"), None::<&str>),
            ("keyword", "util:alt-mix", Some("Alternative mix"), None::<&str>),
            ("keyword", "util:instrumental", Some("No vocals"), None::<&str>),
            ("keyword", "util:with-vocals", Some("Includes vocals"), None::<&str>),
            ("keyword", "util:diegetic", Some("Source music"), None::<&str>),
            ("keyword", "util:non-diegetic", Some("Score music"), None::<&str>),
            ("keyword", "util:stem-percussion", Some("Percussion stem"), None::<&str>),
            ("keyword", "util:stem-ambient", Some("Ambient stem"), None::<&str>),
            ("keyword", "util:stem-melody", Some("Melody stem"), None::<&str>),
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
                ?41, ?42, ?43, ?44, ?45, ?46, ?47, ?48, ?49, ?50
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
    pub fn add_tag_vocabulary(&self, tag_type: &str, tag_value: &str, description: Option<&str>, parent_tag: Option<&str>, is_active: bool) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO tag_vocabulary (tag_type, tag_value, description, parent_tag, is_active)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![tag_type, tag_value, description, parent_tag, is_active],
        )?;
        Ok(())
    }

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

    pub fn clear_all_data(&self) -> Result<()> {
        // Clear all user data but keep vocabulary
        self.conn.execute("DELETE FROM rpg_tags", [])?;
        self.conn.execute("DELETE FROM audio_files", [])?;
        
        // Reset auto-increment counters
        self.conn.execute("DELETE FROM sqlite_sequence WHERE name='audio_files'", [])?;
        self.conn.execute("DELETE FROM sqlite_sequence WHERE name='rpg_tags'", [])?;
        
        Ok(())
    }

    pub fn update_audio_file_duration(&self, id: i64, duration: f64) -> Result<()> {
        self.conn.execute(
            "UPDATE audio_files SET duration = ?1 WHERE id = ?2",
            params![duration, id],
        )?;
        Ok(())
    }

    pub fn update_audio_file_bpm(&self, id: i64, bpm: u32) -> Result<()> {
        self.conn.execute(
            "UPDATE audio_files SET bpm = ?1 WHERE id = ?2",
            params![bpm, id],
        )?;
        Ok(())
    }

    pub fn update_audio_file_duration_and_bpm(&self, id: i64, duration: Option<f64>, bpm: Option<u32>) -> Result<()> {
        match (duration, bpm) {
            (Some(d), Some(b)) => {
                self.conn.execute(
                    "UPDATE audio_files SET duration = ?1, bpm = ?2 WHERE id = ?3",
                    params![d, b, id],
                )?;
            }
            (Some(d), None) => {
                self.conn.execute(
                    "UPDATE audio_files SET duration = ?1 WHERE id = ?2",
                    params![d, id],
                )?;
            }
            (None, Some(b)) => {
                self.conn.execute(
                    "UPDATE audio_files SET bpm = ?1 WHERE id = ?2",
                    params![b, id],
                )?;
            }
            (None, None) => {
                // Nothing to update
                return Ok(());
            }
        }
        Ok(())
    }
}