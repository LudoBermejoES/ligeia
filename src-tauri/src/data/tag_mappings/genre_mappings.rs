use std::collections::HashMap;

/// All 105 genre tags explicitly mapped to virtual folders
pub const GENRE_FOLDER_MAPPINGS: &[(&str, &[&str])] = &[
    // General genres
    ("post-metal", &["Music/Rock & Alternative/Post Rock"]),
    ("blues", &["Music/Jazz & Blues/Blues"]),
    ("lounge", &["Music/Jazz & Blues/Cool Jazz", "Social/Entertainment/Theater"]),
    ("dieselpunk", &["Music/Electronic/Industrial", "Environments/Settlements/Industrial Towns"]),
    ("steampunk", &["Music/Steampunk & Retro-Futurism/Victorian Steampunk", "Environments/Settlements/Industrial Towns"]),
    ("atompunk", &["Music/Electronic/Industrial", "Environments/Futuristic/Atomic Age"]),
    ("solarpunk", &["Music/Electronic/Ambient Electronic", "Environments/Natural Landscapes/Gardens"]),
    ("post-apocalyptic", &["Music/Horror & Tension/Desolation", "Environments/Wastelands/Post-Apocalyptic"]),
    ("western", &["Music/Folk & World/Wild West Folk", "Environments/Wastelands/Desert Towns"]),
    ("mystery-noir", &["Music/Jazz & Blues/Noir Jazz", "Social/Intrigue/Mystery"]),
    ("modern-urban", &["Music/Electronic/Urban Electronic", "Environments/Settlements/Modern Cities"]),
    ("superhero", &["Music/Orchestral/Epic Orchestral", "Combat/Combat Phases/Victory"]),

    // Ambient
    ("ambient:dark-ambient", &["Music/Ambient/Dark Ambient", "Magic/Magical Environments/Shadow Realms"]),
    ("ambient:space-ambient", &["Music/Ambient/Space Ambient", "Environments/Elemental Planes/Void Spaces"]),
    ("ambient:nature-ambient", &["Music/Ambient/Nature Ambient", "Environments/Natural Landscapes/Forests"]),
    ("ambient:ritual", &["Music/Ambient/Ritual Ambient", "Magic/Rituals & Ceremonies/Arcane Rituals", "Social/Ceremonies/Religious Rites"]),
    ("ambient:drone", &["Music/Ambient/Drone", "Magic/Magical Environments/Otherworldly Spaces"]),
    ("ambient:textural", &["Music/Ambient/Textural", "Environments/Abstract/Liminal Spaces"]),
    ("ambient:new-age", &["Music/Ambient/Healing & Meditation", "Magic/Healing & Restoration/Peaceful Sanctuaries"]),
    ("ambient:lofi-ambient", &["Music/Ambient/Lo-Fi Ambient", "Social/Downtime/Meditation"]),

    // Diegetic
    ("diegetic:tavern-band", &["Social/Conversations/Tavern Chatter", "Music/Folk & World/Celtic"]),
    ("diegetic:radio", &["Social/Entertainment/Radio Shows", "Environments/Settlements/Modern Cities"]),
    ("diegetic:gramophone", &["Social/Entertainment/Theater", "Environments/Settlements/Victorian Streets"]),
    ("diegetic:street-musician", &["Social/Entertainment/Busking", "Environments/Settlements/Town Markets"]),

    // Electronic
    ("electronic:cyberpunk", &["Music/Electronic/Cyberpunk", "Environments/Futuristic/Cyberpunk Cities"]),
    ("electronic:idm", &["Music/Electronic/IDM", "Magic/Magical Environments/Digital Realms"]),
    ("electronic:glitch", &["Music/Electronic/Glitch", "Magic/Chaotic Magic/Glitch Phenomena"]),
    ("electronic:industrial", &["Music/Electronic/Industrial", "Environments/Settlements/Industrial Towns"]),
    ("electronic:ebm", &["Music/Electronic/EBM", "Combat/Battle Ambience/Mechanical Warfare"]),
    ("electronic:techno", &["Music/Electronic/Techno", "Social/Entertainment/Dance Clubs"]),
    ("electronic:trance", &["Music/Electronic/Trance", "Magic/Mind Magic/Hypnotic States"]),
    ("electronic:dnb", &["Music/Electronic/Drum & Bass", "Combat/Combat Phases/High-Speed Chase"]),
    ("electronic:downtempo", &["Music/Electronic/Downtempo", "Social/Downtime/Relaxation"]),
    ("electronic:shoegaze-electronic", &["Music/Electronic/Shoegaze", "Magic/Magical Environments/Dreamscapes"]),

    // Fantasy
    ("fantasy:high-fantasy", &["Music/Orchestral/Epic Orchestral", "Magic/Magical Environments/Enchanted Realms"]),
    ("fantasy:grimdark", &["Music/Horror & Tension/Gothic", "Combat/Battle Ambience/Dark Battlefields"]),
    ("fantasy:fairy", &["Music/Folk & World/Fairy Tale", "Magic/Magical Creatures/Fey Courts"]),

    // Folk
    ("folk:celtic", &["Music/Folk & World/Celtic", "Environments/Natural Landscapes/Celtic Highlands"]),
    ("folk:nordic", &["Music/Folk & World/Nordic", "Environments/Natural Landscapes/Frozen Tundra"]),
    ("folk:middle-eastern", &["Music/Folk & World/Middle Eastern", "Environments/Settlements/Desert Oases"]),
    ("folk:mediterranean", &["Music/Folk & World/Mediterranean", "Environments/Natural Landscapes/Coastal Cliffs"]),
    ("folk:asian-east", &["Music/Folk & World/East Asian", "Environments/Settlements/Oriental Gardens"]),
    ("folk:asian-south", &["Music/Folk & World/South Asian", "Environments/Settlements/Spice Markets"]),
    ("folk:african", &["Music/Folk & World/African", "Environments/Natural Landscapes/Savannas"]),
    ("folk:andino", &["Music/Folk & World/Andean", "Environments/Natural Landscapes/Mountain Peaks"]),
    ("folk:balkan", &["Music/Folk & World/Balkan", "Social/Ceremonies/Cultural Festivals"]),
    ("folk:sea-shanty", &["Music/Folk & World/Sea Shanty", "Environments/Natural Landscapes/Ocean Voyages"]),
    ("folk:wild-west-folk", &["Music/Folk & World/Wild West Folk", "Environments/Wastelands/Desert Towns"]),

    // Historical
    ("historical:baroque", &["Music/Classical & Traditional/Baroque", "Social/Ceremonies/Royal Courts"]),
    ("historical:renaissance", &["Music/Classical & Traditional/Renaissance", "Social/Ceremonies/Renaissance Fairs"]),
    ("historical:medieval", &["Music/Classical & Traditional/Medieval", "Social/Ceremonies/Medieval Feasts"]),
    ("historical:romantic", &["Music/Classical & Traditional/Romantic", "Social/Romance/Courtship"]),

    // Horror
    ("horror:atonal", &["Music/Horror & Tension/Atonal", "Magic/Dark Magic/Forbidden Rituals"]),
    ("horror:dissonant-strings", &["Music/Horror & Tension/Psychological", "Combat/Monster Combat/Eldritch Horrors"]),
    ("horror:sound-design", &["SFX/Horror & Supernatural/Otherworldly", "Magic/Dark Magic/Cursed Artifacts"]),
    ("horror:psychological", &["Music/Horror & Tension/Psychological", "Magic/Mind Magic/Mental Intrusion"]),
    ("horror:jump-scare", &["Music/Horror & Tension/Jump Scares", "SFX/Horror & Supernatural/Sudden Scares"]),
    ("horror:ritual", &["Music/Horror & Tension/Ritualistic", "Magic/Dark Magic/Forbidden Rituals"]),
    ("horror:cosmic", &["Music/Horror & Tension/Cosmic Dread", "Magic/Eldritch Magic/Cosmic Horrors"]),
    ("horror:gothic", &["Music/Horror & Tension/Gothic", "Environments/Settlements/Gothic Cathedrals"]),

    // Jazz
    ("jazz:noir", &["Music/Jazz & Blues/Noir Jazz", "Social/Intrigue/Mystery"]),
    ("jazz:swing", &["Music/Jazz & Blues/Swing", "Social/Entertainment/Jazz Clubs"]),
    ("jazz:cool", &["Music/Jazz & Blues/Cool Jazz", "Social/Entertainment/Upscale Lounges"]),
    ("jazz:latin", &["Music/Jazz & Blues/Latin Jazz", "Social/Entertainment/Dance Clubs"]),
    ("jazz:bebop", &["Music/Jazz & Blues/Bebop", "Social/Entertainment/Jazz Clubs"]),

    // Metal
    ("metal:power", &["Music/Rock & Alternative/Power Metal", "Combat/Combat Phases/Victory"]),
    ("metal:symphonic", &["Music/Orchestral/Symphonic Metal", "Combat/Epic Battles/Orchestral Battles"]),
    ("metal:black", &["Music/Rock & Alternative/Black Metal", "Magic/Dark Magic/Shadow Rituals"]),
    ("metal:doom", &["Music/Rock & Alternative/Doom Metal", "Environments/Dungeons & Ruins/Ancient Tombs"]),
    ("metal:folk-metal", &["Music/Folk & World/Folk Metal", "Combat/Battle Ambience/Tribal Warfare"]),
    ("metal:industrial-metal", &["Music/Electronic/Industrial Metal", "Combat/Battle Ambience/Mechanical Warfare"]),

    // Mythic
    ("mythic:norse", &["Music/Folk & World/Nordic", "Magic/Mythological/Norse Legends", "Combat/Legendary Battles/Viking Raids"]),
    ("mythic:greco-roman", &["Music/Classical & Traditional/Greek", "Magic/Mythological/Greek Myths", "Social/Ceremonies/Ancient Temples"]),
    ("mythic:egyptian", &["Music/Folk & World/Egyptian", "Magic/Mythological/Egyptian Mysteries", "Environments/Settlements/Ancient Pyramids"]),
    ("mythic:celtic", &["Music/Folk & World/Celtic", "Magic/Mythological/Celtic Legends", "Magic/Magical Creatures/Fey Courts"]),
    ("mythic:japanese", &["Music/Folk & World/East Asian", "Magic/Mythological/Eastern Legends", "Social/Ceremonies/Tea Ceremonies"]),
    ("mythic:mesoamerican", &["Music/Folk & World/Mesoamerican", "Magic/Mythological/Aztec Rituals", "Environments/Settlements/Temple Cities"]),

    // Orchestral
    ("orchestral:cinematic", &["Music/Orchestral/Epic Orchestral", "Combat/Epic Battles/Orchestral Battles"]),
    ("orchestral:hybrid", &["Music/Orchestral/Hybrid Orchestral", "Combat/Epic Battles/Modern Warfare"]),
    ("orchestral:heroic", &["Music/Orchestral/Heroic", "Combat/Combat Phases/Victory", "Social/Entertainment/Theater"]),
    ("orchestral:dark", &["Music/Orchestral/Dark Orchestral", "Magic/Dark Magic/Shadow Realms"]),
    ("orchestral:minimal", &["Music/Orchestral/Minimalist", "Social/Downtime/Contemplation"]),
    ("orchestral:romantic", &["Music/Orchestral/Romantic", "Social/Romance/Courtship"]),
    ("orchestral:baroque", &["Music/Classical & Traditional/Baroque", "Social/Ceremonies/Royal Courts"]),
    ("orchestral:renaissance", &["Music/Classical & Traditional/Renaissance", "Social/Ceremonies/Renaissance Fairs"]),
    ("orchestral:medieval", &["Music/Classical & Traditional/Medieval", "Social/Ceremonies/Medieval Feasts"]),
    ("orchestral:percussive", &["Music/Orchestral/Percussive", "Combat/Battle Ambience/Battlefield"]),

    // Rock
    ("rock:post-rock", &["Music/Rock & Alternative/Post Rock", "Social/Downtime/Contemplation"]),
    ("rock:gothic-rock", &["Music/Rock & Alternative/Gothic Rock", "Magic/Dark Magic/Gothic Mysteries"]),
    ("rock:progressive", &["Music/Rock & Alternative/Progressive Rock", "Magic/Time Magic/Temporal Shifts"]),

    // Sci-Fi
    ("sci-fi:space-opera", &["Music/Electronic/Space Opera", "Environments/Futuristic/Space Stations"]),
    ("sci-fi:hard-sci-fi", &["Music/Electronic/Scientific", "Environments/Futuristic/Research Facilities"]),
    ("sci-fi:cyberpunk", &["Music/Electronic/Cyberpunk", "Environments/Futuristic/Cyberpunk Cities"]),
    ("sci-fi:biopunk", &["Music/Electronic/Biopunk", "Environments/Futuristic/Bio-Labs"]),
    ("sci-fi:post-human", &["Music/Electronic/Post-Human", "Magic/Transformation Magic/Evolution"]),

    // Sound-Design
    ("sound-design:risers", &["SFX/Musical Stingers/Risers", "Music/Transition & Utility/Build-Ups"]),
    ("sound-design:impacts", &["SFX/Impacts & Crashes/General Impacts", "Combat/Combat SFX/Impact Sounds"]),
    ("sound-design:whooshes", &["SFX/Movement/Whooshes", "Magic/Spell Effects/Air Magic"]),
    ("sound-design:stingers", &["SFX/Musical Stingers/Dramatic Stings", "Music/Transition & Utility/Stingers"]),
    ("sound-design:booms", &["SFX/Explosions & Destruction/Thunder", "Combat/Combat SFX/Explosive Impacts"]),
    ("sound-design:weapons", &["SFX/Weapons/General Weapons", "Combat/Combat SFX/Weapon Sounds"]),
    ("sound-design:movement", &["SFX/Movement/Footsteps", "SFX/Movement/General Movement"]),
    ("sound-design:objects", &["SFX/Objects & Materials/General Objects"]),
    ("sound-design:voice", &["SFX/Creatures & Voices/Vocal Effects", "Social/Conversations/Voice Acting"]),
    ("sound-design:magic", &["SFX/Magical Effects/Spell Casting", "Magic/Spell Effects/General Magic"]),
];

/// Lookup function for genre folders
pub fn lookup_genre_folders(genre_tag: &str) -> Option<&'static [&'static str]> {
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
pub fn build_genre_lookup() -> HashMap<&'static str, &'static [&'static str]> {
    GENRE_FOLDER_MAPPINGS.iter().cloned().collect()
}