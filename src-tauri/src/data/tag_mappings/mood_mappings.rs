use std::collections::HashMap;

/// Mood tag to folder mappings with confidence scores
/// Format: (tag, [(folder_path, confidence_score)])
/// Confidence: 5-10 (5=low, 10=perfect match)
pub const MOOD_FOLDER_MAPPINGS: &[(&str, &[(&str, u8)])] = &[
    // Positive/Heroic moods
    ("heroic", &[("Music/Orchestral/Epic Orchestral", 10), ("Mood/Positive/Heroic", 10)]),
    ("triumphant", &[("Music/Orchestral/Epic Orchestral", 10), ("Mood/Positive/Triumphant", 10)]),
    ("noble", &[("Music/Orchestral/Epic Orchestral", 9), ("Mood/Positive/Noble", 10)]),
    ("uplifting", &[("Music/Orchestral/Epic Orchestral", 8), ("Mood/Positive/Uplifting", 10)]),
    ("hopeful", &[("Music/Electronic/Ambient Electronic", 8), ("Mood/Positive/Hopeful", 10)]),
    ("inspiring", &[("Music/Orchestral/Epic Orchestral", 9), ("Mood/Positive/Inspiring", 10)]),
    ("adventurous", &[("Music/Folk & World", 8), ("Mood/Positive/Adventurous", 10)]),
    ("confident", &[("Music/Jazz & Blues/Swing", 8), ("Mood/Positive/Confident", 10)]),
    ("victorious", &[("Music/Orchestral/Epic Orchestral", 10), ("Mood/Positive/Victorious", 10)]),

    // Playful/Light moods  
    ("festive", &[("Social/Entertainment/Festivals", 10), ("Mood/Playful/Festive", 10)]),
    ("playful", &[("Music/Folk & World", 7), ("Mood/Playful/Playful", 10)]),
    ("whimsical", &[("Environments/Magical Realms/Fairy Realms", 9), ("Mood/Playful/Whimsical", 10)]),
    ("merry", &[("Music/Folk & World/Sea Shanties", 8), ("Mood/Playful/Merry", 10)]),
    ("lighthearted", &[("Music/Jazz & Blues/Swing", 8), ("Mood/Playful/Lighthearted", 10)]),

    // Romantic/Gentle moods
    ("tender", &[("Music/Electronic/Ambient Electronic", 8), ("Mood/Romantic/Tender", 10)]),
    ("romantic", &[("Social/Ceremonies/Weddings", 9), ("Mood/Romantic/Romantic", 10)]),
    ("serene", &[("Environments/Natural Landscapes", 9), ("Mood/Romantic/Serene", 10)]),
    ("pastoral", &[("Environments/Natural Landscapes", 10), ("Mood/Romantic/Pastoral", 10)]),
    ("warm", &[("Music/Electronic/Ambient Electronic", 7), ("Mood/Romantic/Warm", 10)]),
    ("comforting", &[("Music/Electronic/Ambient Electronic", 7), ("Mood/Romantic/Comforting", 10)]),
    ("nostalgic", &[("Music/Jazz & Blues", 9), ("Mood/Romantic/Nostalgic", 10)]),
    ("bittersweet", &[("Music/Orchestral", 8), ("Mood/Romantic/Bittersweet", 10)]),

    // Mysterious/Arcane moods
    ("mysterious", &[("Music/Electronic/Drone", 9), ("Mood/Mysterious/Mysterious", 10)]),
    ("enigmatic", &[("Magic/Spell Schools/Illusion", 9), ("Mood/Mysterious/Enigmatic", 10)]),
    ("curious", &[("Social/Conversations", 7), ("Mood/Mysterious/Curious", 10)]),
    ("contemplative", &[("Music/Electronic/Ambient Electronic", 8), ("Mood/Mysterious/Contemplative", 10)]),
    ("dreamlike", &[("Environments/Magical Realms", 8), ("Mood/Mysterious/Dreamlike", 10)]),
    ("ethereal", &[("Music/Electronic/Ambient Electronic", 9), ("Mood/Mysterious/Ethereal", 10)]),
    ("mythic", &[("Magic/Magical Creatures", 9), ("Mood/Mysterious/Mythic", 10)]),
    ("arcane", &[("Magic/Spell Schools", 9), ("Mood/Mysterious/Arcane", 10)]),
    ("otherworldly", &[("Environments/Magical Realms", 10), ("Mood/Mysterious/Otherworldly", 10)]),

    // Sacred/Ceremonial moods
    ("solemn", &[("Social/Ceremonies/Funerals", 10), ("Mood/Sacred/Solemn", 10)]),
    ("ceremonial", &[("Social/Ceremonies", 9), ("Mood/Sacred/Ceremonial", 10)]),
    ("sacred", &[("Environments/Settlements/Temples", 9), ("Mood/Sacred/Sacred", 10)]),
    ("ritualistic", &[("Magic/Rituals & Ceremonies", 10), ("Mood/Sacred/Ritualistic", 10)]),
    ("austere", &[("Music/Orchestral/Minimalist Orchestral", 9), ("Mood/Sacred/Austere", 10)]),
    ("stoic", &[("Music/Orchestral/Minimalist Orchestral", 8), ("Mood/Sacred/Stoic", 10)]),

    // Dark/Threatening moods
    ("ominous", &[("Music/Horror & Tension", 9), ("Mood/Dark/Ominous", 10)]),
    ("foreboding", &[("Music/Horror & Tension", 9), ("Mood/Dark/Foreboding", 10)]),
    ("tense", &[("Combat/Combat Phases/Pre-Battle", 9), ("Mood/Dark/Tense", 10)]),
    ("suspenseful", &[("Music/Horror & Tension", 10), ("Mood/Dark/Suspenseful", 10)]),
    ("uneasy", &[("Music/Horror & Tension", 8), ("Mood/Dark/Uneasy", 10)]),
    ("eerie", &[("Environments/Dungeons & Ruins", 9), ("Mood/Dark/Eerie", 10)]),
    ("creepy", &[("Music/Horror & Tension", 9), ("Mood/Dark/Creepy", 10)]),
    ("unsettling", &[("Music/Horror & Tension", 9), ("Mood/Dark/Unsettling", 10)]),
    ("menacing", &[("Combat/Monster Combat", 9), ("Mood/Dark/Menacing", 10)]),
    ("sinister", &[("Music/Horror & Tension", 9), ("Mood/Dark/Sinister", 10)]),
    ("gothic", &[("Music/Horror & Tension/Gothic", 10), ("Mood/Dark/Gothic", 10)]),

    // Despair/Tragic moods
    ("dread", &[("Music/Horror & Tension/Cosmic Horror", 10), ("Mood/Tragic/Dread", 10)]),
    ("grim", &[("Combat/Combat Phases/Aftermath", 8), ("Mood/Tragic/Grim", 10)]),
    ("bleak", &[("Environments/Dungeons & Ruins", 8), ("Mood/Tragic/Bleak", 10)]),
    ("oppressive", &[("Music/Horror & Tension", 9), ("Mood/Tragic/Oppressive", 10)]),
    ("claustrophobic", &[("Environments/Dungeons & Ruins/Sewers", 10), ("Mood/Tragic/Claustrophobic", 10)]),
    ("tragic", &[("Social/Ceremonies/Funerals", 9), ("Mood/Tragic/Tragic", 10)]),
    ("melancholic", &[("Music/Orchestral/Dark Orchestral", 9), ("Mood/Tragic/Melancholic", 10)]),
    ("sorrowful", &[("Music/Orchestral/Dark Orchestral", 9), ("Mood/Tragic/Sorrowful", 10)]),
    ("desolate", &[("Environments/Natural Landscapes/Deserts", 9), ("Mood/Tragic/Desolate", 10)]),
    ("lonely", &[("Environments/Natural Landscapes", 8), ("Mood/Tragic/Lonely", 10)]),
    ("fatalistic", &[("Music/Horror & Tension", 8), ("Combat/Combat Phases/Defeat", 9)]),
    ("nihilistic", &[("Music/Horror & Tension", 8), ("Environments/Magical Realms/Void", 9)]),

    // Intense/Aggressive moods
    ("driving", &[("Music/Electronic/Industrial", 9), ("Mood/Intense/Driving", 10)]),
    ("relentless", &[("Combat/Battle Ambience/Battlefield", 9), ("Mood/Intense/Relentless", 10)]),
    ("frenetic", &[("Music/Electronic/Glitch", 10), ("Mood/Intense/Frenetic", 10)]),
    ("furious", &[("Music/Rock & Metal/Metal", 10), ("Mood/Intense/Furious", 10)]),
    ("aggressive", &[("Combat/Monster Combat", 9), ("Mood/Intense/Aggressive", 10)]),
    ("percussive", &[("Music/Orchestral", 8), ("Mood/Intense/Percussive", 10)]),
    ("charged", &[("Music/Electronic/Industrial", 8), ("Mood/Intense/Charged", 10)]),
    ("urgent", &[("Combat/Combat Phases/Pre-Battle", 10), ("Mood/Intense/Urgent", 10)]),
    ("high-stakes", &[("Combat/Combat Phases/Climax", 10), ("Combat/Monster Combat/Dragon Fights", 9)]),
    ("chaotic", &[("Music/Electronic/Glitch", 10), ("Mood/Intense/Chaotic", 10)]),
    ("volatile", &[("Music/Electronic/Industrial", 9), ("Mood/Intense/Volatile", 10)]),
    ("explosive", &[("SFX/Impacts & Crashes/Explosion Impacts", 10), ("Mood/Intense/Explosive", 10)]),

    // Atmospheric/Textural moods
    ("brooding-intensity", &[("Music/Horror & Tension", 9), ("Combat/Combat Phases/Pre-Battle", 8)]),
    ("building", &[("Music/Electronic/Synthwave", 8), ("Combat/Combat Phases/Pre-Battle", 9)]),
    ("rising-tension", &[("Music/Horror & Tension", 9), ("Combat/Combat Phases/Pre-Battle", 9)]),
    ("calm-before-storm", &[("Environments/Weather", 8), ("Combat/Combat Phases/Pre-Battle", 8)]),
    ("airy", &[("Music/Electronic/Ambient Electronic", 8), ("Environments/Natural Landscapes/Mountains", 7)]),
    ("hazy", &[("Music/Electronic/Ambient Electronic", 8), ("Environments/Weather/Fog", 10)]),
    ("glacial", &[("Music/Electronic/Ambient Electronic", 8), ("Environments/Weather/Snow", 10)]),
    ("glitchy", &[("Music/Electronic/Glitch", 10), ("SFX", 7)]),
    ("noisy", &[("Music/Electronic/Industrial", 9), ("SFX/Impacts & Crashes", 8)]),
    ("grainy", &[("Music/Electronic/Industrial", 8), ("SFX", 6)]),
    ("organic", &[("Environments/Natural Landscapes", 9), ("Music/Electronic/Ambient Electronic", 7)]),
    ("mechanical", &[("Music/Electronic/Industrial", 10), ("Tech & Vehicles/Technology/Industrial", 9)]),
    ("rusted", &[("Music/Electronic/Industrial", 8), ("Environments/Dungeons & Ruins", 8)]),
    ("industrial", &[("Music/Electronic/Industrial", 10), ("Tech & Vehicles/Technology/Industrial", 9)]),
    ("neon", &[("Music/Electronic/Synthwave", 10), ("Environments/Settlements/Cities", 8)]),
    ("digital-cold", &[("Music/Electronic", 8), ("Tech & Vehicles/Technology/Advanced", 7)]),
    ("bio-organic", &[("Magic/Spell Schools/Transmutation", 9), ("Music/Electronic", 7)]),
    ("wet", &[("Environments/Weather/Rain", 10), ("Music/Electronic/Ambient Electronic", 7)]),
    ("dry", &[("Environments/Natural Landscapes/Deserts", 10), ("Music/Electronic/Ambient Electronic", 6)]),

    // Horror/Eldritch moods
    ("eldritch", &[("Music/Horror & Tension/Cosmic Horror", 10), ("Creatures & People/Monsters/Aberrations", 9)]),
    ("uncanny", &[("Music/Horror & Tension", 9), ("Environments/Magical Realms/Void", 8)]),
    ("body-horror", &[("Music/Horror & Tension", 9), ("Magic/Spell Schools/Transmutation", 8)]),
    ("liminal", &[("Environments/Dungeons & Ruins", 8), ("Environments/Magical Realms/Void", 9)]),
    ("dissonant", &[("Music/Horror & Tension/Atonal Horror", 10), ("Music/Orchestral/Dark Orchestral", 9)]),
    ("abhorrent", &[("Music/Horror & Tension", 9), ("Creatures & People/Monsters/Aberrations", 8)]),
    ("ritual-fear", &[("Music/Horror & Tension/Ritual", 10), ("Magic/Rituals & Ceremonies", 9)]),
    ("cosmic-dread", &[("Music/Horror & Tension/Cosmic Horror", 10), ("Environments/Magical Realms/Void", 9)]),
];

/// Lookup function for mood folders with confidence scores
pub fn lookup_mood_folders(mood_tag: &str) -> Option<&'static [(&'static str, u8)]> {
    MOOD_FOLDER_MAPPINGS.iter()
        .find(|(tag, _)| *tag == mood_tag)
        .map(|(_, folders)| *folders)
}

/// Get all mood tags
pub fn get_all_mood_tags() -> Vec<&'static str> {
    MOOD_FOLDER_MAPPINGS.iter()
        .map(|(tag, _)| *tag)
        .collect()
}

/// Build lookup HashMap for efficient access
pub fn build_mood_lookup() -> HashMap<&'static str, &'static [(&'static str, u8)]> {
    MOOD_FOLDER_MAPPINGS.iter().cloned().collect()
}