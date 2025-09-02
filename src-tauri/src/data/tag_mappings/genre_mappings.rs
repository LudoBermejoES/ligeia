use std::collections::HashMap;

/// Genre tag to folder mappings with confidence scores
/// Format: (tag, [(folder_path, confidence_score)])
/// Confidence: 5-10 (5=low, 10=perfect match)
pub const GENRE_FOLDER_MAPPINGS: &[(&str, &[(&str, u8)])] = &[
    // General genres
    ("post-metal", &[("Music/Rock & Metal/Post Rock", 10), ("Genre/General/Post-Metal", 10)]),
    ("blues", &[("Music/Jazz & Blues/Blues", 10), ("Genre/General/Blues", 10)]),
    ("lounge", &[("Music/Jazz & Blues/Lounge", 10), ("Genre/General/Lounge", 10)]),
    ("dieselpunk", &[("Genre/General/Dieselpunk", 10), ("Music/Electronic/Industrial", 8)]),
    ("steampunk", &[("Genre/General/Steampunk", 10), ("Tech & Vehicles/Technology/Industrial", 8)]),
    ("atompunk", &[("Tech & Vehicles/Technology/Advanced", 8), ("Music/Electronic/Industrial", 7)]),
    ("solarpunk", &[("Environments/Natural Landscapes", 7), ("Tech & Vehicles/Technology/Advanced", 6)]),
    ("post-apocalyptic", &[("Music/Electronic/Industrial", 8), ("Environments/Dungeons & Ruins", 7)]),
    ("western", &[("Music/Folk & World", 8), ("Genre/General/Western", 10)]),
    ("mystery-noir", &[("Music/Jazz & Blues/Noir Jazz", 10), ("Genre/General/Mystery Noir", 10)]),
    ("modern-urban", &[("Music/Electronic", 7), ("Environments/Settlements/Cities", 8)]),
    ("superhero", &[("Music/Orchestral/Epic Orchestral", 9), ("Genre/General/Superhero", 10)]),

    // Ambient Sub-genres
    ("ambient:dark-ambient", &[("Music/Electronic/Ambient Electronic", 9), ("Genre/Ambient/Dark Ambient", 10)]),
    ("ambient:space-ambient", &[("Music/Electronic/Ambient Electronic", 9), ("Genre/Ambient/Space Ambient", 10)]),
    ("ambient:nature-ambient", &[("Environments/Natural Landscapes", 8), ("Genre/Ambient/Nature Ambient", 10)]),
    ("ambient:ritual", &[("Magic/Rituals & Ceremonies", 9), ("Genre/Ambient/Ritual", 10)]),
    ("ambient:drone", &[("Music/Electronic/Drone", 10), ("Genre/Ambient/Drone", 10)]),
    ("ambient:textural", &[("Music/Electronic/Ambient Electronic", 8), ("Genre/Ambient/Textural", 10)]),
    ("ambient:new-age", &[("Music/Electronic/Ambient Electronic", 8), ("Genre/Ambient/New Age", 10)]),
    ("ambient:lofi-ambient", &[("Music/Electronic/Ambient Electronic", 8), ("Genre/Ambient/Lofi Ambient", 10)]),

    // Diegetic Sub-genres
    ("diegetic:tavern-band", &[("Environments/Settlements/Taverns", 10), ("Social/Entertainment/Bard Performances", 9)]),
    ("diegetic:radio", &[("Music/Electronic", 7), ("Environments/Settlements/Cities", 6)]),
    ("diegetic:gramophone", &[("Music/Jazz & Blues", 8), ("Environments/Settlements", 6)]),
    ("diegetic:street-musician", &[("Social/Entertainment/Street Performers", 10), ("Environments/Settlements/Cities", 7)]),

    // Electronic Sub-genres
    ("electronic:cyberpunk", &[("Music/Electronic", 8), ("Genre/Electronic/Cyberpunk", 10)]),
    ("electronic:idm", &[("Music/Electronic/IDM", 10), ("Genre/Electronic/IDM", 10)]),
    ("electronic:glitch", &[("Music/Electronic/Glitch", 10), ("Genre/Electronic/Glitch", 10)]),
    ("electronic:industrial", &[("Music/Electronic/Industrial", 10), ("Genre/Electronic/Industrial", 10)]),
    ("electronic:ebm", &[("Music/Electronic", 8), ("Genre/Electronic/EBM", 10)]),
    ("electronic:techno", &[("Music/Electronic", 8), ("Genre/Electronic/Techno", 10)]),
    ("electronic:trance", &[("Music/Electronic", 8), ("Genre/Electronic/Trance", 10)]),
    ("electronic:dnb", &[("Music/Electronic", 8), ("Genre/Electronic/DNB", 10)]),
    ("electronic:downtempo", &[("Music/Electronic", 8), ("Genre/Electronic/Downtempo", 10)]),
    ("electronic:shoegaze-electronic", &[("Music/Electronic", 7), ("Music/Rock & Metal", 6)]),

    // Fantasy Sub-genres
    ("fantasy:high-fantasy", &[("Music/Orchestral/Epic Orchestral", 9), ("Genre/Fantasy/High Fantasy", 10)]),
    ("fantasy:grimdark", &[("Music/Orchestral/Dark Orchestral", 9), ("Genre/Fantasy/Grimdark", 10)]),
    ("fantasy:fairy", &[("Environments/Magical Realms/Fairy Realms", 9), ("Genre/Fantasy/Fairy", 10)]),

    // Folk Sub-genres
    ("folk:celtic", &[("Music/Folk & World/Celtic", 10), ("Genre/Folk/Celtic", 10)]),
    ("folk:nordic", &[("Music/Folk & World/Nordic", 10), ("Genre/Folk/Nordic", 10)]),
    ("folk:middle-eastern", &[("Music/Folk & World/Eastern", 10), ("Genre/Folk/Eastern", 9)]),
    ("folk:mediterranean", &[("Music/Folk & World", 8), ("Genre/Folk/Mediterranean", 10)]),
    ("folk:asian-east", &[("Music/Folk & World/Eastern", 10), ("Genre/Folk/Eastern", 9)]),
    ("folk:asian-south", &[("Music/Folk & World/Eastern", 10), ("Genre/Folk/Eastern", 8)]),
    ("folk:african", &[("Music/Folk & World/Tribal", 9), ("Genre/Folk/African", 10)]),
    ("folk:andino", &[("Music/Folk & World", 8), ("Genre/Folk/Tribal", 8)]),
    ("folk:balkan", &[("Music/Folk & World", 8), ("Genre/Folk/Eastern", 7)]),
    ("folk:sea-shanty", &[("Music/Folk & World/Sea Shanties", 10), ("Genre/Folk/Sea Shanties", 10)]),
    ("folk:wild-west-folk", &[("Music/Folk & World", 8), ("Genre/General/Western", 9)]),

    // Historical Sub-genres
    ("historical:baroque", &[("Music/Orchestral", 8), ("Music/Folk & World/Medieval", 7)]),
    ("historical:renaissance", &[("Music/Orchestral", 8), ("Music/Folk & World/Medieval", 8)]),
    ("historical:medieval", &[("Music/Folk & World/Medieval", 10), ("Environments/Settlements/Castles", 7)]),
    ("historical:romantic", &[("Music/Orchestral", 8), ("Social/Ceremonies/Weddings", 6)]),

    // Horror Sub-genres
    ("horror:atonal", &[("Music/Horror & Tension/Atonal Horror", 10), ("Genre/Horror/Atonal Horror", 10)]),
    ("horror:dissonant-strings", &[("Music/Horror & Tension", 9), ("Music/Orchestral/Dark Orchestral", 8)]),
    ("horror:sound-design", &[("SFX", 8), ("Music/Horror & Tension", 7)]),
    ("horror:psychological", &[("Music/Horror & Tension/Psychological", 10), ("Genre/Horror/Psychological", 10)]),
    ("horror:jump-scare", &[("Music/Horror & Tension/Jump Scare", 10), ("Genre/Horror/Jump Scare", 10)]),
    ("horror:ritual", &[("Music/Horror & Tension/Ritual", 10), ("Genre/Horror/Ritual", 10)]),
    ("horror:cosmic", &[("Music/Horror & Tension/Cosmic Horror", 10), ("Genre/Horror/Cosmic Horror", 10)]),
    ("horror:gothic", &[("Music/Horror & Tension/Gothic", 10), ("Genre/Horror/Gothic", 10)]),

    // Jazz Sub-genres
    ("jazz:noir", &[("Music/Jazz & Blues/Noir Jazz", 10), ("Genre/Jazz/Noir Jazz", 10)]),
    ("jazz:swing", &[("Music/Jazz & Blues/Swing", 10), ("Genre/Jazz/Swing", 10)]),
    ("jazz:cool", &[("Music/Jazz & Blues", 8), ("Genre/Jazz/Cool", 10)]),
    ("jazz:latin", &[("Music/Jazz & Blues", 8), ("Music/Folk & World", 6)]),
    ("jazz:bebop", &[("Music/Jazz & Blues/Bebop", 10), ("Genre/Jazz/Bebop", 10)]),

    // Metal Sub-genres
    ("metal:power", &[("Music/Rock & Metal/Metal", 9), ("Genre/Metal/Power Metal", 10)]),
    ("metal:symphonic", &[("Music/Rock & Metal/Metal", 8), ("Genre/Metal/Symphonic Metal", 10)]),
    ("metal:black", &[("Music/Rock & Metal/Metal", 9), ("Genre/Metal/Black Metal", 10)]),
    ("metal:doom", &[("Music/Rock & Metal/Metal", 9), ("Genre/Metal/Doom Metal", 10)]),
    ("metal:folk-metal", &[("Music/Rock & Metal/Folk Metal", 10), ("Genre/Metal/Folk Metal", 10)]),
    ("metal:industrial-metal", &[("Music/Rock & Metal/Metal", 8), ("Music/Electronic/Industrial", 8)]),

    // Mythic Sub-genres
    ("mythic:norse", &[("Music/Folk & World/Nordic", 9), ("Magic/Magical Creatures", 7)]),
    ("mythic:greco-roman", &[("Music/Orchestral", 7), ("Environments/Settlements/Temples", 8)]),
    ("mythic:egyptian", &[("Music/Folk & World/Eastern", 8), ("Environments/Dungeons & Ruins/Tombs", 8)]),
    ("mythic:celtic", &[("Music/Folk & World/Celtic", 9), ("Magic/Magical Creatures/Fae", 8)]),
    ("mythic:japanese", &[("Music/Folk & World/Eastern", 9), ("Magic/Magical Creatures/Spirits", 7)]),
    ("mythic:mesoamerican", &[("Music/Folk & World/Tribal", 8), ("Environments/Dungeons & Ruins/Ancient Ruins", 7)]),

    // Orchestral Sub-genres
    ("orchestral:cinematic", &[("Music/Orchestral", 9), ("Genre/Orchestral/Cinematic", 10)]),
    ("orchestral:hybrid", &[("Music/Orchestral/Hybrid Orchestral", 10), ("Genre/Orchestral/Hybrid", 10)]),
    ("orchestral:heroic", &[("Music/Orchestral/Epic Orchestral", 9), ("Genre/Orchestral/Heroic", 10)]),
    ("orchestral:dark", &[("Music/Orchestral/Dark Orchestral", 10), ("Genre/Orchestral/Dark", 10)]),
    ("orchestral:minimal", &[("Music/Orchestral/Minimalist Orchestral", 10), ("Genre/Orchestral/Minimal", 10)]),
    ("orchestral:romantic", &[("Music/Orchestral", 8), ("Social/Ceremonies/Weddings", 6)]),
    ("orchestral:baroque", &[("Music/Orchestral", 8), ("Music/Folk & World/Medieval", 6)]),
    ("orchestral:renaissance", &[("Music/Orchestral", 8), ("Music/Folk & World/Medieval", 7)]),
    ("orchestral:medieval", &[("Music/Folk & World/Medieval", 9), ("Music/Orchestral", 7)]),
    ("orchestral:percussive", &[("Music/Orchestral", 8), ("Combat/Battle Ambience", 6)]),

    // Rock Sub-genres
    ("rock:post-rock", &[("Music/Rock & Metal/Post Rock", 10), ("Music/Rock & Metal", 8)]),
    ("rock:gothic-rock", &[("Music/Rock & Metal/Gothic Rock", 10), ("Music/Horror & Tension/Gothic", 8)]),
    ("rock:progressive", &[("Music/Rock & Metal", 9), ("Music/Orchestral", 6)]),

    // Sci-Fi Sub-genres
    ("sci-fi:space-opera", &[("Music/Electronic", 7), ("Environments/Magical Realms/Astral Plane", 6)]),
    ("sci-fi:hard-sci-fi", &[("Music/Electronic", 7), ("SFX/Objects/Machinery", 6)]),
    ("sci-fi:cyberpunk", &[("Music/Electronic", 8), ("Genre/Electronic/Cyberpunk", 10)]),
    ("sci-fi:biopunk", &[("Music/Electronic/Industrial", 7), ("Magic/Spell Schools/Transmutation", 5)]),
    ("sci-fi:post-human", &[("Music/Electronic", 7), ("Environments/Magical Realms/Void", 5)]),

    // Sound-Design Sub-genres
    ("sound-design:risers", &[("SFX", 9), ("Music/Electronic", 6)]),
    ("sound-design:impacts", &[("SFX/Impacts & Crashes", 10), ("SFX/Weapons/Impacts", 9)]),
    ("sound-design:whooshes", &[("SFX/Movement/Magic Movement", 9), ("SFX/Magical Effects", 8)]),
    ("sound-design:stingers", &[("SFX", 8), ("Music/Horror & Tension/Jump Scare", 7)]),
    ("sound-design:booms", &[("SFX/Impacts & Crashes/Explosion Impacts", 10), ("SFX/Weapons/Impacts", 8)]),
    ("sound-design:weapons", &[("SFX/Weapons", 10), ("Combat", 7)]),
    ("sound-design:movement", &[("SFX/Movement", 10), ("Combat/Battle Ambience", 6)]),
    ("sound-design:objects", &[("SFX/Objects", 10), ("Environments/Settlements", 5)]),
    ("sound-design:voice", &[("SFX/Voice & Vocal", 10), ("Social/Conversations", 6)]),
    ("sound-design:magic", &[("SFX/Magical Effects", 10), ("Magic", 8)]),
];

/// Lookup function for genre folders with confidence scores
pub fn lookup_genre_folders(genre_tag: &str) -> Option<&'static [(&'static str, u8)]> {
    GENRE_FOLDER_MAPPINGS.iter()
        .find(|(tag, _)| *tag == genre_tag)
        .map(|(_, folders)| *folders)
}

/// Get all genre tags
pub fn get_all_genre_tags() -> Vec<&'static str> {
    GENRE_FOLDER_MAPPINGS.iter()
        .map(|(tag, _)| *tag)
        .collect()
}

/// Build lookup HashMap for efficient access
pub fn build_genre_lookup() -> HashMap<&'static str, &'static [(&'static str, u8)]> {
    GENRE_FOLDER_MAPPINGS.iter().cloned().collect()
}