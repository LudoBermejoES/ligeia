use std::collections::HashMap;

/// All 297 occasion tags explicitly mapped to virtual folders
pub const OCCASION_FOLDER_MAPPINGS: &[(&str, &[&str])] = &[
    // Session management
    ("session-start", &["Social/Downtime/Session Opening"]),
    ("recap", &["Social/Conversations/Storytelling"]),
    ("table-chatter", &["Social/Conversations/Casual Chatter"]),
    ("break", &["Social/Downtime/Break Time"]),
    ("session-end", &["Social/Downtime/Session Closing"]),
    ("credits", &["Social/Ceremonies/End Credits"]),
    ("level-up", &["Social/Ceremonies/Achievements"]),
    ("quest-complete", &["Social/Ceremonies/Quest Completion"]),
    ("achievement", &["Social/Ceremonies/Achievements"]),
    ("loot-found", &["Social/Ceremonies/Treasure Discovery"]),
    ("character-death", &["Social/Ceremonies/Memorial"]),
    ("epilogue", &["Social/Downtime/Story Conclusion"]),
    ("flashback", &["Social/Conversations/Storytelling"]),
    ("montage", &["Social/Entertainment/Montage Sequences"]),

    // Travel and exploration
    ("overworld-travel", &["Environments/Natural Landscapes/Wilderness Paths"]),
    ("wilderness-exploration", &["Environments/Natural Landscapes/Untamed Wilds"]),
    ("urban-exploration", &["Environments/Settlements/City Streets"]),
    ("dungeon-crawl", &["Environments/Dungeons & Ruins/Deep Dungeons"]),
    ("ruin-delving", &["Environments/Dungeons & Ruins/Ancient Ruins"]),
    ("underdark-journey", &["Environments/Dungeons & Ruins/Underground Networks"]),
    ("sewers", &["Environments/Settlements/Underground Passages"]),
    ("cave-exploration", &["Environments/Natural Landscapes/Cave Systems"]),
    ("mountain-pass", &["Environments/Natural Landscapes/Mountain Passes"]),
    ("desert-crossing", &["Environments/Natural Landscapes/Desert Crossings"]),
    ("jungle-trek", &["Environments/Natural Landscapes/Dense Jungles"]),
    ("swamp-march", &["Environments/Natural Landscapes/Treacherous Swamps"]),
    ("arctic-trek", &["Environments/Natural Landscapes/Frozen Tundra"]),
    ("sea-voyage", &["Environments/Natural Landscapes/Ocean Voyages"]),
    ("river-journey", &["Environments/Natural Landscapes/River Journeys"]),
    ("airship-voyage", &["Environments/Futuristic/Sky Vessels"]),
    ("space-cruise", &["Environments/Futuristic/Space Vessels"]),
    ("hyperspace-transit", &["Environments/Futuristic/Hyperspace Travel"]),
    ("derelict-ship-exploration", &["Environments/Futuristic/Derelict Ships"]),
    ("space-station-walk", &["Environments/Futuristic/Space Stations"]),

    // Social environments
    ("tavern", &["Social/Conversations/Tavern Chatter", "Environments/Settlements/Taverns"]),
    ("inn", &["Social/Downtime/Rest & Recovery", "Environments/Settlements/Cozy Inns"]),
    ("market", &["Social/Commerce/Market Haggling", "Environments/Settlements/Bustling Markets"]),
    ("black-market", &["Social/Commerce/Illegal Trade", "Environments/Settlements/Hidden Markets"]),
    ("noble-court", &["Social/Political/Royal Courts", "Environments/Settlements/Noble Palaces"]),
    ("audience-with-ruler", &["Social/Political/Royal Audiences"]),
    ("council-debate", &["Social/Political/Council Chambers"]),
    ("negotiation", &["Social/Political/Diplomatic Talks"]),
    ("interrogation", &["Social/Intrigue/Interrogation Rooms"]),
    ("trial", &["Social/Political/Court Proceedings"]),
    ("festival", &["Social/Ceremonies/Public Festivals"]),
    ("wedding", &["Social/Ceremonies/Wedding Ceremonies"]),
    ("funeral", &["Social/Ceremonies/Funeral Rites"]),
    ("ceremony", &["Social/Ceremonies/Formal Ceremonies"]),
    ("religious-service", &["Social/Ceremonies/Religious Services"]),
    ("gambling-den", &["Social/Entertainment/Gambling Houses"]),
    ("speakeasy", &["Social/Entertainment/Secret Bars"]),
    ("noir-club", &["Social/Entertainment/Jazz Clubs"]),
    ("tea-house", &["Social/Conversations/Tea Houses"]),
    ("conversation", &["Social/Conversations/General Dialogue"]),

    // Crowd dynamics
    ("crowd-celebration", &["Social/Ceremonies/Public Celebrations"]),
    ("crowd-angry", &["Social/Political/Angry Mobs"]),
    ("crowd-market", &["Social/Commerce/Market Crowds"]),
    ("crowd-religious", &["Social/Ceremonies/Religious Gatherings"]),
    ("crowd-funeral", &["Social/Ceremonies/Funeral Processions"]),
    ("crowd-panic", &["Social/Crisis/Mass Panic"]),

    // Entertainment
    ("entertainment", &["Social/Entertainment/General Entertainment"]),
    ("bard-performance", &["Social/Entertainment/Musical Performances"]),
    ("theater", &["Social/Entertainment/Theater"]),
    ("gambling", &["Social/Entertainment/Games of Chance"]),
    ("sports", &["Social/Entertainment/Athletic Events"]),
    ("street-performance", &["Social/Entertainment/Busking"]),

    // Organizational meetings
    ("guild-meeting", &["Social/Political/Guild Assemblies"]),
    ("council-session", &["Social/Political/Council Sessions"]),
    ("court-proceeding", &["Social/Political/Legal Proceedings"]),
    ("academic-discourse", &["Social/Conversations/Scholarly Debates"]),
    ("military-order", &["Combat/Military/Command Structure"]),
    ("trade-negotiation", &["Social/Commerce/Business Deals"]),

    // Investigation and stealth
    ("crime-scene", &["Social/Intrigue/Crime Investigations"]),
    ("library-research", &["Social/Downtime/Library Studies"]),
    ("occult-research", &["Magic/Research & Study/Occult Studies"]),
    ("stakeout", &["Social/Intrigue/Surveillance Operations"]),
    ("tailing", &["Social/Intrigue/Covert Following"]),
    ("surveillance", &["Social/Intrigue/Observation Posts"]),
    ("infiltration", &["Combat/Stealth Operations/Enemy Territory"]),
    ("lockpicking", &["Combat/Stealth Operations/Security Bypass"]),
    ("safecracking", &["Combat/Stealth Operations/Vault Breaking"]),
    ("hacking", &["Combat/Stealth Operations/Digital Infiltration"]),
    ("netrun", &["Combat/Cyber Operations/Virtual Reality"]),
    ("vault-breach", &["Combat/Stealth Operations/High Security"]),
    ("disguise", &["Social/Intrigue/Identity Deception"]),
    ("escape", &["Combat/Combat Phases/Emergency Retreat"]),
    ("extraction", &["Combat/Combat Phases/Tactical Withdrawal"]),
    ("clean-getaway", &["Combat/Combat Phases/Successful Escape"]),

    // Puzzles and challenges
    ("riddle-solving", &["Social/Downtime/Mental Challenges"]),
    ("mechanism-puzzle", &["Environments/Dungeons & Ruins/Mechanical Puzzles"]),
    ("arcane-puzzle", &["Magic/Research & Study/Magical Mysteries"]),
    ("trap-primed", &["Environments/Dungeons & Ruins/Trapped Areas"]),
    ("trap-triggered", &["Combat/Environmental Hazards/Triggered Traps"]),

    // Time pressure scenarios
    ("chase-timer", &["Combat/Combat Phases/Time Pressure"]),
    ("bomb-timer", &["Combat/Environmental Hazards/Explosive Devices"]),
    ("reactor-meltdown", &["Combat/Environmental Hazards/Catastrophic Events"]),
    ("airlock-timer", &["Combat/Environmental Hazards/Space Hazards"]),

    // Combat types
    ("combat-ambush", &["Combat/Combat Phases/Surprise Attacks"]),
    ("combat-skirmish", &["Combat/Battle Ambience/Small Engagements"]),
    ("combat-duel", &["Combat/Legendary Battles/Honorable Duels"]),
    ("combat-horde", &["Combat/Battle Ambience/Mass Combat"]),
    ("combat-siege", &["Combat/Epic Battles/Siege Warfare"]),
    ("combat-naval", &["Combat/Epic Battles/Naval Battles"]),
    ("combat-aerial", &["Combat/Epic Battles/Aerial Combat"]),
    ("combat-vehicular", &["Combat/Epic Battles/Vehicle Combat"]),
    ("combat-mecha", &["Combat/Epic Battles/Mech Warfare"]),
    ("combat-space-battle", &["Combat/Epic Battles/Space Battles"]),

    // Boss encounters
    ("boss-intro", &["Combat/Combat Phases/Pre-Battle"]),
    ("boss-loop", &["Combat/Epic Battles/Boss Encounters"]),
    ("boss-final-phase", &["Combat/Epic Battles/Final Showdown"]),
    ("victory-fanfare", &["Combat/Combat Phases/Victory"]),
    ("defeat-lament", &["Combat/Combat Phases/Defeat"]),

    // Chase sequences
    ("chase", &["Combat/Combat Phases/High-Speed Chase"]),
    ("car-chase", &["Combat/Vehicle Combat/Ground Pursuit"]),
    ("foot-chase", &["Combat/Combat Phases/Running Battle"]),
    ("dogfight", &["Combat/Epic Battles/Aerial Dogfights"]),
    ("boarding-action", &["Combat/Epic Battles/Ship Boarding"]),

    // Combat phases
    ("battle-ambience", &["Combat/Battle Ambience/General Combat"]),
    ("pre-battle", &["Combat/Combat Phases/Pre-Battle"]),
    ("climax-combat", &["Combat/Epic Battles/Climactic Battles"]),
    ("victory", &["Combat/Combat Phases/Victory"]),
    ("defeat", &["Combat/Combat Phases/Defeat"]),
    ("aftermath", &["Combat/Combat Phases/Post-Battle"]),

    // Defense and armor
    ("armor-defense", &["Combat/Combat SFX/Defensive Sounds"]),
    ("plate-armor", &["Combat/Combat SFX/Heavy Armor"]),
    ("chain-mail", &["Combat/Combat SFX/Medium Armor"]),
    ("leather-armor", &["Combat/Combat SFX/Light Armor"]),
    ("shields", &["Combat/Combat SFX/Shield Defense"]),
    ("magical-protection", &["Magic/Protection Magic/Defensive Barriers"]),
    ("breaking-armor", &["Combat/Combat SFX/Equipment Damage"]),

    // Monster encounters
    ("monster-combat", &["Combat/Monster Combat/General Creatures"]),
    ("dragon-fight", &["Combat/Monster Combat/Dragon Fights"]),
    ("undead-combat", &["Combat/Monster Combat/Undead Encounters"]),
    ("beast-battle", &["Combat/Monster Combat/Wild Creatures"]),
    ("demon-fight", &["Combat/Monster Combat/Demonic Entities"]),
    ("giant-combat", &["Combat/Monster Combat/Colossal Foes"]),
    ("swarm-attack", &["Combat/Monster Combat/Swarm Creatures"]),

    // Supernatural encounters
    ("haunting", &["Environments/Haunted Locations/Ghostly Presence"]),
    ("possession", &["Magic/Dark Magic/Spirit Possession"]),
    ("ritual", &["Magic/Rituals & Ceremonies/Arcane Rituals"]),
    ("summoning", &["Magic/Conjuration Magic/Entity Summoning"]),
    ("banishment", &["Magic/Abjuration Magic/Entity Banishment"]),
    ("eldritch-reveal", &["Magic/Eldritch Magic/Cosmic Revelations"]),
    ("sanity-slip", &["Magic/Mind Magic/Mental Breakdown"]),
    ("ghost-encounter", &["Combat/Monster Combat/Spectral Entities"]),
    ("vampire-lair", &["Environments/Haunted Locations/Vampire Domains"]),
    ("werewolf-hunt", &["Combat/Monster Combat/Lycanthrope Encounters"]),
    ("zombie-siege", &["Combat/Monster Combat/Undead Hordes"]),
    ("cult-gathering", &["Social/Ceremonies/Cult Meetings"]),
    ("sacrificial-altar", &["Magic/Dark Magic/Blood Rituals"]),

    // Magic types
    ("spellcasting-prep", &["Magic/Spell Preparation/Arcane Focus"]),
    ("battle-magic", &["Magic/Combat Magic/Battlefield Spells"]),
    ("divination", &["Magic/Divination Magic/Future Sight"]),
    ("telepathy", &["Magic/Mind Magic/Mental Communication"]),
    ("dream-walk", &["Magic/Mind Magic/Dream Navigation"]),
    ("astral-travel", &["Magic/Planar Magic/Astral Projection"]),
    ("time-warp", &["Magic/Time Magic/Temporal Manipulation"]),
    ("portal-crossing", &["Magic/Planar Magic/Dimensional Travel"]),
    ("teleportation", &["Magic/Movement Magic/Instant Travel"]),

    // Technology and sci-fi
    ("lab-experiment", &["Environments/Futuristic/Research Labs"]),
    ("biotech-lab", &["Environments/Futuristic/Bio-Labs"]),
    ("nanotech-swarm", &["Combat/Futuristic/Nanotechnology"]),
    ("ai-core", &["Environments/Futuristic/AI Core Rooms"]),
    ("cyberdeck-dive", &["Combat/Cyber Operations/Data Mining"]),
    ("cyber-combat", &["Combat/Cyber Operations/Digital Warfare"]),
    ("drone-control", &["Combat/Futuristic/Drone Operations"]),
    ("mech-dock", &["Environments/Futuristic/Mech Bays"]),
    ("warp-jump", &["Environments/Futuristic/FTL Travel"]),
    ("tractor-beam", &["Combat/Futuristic/Energy Weapons"]),
    ("ship-docking", &["Environments/Futuristic/Docking Procedures"]),
    ("eva-walk", &["Environments/Futuristic/Zero-G Operations"]),

    // Elemental magic
    ("elemental-magic", &["Magic/Elemental Magic/General Elements"]),
    ("fire-magic", &["Magic/Elemental Magic/Fire Magic"]),
    ("ice-magic", &["Magic/Elemental Magic/Ice Magic"]),
    ("lightning-magic", &["Magic/Elemental Magic/Lightning Magic"]),
    ("earth-magic", &["Magic/Elemental Magic/Earth Magic"]),
    ("water-magic", &["Magic/Elemental Magic/Water Magic"]),
    ("air-magic", &["Magic/Elemental Magic/Air Magic"]),
    ("healing-magic", &["Magic/Healing & Restoration/Healing Spells"]),
    ("necromancy", &["Magic/Dark Magic/Death Magic"]),
    ("illusion-magic", &["Magic/Illusion Magic/Reality Distortion"]),
    ("enchantment", &["Magic/Enhancement Magic/Object Enchantment"]),
    ("transmutation", &["Magic/Transformation Magic/Matter Change"]),

    // Magical creatures and environments
    ("magical-creatures", &["Magic/Magical Creatures/General Beings"]),
    ("dragons", &["Magic/Magical Creatures/Dragons"]),
    ("fae", &["Magic/Magical Creatures/Fey Courts"]),
    ("demons", &["Magic/Magical Creatures/Demonic Entities"]),
    ("angels", &["Magic/Magical Creatures/Celestial Beings"]),
    ("spirits", &["Magic/Magical Creatures/Spiritual Entities"]),
    ("elementals", &["Magic/Magical Creatures/Elemental Beings"]),
    ("magical-environments", &["Magic/Magical Environments/Enchanted Spaces"]),
    ("ley-lines", &["Magic/Magical Environments/Power Nexuses"]),
    ("portals", &["Magic/Magical Environments/Dimensional Gates"]),
    ("magical-laboratories", &["Magic/Research & Study/Arcane Laboratories"]),
    ("sacred-groves", &["Magic/Magical Environments/Sacred Spaces"]),
    ("cursed-lands", &["Magic/Magical Environments/Corrupted Realms"]),
    ("magical-storms", &["Magic/Weather Magic/Supernatural Weather"]),

    // Magical items and components
    ("artifacts", &["Magic/Magical Items/Legendary Artifacts"]),
    ("enchanted-weapons", &["Magic/Magical Items/Enchanted Arsenal"]),
    ("spell-components", &["Magic/Spell Preparation/Material Components"]),
    ("magical-books", &["Magic/Research & Study/Arcane Tomes"]),
    ("crystals", &["Magic/Magical Items/Power Crystals"]),
    ("potions", &["Magic/Alchemy/Magical Brews"]),
    ("talismans", &["Magic/Magical Items/Protective Charms"]),

    // Magical rituals and practices
    ("rituals", &["Magic/Rituals & Ceremonies/General Rituals"]),
    ("binding", &["Magic/Abjuration Magic/Binding Spells"]),
    ("transformation", &["Magic/Transformation Magic/Shape Change"]),
    ("communication-magic", &["Magic/Communication Magic/Mystical Messages"]),
    ("protection-magic", &["Magic/Protection Magic/Ward Spells"]),

    // Rest and crafting
    ("campfire", &["Social/Downtime/Campfire Gatherings"]),
    ("short-rest", &["Social/Downtime/Brief Respite"]),
    ("long-rest", &["Social/Downtime/Extended Rest"]),
    ("foraging", &["Environments/Natural Landscapes/Resource Gathering"]),
    ("hunting", &["Combat/Survival/Wilderness Hunting"]),
    ("tracking", &["Combat/Survival/Trail Following"]),
    ("blacksmithing", &["Social/Crafting/Metalwork"]),
    ("fletching", &["Social/Crafting/Arrow Making"]),
    ("alchemy", &["Magic/Alchemy/Potion Brewing"]),
    ("enchanting", &["Magic/Enhancement Magic/Item Enhancement"]),
    ("cooking", &["Social/Downtime/Meal Preparation"]),
    ("base-building", &["Social/Construction/Settlement Building"]),
    ("training", &["Social/Downtime/Skill Development"]),
    ("shopping", &["Social/Commerce/Retail Browsing"]),
    ("bargain", &["Social/Commerce/Price Negotiation"]),
    ("healing", &["Social/Downtime/Medical Care"]),
    ("hospital-ward", &["Social/Downtime/Medical Treatment"]),

    // Time and weather
    ("sunrise", &["Environments/Time Cycles/Morning Dawn"]),
    ("sunset", &["Environments/Time Cycles/Evening Dusk"]),
    ("night-watch", &["Combat/Survival/Night Vigilance"]),
    ("storm", &["Environments/Weather/Thunderstorms"]),
    ("rain", &["Environments/Weather/Rainfall"]),
    ("blizzard", &["Environments/Weather/Snow Storms"]),
    ("sandstorm", &["Environments/Weather/Desert Storms"]),
    ("earthquake", &["Environments/Natural Disasters/Seismic Events"]),
    ("flood", &["Environments/Natural Disasters/Water Disasters"]),
    ("eclipse", &["Environments/Celestial Events/Solar Eclipse"]),
    ("meteor-shower", &["Environments/Celestial Events/Cosmic Events"]),
    ("radiation-storm", &["Environments/Futuristic/Radioactive Weather"]),
    ("anomaly-event", &["Magic/Chaotic Magic/Reality Anomalies"]),
    ("void-rift", &["Magic/Eldritch Magic/Dimensional Tears"]),
    ("volcanic-eruption", &["Environments/Natural Disasters/Volcanic Activity"]),

    // Environment types
    ("natural-landscapes", &["Environments/Natural Landscapes/General Wilderness"]),
    ("forests", &["Environments/Natural Landscapes/Dense Forests"]),
    ("mountains", &["Environments/Natural Landscapes/Mountain Ranges"]),
    ("deserts", &["Environments/Natural Landscapes/Arid Wastelands"]),
    ("oceans", &["Environments/Natural Landscapes/Ocean Depths"]),
    ("rivers", &["Environments/Natural Landscapes/Flowing Waters"]),
    ("caves", &["Environments/Natural Landscapes/Underground Caverns"]),
    ("weather", &["Environments/Weather/General Weather"]),
    ("storms", &["Environments/Weather/Storm Systems"]),
    ("snow", &["Environments/Weather/Snow Conditions"]),
    ("wind", &["Environments/Weather/Wind Patterns"]),
    ("fog", &["Environments/Weather/Misty Conditions"]),
    ("heat", &["Environments/Weather/Hot Conditions"]),

    // Settlement types
    ("settlements", &["Environments/Settlements/General Communities"]),
    ("taverns", &["Environments/Settlements/Drinking Establishments"]),
    ("markets", &["Environments/Settlements/Commercial Districts"]),
    ("cities", &["Environments/Settlements/Urban Centers"]),
    ("villages", &["Environments/Settlements/Rural Communities"]),
    ("temples", &["Environments/Settlements/Religious Sites"]),
    ("castles", &["Environments/Settlements/Fortified Strongholds"]),

    // Dungeons and ruins
    ("dungeons-ruins", &["Environments/Dungeons & Ruins/General Ruins"]),
    ("ancient-ruins", &["Environments/Dungeons & Ruins/Lost Civilizations"]),
    ("tombs", &["Environments/Dungeons & Ruins/Burial Sites"]),
    ("mines", &["Environments/Dungeons & Ruins/Underground Mines"]),
    ("crypts", &["Environments/Dungeons & Ruins/Sacred Burial Chambers"]),
    ("forgotten-places", &["Environments/Dungeons & Ruins/Lost Locations"]),

    // Magical realms
    ("magical-realms", &["Magic/Magical Environments/Otherworldly Realms"]),
    ("fairy-realms", &["Magic/Magical Creatures/Fey Domains"]),
    ("elemental-planes", &["Magic/Elemental Magic/Planar Realms"]),
    ("astral-plane", &["Magic/Planar Magic/Astral Dimensions"]),
    ("shadow-realm", &["Magic/Shadow Magic/Shadow Planes"]),
    ("divine-realms", &["Magic/Divine Magic/Celestial Planes"]),
    ("void", &["Magic/Eldritch Magic/Empty Dimensions"]),

    // Time cycles
    ("time-seasons", &["Environments/Time Cycles/Seasonal Changes"]),
    ("dawn", &["Environments/Time Cycles/Early Morning"]),
    ("day", &["Environments/Time Cycles/Daylight Hours"]),
    ("dusk", &["Environments/Time Cycles/Evening Hours"]),
    ("night", &["Environments/Time Cycles/Nighttime"]),
    ("spring", &["Environments/Time Cycles/Spring Season"]),
    ("summer", &["Environments/Time Cycles/Summer Season"]),
    ("autumn", &["Environments/Time Cycles/Autumn Season"]),
    ("winter", &["Environments/Time Cycles/Winter Season"]),

    // Game mechanics and transitions
    ("scene-transition", &["Music/Transition & Utility/Scene Changes"]),
    ("reveal-stinger", &["Music/Transition & Utility/Revelation Stings"]),
    ("jump-scare", &["SFX/Horror & Supernatural/Sudden Scares"]),
    ("mystery-sting", &["Music/Transition & Utility/Mystery Reveals"]),
    ("map-open", &["Social/Interface/Map Navigation"]),
    ("map-close", &["Social/Interface/Interface Sounds"]),
    ("quest-accepted", &["Social/Interface/Quest Management"]),
    ("quest-failed", &["Social/Interface/Failure Notifications"]),
    ("dice-roll", &["Social/Interface/Game Mechanics"]),
    ("success-cue", &["Social/Interface/Success Notifications"]),
    ("failure-cue", &["Social/Interface/Failure Notifications"]),
];

/// Lookup function for occasion folders
pub fn lookup_occasion_folders(occasion_tag: &str) -> Option<&'static [&'static str]> {
    OCCASION_FOLDER_MAPPINGS.iter()
        .find(|(tag, _)| *tag == occasion_tag)
        .map(|(_, folders)| *folders)
}

/// Get all occasion tags
pub fn get_all_occasion_tags() -> Vec<&'static str> {
    OCCASION_FOLDER_MAPPINGS.iter()
        .map(|(tag, _)| *tag)
        .collect()
}

/// Build lookup HashMap for efficient access
pub fn build_occasion_lookup() -> HashMap<&'static str, &'static [&'static str]> {
    OCCASION_FOLDER_MAPPINGS.iter().cloned().collect()
}