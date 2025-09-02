use std::collections::HashMap;

/// All 99 mood tags explicitly mapped to virtual folders
pub const MOOD_FOLDER_MAPPINGS: &[(&str, &[&str])] = &[
    // Positive/Heroic moods
    ("heroic", &["Combat/Combat Phases/Victory", "Social/Entertainment/Theater", "Music/Orchestral/Heroic"]),
    ("triumphant", &["Combat/Combat Phases/Victory", "Social/Ceremonies/Coronations", "Music/Orchestral/Epic Orchestral"]),
    ("noble", &["Social/Ceremonies/Royal Courts", "Combat/Legendary Battles/Heroic Duels", "Music/Classical & Traditional/Baroque"]),
    ("uplifting", &["Social/Entertainment/Concerts", "Magic/Healing & Restoration/Peaceful Sanctuaries", "Music/Orchestral/Heroic"]),
    ("hopeful", &["Social/Downtime/Contemplation", "Magic/Healing & Restoration/Hope Springs", "Music/Ambient/Healing & Meditation"]),
    ("inspiring", &["Social/Entertainment/Theater", "Magic/Divine Magic/Inspiration", "Music/Orchestral/Epic Orchestral"]),
    ("adventurous", &["Combat/Combat Phases/Pre-Battle", "Environments/Natural Landscapes/Wilderness Paths", "Music/Folk & World/Celtic"]),
    ("confident", &["Social/Leadership/Command Presence", "Combat/Combat Phases/Victory", "Music/Jazz & Blues/Swing"]),
    ("victorious", &["Combat/Combat Phases/Victory", "Social/Ceremonies/Celebrations", "Music/Orchestral/Triumphant"]),

    // Playful/Light moods  
    ("festive", &["Social/Ceremonies/Celebrations", "Social/Entertainment/Festivals", "Music/Folk & World/Celtic"]),
    ("playful", &["Social/Entertainment/Children's Games", "Magic/Magical Creatures/Fey Courts", "Music/Folk & World/Fairy Tale"]),
    ("whimsical", &["Magic/Magical Creatures/Fey Courts", "Social/Entertainment/Comedy Shows", "Music/Folk & World/Fairy Tale"]),
    ("merry", &["Social/Conversations/Tavern Chatter", "Social/Ceremonies/Celebrations", "Music/Folk & World/Sea Shanty"]),
    ("lighthearted", &["Social/Downtime/Casual Gatherings", "Social/Entertainment/Comedy Shows", "Music/Jazz & Blues/Swing"]),

    // Romantic/Gentle moods
    ("tender", &["Social/Romance/Intimate Moments", "Magic/Healing & Restoration/Peaceful Sanctuaries", "Music/Classical & Traditional/Romantic"]),
    ("romantic", &["Social/Romance/Courtship", "Social/Romance/Intimate Moments", "Music/Classical & Traditional/Romantic"]),
    ("serene", &["Environments/Natural Landscapes/Peaceful Meadows", "Magic/Healing & Restoration/Peaceful Sanctuaries", "Music/Ambient/Healing & Meditation"]),
    ("pastoral", &["Environments/Natural Landscapes/Peaceful Meadows", "Social/Downtime/Rural Life", "Music/Folk & World/Celtic"]),
    ("warm", &["Social/Conversations/Fireside Chats", "Environments/Settlements/Cozy Cottages", "Music/Ambient/Healing & Meditation"]),
    ("comforting", &["Social/Downtime/Rest & Recovery", "Magic/Healing & Restoration/Peaceful Sanctuaries", "Music/Ambient/Healing & Meditation"]),
    ("nostalgic", &["Social/Downtime/Reminiscing", "Environments/Settlements/Abandoned Places", "Music/Classical & Traditional/Romantic"]),
    ("bittersweet", &["Social/Romance/Farewells", "Social/Downtime/Melancholy", "Music/Classical & Traditional/Romantic"]),

    // Mysterious/Arcane moods
    ("mysterious", &["Environments/Dungeons & Ruins/Hidden Chambers", "Magic/Magical Environments/Otherworldly Spaces", "Music/Ambient/Dark Ambient"]),
    ("enigmatic", &["Magic/Arcane Magic/Ancient Mysteries", "Social/Intrigue/Secret Meetings", "Music/Horror & Tension/Mysterious"]),
    ("curious", &["Environments/Dungeons & Ruins/Exploration", "Magic/Divination Magic/Seeking Knowledge", "Music/Ambient/Textural"]),
    ("contemplative", &["Social/Downtime/Contemplation", "Magic/Meditation & Focus/Deep Thought", "Music/Ambient/Drone"]),
    ("dreamlike", &["Magic/Mind Magic/Dream States", "Environments/Abstract/Liminal Spaces", "Music/Ambient/Ethereal"]),
    ("ethereal", &["Magic/Magical Environments/Dreamscapes", "Environments/Elemental Planes/Air Realms", "Music/Ambient/Ethereal"]),
    ("mythic", &["Magic/Mythological/Ancient Powers", "Environments/Legendary Locations/Sacred Groves", "Music/Folk & World/Mythic"]),
    ("arcane", &["Magic/Arcane Magic/Spell Research", "Environments/Settlements/Wizard Towers", "Music/Ambient/Arcane"]),
    ("otherworldly", &["Magic/Eldritch Magic/Alien Dimensions", "Environments/Abstract/Otherworldly", "Music/Ambient/Otherworldly"]),

    // Sacred/Ceremonial moods
    ("solemn", &["Social/Ceremonies/Funeral Rites", "Magic/Divine Magic/Sacred Rituals", "Music/Classical & Traditional/Baroque"]),
    ("ceremonial", &["Social/Ceremonies/Religious Rites", "Magic/Rituals & Ceremonies/Formal Ceremonies", "Music/Classical & Traditional/Medieval"]),
    ("sacred", &["Social/Ceremonies/Religious Rites", "Magic/Divine Magic/Sacred Spaces", "Music/Classical & Traditional/Sacred"]),
    ("ritualistic", &["Magic/Rituals & Ceremonies/Arcane Rituals", "Social/Ceremonies/Cult Gatherings", "Music/Ambient/Ritual Ambient"]),
    ("austere", &["Social/Ceremonies/Monastic Life", "Environments/Settlements/Monasteries", "Music/Classical & Traditional/Minimal"]),
    ("stoic", &["Social/Leadership/Stoic Resolve", "Combat/Combat Phases/Grim Determination", "Music/Orchestral/Minimalist"]),

    // Dark/Threatening moods
    ("ominous", &["Combat/Combat Phases/Approaching Danger", "Magic/Dark Magic/Foreboding", "Music/Horror & Tension/Ominous"]),
    ("foreboding", &["Magic/Divination Magic/Dark Prophecies", "Environments/Dungeons & Ruins/Cursed Places", "Music/Horror & Tension/Foreboding"]),
    ("tense", &["Combat/Combat Phases/High Tension", "Social/Intrigue/Dangerous Negotiations", "Music/Horror & Tension/Suspense"]),
    ("suspenseful", &["Social/Intrigue/Espionage", "Combat/Stealth Operations/Infiltration", "Music/Horror & Tension/Suspense"]),
    ("uneasy", &["Social/Intrigue/Uncomfortable Encounters", "Magic/Mind Magic/Paranoia", "Music/Horror & Tension/Unease"]),
    ("eerie", &["Environments/Haunted Locations/Ghostly Presence", "Magic/Necromancy/Undead Encounters", "Music/Horror & Tension/Eerie"]),
    ("creepy", &["Environments/Haunted Locations/Haunted Houses", "Magic/Dark Magic/Creeping Dread", "Music/Horror & Tension/Creepy"]),
    ("unsettling", &["Magic/Mind Magic/Disturbing Visions", "Environments/Abstract/Uncanny Valley", "Music/Horror & Tension/Unsettling"]),
    ("menacing", &["Combat/Monster Combat/Threatening Presence", "Social/Intrigue/Intimidation", "Music/Horror & Tension/Menacing"]),
    ("sinister", &["Magic/Dark Magic/Evil Schemes", "Social/Intrigue/Villainous Plots", "Music/Horror & Tension/Sinister"]),
    ("gothic", &["Environments/Settlements/Gothic Cathedrals", "Magic/Dark Magic/Gothic Horror", "Music/Horror & Tension/Gothic"]),

    // Despair/Tragic moods
    ("dread", &["Combat/Monster Combat/Eldritch Horrors", "Magic/Eldritch Magic/Cosmic Dread", "Music/Horror & Tension/Dread"]),
    ("grim", &["Combat/Battle Ambience/Dark Battlefields", "Environments/Wastelands/Blighted Lands", "Music/Horror & Tension/Grim"]),
    ("bleak", &["Environments/Wastelands/Post-Apocalyptic", "Social/Downtime/Depression", "Music/Horror & Tension/Desolation"]),
    ("oppressive", &["Social/Political/Tyrannical Rule", "Environments/Settlements/Prison Complexes", "Music/Horror & Tension/Oppressive"]),
    ("claustrophobic", &["Environments/Dungeons & Ruins/Narrow Passages", "Magic/Earth Magic/Crushing Weight", "Music/Horror & Tension/Claustrophobic"]),
    ("tragic", &["Social/Romance/Lost Love", "Social/Ceremonies/Funeral Rites", "Music/Classical & Traditional/Tragic"]),
    ("melancholic", &["Social/Downtime/Melancholy", "Social/Romance/Unrequited Love", "Music/Classical & Traditional/Melancholic"]),
    ("sorrowful", &["Social/Ceremonies/Mourning", "Magic/Healing & Restoration/Grief Processing", "Music/Classical & Traditional/Sorrowful"]),
    ("desolate", &["Environments/Wastelands/Abandoned Ruins", "Social/Downtime/Isolation", "Music/Ambient/Desolation"]),
    ("lonely", &["Social/Downtime/Solitude", "Environments/Natural Landscapes/Empty Wilderness", "Music/Ambient/Solitude"]),
    ("fatalistic", &["Combat/Combat Phases/Last Stands", "Magic/Divination Magic/Inevitable Doom", "Music/Horror & Tension/Fatalism"]),
    ("nihilistic", &["Magic/Dark Magic/Void Worship", "Social/Political/Anarchic Chaos", "Music/Horror & Tension/Nihilism"]),

    // Intense/Aggressive moods
    ("driving", &["Combat/Combat Phases/High-Speed Chase", "Music/Electronic/Drum & Bass", "Combat/Battle Ambience/Relentless Assault"]),
    ("relentless", &["Combat/Battle Ambience/Relentless Assault", "Combat/Combat Phases/Endurance Battle", "Music/Rock & Alternative/Doom Metal"]),
    ("frenetic", &["Combat/Combat Phases/Chaotic Melee", "Magic/Chaotic Magic/Wild Surges", "Music/Electronic/Glitch"]),
    ("furious", &["Combat/Battle Ambience/Berserker Rage", "Magic/Fire Magic/Infernal Fury", "Music/Rock & Alternative/Black Metal"]),
    ("aggressive", &["Combat/Battle Ambience/Battlefield", "Combat/Monster Combat/Predator Encounters", "Music/Rock & Alternative/Power Metal"]),
    ("percussive", &["Music/Orchestral/Percussive", "Combat/Battle Ambience/Rhythmic Warfare", "Social/Ceremonies/War Drums"]),
    ("charged", &["Combat/Combat Phases/Battle Preparation", "Magic/Lightning Magic/Electric Energy", "Music/Electronic/EBM"]),
    ("urgent", &["Combat/Combat Phases/Emergency Response", "Social/Crisis/Urgent Missions", "Music/Electronic/Industrial"]),
    ("high-stakes", &["Combat/Legendary Battles/Final Confrontations", "Social/Political/Critical Negotiations", "Music/Orchestral/Epic Orchestral"]),
    ("chaotic", &["Combat/Combat Phases/Chaotic Melee", "Magic/Chaotic Magic/Reality Breakdown", "Music/Electronic/Glitch"]),
    ("volatile", &["Combat/Monster Combat/Unstable Creatures", "Magic/Chaos Magic/Unpredictable Forces", "Music/Electronic/Industrial"]),
    ("explosive", &["Combat/Combat SFX/Explosive Impacts", "Magic/Fire Magic/Explosive Spells", "SFX/Explosions & Destruction/Large Explosions"]),

    // Atmospheric/Textural moods
    ("brooding-intensity", &["Combat/Combat Phases/Building Tension", "Magic/Dark Magic/Gathering Power", "Music/Horror & Tension/Building Dread"]),
    ("building", &["Music/Transition & Utility/Build-Ups", "Combat/Combat Phases/Escalating Conflict", "Magic/Spell Effects/Power Buildup"]),
    ("rising-tension", &["Combat/Combat Phases/Building Tension", "Social/Intrigue/Escalating Stakes", "Music/Horror & Tension/Rising Tension"]),
    ("calm-before-storm", &["Combat/Combat Phases/Pre-Battle", "Magic/Weather Magic/Storm Gathering", "Music/Horror & Tension/Calm Before Storm"]),
    ("airy", &["Environments/Elemental Planes/Air Realms", "Magic/Air Magic/Wind Currents", "Music/Ambient/Ethereal"]),
    ("hazy", &["Magic/Illusion Magic/Obscured Vision", "Environments/Natural Landscapes/Misty Mornings", "Music/Ambient/Textural"]),
    ("glacial", &["Environments/Natural Landscapes/Frozen Tundra", "Magic/Ice Magic/Frozen Realms", "Music/Ambient/Cold Ambient"]),
    ("glitchy", &["Magic/Technology Magic/Digital Corruption", "Environments/Futuristic/Malfunctioning Systems", "Music/Electronic/Glitch"]),
    ("noisy", &["Environments/Settlements/Industrial Towns", "Combat/Battle Ambience/Chaotic Warfare", "Music/Electronic/Industrial"]),
    ("grainy", &["Magic/Decay Magic/Deterioration", "Environments/Wastelands/Dusty Ruins", "Music/Electronic/Lo-Fi"]),
    ("organic", &["Environments/Natural Landscapes/Living Ecosystems", "Magic/Nature Magic/Growth Cycles", "Music/Ambient/Nature Ambient"]),
    ("mechanical", &["Environments/Futuristic/Industrial Complexes", "Magic/Technology Magic/Clockwork Magic", "Music/Electronic/Industrial"]),
    ("rusted", &["Environments/Settlements/Abandoned Places", "Magic/Decay Magic/Corrosion", "Music/Electronic/Industrial"]),
    ("industrial", &["Environments/Settlements/Industrial Towns", "Combat/Battle Ambience/Mechanical Warfare", "Music/Electronic/Industrial"]),
    ("neon", &["Environments/Futuristic/Cyberpunk Cities", "Magic/Light Magic/Artificial Illumination", "Music/Electronic/Synthwave"]),
    ("digital-cold", &["Environments/Futuristic/AI Core Rooms", "Magic/Technology Magic/Digital Realms", "Music/Electronic/Cyberpunk"]),
    ("bio-organic", &["Environments/Futuristic/Bio-Labs", "Magic/Nature Magic/Living Technology", "Music/Electronic/Biopunk"]),
    ("wet", &["Environments/Natural Landscapes/Swamplands", "Magic/Water Magic/Aquatic Realms", "Music/Ambient/Underwater"]),
    ("dry", &["Environments/Wastelands/Desert Wastes", "Magic/Fire Magic/Desiccation", "Music/Ambient/Desert Ambient"]),

    // Horror/Eldritch moods
    ("eldritch", &["Magic/Eldritch Magic/Alien Geometries", "Combat/Monster Combat/Eldritch Horrors", "Music/Horror & Tension/Cosmic Dread"]),
    ("uncanny", &["Magic/Mind Magic/Disturbing Familiarity", "Environments/Abstract/Uncanny Valley", "Music/Horror & Tension/Uncanny"]),
    ("body-horror", &["Magic/Transformation Magic/Grotesque Changes", "Combat/Monster Combat/Aberrant Creatures", "Music/Horror & Tension/Body Horror"]),
    ("liminal", &["Environments/Abstract/Liminal Spaces", "Magic/Space Magic/Threshold Realms", "Music/Ambient/Liminal"]),
    ("dissonant", &["Magic/Chaos Magic/Reality Distortion", "Combat/Psychic Warfare/Mental Assault", "Music/Horror & Tension/Dissonant"]),
    ("abhorrent", &["Magic/Dark Magic/Vile Corruption", "Combat/Monster Combat/Disgusting Creatures", "Music/Horror & Tension/Abhorrent"]),
    ("ritual-fear", &["Magic/Dark Magic/Terror Rituals", "Social/Ceremonies/Fear Worship", "Music/Horror & Tension/Ritual Terror"]),
    ("cosmic-dread", &["Magic/Eldritch Magic/Cosmic Horrors", "Environments/Elemental Planes/Void Spaces", "Music/Horror & Tension/Cosmic Dread"]),
];

/// Lookup function for mood folders
pub fn lookup_mood_folders(mood_tag: &str) -> Option<&'static [&'static str]> {
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
pub fn build_mood_lookup() -> HashMap<&'static str, &'static [&'static str]> {
    MOOD_FOLDER_MAPPINGS.iter().cloned().collect()
}