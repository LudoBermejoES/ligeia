// Virtual Folder Structure Definition
// Format: (path, parent_path, icon, description)
// None for parent_path means it's a root folder
// None for icon means no custom icon

[
    // === ROOT FOLDERS ===
    ("Combat", None, Some("⚔️"), "Combat and warfare related sounds"),
    ("Environments", None, Some("🌍"), "Environmental and location sounds"),
    ("Creatures", None, Some("🐲"), "Creature and character sounds"),
    ("Magic & Powers", None, Some("✨"), "Magic and supernatural power sounds"),
    ("Social Encounters", None, Some("🗣️"), "Social interaction sounds"),
    ("Horror & Terror", None, Some("👻"), "Horror and scary atmosphere sounds"),
    ("Superhero & Comic Book", None, Some("🦸"), "Superhero and comic book sounds"),
    ("Moods & Atmosphere", None, Some("🎭"), "Mood and atmosphere sounds"),
    ("Activities & Crafts", None, Some("🔨"), "Activity and crafting sounds"),
    ("Cultural Styles", None, Some("🏛️"), "Cultural and historical style sounds"),
    ("Fantasy Genres", None, Some("🗡️"), "Fantasy genre specific sounds"),
    ("SFX & Foley", None, Some("🎬"), "Sound effects and foley"),
    ("Session Structure", None, Some("🎲"), "RPG session structure sounds"),

    // === COMBAT ===
    ("Combat/Weapons", Some("Combat"), Some("⚡"), "Weapon sounds"),
    ("Combat/Weapons/Melee", Some("Combat/Weapons"), Some("🗡️"), "Melee weapon sounds"),
    ("Combat/Weapons/Melee/Swords", Some("Combat/Weapons/Melee"), None, "Sword sounds"),
    ("Combat/Weapons/Melee/Axes", Some("Combat/Weapons/Melee"), None, "Axe sounds"),
    ("Combat/Weapons/Melee/Clubs", Some("Combat/Weapons/Melee"), None, "Club sounds"),
    ("Combat/Weapons/Melee/Hammers", Some("Combat/Weapons/Melee"), None, "Hammer sounds"),
    ("Combat/Weapons/Melee/Daggers", Some("Combat/Weapons/Melee"), None, "Dagger sounds"),
    
    ("Combat/Weapons/Ranged", Some("Combat/Weapons"), Some("🏹"), "Ranged weapon sounds"),
    ("Combat/Weapons/Ranged/Bows", Some("Combat/Weapons/Ranged"), None, "Bow sounds"),
    ("Combat/Weapons/Ranged/Crossbows", Some("Combat/Weapons/Ranged"), None, "Crossbow sounds"),
    ("Combat/Weapons/Ranged/Firearms", Some("Combat/Weapons/Ranged"), Some("🔫"), "Firearm sounds"),
    ("Combat/Weapons/Ranged/Firearms/Pistols", Some("Combat/Weapons/Ranged/Firearms"), None, "Pistol sounds"),
    ("Combat/Weapons/Ranged/Firearms/Rifles", Some("Combat/Weapons/Ranged/Firearms"), None, "Rifle sounds"),
    ("Combat/Weapons/Ranged/Firearms/Machine Guns", Some("Combat/Weapons/Ranged/Firearms"), None, "Machine gun sounds"),
    ("Combat/Weapons/Ranged/Thrown", Some("Combat/Weapons/Ranged"), None, "Thrown weapon sounds"),
    
    ("Combat/Weapons/Magical", Some("Combat/Weapons"), Some("✨"), "Magical weapon sounds"),
    ("Combat/Weapons/Magical/Battle Magic", Some("Combat/Weapons/Magical"), None, "Battle magic sounds"),
    ("Combat/Weapons/Magical/Spell Impacts", Some("Combat/Weapons/Magical"), None, "Spell impact sounds"),
    ("Combat/Weapons/Magical/Enchanted Weapons", Some("Combat/Weapons/Magical"), None, "Enchanted weapon sounds"),
    
    ("Combat/Armor & Defense", Some("Combat"), Some("🛡️"), "Armor and defense sounds"),
    ("Combat/Armor & Defense/Leather", Some("Combat/Armor & Defense"), None, "Leather armor sounds"),
    ("Combat/Armor & Defense/Chain Mail", Some("Combat/Armor & Defense"), None, "Chain mail sounds"),
    ("Combat/Armor & Defense/Plate", Some("Combat/Armor & Defense"), None, "Plate armor sounds"),
    ("Combat/Armor & Defense/Shields", Some("Combat/Armor & Defense"), None, "Shield sounds"),
    
    ("Combat/Combat Phases", Some("Combat"), Some("⚡"), "Combat phase sounds"),
    ("Combat/Combat Phases/Ambush", Some("Combat/Combat Phases"), None, "Ambush sounds"),
    ("Combat/Combat Phases/Skirmish", Some("Combat/Combat Phases"), None, "Skirmish sounds"),
    ("Combat/Combat Phases/Siege", Some("Combat/Combat Phases"), None, "Siege sounds"),
    ("Combat/Combat Phases/Final Battle", Some("Combat/Combat Phases"), None, "Final battle sounds"),
    
    ("Combat/Victory & Defeat", Some("Combat"), Some("🏆"), "Victory and defeat sounds"),
    ("Combat/Victory & Defeat/Triumph", Some("Combat/Victory & Defeat"), None, "Triumph sounds"),
    ("Combat/Victory & Defeat/Retreat", Some("Combat/Victory & Defeat"), None, "Retreat sounds"),
    ("Combat/Victory & Defeat/Last Stand", Some("Combat/Victory & Defeat"), None, "Last stand sounds"),

    // === ENVIRONMENTS ===
    ("Environments/Natural", Some("Environments"), Some("🌲"), "Natural environment sounds"),
    ("Environments/Natural/Forest", Some("Environments/Natural"), Some("🌳"), "Forest sounds"),
    ("Environments/Natural/Forest/Ancient Forest", Some("Environments/Natural/Forest"), None, "Ancient forest sounds"),
    ("Environments/Natural/Forest/Dark Woods", Some("Environments/Natural/Forest"), None, "Dark woods sounds"),
    ("Environments/Natural/Forest/Fairy Groves", Some("Environments/Natural/Forest"), None, "Fairy grove sounds"),
    
    ("Environments/Natural/Mountains", Some("Environments/Natural"), Some("⛰️"), "Mountain sounds"),
    ("Environments/Natural/Mountains/High Peaks", Some("Environments/Natural/Mountains"), None, "High peak sounds"),
    ("Environments/Natural/Mountains/Cave Systems", Some("Environments/Natural/Mountains"), None, "Cave system sounds"),
    ("Environments/Natural/Mountains/Mining Areas", Some("Environments/Natural/Mountains"), None, "Mining area sounds"),
    
    ("Environments/Natural/Water", Some("Environments/Natural"), Some("🌊"), "Water environment sounds"),
    ("Environments/Natural/Water/Ocean", Some("Environments/Natural/Water"), None, "Ocean sounds"),
    ("Environments/Natural/Water/Rivers", Some("Environments/Natural/Water"), None, "River sounds"),
    ("Environments/Natural/Water/Swamps", Some("Environments/Natural/Water"), None, "Swamp sounds"),
    
    ("Environments/Natural/Weather", Some("Environments/Natural"), Some("⛈️"), "Weather sounds"),
    ("Environments/Natural/Weather/Storms", Some("Environments/Natural/Weather"), None, "Storm sounds"),
    ("Environments/Natural/Weather/Blizzards", Some("Environments/Natural/Weather"), None, "Blizzard sounds"),
    ("Environments/Natural/Weather/Calm", Some("Environments/Natural/Weather"), None, "Calm weather sounds"),
    
    ("Environments/Urban", Some("Environments"), Some("🏘️"), "Urban environment sounds"),
    ("Environments/Urban/Cities", Some("Environments/Urban"), Some("🏙️"), "City sounds"),
    ("Environments/Urban/Cities/Noble Districts", Some("Environments/Urban/Cities"), None, "Noble district sounds"),
    ("Environments/Urban/Cities/Markets", Some("Environments/Urban/Cities"), None, "Market sounds"),
    ("Environments/Urban/Cities/Slums", Some("Environments/Urban/Cities"), None, "Slum sounds"),
    
    ("Environments/Urban/Villages", Some("Environments/Urban"), Some("🏠"), "Village sounds"),
    ("Environments/Urban/Villages/Peaceful", Some("Environments/Urban/Villages"), None, "Peaceful village sounds"),
    ("Environments/Urban/Villages/Under Threat", Some("Environments/Urban/Villages"), None, "Threatened village sounds"),
    
    ("Environments/Urban/Buildings", Some("Environments/Urban"), Some("🏢"), "Building sounds"),
    ("Environments/Urban/Buildings/Taverns", Some("Environments/Urban/Buildings"), None, "Tavern sounds"),
    ("Environments/Urban/Buildings/Temples", Some("Environments/Urban/Buildings"), None, "Temple sounds"),
    ("Environments/Urban/Buildings/Shops", Some("Environments/Urban/Buildings"), None, "Shop sounds"),
    
    ("Environments/Dungeons", Some("Environments"), Some("🏰"), "Dungeon sounds"),
    ("Environments/Dungeons/Stone Corridors", Some("Environments/Dungeons"), None, "Stone corridor sounds"),
    ("Environments/Dungeons/Trap Rooms", Some("Environments/Dungeons"), None, "Trap room sounds"),
    ("Environments/Dungeons/Boss Chambers", Some("Environments/Dungeons"), None, "Boss chamber sounds"),
    ("Environments/Dungeons/Treasure Vaults", Some("Environments/Dungeons"), None, "Treasure vault sounds"),

    // === CREATURES ===
    ("Creatures/Humanoids", Some("Creatures"), Some("👥"), "Humanoid sounds"),
    ("Creatures/Humanoids/Civilized", Some("Creatures/Humanoids"), Some("🤝"), "Civilized humanoid sounds"),
    ("Creatures/Humanoids/Civilized/Humans", Some("Creatures/Humanoids/Civilized"), None, "Human sounds"),
    ("Creatures/Humanoids/Civilized/Elves", Some("Creatures/Humanoids/Civilized"), None, "Elf sounds"),
    ("Creatures/Humanoids/Civilized/Dwarves", Some("Creatures/Humanoids/Civilized"), None, "Dwarf sounds"),
    
    ("Creatures/Humanoids/Hostile", Some("Creatures/Humanoids"), Some("⚔️"), "Hostile humanoid sounds"),
    ("Creatures/Humanoids/Hostile/Orcs", Some("Creatures/Humanoids/Hostile"), None, "Orc sounds"),
    ("Creatures/Humanoids/Hostile/Goblins", Some("Creatures/Humanoids/Hostile"), None, "Goblin sounds"),
    ("Creatures/Humanoids/Hostile/Bandits", Some("Creatures/Humanoids/Hostile"), None, "Bandit sounds"),
    
    ("Creatures/Beasts", Some("Creatures"), Some("🐺"), "Beast sounds"),
    ("Creatures/Beasts/Predators", Some("Creatures/Beasts"), Some("🦁"), "Predator beast sounds"),
    ("Creatures/Beasts/Predators/Wolves", Some("Creatures/Beasts/Predators"), None, "Wolf sounds"),
    ("Creatures/Beasts/Predators/Bears", Some("Creatures/Beasts/Predators"), None, "Bear sounds"),
    ("Creatures/Beasts/Predators/Big Cats", Some("Creatures/Beasts/Predators"), None, "Big cat sounds"),
    
    ("Creatures/Beasts/Magical", Some("Creatures/Beasts"), Some("🦄"), "Magical beast sounds"),
    ("Creatures/Beasts/Magical/Dragons", Some("Creatures/Beasts/Magical"), None, "Dragon sounds"),
    ("Creatures/Beasts/Magical/Griffons", Some("Creatures/Beasts/Magical"), None, "Griffon sounds"),
    ("Creatures/Beasts/Magical/Unicorns", Some("Creatures/Beasts/Magical"), None, "Unicorn sounds"),
    ("Creatures/Beasts/Magical/Phoenixes", Some("Creatures/Beasts/Magical"), None, "Phoenix sounds"),
    
    ("Creatures/Beasts/Mounts", Some("Creatures/Beasts"), Some("🐎"), "Mount sounds"),
    ("Creatures/Beasts/Mounts/Horses", Some("Creatures/Beasts/Mounts"), None, "Horse sounds"),
    ("Creatures/Beasts/Mounts/Pegasi", Some("Creatures/Beasts/Mounts"), None, "Pegasus sounds"),
    ("Creatures/Beasts/Mounts/War Beasts", Some("Creatures/Beasts/Mounts"), None, "War beast sounds"),
    
    ("Creatures/Undead", Some("Creatures"), Some("☠️"), "Undead sounds"),
    ("Creatures/Undead/Lesser", Some("Creatures/Undead"), Some("💀"), "Lesser undead sounds"),
    ("Creatures/Undead/Lesser/Skeletons", Some("Creatures/Undead/Lesser"), None, "Skeleton sounds"),
    ("Creatures/Undead/Lesser/Zombies", Some("Creatures/Undead/Lesser"), None, "Zombie sounds"),
    ("Creatures/Undead/Lesser/Ghosts", Some("Creatures/Undead/Lesser"), None, "Ghost sounds"),
    
    ("Creatures/Undead/Greater", Some("Creatures/Undead"), Some("👑"), "Greater undead sounds"),
    ("Creatures/Undead/Greater/Liches", Some("Creatures/Undead/Greater"), None, "Lich sounds"),
    ("Creatures/Undead/Greater/Vampires", Some("Creatures/Undead/Greater"), None, "Vampire sounds"),
    ("Creatures/Undead/Greater/Death Knights", Some("Creatures/Undead/Greater"), None, "Death knight sounds"),
    
    ("Creatures/Supernatural", Some("Creatures"), Some("👻"), "Supernatural creature sounds"),
    ("Creatures/Supernatural/Demons", Some("Creatures/Supernatural"), None, "Demon sounds"),
    ("Creatures/Supernatural/Angels", Some("Creatures/Supernatural"), None, "Angel sounds"),
    ("Creatures/Supernatural/Fae", Some("Creatures/Supernatural"), None, "Fae sounds"),
    ("Creatures/Supernatural/Elementals", Some("Creatures/Supernatural"), None, "Elemental sounds"),

    // === MAGIC & POWERS ===
    ("Magic & Powers/Schools of Magic", Some("Magic & Powers"), Some("📚"), "Schools of magic sounds"),
    ("Magic & Powers/Schools of Magic/Evocation", Some("Magic & Powers/Schools of Magic"), Some("🔥"), "Evocation magic sounds"),
    ("Magic & Powers/Schools of Magic/Evocation/Fire", Some("Magic & Powers/Schools of Magic/Evocation"), None, "Fire magic sounds"),
    ("Magic & Powers/Schools of Magic/Evocation/Ice", Some("Magic & Powers/Schools of Magic/Evocation"), None, "Ice magic sounds"),
    ("Magic & Powers/Schools of Magic/Evocation/Lightning", Some("Magic & Powers/Schools of Magic/Evocation"), None, "Lightning magic sounds"),
    
    ("Magic & Powers/Schools of Magic/Necromancy", Some("Magic & Powers/Schools of Magic"), Some("💀"), "Necromancy magic sounds"),
    ("Magic & Powers/Schools of Magic/Necromancy/Death Magic", Some("Magic & Powers/Schools of Magic/Necromancy"), None, "Death magic sounds"),
    ("Magic & Powers/Schools of Magic/Necromancy/Soul Binding", Some("Magic & Powers/Schools of Magic/Necromancy"), None, "Soul binding sounds"),
    ("Magic & Powers/Schools of Magic/Necromancy/Undead Control", Some("Magic & Powers/Schools of Magic/Necromancy"), None, "Undead control sounds"),
    
    ("Magic & Powers/Schools of Magic/Illusion", Some("Magic & Powers/Schools of Magic"), Some("🎭"), "Illusion magic sounds"),
    ("Magic & Powers/Schools of Magic/Illusion/Mind Control", Some("Magic & Powers/Schools of Magic/Illusion"), None, "Mind control sounds"),
    ("Magic & Powers/Schools of Magic/Illusion/Deception", Some("Magic & Powers/Schools of Magic/Illusion"), None, "Deception magic sounds"),
    ("Magic & Powers/Schools of Magic/Illusion/Invisibility", Some("Magic & Powers/Schools of Magic/Illusion"), None, "Invisibility sounds"),
    
    ("Magic & Powers/Schools of Magic/Divine", Some("Magic & Powers/Schools of Magic"), Some("☀️"), "Divine magic sounds"),
    ("Magic & Powers/Schools of Magic/Divine/Healing", Some("Magic & Powers/Schools of Magic/Divine"), None, "Healing magic sounds"),
    ("Magic & Powers/Schools of Magic/Divine/Blessing", Some("Magic & Powers/Schools of Magic/Divine"), None, "Blessing sounds"),
    ("Magic & Powers/Schools of Magic/Divine/Smiting", Some("Magic & Powers/Schools of Magic/Divine"), None, "Smiting sounds"),
    
    ("Magic & Powers/Schools of Magic/Conjuration", Some("Magic & Powers/Schools of Magic"), Some("🌀"), "Conjuration magic sounds"),
    ("Magic & Powers/Schools of Magic/Conjuration/Summoning", Some("Magic & Powers/Schools of Magic/Conjuration"), None, "Summoning sounds"),
    ("Magic & Powers/Schools of Magic/Conjuration/Teleportation", Some("Magic & Powers/Schools of Magic/Conjuration"), None, "Teleportation sounds"),
    ("Magic & Powers/Schools of Magic/Conjuration/Portal Creation", Some("Magic & Powers/Schools of Magic/Conjuration"), None, "Portal creation sounds"),
    
    ("Magic & Powers/Schools of Magic/Divination", Some("Magic & Powers/Schools of Magic"), Some("🔮"), "Divination magic sounds"),
    ("Magic & Powers/Schools of Magic/Divination/Prophecy", Some("Magic & Powers/Schools of Magic/Divination"), None, "Prophecy sounds"),
    ("Magic & Powers/Schools of Magic/Divination/Scrying", Some("Magic & Powers/Schools of Magic/Divination"), None, "Scrying sounds"),
    ("Magic & Powers/Schools of Magic/Divination/Truth Seeking", Some("Magic & Powers/Schools of Magic/Divination"), None, "Truth seeking sounds"),
    
    ("Magic & Powers/Magical Events", Some("Magic & Powers"), Some("🌟"), "Magical event sounds"),
    ("Magic & Powers/Magical Events/Rituals", Some("Magic & Powers/Magical Events"), None, "Ritual sounds"),
    ("Magic & Powers/Magical Events/Summoning", Some("Magic & Powers/Magical Events"), None, "Summoning event sounds"),
    ("Magic & Powers/Magical Events/Portal Travel", Some("Magic & Powers/Magical Events"), None, "Portal travel sounds"),
    ("Magic & Powers/Magical Events/Time Manipulation", Some("Magic & Powers/Magical Events"), None, "Time manipulation sounds"),
    ("Magic & Powers/Magical Events/Spell Failures", Some("Magic & Powers/Magical Events"), None, "Spell failure sounds"),
    ("Magic & Powers/Magical Events/Magic Surges", Some("Magic & Powers/Magical Events"), None, "Magic surge sounds"),
    ("Magic & Powers/Magical Events/Divine Intervention", Some("Magic & Powers/Magical Events"), None, "Divine intervention sounds"),
    
    ("Magic & Powers/Technology", Some("Magic & Powers"), Some("⚙️"), "Technology sounds"),
    ("Magic & Powers/Technology/Medieval", Some("Magic & Powers/Technology"), Some("🔧"), "Medieval technology sounds"),
    ("Magic & Powers/Technology/Medieval/Clockwork", Some("Magic & Powers/Technology/Medieval"), None, "Clockwork sounds"),
    ("Magic & Powers/Technology/Medieval/Alchemical", Some("Magic & Powers/Technology/Medieval"), None, "Alchemical sounds"),
    
    ("Magic & Powers/Technology/Steampunk", Some("Magic & Powers/Technology"), Some("🚂"), "Steampunk sounds"),
    ("Magic & Powers/Technology/Steampunk/Steam Engines", Some("Magic & Powers/Technology/Steampunk"), None, "Steam engine sounds"),
    ("Magic & Powers/Technology/Steampunk/Airships", Some("Magic & Powers/Technology/Steampunk"), None, "Airship sounds"),
    
    ("Magic & Powers/Technology/Sci-Fi", Some("Magic & Powers/Technology"), Some("🚀"), "Sci-fi technology sounds"),
    ("Magic & Powers/Technology/Sci-Fi/Cybernetics", Some("Magic & Powers/Technology/Sci-Fi"), None, "Cybernetics sounds"),
    ("Magic & Powers/Technology/Sci-Fi/Spaceships", Some("Magic & Powers/Technology/Sci-Fi"), None, "Spaceship sounds"),
    ("Magic & Powers/Technology/Sci-Fi/AI Systems", Some("Magic & Powers/Technology/Sci-Fi"), None, "AI system sounds"),

    // === SOCIAL ENCOUNTERS ===
    ("Social Encounters/Taverns & Inns", Some("Social Encounters"), Some("🍺"), "Tavern and inn sounds"),
    ("Social Encounters/Taverns & Inns/Cheerful", Some("Social Encounters/Taverns & Inns"), None, "Cheerful tavern sounds"),
    ("Social Encounters/Taverns & Inns/Seedy", Some("Social Encounters/Taverns & Inns"), None, "Seedy tavern sounds"),
    ("Social Encounters/Taverns & Inns/Haunted", Some("Social Encounters/Taverns & Inns"), None, "Haunted tavern sounds"),
    
    ("Social Encounters/Courts & Politics", Some("Social Encounters"), Some("👑"), "Court and political sounds"),
    ("Social Encounters/Courts & Politics/Royal Court", Some("Social Encounters/Courts & Politics"), None, "Royal court sounds"),
    ("Social Encounters/Courts & Politics/Negotiations", Some("Social Encounters/Courts & Politics"), None, "Negotiation sounds"),
    ("Social Encounters/Courts & Politics/Intrigue", Some("Social Encounters/Courts & Politics"), None, "Intrigue sounds"),
    
    ("Social Encounters/Markets & Trade", Some("Social Encounters"), Some("🏪"), "Market and trade sounds"),
    ("Social Encounters/Markets & Trade/Bustling Markets", Some("Social Encounters/Markets & Trade"), None, "Bustling market sounds"),
    ("Social Encounters/Markets & Trade/Black Markets", Some("Social Encounters/Markets & Trade"), None, "Black market sounds"),
    ("Social Encounters/Markets & Trade/Merchant Caravans", Some("Social Encounters/Markets & Trade"), None, "Merchant caravan sounds"),
    
    ("Social Encounters/Religious", Some("Social Encounters"), Some("⛪"), "Religious encounter sounds"),
    ("Social Encounters/Religious/Temples", Some("Social Encounters/Religious"), None, "Temple sounds"),
    ("Social Encounters/Religious/Ceremonies", Some("Social Encounters/Religious"), None, "Ceremony sounds"),
    ("Social Encounters/Religious/Divine Intervention", Some("Social Encounters/Religious"), None, "Divine intervention sounds"),
    
    ("Social Encounters/Investigation", Some("Social Encounters"), Some("🔍"), "Investigation sounds"),
    ("Social Encounters/Investigation/Crime Scenes", Some("Social Encounters/Investigation"), None, "Crime scene sounds"),
    ("Social Encounters/Investigation/Library Research", Some("Social Encounters/Investigation"), None, "Library research sounds"),
    ("Social Encounters/Investigation/Interrogation", Some("Social Encounters/Investigation"), None, "Interrogation sounds"),
    ("Social Encounters/Investigation/Clue Discovery", Some("Social Encounters/Investigation"), None, "Clue discovery sounds"),

    // === HORROR & TERROR ===
    ("Horror & Terror/Classic Horror Locations", Some("Horror & Terror"), Some("🏚️"), "Classic horror location sounds"),
    ("Horror & Terror/Classic Horror Locations/Haunted Houses", Some("Horror & Terror/Classic Horror Locations"), Some("🏚️"), "Haunted house sounds"),
    ("Horror & Terror/Classic Horror Locations/Haunted Houses/Victorian Mansions", Some("Horror & Terror/Classic Horror Locations/Haunted Houses"), None, "Victorian mansion sounds"),
    ("Horror & Terror/Classic Horror Locations/Haunted Houses/Abandoned Estates", Some("Horror & Terror/Classic Horror Locations/Haunted Houses"), None, "Abandoned estate sounds"),
    ("Horror & Terror/Classic Horror Locations/Haunted Houses/Cursed Residences", Some("Horror & Terror/Classic Horror Locations/Haunted Houses"), None, "Cursed residence sounds"),
    ("Horror & Terror/Classic Horror Locations/Haunted Houses/Basement Horrors", Some("Horror & Terror/Classic Horror Locations/Haunted Houses"), None, "Basement horror sounds"),
    
    ("Horror & Terror/Classic Horror Locations/Cemeteries & Graveyards", Some("Horror & Terror/Classic Horror Locations"), Some("⚰️"), "Cemetery and graveyard sounds"),
    ("Horror & Terror/Classic Horror Locations/Cemeteries & Graveyards/Ancient Burial Grounds", Some("Horror & Terror/Classic Horror Locations/Cemeteries & Graveyards"), None, "Ancient burial ground sounds"),
    ("Horror & Terror/Classic Horror Locations/Cemeteries & Graveyards/Forgotten Graveyards", Some("Horror & Terror/Classic Horror Locations/Cemeteries & Graveyards"), None, "Forgotten graveyard sounds"),
    ("Horror & Terror/Classic Horror Locations/Cemeteries & Graveyards/Mausoleums", Some("Horror & Terror/Classic Horror Locations/Cemeteries & Graveyards"), None, "Mausoleum sounds"),
    ("Horror & Terror/Classic Horror Locations/Cemeteries & Graveyards/Crypts", Some("Horror & Terror/Classic Horror Locations/Cemeteries & Graveyards"), None, "Crypt sounds"),
    
    ("Horror & Terror/Classic Horror Locations/Abandoned Institutions", Some("Horror & Terror/Classic Horror Locations"), Some("🏥"), "Abandoned institution sounds"),
    ("Horror & Terror/Classic Horror Locations/Abandoned Institutions/Psychiatric Asylums", Some("Horror & Terror/Classic Horror Locations/Abandoned Institutions"), None, "Psychiatric asylum sounds"),
    ("Horror & Terror/Classic Horror Locations/Abandoned Institutions/Hospitals", Some("Horror & Terror/Classic Horror Locations/Abandoned Institutions"), None, "Abandoned hospital sounds"),
    ("Horror & Terror/Classic Horror Locations/Abandoned Institutions/Orphanages", Some("Horror & Terror/Classic Horror Locations/Abandoned Institutions"), None, "Abandoned orphanage sounds"),
    ("Horror & Terror/Classic Horror Locations/Abandoned Institutions/Prisons", Some("Horror & Terror/Classic Horror Locations/Abandoned Institutions"), None, "Abandoned prison sounds"),
    
    ("Horror & Terror/Classic Horror Locations/Religious Horror", Some("Horror & Terror/Classic Horror Locations"), Some("⛪"), "Religious horror sounds"),
    ("Horror & Terror/Classic Horror Locations/Religious Horror/Desecrated Churches", Some("Horror & Terror/Classic Horror Locations/Religious Horror"), None, "Desecrated church sounds"),
    ("Horror & Terror/Classic Horror Locations/Religious Horror/Occult Temples", Some("Horror & Terror/Classic Horror Locations/Religious Horror"), None, "Occult temple sounds"),
    ("Horror & Terror/Classic Horror Locations/Religious Horror/Monasteries", Some("Horror & Terror/Classic Horror Locations/Religious Horror"), None, "Monastery sounds"),
    ("Horror & Terror/Classic Horror Locations/Religious Horror/Ritual Sites", Some("Horror & Terror/Classic Horror Locations/Religious Horror"), None, "Ritual site sounds"),
    
    ("Horror & Terror/Classic Horror Locations/Isolated Places", Some("Horror & Terror/Classic Horror Locations"), Some("🏕️"), "Isolated place sounds"),
    ("Horror & Terror/Classic Horror Locations/Isolated Places/Cabins in Woods", Some("Horror & Terror/Classic Horror Locations/Isolated Places"), None, "Cabin in woods sounds"),
    ("Horror & Terror/Classic Horror Locations/Isolated Places/Lighthouses", Some("Horror & Terror/Classic Horror Locations/Isolated Places"), None, "Lighthouse sounds"),
    ("Horror & Terror/Classic Horror Locations/Isolated Places/Ghost Towns", Some("Horror & Terror/Classic Horror Locations/Isolated Places"), None, "Ghost town sounds"),
    ("Horror & Terror/Classic Horror Locations/Isolated Places/Abandoned Mines", Some("Horror & Terror/Classic Horror Locations/Isolated Places"), None, "Abandoned mine sounds"),
    
    ("Horror & Terror/Cosmic Horror", Some("Horror & Terror"), Some("🌌"), "Cosmic horror sounds"),
    ("Horror & Terror/Cosmic Horror/Eldritch Entities", Some("Horror & Terror/Cosmic Horror"), Some("👁️"), "Eldritch entity sounds"),
    ("Horror & Terror/Cosmic Horror/Eldritch Entities/Great Old Ones", Some("Horror & Terror/Cosmic Horror/Eldritch Entities"), None, "Great Old One sounds"),
    ("Horror & Terror/Cosmic Horror/Eldritch Entities/Outer Gods", Some("Horror & Terror/Cosmic Horror/Eldritch Entities"), None, "Outer God sounds"),
    ("Horror & Terror/Cosmic Horror/Eldritch Entities/Tentacled Horrors", Some("Horror & Terror/Cosmic Horror/Eldritch Entities"), None, "Tentacled horror sounds"),
    ("Horror & Terror/Cosmic Horror/Eldritch Entities/Incomprehensible Beings", Some("Horror & Terror/Cosmic Horror/Eldritch Entities"), None, "Incomprehensible being sounds"),
    
    ("Horror & Terror/Cosmic Horror/Otherworldly Locations", Some("Horror & Terror/Cosmic Horror"), Some("🌀"), "Otherworldly location sounds"),
    ("Horror & Terror/Cosmic Horror/Otherworldly Locations/R'lyeh", Some("Horror & Terror/Cosmic Horror/Otherworldly Locations"), None, "R'lyeh sounds"),
    ("Horror & Terror/Cosmic Horror/Otherworldly Locations/Arkham", Some("Horror & Terror/Cosmic Horror/Otherworldly Locations"), None, "Arkham sounds"),
    ("Horror & Terror/Cosmic Horror/Otherworldly Locations/Miskatonic University", Some("Horror & Terror/Cosmic Horror/Otherworldly Locations"), None, "Miskatonic University sounds"),
    ("Horror & Terror/Cosmic Horror/Otherworldly Locations/Dunwich", Some("Horror & Terror/Cosmic Horror/Otherworldly Locations"), None, "Dunwich sounds"),
    
    ("Horror & Terror/Cosmic Horror/Sanity & Madness", Some("Horror & Terror/Cosmic Horror"), Some("🧠"), "Sanity and madness sounds"),
    ("Horror & Terror/Cosmic Horror/Sanity & Madness/Mental Deterioration", Some("Horror & Terror/Cosmic Horror/Sanity & Madness"), None, "Mental deterioration sounds"),
    ("Horror & Terror/Cosmic Horror/Sanity & Madness/Paranoid Delusions", Some("Horror & Terror/Cosmic Horror/Sanity & Madness"), None, "Paranoid delusion sounds"),
    ("Horror & Terror/Cosmic Horror/Sanity & Madness/Cosmic Revelations", Some("Horror & Terror/Cosmic Horror/Sanity & Madness"), None, "Cosmic revelation sounds"),
    ("Horror & Terror/Cosmic Horror/Sanity & Madness/Insanity Spirals", Some("Horror & Terror/Cosmic Horror/Sanity & Madness"), None, "Insanity spiral sounds"),
    
    ("Horror & Terror/Cosmic Horror/Ancient Knowledge", Some("Horror & Terror/Cosmic Horror"), Some("📜"), "Ancient knowledge sounds"),
    ("Horror & Terror/Cosmic Horror/Ancient Knowledge/Forbidden Texts", Some("Horror & Terror/Cosmic Horror/Ancient Knowledge"), None, "Forbidden text sounds"),
    ("Horror & Terror/Cosmic Horror/Ancient Knowledge/Eldritch Discoveries", Some("Horror & Terror/Cosmic Horror/Ancient Knowledge"), None, "Eldritch discovery sounds"),
    ("Horror & Terror/Cosmic Horror/Ancient Knowledge/Cosmic Truths", Some("Horror & Terror/Cosmic Horror/Ancient Knowledge"), None, "Cosmic truth sounds"),
    ("Horror & Terror/Cosmic Horror/Ancient Knowledge/Dangerous Research", Some("Horror & Terror/Cosmic Horror/Ancient Knowledge"), None, "Dangerous research sounds"),
    
    ("Horror & Terror/Cosmic Horror/Cult Activities", Some("Horror & Terror/Cosmic Horror"), Some("🕯️"), "Cult activity sounds"),
    ("Horror & Terror/Cosmic Horror/Cult Activities/Summoning Rituals", Some("Horror & Terror/Cosmic Horror/Cult Activities"), None, "Summoning ritual sounds"),
    ("Horror & Terror/Cosmic Horror/Cult Activities/Sacrificial Ceremonies", Some("Horror & Terror/Cosmic Horror/Cult Activities"), None, "Sacrificial ceremony sounds"),
    ("Horror & Terror/Cosmic Horror/Cult Activities/Secret Societies", Some("Horror & Terror/Cosmic Horror/Cult Activities"), None, "Secret society sounds"),
    ("Horror & Terror/Cosmic Horror/Cult Activities/Occult Gatherings", Some("Horror & Terror/Cosmic Horror/Cult Activities"), None, "Occult gathering sounds"),
    
    ("Horror & Terror/Body Horror", Some("Horror & Terror"), Some("🧬"), "Body horror sounds"),
    ("Horror & Terror/Body Horror/Physical Transformation", Some("Horror & Terror/Body Horror"), Some("🔬"), "Physical transformation sounds"),
    ("Horror & Terror/Body Horror/Physical Transformation/Mutations", Some("Horror & Terror/Body Horror/Physical Transformation"), None, "Mutation sounds"),
    ("Horror & Terror/Body Horror/Physical Transformation/Parasitic Infections", Some("Horror & Terror/Body Horror/Physical Transformation"), None, "Parasitic infection sounds"),
    ("Horror & Terror/Body Horror/Physical Transformation/Flesh Melding", Some("Horror & Terror/Body Horror/Physical Transformation"), None, "Flesh melding sounds"),
    ("Horror & Terror/Body Horror/Physical Transformation/Bone Distortion", Some("Horror & Terror/Body Horror/Physical Transformation"), None, "Bone distortion sounds"),
    
    ("Horror & Terror/Body Horror/Medical Horror", Some("Horror & Terror/Body Horror"), Some("🏥"), "Medical horror sounds"),
    ("Horror & Terror/Body Horror/Medical Horror/Surgical Nightmares", Some("Horror & Terror/Body Horror/Medical Horror"), None, "Surgical nightmare sounds"),
    ("Horror & Terror/Body Horror/Medical Horror/Experimental Procedures", Some("Horror & Terror/Body Horror/Medical Horror"), None, "Experimental procedure sounds"),
    ("Horror & Terror/Body Horror/Medical Horror/Disease Outbreaks", Some("Horror & Terror/Body Horror/Medical Horror"), None, "Disease outbreak sounds"),
    ("Horror & Terror/Body Horror/Medical Horror/Viral Infections", Some("Horror & Terror/Body Horror/Medical Horror"), None, "Viral infection sounds"),
    
    ("Horror & Terror/Body Horror/Biological Anomalies", Some("Horror & Terror/Body Horror"), Some("🔬"), "Biological anomaly sounds"),
    ("Horror & Terror/Body Horror/Biological Anomalies/Genetic Aberrations", Some("Horror & Terror/Body Horror/Biological Anomalies"), None, "Genetic aberration sounds"),
    ("Horror & Terror/Body Horror/Biological Anomalies/Cancerous Growths", Some("Horror & Terror/Body Horror/Biological Anomalies"), None, "Cancerous growth sounds"),
    ("Horror & Terror/Body Horror/Biological Anomalies/Organ Malfunction", Some("Horror & Terror/Body Horror/Biological Anomalies"), None, "Organ malfunction sounds"),
    ("Horror & Terror/Body Horror/Biological Anomalies/Cellular Breakdown", Some("Horror & Terror/Body Horror/Biological Anomalies"), None, "Cellular breakdown sounds"),
    
    ("Horror & Terror/Body Horror/Biomechanical", Some("Horror & Terror/Body Horror"), Some("🤖"), "Biomechanical sounds"),
    ("Horror & Terror/Body Horror/Biomechanical/Cybernetic Rejection", Some("Horror & Terror/Body Horror/Biomechanical"), None, "Cybernetic rejection sounds"),
    ("Horror & Terror/Body Horror/Biomechanical/Machine Integration", Some("Horror & Terror/Body Horror/Biomechanical"), None, "Machine integration sounds"),
    ("Horror & Terror/Body Horror/Biomechanical/Synthetic Biology", Some("Horror & Terror/Body Horror/Biomechanical"), None, "Synthetic biology sounds"),
    ("Horror & Terror/Body Horror/Biomechanical/Technological Parasites", Some("Horror & Terror/Body Horror/Biomechanical"), None, "Technological parasite sounds"),
    
    ("Horror & Terror/Psychological Horror", Some("Horror & Terror"), Some("🧠"), "Psychological horror sounds"),
    ("Horror & Terror/Psychological Horror/Mental Deterioration", Some("Horror & Terror/Psychological Horror"), Some("😵‍💫"), "Mental deterioration sounds"),
    ("Horror & Terror/Psychological Horror/Mental Deterioration/Schizophrenia", Some("Horror & Terror/Psychological Horror/Mental Deterioration"), None, "Schizophrenia sounds"),
    ("Horror & Terror/Psychological Horror/Mental Deterioration/Multiple Personalities", Some("Horror & Terror/Psychological Horror/Mental Deterioration"), None, "Multiple personality sounds"),
    ("Horror & Terror/Psychological Horror/Mental Deterioration/Memory Loss", Some("Horror & Terror/Psychological Horror/Mental Deterioration"), None, "Memory loss sounds"),
    ("Horror & Terror/Psychological Horror/Mental Deterioration/Reality Distortion", Some("Horror & Terror/Psychological Horror/Mental Deterioration"), None, "Reality distortion sounds"),
    
    ("Horror & Terror/Psychological Horror/Phobias & Fears", Some("Horror & Terror/Psychological Horror"), Some("😨"), "Phobia and fear sounds"),
    ("Horror & Terror/Psychological Horror/Phobias & Fears/Claustrophobia", Some("Horror & Terror/Psychological Horror/Phobias & Fears"), None, "Claustrophobia sounds"),
    ("Horror & Terror/Psychological Horror/Phobias & Fears/Agoraphobia", Some("Horror & Terror/Psychological Horror/Phobias & Fears"), None, "Agoraphobia sounds"),
    ("Horror & Terror/Psychological Horror/Phobias & Fears/Paranoia", Some("Horror & Terror/Psychological Horror/Phobias & Fears"), None, "Paranoia sounds"),
    ("Horror & Terror/Psychological Horror/Phobias & Fears/Existential Dread", Some("Horror & Terror/Psychological Horror/Phobias & Fears"), None, "Existential dread sounds"),
    
    ("Horror & Terror/Psychological Horror/Trauma & PTSD", Some("Horror & Terror/Psychological Horror"), Some("💔"), "Trauma and PTSD sounds"),
    ("Horror & Terror/Psychological Horror/Trauma & PTSD/Childhood Trauma", Some("Horror & Terror/Psychological Horror/Trauma & PTSD"), None, "Childhood trauma sounds"),
    ("Horror & Terror/Psychological Horror/Trauma & PTSD/War Trauma", Some("Horror & Terror/Psychological Horror/Trauma & PTSD"), None, "War trauma sounds"),
    ("Horror & Terror/Psychological Horror/Trauma & PTSD/Abuse Recovery", Some("Horror & Terror/Psychological Horror/Trauma & PTSD"), None, "Abuse recovery sounds"),
    ("Horror & Terror/Psychological Horror/Trauma & PTSD/Survivor Guilt", Some("Horror & Terror/Psychological Horror/Trauma & PTSD"), None, "Survivor guilt sounds"),
    
    ("Horror & Terror/Psychological Horror/Gaslighting & Manipulation", Some("Horror & Terror/Psychological Horror"), Some("🎭"), "Gaslighting and manipulation sounds"),
    ("Horror & Terror/Psychological Horror/Gaslighting & Manipulation/Reality Questioning", Some("Horror & Terror/Psychological Horror/Gaslighting & Manipulation"), None, "Reality questioning sounds"),
    ("Horror & Terror/Psychological Horror/Gaslighting & Manipulation/Trust Erosion", Some("Horror & Terror/Psychological Horror/Gaslighting & Manipulation"), None, "Trust erosion sounds"),
    ("Horror & Terror/Psychological Horror/Gaslighting & Manipulation/Mind Games", Some("Horror & Terror/Psychological Horror/Gaslighting & Manipulation"), None, "Mind game sounds"),
    ("Horror & Terror/Psychological Horror/Gaslighting & Manipulation/Psychological Abuse", Some("Horror & Terror/Psychological Horror/Gaslighting & Manipulation"), None, "Psychological abuse sounds"),
    
    ("Horror & Terror/Psychological Horror/Isolation & Loneliness", Some("Horror & Terror/Psychological Horror"), Some("😔"), "Isolation and loneliness sounds"),
    ("Horror & Terror/Psychological Horror/Isolation & Loneliness/Social Isolation", Some("Horror & Terror/Psychological Horror/Isolation & Loneliness"), None, "Social isolation sounds"),
    ("Horror & Terror/Psychological Horror/Isolation & Loneliness/Sensory Deprivation", Some("Horror & Terror/Psychological Horror/Isolation & Loneliness"), None, "Sensory deprivation sounds"),
    ("Horror & Terror/Psychological Horror/Isolation & Loneliness/Cabin Fever", Some("Horror & Terror/Psychological Horror/Isolation & Loneliness"), None, "Cabin fever sounds"),
    ("Horror & Terror/Psychological Horror/Isolation & Loneliness/Abandonment", Some("Horror & Terror/Psychological Horror/Isolation & Loneliness"), None, "Abandonment sounds"),
    
    ("Horror & Terror/Supernatural Horror", Some("Horror & Terror"), Some("👻"), "Supernatural horror sounds"),
    ("Horror & Terror/Supernatural Horror/Ghosts & Spirits", Some("Horror & Terror/Supernatural Horror"), Some("👻"), "Ghost and spirit sounds"),
    ("Horror & Terror/Supernatural Horror/Ghosts & Spirits/Poltergeists", Some("Horror & Terror/Supernatural Horror/Ghosts & Spirits"), None, "Poltergeist sounds"),
    ("Horror & Terror/Supernatural Horror/Ghosts & Spirits/Vengeful Spirits", Some("Horror & Terror/Supernatural Horror/Ghosts & Spirits"), None, "Vengeful spirit sounds"),
    ("Horror & Terror/Supernatural Horror/Ghosts & Spirits/Lost Souls", Some("Horror & Terror/Supernatural Horror/Ghosts & Spirits"), None, "Lost soul sounds"),
    ("Horror & Terror/Supernatural Horror/Ghosts & Spirits/Ancestral Ghosts", Some("Horror & Terror/Supernatural Horror/Ghosts & Spirits"), None, "Ancestral ghost sounds"),
    
    ("Horror & Terror/Supernatural Horror/Demonic Entities", Some("Horror & Terror/Supernatural Horror"), Some("😈"), "Demonic entity sounds"),
    ("Horror & Terror/Supernatural Horror/Demonic Entities/Demon Possession", Some("Horror & Terror/Supernatural Horror/Demonic Entities"), None, "Demon possession sounds"),
    ("Horror & Terror/Supernatural Horror/Demonic Entities/Exorcisms", Some("Horror & Terror/Supernatural Horror/Demonic Entities"), None, "Exorcism sounds"),
    ("Horror & Terror/Supernatural Horror/Demonic Entities/Demonic Pacts", Some("Horror & Terror/Supernatural Horror/Demonic Entities"), None, "Demonic pact sounds"),
    ("Horror & Terror/Supernatural Horror/Demonic Entities/Hell Portals", Some("Horror & Terror/Supernatural Horror/Demonic Entities"), None, "Hell portal sounds"),
    
    ("Horror & Terror/Supernatural Horror/Curses & Hexes", Some("Horror & Terror/Supernatural Horror"), Some("🔮"), "Curse and hex sounds"),
    ("Horror & Terror/Supernatural Horror/Curses & Hexes/Ancient Curses", Some("Horror & Terror/Supernatural Horror/Curses & Hexes"), None, "Ancient curse sounds"),
    ("Horror & Terror/Supernatural Horror/Curses & Hexes/Family Curses", Some("Horror & Terror/Supernatural Horror/Curses & Hexes"), None, "Family curse sounds"),
    ("Horror & Terror/Supernatural Horror/Curses & Hexes/Object Curses", Some("Horror & Terror/Supernatural Horror/Curses & Hexes"), None, "Object curse sounds"),
    ("Horror & Terror/Supernatural Horror/Curses & Hexes/Witch Hexes", Some("Horror & Terror/Supernatural Horror/Curses & Hexes"), None, "Witch hex sounds"),
    
    ("Horror & Terror/Supernatural Horror/Undead", Some("Horror & Terror/Supernatural Horror"), Some("☠️"), "Undead sounds"),
    ("Horror & Terror/Supernatural Horror/Undead/Zombies", Some("Horror & Terror/Supernatural Horror/Undead"), None, "Zombie sounds"),
    ("Horror & Terror/Supernatural Horror/Undead/Vampires", Some("Horror & Terror/Supernatural Horror/Undead"), None, "Vampire sounds"),
    ("Horror & Terror/Supernatural Horror/Undead/Revenants", Some("Horror & Terror/Supernatural Horror/Undead"), None, "Revenant sounds"),
    ("Horror & Terror/Supernatural Horror/Undead/Ghouls", Some("Horror & Terror/Supernatural Horror/Undead"), None, "Ghoul sounds"),
    
    ("Horror & Terror/Supernatural Horror/Dark Magic", Some("Horror & Terror/Supernatural Horror"), Some("🌑"), "Dark magic sounds"),
    ("Horror & Terror/Supernatural Horror/Dark Magic/Necromancy", Some("Horror & Terror/Supernatural Horror/Dark Magic"), None, "Necromancy sounds"),
    ("Horror & Terror/Supernatural Horror/Dark Magic/Blood Magic", Some("Horror & Terror/Supernatural Horror/Dark Magic"), None, "Blood magic sounds"),
    ("Horror & Terror/Supernatural Horror/Dark Magic/Soul Binding", Some("Horror & Terror/Supernatural Horror/Dark Magic"), None, "Soul binding sounds"),
    ("Horror & Terror/Supernatural Horror/Dark Magic/Shadow Manipulation", Some("Horror & Terror/Supernatural Horror/Dark Magic"), None, "Shadow manipulation sounds"),
    
    ("Horror & Terror/Monster Horror", Some("Horror & Terror"), Some("👹"), "Monster horror sounds"),
    ("Horror & Terror/Monster Horror/Classic Monsters", Some("Horror & Terror/Monster Horror"), Some("🧛"), "Classic monster sounds"),
    ("Horror & Terror/Monster Horror/Classic Monsters/Werewolves", Some("Horror & Terror/Monster Horror/Classic Monsters"), None, "Werewolf sounds"),
    ("Horror & Terror/Monster Horror/Classic Monsters/Vampires", Some("Horror & Terror/Monster Horror/Classic Monsters"), None, "Vampire sounds"),
    ("Horror & Terror/Monster Horror/Classic Monsters/Frankenstein's Monster", Some("Horror & Terror/Monster Horror/Classic Monsters"), None, "Frankenstein's Monster sounds"),
    ("Horror & Terror/Monster Horror/Classic Monsters/Mummies", Some("Horror & Terror/Monster Horror/Classic Monsters"), None, "Mummy sounds"),
    
    ("Horror & Terror/Monster Horror/Cryptid Encounters", Some("Horror & Terror/Monster Horror"), Some("🦶"), "Cryptid encounter sounds"),
    ("Horror & Terror/Monster Horror/Cryptid Encounters/Bigfoot/Sasquatch", Some("Horror & Terror/Monster Horror/Cryptid Encounters"), None, "Bigfoot/Sasquatch sounds"),
    ("Horror & Terror/Monster Horror/Cryptid Encounters/Mothman", Some("Horror & Terror/Monster Horror/Cryptid Encounters"), None, "Mothman sounds"),
    ("Horror & Terror/Monster Horror/Cryptid Encounters/Chupacabra", Some("Horror & Terror/Monster Horror/Cryptid Encounters"), None, "Chupacabra sounds"),
    ("Horror & Terror/Monster Horror/Cryptid Encounters/Jersey Devil", Some("Horror & Terror/Monster Horror/Cryptid Encounters"), None, "Jersey Devil sounds"),
    
    ("Horror & Terror/Monster Horror/Aquatic Horrors", Some("Horror & Terror/Monster Horror"), Some("🌊"), "Aquatic horror sounds"),
    ("Horror & Terror/Monster Horror/Aquatic Horrors/Deep Sea Creatures", Some("Horror & Terror/Monster Horror/Aquatic Horrors"), None, "Deep sea creature sounds"),
    ("Horror & Terror/Monster Horror/Aquatic Horrors/Lake Monsters", Some("Horror & Terror/Monster Horror/Aquatic Horrors"), None, "Lake monster sounds"),
    ("Horror & Terror/Monster Horror/Aquatic Horrors/Sirens", Some("Horror & Terror/Monster Horror/Aquatic Horrors"), None, "Siren sounds"),
    ("Horror & Terror/Monster Horror/Aquatic Horrors/Krakens", Some("Horror & Terror/Monster Horror/Aquatic Horrors"), None, "Kraken sounds"),
    
    ("Horror & Terror/Monster Horror/Insectoid Nightmares", Some("Horror & Terror/Monster Horror"), Some("🕷️"), "Insectoid nightmare sounds"),
    ("Horror & Terror/Monster Horror/Insectoid Nightmares/Giant Spiders", Some("Horror & Terror/Monster Horror/Insectoid Nightmares"), None, "Giant spider sounds"),
    ("Horror & Terror/Monster Horror/Insectoid Nightmares/Swarm Attacks", Some("Horror & Terror/Monster Horror/Insectoid Nightmares"), None, "Swarm attack sounds"),
    ("Horror & Terror/Monster Horror/Insectoid Nightmares/Hive Minds", Some("Horror & Terror/Monster Horror/Insectoid Nightmares"), None, "Hive mind sounds"),
    ("Horror & Terror/Monster Horror/Insectoid Nightmares/Parasitic Wasps", Some("Horror & Terror/Monster Horror/Insectoid Nightmares"), None, "Parasitic wasp sounds"),
    
    ("Horror & Terror/Monster Horror/Aberrant Creatures", Some("Horror & Terror/Monster Horror"), Some("🦎"), "Aberrant creature sounds"),
    ("Horror & Terror/Monster Horror/Aberrant Creatures/Shapeshifters", Some("Horror & Terror/Monster Horror/Aberrant Creatures"), None, "Shapeshifter sounds"),
    ("Horror & Terror/Monster Horror/Aberrant Creatures/Mimics", Some("Horror & Terror/Monster Horror/Aberrant Creatures"), None, "Mimic sounds"),
    ("Horror & Terror/Monster Horror/Aberrant Creatures/Doppelgangers", Some("Horror & Terror/Monster Horror/Aberrant Creatures"), None, "Doppelganger sounds"),
    ("Horror & Terror/Monster Horror/Aberrant Creatures/Chimeras", Some("Horror & Terror/Monster Horror/Aberrant Creatures"), None, "Chimera sounds"),
    
    ("Horror & Terror/Apocalyptic Horror", Some("Horror & Terror"), Some("☢️"), "Apocalyptic horror sounds"),
    ("Horror & Terror/Apocalyptic Horror/Zombie Apocalypse", Some("Horror & Terror/Apocalyptic Horror"), Some("🧟"), "Zombie apocalypse sounds"),
    ("Horror & Terror/Apocalyptic Horror/Zombie Apocalypse/Outbreak Origins", Some("Horror & Terror/Apocalyptic Horror/Zombie Apocalypse"), None, "Outbreak origin sounds"),
    ("Horror & Terror/Apocalyptic Horror/Zombie Apocalypse/Safe Houses", Some("Horror & Terror/Apocalyptic Horror/Zombie Apocalypse"), None, "Safe house sounds"),
    ("Horror & Terror/Apocalyptic Horror/Zombie Apocalypse/Survivor Communities", Some("Horror & Terror/Apocalyptic Horror/Zombie Apocalypse"), None, "Survivor community sounds"),
    ("Horror & Terror/Apocalyptic Horror/Zombie Apocalypse/Zombie Hordes", Some("Horror & Terror/Apocalyptic Horror/Zombie Apocalypse"), None, "Zombie horde sounds"),
    
    ("Horror & Terror/Apocalyptic Horror/Nuclear Horror", Some("Horror & Terror/Apocalyptic Horror"), Some("☢️"), "Nuclear horror sounds"),
    ("Horror & Terror/Apocalyptic Horror/Nuclear Horror/Radiation Sickness", Some("Horror & Terror/Apocalyptic Horror/Nuclear Horror"), None, "Radiation sickness sounds"),
    ("Horror & Terror/Apocalyptic Horror/Nuclear Horror/Mutant Creatures", Some("Horror & Terror/Apocalyptic Horror/Nuclear Horror"), None, "Mutant creature sounds"),
    ("Horror & Terror/Apocalyptic Horror/Nuclear Horror/Fallout Zones", Some("Horror & Terror/Apocalyptic Horror/Nuclear Horror"), None, "Fallout zone sounds"),
    ("Horror & Terror/Apocalyptic Horror/Nuclear Horror/Nuclear Winter", Some("Horror & Terror/Apocalyptic Horror/Nuclear Horror"), None, "Nuclear winter sounds"),
    
    ("Horror & Terror/Apocalyptic Horror/Pandemic Horror", Some("Horror & Terror/Apocalyptic Horror"), Some("🦠"), "Pandemic horror sounds"),
    ("Horror & Terror/Apocalyptic Horror/Pandemic Horror/Viral Outbreaks", Some("Horror & Terror/Apocalyptic Horror/Pandemic Horror"), None, "Viral outbreak sounds"),
    ("Horror & Terror/Apocalyptic Horror/Pandemic Horror/Quarantine Zones", Some("Horror & Terror/Apocalyptic Horror/Pandemic Horror"), None, "Quarantine zone sounds"),
    ("Horror & Terror/Apocalyptic Horror/Pandemic Horror/Medical Collapse", Some("Horror & Terror/Apocalyptic Horror/Pandemic Horror"), None, "Medical collapse sounds"),
    ("Horror & Terror/Apocalyptic Horror/Pandemic Horror/Social Breakdown", Some("Horror & Terror/Apocalyptic Horror/Pandemic Horror"), None, "Social breakdown sounds"),
    
    ("Horror & Terror/Apocalyptic Horror/Environmental Collapse", Some("Horror & Terror/Apocalyptic Horror"), Some("🌍"), "Environmental collapse sounds"),
    ("Horror & Terror/Apocalyptic Horror/Environmental Collapse/Climate Disasters", Some("Horror & Terror/Apocalyptic Horror/Environmental Collapse"), None, "Climate disaster sounds"),
    ("Horror & Terror/Apocalyptic Horror/Environmental Collapse/Ecosystem Collapse", Some("Horror & Terror/Apocalyptic Horror/Environmental Collapse"), None, "Ecosystem collapse sounds"),
    ("Horror & Terror/Apocalyptic Horror/Environmental Collapse/Resource Wars", Some("Horror & Terror/Apocalyptic Horror/Environmental Collapse"), None, "Resource war sounds"),
    ("Horror & Terror/Apocalyptic Horror/Environmental Collapse/Extinction Events", Some("Horror & Terror/Apocalyptic Horror/Environmental Collapse"), None, "Extinction event sounds"),
    
    ("Horror & Terror/Horror Atmospherics", Some("Horror & Terror"), Some("🌫️"), "Horror atmospheric sounds"),
    ("Horror & Terror/Horror Atmospherics/Weather & Environment", Some("Horror & Terror/Horror Atmospherics"), Some("⛈️"), "Horror weather sounds"),
    ("Horror & Terror/Horror Atmospherics/Weather & Environment/Perpetual Storms", Some("Horror & Terror/Horror Atmospherics/Weather & Environment"), None, "Perpetual storm sounds"),
    ("Horror & Terror/Horror Atmospherics/Weather & Environment/Unnatural Fog", Some("Horror & Terror/Horror Atmospherics/Weather & Environment"), None, "Unnatural fog sounds"),
    ("Horror & Terror/Horror Atmospherics/Weather & Environment/Blood Rain", Some("Horror & Terror/Horror Atmospherics/Weather & Environment"), None, "Blood rain sounds"),
    ("Horror & Terror/Horror Atmospherics/Weather & Environment/Darkness Entities", Some("Horror & Terror/Horror Atmospherics/Weather & Environment"), None, "Darkness entity sounds"),
    
    ("Horror & Terror/Horror Atmospherics/Sound Design", Some("Horror & Terror/Horror Atmospherics"), Some("🔊"), "Horror sound design"),
    ("Horror & Terror/Horror Atmospherics/Sound Design/Whispers in Walls", Some("Horror & Terror/Horror Atmospherics/Sound Design"), None, "Whispers in walls sounds"),
    ("Horror & Terror/Horror Atmospherics/Sound Design/Screaming Winds", Some("Horror & Terror/Horror Atmospherics/Sound Design"), None, "Screaming wind sounds"),
    ("Horror & Terror/Horror Atmospherics/Sound Design/Chains Rattling", Some("Horror & Terror/Horror Atmospherics/Sound Design"), None, "Chains rattling sounds"),
    ("Horror & Terror/Horror Atmospherics/Sound Design/Children Laughing", Some("Horror & Terror/Horror Atmospherics/Sound Design"), None, "Children laughing sounds"),
    
    ("Horror & Terror/Horror Atmospherics/Visual Disturbances", Some("Horror & Terror/Horror Atmospherics"), Some("👁️"), "Visual disturbance sounds"),
    ("Horror & Terror/Horror Atmospherics/Visual Disturbances/Shadow Movement", Some("Horror & Terror/Horror Atmospherics/Visual Disturbances"), None, "Shadow movement sounds"),
    ("Horror & Terror/Horror Atmospherics/Visual Disturbances/Flickering Lights", Some("Horror & Terror/Horror Atmospherics/Visual Disturbances"), None, "Flickering light sounds"),
    ("Horror & Terror/Horror Atmospherics/Visual Disturbances/Mirror Anomalies", Some("Horror & Terror/Horror Atmospherics/Visual Disturbances"), None, "Mirror anomaly sounds"),
    ("Horror & Terror/Horror Atmospherics/Visual Disturbances/Peripheral Horrors", Some("Horror & Terror/Horror Atmospherics/Visual Disturbances"), None, "Peripheral horror sounds"),
    
    ("Horror & Terror/Horror Atmospherics/Temporal Anomalies", Some("Horror & Terror/Horror Atmospherics"), Some("⏰"), "Temporal anomaly sounds"),
    ("Horror & Terror/Horror Atmospherics/Temporal Anomalies/Time Loops", Some("Horror & Terror/Horror Atmospherics/Temporal Anomalies"), None, "Time loop sounds"),
    ("Horror & Terror/Horror Atmospherics/Temporal Anomalies/Temporal Decay", Some("Horror & Terror/Horror Atmospherics/Temporal Anomalies"), None, "Temporal decay sounds"),
    ("Horror & Terror/Horror Atmospherics/Temporal Anomalies/Chronological Bleeding", Some("Horror & Terror/Horror Atmospherics/Temporal Anomalies"), None, "Chronological bleeding sounds"),
    ("Horror & Terror/Horror Atmospherics/Temporal Anomalies/Past Intrusions", Some("Horror & Terror/Horror Atmospherics/Temporal Anomalies"), None, "Past intrusion sounds"),

    // === SUPERHERO & COMIC BOOK ===
    ("Superhero & Comic Book/Urban Settings", Some("Superhero & Comic Book"), Some("🏢"), "Superhero urban setting sounds"),
    ("Superhero & Comic Book/Urban Settings/Metropolis", Some("Superhero & Comic Book/Urban Settings"), Some("🌆"), "Metropolis sounds"),
    ("Superhero & Comic Book/Urban Settings/Metropolis/Daily Planet", Some("Superhero & Comic Book/Urban Settings/Metropolis"), None, "Daily Planet sounds"),
    ("Superhero & Comic Book/Urban Settings/Metropolis/LexCorp Tower", Some("Superhero & Comic Book/Urban Settings/Metropolis"), None, "LexCorp Tower sounds"),
    ("Superhero & Comic Book/Urban Settings/Metropolis/City Center", Some("Superhero & Comic Book/Urban Settings/Metropolis"), None, "City center sounds"),
    
    ("Superhero & Comic Book/Urban Settings/Gotham City", Some("Superhero & Comic Book/Urban Settings"), Some("🦇"), "Gotham City sounds"),
    ("Superhero & Comic Book/Urban Settings/Gotham City/Wayne Manor", Some("Superhero & Comic Book/Urban Settings/Gotham City"), None, "Wayne Manor sounds"),
    ("Superhero & Comic Book/Urban Settings/Gotham City/Arkham Asylum", Some("Superhero & Comic Book/Urban Settings/Gotham City"), None, "Arkham Asylum sounds"),
    ("Superhero & Comic Book/Urban Settings/Gotham City/Crime Alley", Some("Superhero & Comic Book/Urban Settings/Gotham City"), None, "Crime Alley sounds"),
    ("Superhero & Comic Book/Urban Settings/Gotham City/GCPD", Some("Superhero & Comic Book/Urban Settings/Gotham City"), None, "GCPD sounds"),
    
    ("Superhero & Comic Book/Urban Settings/New York City", Some("Superhero & Comic Book/Urban Settings"), Some("🏙️"), "New York City sounds"),
    ("Superhero & Comic Book/Urban Settings/New York City/Daily Bugle", Some("Superhero & Comic Book/Urban Settings/New York City"), None, "Daily Bugle sounds"),
    ("Superhero & Comic Book/Urban Settings/New York City/Avengers Mansion", Some("Superhero & Comic Book/Urban Settings/New York City"), None, "Avengers Mansion sounds"),
    ("Superhero & Comic Book/Urban Settings/New York City/Hell's Kitchen", Some("Superhero & Comic Book/Urban Settings/New York City"), None, "Hell's Kitchen sounds"),
    ("Superhero & Comic Book/Urban Settings/New York City/Times Square", Some("Superhero & Comic Book/Urban Settings/New York City"), None, "Times Square sounds"),
    
    ("Superhero & Comic Book/Urban Settings/Generic Urban", Some("Superhero & Comic Book/Urban Settings"), Some("🌃"), "Generic urban sounds"),
    ("Superhero & Comic Book/Urban Settings/Generic Urban/Skyscrapers", Some("Superhero & Comic Book/Urban Settings/Generic Urban"), None, "Skyscraper sounds"),
    ("Superhero & Comic Book/Urban Settings/Generic Urban/Street Level", Some("Superhero & Comic Book/Urban Settings/Generic Urban"), None, "Street level sounds"),
    ("Superhero & Comic Book/Urban Settings/Generic Urban/Rooftops", Some("Superhero & Comic Book/Urban Settings/Generic Urban"), None, "Rooftop sounds"),
    ("Superhero & Comic Book/Urban Settings/Generic Urban/Underground", Some("Superhero & Comic Book/Urban Settings/Generic Urban"), None, "Underground sounds"),
    
    ("Superhero & Comic Book/Hero Headquarters", Some("Superhero & Comic Book"), Some("🏛️"), "Hero headquarters sounds"),
    ("Superhero & Comic Book/Hero Headquarters/Secret Lairs", Some("Superhero & Comic Book/Hero Headquarters"), Some("🕳️"), "Secret lair sounds"),
    ("Superhero & Comic Book/Hero Headquarters/Secret Lairs/Batcave", Some("Superhero & Comic Book/Hero Headquarters/Secret Lairs"), None, "Batcave sounds"),
    ("Superhero & Comic Book/Hero Headquarters/Secret Lairs/Fortress of Solitude", Some("Superhero & Comic Book/Hero Headquarters/Secret Lairs"), None, "Fortress of Solitude sounds"),
    ("Superhero & Comic Book/Hero Headquarters/Secret Lairs/Sanctum Sanctorum", Some("Superhero & Comic Book/Hero Headquarters/Secret Lairs"), None, "Sanctum Sanctorum sounds"),
    ("Superhero & Comic Book/Hero Headquarters/Secret Lairs/Hidden Bases", Some("Superhero & Comic Book/Hero Headquarters/Secret Lairs"), None, "Hidden base sounds"),
    
    ("Superhero & Comic Book/Hero Headquarters/Public Headquarters", Some("Superhero & Comic Book/Hero Headquarters"), Some("🏢"), "Public headquarters sounds"),
    ("Superhero & Comic Book/Hero Headquarters/Public Headquarters/Baxter Building", Some("Superhero & Comic Book/Hero Headquarters/Public Headquarters"), None, "Baxter Building sounds"),
    ("Superhero & Comic Book/Hero Headquarters/Public Headquarters/Avengers Tower", Some("Superhero & Comic Book/Hero Headquarters/Public Headquarters"), None, "Avengers Tower sounds"),
    ("Superhero & Comic Book/Hero Headquarters/Public Headquarters/Hall of Justice", Some("Superhero & Comic Book/Hero Headquarters/Public Headquarters"), None, "Hall of Justice sounds"),
    ("Superhero & Comic Book/Hero Headquarters/Public Headquarters/Xavier's School", Some("Superhero & Comic Book/Hero Headquarters/Public Headquarters"), None, "Xavier's School sounds"),
    
    ("Superhero & Comic Book/Hero Headquarters/Mobile Bases", Some("Superhero & Comic Book/Hero Headquarters"), Some("✈️"), "Mobile base sounds"),
    ("Superhero & Comic Book/Hero Headquarters/Mobile Bases/Helicarrier", Some("Superhero & Comic Book/Hero Headquarters/Mobile Bases"), None, "Helicarrier sounds"),
    ("Superhero & Comic Book/Hero Headquarters/Mobile Bases/Blackbird", Some("Superhero & Comic Book/Hero Headquarters/Mobile Bases"), None, "Blackbird sounds"),
    ("Superhero & Comic Book/Hero Headquarters/Mobile Bases/Invisible Jet", Some("Superhero & Comic Book/Hero Headquarters/Mobile Bases"), None, "Invisible Jet sounds"),
    ("Superhero & Comic Book/Hero Headquarters/Mobile Bases/Batjet", Some("Superhero & Comic Book/Hero Headquarters/Mobile Bases"), None, "Batjet sounds"),
    
    ("Superhero & Comic Book/Hero Headquarters/Orbital Stations", Some("Superhero & Comic Book/Hero Headquarters"), Some("🛰️"), "Orbital station sounds"),
    ("Superhero & Comic Book/Hero Headquarters/Orbital Stations/Watchtower", Some("Superhero & Comic Book/Hero Headquarters/Orbital Stations"), None, "Watchtower sounds"),
    ("Superhero & Comic Book/Hero Headquarters/Orbital Stations/Peak Station", Some("Superhero & Comic Book/Hero Headquarters/Orbital Stations"), None, "Peak Station sounds"),
    ("Superhero & Comic Book/Hero Headquarters/Orbital Stations/Space Bases", Some("Superhero & Comic Book/Hero Headquarters/Orbital Stations"), None, "Space base sounds"),
    
    ("Superhero & Comic Book/Villain Lairs", Some("Superhero & Comic Book"), Some("💀"), "Villain lair sounds"),
    ("Superhero & Comic Book/Villain Lairs/Underground", Some("Superhero & Comic Book/Villain Lairs"), Some("⛏️"), "Underground lair sounds"),
    ("Superhero & Comic Book/Villain Lairs/Underground/Subterranea", Some("Superhero & Comic Book/Villain Lairs/Underground"), None, "Subterranea sounds"),
    ("Superhero & Comic Book/Villain Lairs/Underground/Sewer Systems", Some("Superhero & Comic Book/Villain Lairs/Underground"), None, "Sewer system sounds"),
    ("Superhero & Comic Book/Villain Lairs/Underground/Secret Tunnels", Some("Superhero & Comic Book/Villain Lairs/Underground"), None, "Secret tunnel sounds"),
    ("Superhero & Comic Book/Villain Lairs/Underground/Cave Networks", Some("Superhero & Comic Book/Villain Lairs/Underground"), None, "Cave network sounds"),
    
    ("Superhero & Comic Book/Villain Lairs/High Tech", Some("Superhero & Comic Book/Villain Lairs"), Some("🏗️"), "High tech lair sounds"),
    ("Superhero & Comic Book/Villain Lairs/High Tech/Castle Doom", Some("Superhero & Comic Book/Villain Lairs/High Tech"), None, "Castle Doom sounds"),
    ("Superhero & Comic Book/Villain Lairs/High Tech/Corporate Towers", Some("Superhero & Comic Book/Villain Lairs/High Tech"), None, "Corporate tower sounds"),
    ("Superhero & Comic Book/Villain Lairs/High Tech/Space Stations", Some("Superhero & Comic Book/Villain Lairs/High Tech"), None, "Space station sounds"),
    ("Superhero & Comic Book/Villain Lairs/High Tech/Undersea Bases", Some("Superhero & Comic Book/Villain Lairs/High Tech"), None, "Undersea base sounds"),
    
    ("Superhero & Comic Book/Villain Lairs/Mystical", Some("Superhero & Comic Book/Villain Lairs"), Some("🔮"), "Mystical lair sounds"),
    ("Superhero & Comic Book/Villain Lairs/Mystical/Dark Dimensions", Some("Superhero & Comic Book/Villain Lairs/Mystical"), None, "Dark dimension sounds"),
    ("Superhero & Comic Book/Villain Lairs/Mystical/Hell Realms", Some("Superhero & Comic Book/Villain Lairs/Mystical"), None, "Hell realm sounds"),
    ("Superhero & Comic Book/Villain Lairs/Mystical/Shadow Realms", Some("Superhero & Comic Book/Villain Lairs/Mystical"), None, "Shadow realm sounds"),
    ("Superhero & Comic Book/Villain Lairs/Mystical/Pocket Dimensions", Some("Superhero & Comic Book/Villain Lairs/Mystical"), None, "Pocket dimension sounds"),
    
    ("Superhero & Comic Book/Villain Lairs/Industrial", Some("Superhero & Comic Book/Villain Lairs"), Some("🏭"), "Industrial lair sounds"),
    ("Superhero & Comic Book/Villain Lairs/Industrial/Factories", Some("Superhero & Comic Book/Villain Lairs/Industrial"), None, "Factory sounds"),
    ("Superhero & Comic Book/Villain Lairs/Industrial/Refineries", Some("Superhero & Comic Book/Villain Lairs/Industrial"), None, "Refinery sounds"),
    ("Superhero & Comic Book/Villain Lairs/Industrial/Laboratories", Some("Superhero & Comic Book/Villain Lairs/Industrial"), None, "Laboratory sounds"),
    ("Superhero & Comic Book/Villain Lairs/Industrial/Warehouses", Some("Superhero & Comic Book/Villain Lairs/Industrial"), None, "Warehouse sounds"),
    
    ("Superhero & Comic Book/Powers & Abilities", Some("Superhero & Comic Book"), Some("⚡"), "Superhero power sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Flight", Some("Superhero & Comic Book/Powers & Abilities"), Some("🚁"), "Flight power sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Flight/Supersonic Flight", Some("Superhero & Comic Book/Powers & Abilities/Flight"), None, "Supersonic flight sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Flight/Jetpack Flight", Some("Superhero & Comic Book/Powers & Abilities/Flight"), None, "Jetpack flight sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Flight/Magical Flight", Some("Superhero & Comic Book/Powers & Abilities/Flight"), None, "Magical flight sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Flight/Anti-Gravity", Some("Superhero & Comic Book/Powers & Abilities/Flight"), None, "Anti-gravity sounds"),
    
    ("Superhero & Comic Book/Powers & Abilities/Strength & Combat", Some("Superhero & Comic Book/Powers & Abilities"), Some("💪"), "Strength and combat sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Strength & Combat/Super Strength", Some("Superhero & Comic Book/Powers & Abilities/Strength & Combat"), None, "Super strength sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Strength & Combat/Martial Arts", Some("Superhero & Comic Book/Powers & Abilities/Strength & Combat"), None, "Martial arts sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Strength & Combat/Energy Blasts", Some("Superhero & Comic Book/Powers & Abilities/Strength & Combat"), None, "Energy blast sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Strength & Combat/Weapon Mastery", Some("Superhero & Comic Book/Powers & Abilities/Strength & Combat"), None, "Weapon mastery sounds"),
    
    ("Superhero & Comic Book/Powers & Abilities/Mental Powers", Some("Superhero & Comic Book/Powers & Abilities"), Some("🧠"), "Mental power sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Mental Powers/Telepathy", Some("Superhero & Comic Book/Powers & Abilities/Mental Powers"), None, "Telepathy sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Mental Powers/Telekinesis", Some("Superhero & Comic Book/Powers & Abilities/Mental Powers"), None, "Telekinesis sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Mental Powers/Mind Control", Some("Superhero & Comic Book/Powers & Abilities/Mental Powers"), None, "Mind control sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Mental Powers/Precognition", Some("Superhero & Comic Book/Powers & Abilities/Mental Powers"), None, "Precognition sounds"),
    
    ("Superhero & Comic Book/Powers & Abilities/Elemental Powers", Some("Superhero & Comic Book/Powers & Abilities"), Some("🔥"), "Elemental power sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Elemental Powers/Fire Control", Some("Superhero & Comic Book/Powers & Abilities/Elemental Powers"), None, "Fire control sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Elemental Powers/Ice Control", Some("Superhero & Comic Book/Powers & Abilities/Elemental Powers"), None, "Ice control sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Elemental Powers/Lightning Control", Some("Superhero & Comic Book/Powers & Abilities/Elemental Powers"), None, "Lightning control sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Elemental Powers/Weather Control", Some("Superhero & Comic Book/Powers & Abilities/Elemental Powers"), None, "Weather control sounds"),
    
    ("Superhero & Comic Book/Powers & Abilities/Special Abilities", Some("Superhero & Comic Book/Powers & Abilities"), Some("✨"), "Special ability sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Special Abilities/Teleportation", Some("Superhero & Comic Book/Powers & Abilities/Special Abilities"), None, "Teleportation sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Special Abilities/Invisibility", Some("Superhero & Comic Book/Powers & Abilities/Special Abilities"), None, "Invisibility sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Special Abilities/Shape Shifting", Some("Superhero & Comic Book/Powers & Abilities/Special Abilities"), None, "Shape shifting sounds"),
    ("Superhero & Comic Book/Powers & Abilities/Special Abilities/Time Manipulation", Some("Superhero & Comic Book/Powers & Abilities/Special Abilities"), None, "Time manipulation sounds"),
    
    ("Superhero & Comic Book/Comic Book SFX", Some("Superhero & Comic Book"), Some("💥"), "Comic book sound effects"),
    ("Superhero & Comic Book/Comic Book SFX/Classic Onomatopoeia", Some("Superhero & Comic Book/Comic Book SFX"), Some("💥"), "Classic comic book sounds"),
    ("Superhero & Comic Book/Comic Book SFX/Classic Onomatopoeia/POW!", Some("Superhero & Comic Book/Comic Book SFX/Classic Onomatopoeia"), None, "POW! sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Classic Onomatopoeia/BAM!", Some("Superhero & Comic Book/Comic Book SFX/Classic Onomatopoeia"), None, "BAM! sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Classic Onomatopoeia/ZAP!", Some("Superhero & Comic Book/Comic Book SFX/Classic Onomatopoeia"), None, "ZAP! sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Classic Onomatopoeia/KAPOW!", Some("Superhero & Comic Book/Comic Book SFX/Classic Onomatopoeia"), None, "KAPOW! sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Classic Onomatopoeia/WHAM!", Some("Superhero & Comic Book/Comic Book SFX/Classic Onomatopoeia"), None, "WHAM! sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Classic Onomatopoeia/BOOM!", Some("Superhero & Comic Book/Comic Book SFX/Classic Onomatopoeia"), None, "BOOM! sound effect"),
    
    ("Superhero & Comic Book/Comic Book SFX/Character Specific", Some("Superhero & Comic Book/Comic Book SFX"), Some("🦸"), "Character specific sounds"),
    ("Superhero & Comic Book/Comic Book SFX/Character Specific/THWIP (Spider-Man)", Some("Superhero & Comic Book/Comic Book SFX/Character Specific"), None, "THWIP sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Character Specific/SNIKT (Wolverine)", Some("Superhero & Comic Book/Comic Book SFX/Character Specific"), None, "SNIKT sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Character Specific/BAMF (Nightcrawler)", Some("Superhero & Comic Book/Comic Book SFX/Character Specific"), None, "BAMF sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Character Specific/HULK SMASH!", Some("Superhero & Comic Book/Comic Book SFX/Character Specific"), None, "HULK SMASH sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Character Specific/SHAZAM!", Some("Superhero & Comic Book/Comic Book SFX/Character Specific"), None, "SHAZAM sound effect"),
    
    ("Superhero & Comic Book/Comic Book SFX/Energy & Tech", Some("Superhero & Comic Book/Comic Book SFX"), Some("⚡"), "Energy and tech sounds"),
    ("Superhero & Comic Book/Comic Book SFX/Energy & Tech/BZZT!", Some("Superhero & Comic Book/Comic Book SFX/Energy & Tech"), None, "BZZT sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Energy & Tech/WHIRRRR!", Some("Superhero & Comic Book/Comic Book SFX/Energy & Tech"), None, "WHIRRRR sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Energy & Tech/BEEP!", Some("Superhero & Comic Book/Comic Book SFX/Energy & Tech"), None, "BEEP sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Energy & Tech/CLANK!", Some("Superhero & Comic Book/Comic Book SFX/Energy & Tech"), None, "CLANK sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Energy & Tech/HISSSS!", Some("Superhero & Comic Book/Comic Book SFX/Energy & Tech"), None, "HISSSS sound effect"),
    
    ("Superhero & Comic Book/Comic Book SFX/Movement & Action", Some("Superhero & Comic Book/Comic Book SFX"), Some("💨"), "Movement and action sounds"),
    ("Superhero & Comic Book/Comic Book SFX/Movement & Action/SWOOSH!", Some("Superhero & Comic Book/Comic Book SFX/Movement & Action"), None, "SWOOSH sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Movement & Action/WHOOSH!", Some("Superhero & Comic Book/Comic Book SFX/Movement & Action"), None, "WHOOSH sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Movement & Action/THUD!", Some("Superhero & Comic Book/Comic Book SFX/Movement & Action"), None, "THUD sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Movement & Action/CRASH!", Some("Superhero & Comic Book/Comic Book SFX/Movement & Action"), None, "CRASH sound effect"),
    ("Superhero & Comic Book/Comic Book SFX/Movement & Action/SLAM!", Some("Superhero & Comic Book/Comic Book SFX/Movement & Action"), None, "SLAM sound effect"),
    
    ("Superhero & Comic Book/Scenarios & Encounters", Some("Superhero & Comic Book"), Some("🎬"), "Superhero scenario sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Origin Stories", Some("Superhero & Comic Book/Scenarios & Encounters"), Some("🌟"), "Origin story sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Origin Stories/Lab Accidents", Some("Superhero & Comic Book/Scenarios & Encounters/Origin Stories"), None, "Lab accident sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Origin Stories/Alien Encounters", Some("Superhero & Comic Book/Scenarios & Encounters/Origin Stories"), None, "Alien encounter sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Origin Stories/Mystical Events", Some("Superhero & Comic Book/Scenarios & Encounters/Origin Stories"), None, "Mystical event sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Origin Stories/Tragic Backstories", Some("Superhero & Comic Book/Scenarios & Encounters/Origin Stories"), None, "Tragic backstory sounds"),
    
    ("Superhero & Comic Book/Scenarios & Encounters/Villain Confrontations", Some("Superhero & Comic Book/Scenarios & Encounters"), Some("👹"), "Villain confrontation sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Villain Confrontations/Bank Heists", Some("Superhero & Comic Book/Scenarios & Encounters/Villain Confrontations"), None, "Bank heist sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Villain Confrontations/Hostage Situations", Some("Superhero & Comic Book/Scenarios & Encounters/Villain Confrontations"), None, "Hostage situation sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Villain Confrontations/World Domination", Some("Superhero & Comic Book/Scenarios & Encounters/Villain Confrontations"), None, "World domination sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Villain Confrontations/Personal Vendettas", Some("Superhero & Comic Book/Scenarios & Encounters/Villain Confrontations"), None, "Personal vendetta sounds"),
    
    ("Superhero & Comic Book/Scenarios & Encounters/Team Dynamics", Some("Superhero & Comic Book/Scenarios & Encounters"), Some("👥"), "Team dynamic sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Team Dynamics/Team Formations", Some("Superhero & Comic Book/Scenarios & Encounters/Team Dynamics"), None, "Team formation sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Team Dynamics/Internal Conflicts", Some("Superhero & Comic Book/Scenarios & Encounters/Team Dynamics"), None, "Internal conflict sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Team Dynamics/Training Sessions", Some("Superhero & Comic Book/Scenarios & Encounters/Team Dynamics"), None, "Training session sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Team Dynamics/Team Bonding", Some("Superhero & Comic Book/Scenarios & Encounters/Team Dynamics"), None, "Team bonding sounds"),
    
    ("Superhero & Comic Book/Scenarios & Encounters/Crisis Events", Some("Superhero & Comic Book/Scenarios & Encounters"), Some("🌍"), "Crisis event sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Crisis Events/City-Wide Threats", Some("Superhero & Comic Book/Scenarios & Encounters/Crisis Events"), None, "City-wide threat sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Crisis Events/Dimensional Invasions", Some("Superhero & Comic Book/Scenarios & Encounters/Crisis Events"), None, "Dimensional invasion sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Crisis Events/Time Paradoxes", Some("Superhero & Comic Book/Scenarios & Encounters/Crisis Events"), None, "Time paradox sounds"),
    ("Superhero & Comic Book/Scenarios & Encounters/Crisis Events/Cosmic Events", Some("Superhero & Comic Book/Scenarios & Encounters/Crisis Events"), None, "Cosmic event sounds"),
    
    ("Superhero & Comic Book/Civilian Life", Some("Superhero & Comic Book"), Some("👔"), "Civilian life sounds"),
    ("Superhero & Comic Book/Civilian Life/Secret Identity", Some("Superhero & Comic Book/Civilian Life"), Some("🕵️"), "Secret identity sounds"),
    ("Superhero & Comic Book/Civilian Life/Secret Identity/Day Jobs", Some("Superhero & Comic Book/Civilian Life/Secret Identity"), None, "Day job sounds"),
    ("Superhero & Comic Book/Civilian Life/Secret Identity/Relationships", Some("Superhero & Comic Book/Civilian Life/Secret Identity"), None, "Relationship sounds"),
    ("Superhero & Comic Book/Civilian Life/Secret Identity/Family Life", Some("Superhero & Comic Book/Civilian Life/Secret Identity"), None, "Family life sounds"),
    ("Superhero & Comic Book/Civilian Life/Secret Identity/Identity Crises", Some("Superhero & Comic Book/Civilian Life/Secret Identity"), None, "Identity crisis sounds"),
    
    ("Superhero & Comic Book/Civilian Life/Media & Press", Some("Superhero & Comic Book/Civilian Life"), Some("📺"), "Media and press sounds"),
    ("Superhero & Comic Book/Civilian Life/Media & Press/News Reports", Some("Superhero & Comic Book/Civilian Life/Media & Press"), None, "News report sounds"),
    ("Superhero & Comic Book/Civilian Life/Media & Press/Interviews", Some("Superhero & Comic Book/Civilian Life/Media & Press"), None, "Interview sounds"),
    ("Superhero & Comic Book/Civilian Life/Media & Press/Public Opinion", Some("Superhero & Comic Book/Civilian Life/Media & Press"), None, "Public opinion sounds"),
    ("Superhero & Comic Book/Civilian Life/Media & Press/Paparazzi", Some("Superhero & Comic Book/Civilian Life/Media & Press"), None, "Paparazzi sounds"),
    
    ("Superhero & Comic Book/Civilian Life/Government Relations", Some("Superhero & Comic Book/Civilian Life"), Some("🏛️"), "Government relation sounds"),
    ("Superhero & Comic Book/Civilian Life/Government Relations/Registration Acts", Some("Superhero & Comic Book/Civilian Life/Government Relations"), None, "Registration act sounds"),
    ("Superhero & Comic Book/Civilian Life/Government Relations/Oversight Committees", Some("Superhero & Comic Book/Civilian Life/Government Relations"), None, "Oversight committee sounds"),
    ("Superhero & Comic Book/Civilian Life/Government Relations/Military Cooperation", Some("Superhero & Comic Book/Civilian Life/Government Relations"), None, "Military cooperation sounds"),
    ("Superhero & Comic Book/Civilian Life/Government Relations/Legal Issues", Some("Superhero & Comic Book/Civilian Life/Government Relations"), None, "Legal issue sounds"),
    
    ("Superhero & Comic Book/Civilian Life/Public Events", Some("Superhero & Comic Book/Civilian Life"), Some("🎉"), "Public event sounds"),
    ("Superhero & Comic Book/Civilian Life/Public Events/Charity Functions", Some("Superhero & Comic Book/Civilian Life/Public Events"), None, "Charity function sounds"),
    ("Superhero & Comic Book/Civilian Life/Public Events/Award Ceremonies", Some("Superhero & Comic Book/Civilian Life/Public Events"), None, "Award ceremony sounds"),
    ("Superhero & Comic Book/Civilian Life/Public Events/Parades", Some("Superhero & Comic Book/Civilian Life/Public Events"), None, "Parade sounds"),
    ("Superhero & Comic Book/Civilian Life/Public Events/Protests", Some("Superhero & Comic Book/Civilian Life/Public Events"), None, "Protest sounds"),

    // === MOODS & ATMOSPHERE ===
    ("Moods & Atmosphere/Positive", Some("Moods & Atmosphere"), Some("😊"), "Positive mood sounds"),
    ("Moods & Atmosphere/Positive/Heroic & Triumphant", Some("Moods & Atmosphere/Positive"), None, "Heroic and triumphant sounds"),
    ("Moods & Atmosphere/Positive/Peaceful & Serene", Some("Moods & Atmosphere/Positive"), None, "Peaceful and serene sounds"),
    ("Moods & Atmosphere/Positive/Cheerful & Upbeat", Some("Moods & Atmosphere/Positive"), None, "Cheerful and upbeat sounds"),
    ("Moods & Atmosphere/Positive/Romantic & Loving", Some("Moods & Atmosphere/Positive"), None, "Romantic and loving sounds"),
    ("Moods & Atmosphere/Positive/Adventurous", Some("Moods & Atmosphere/Positive"), None, "Adventurous sounds"),
    ("Moods & Atmosphere/Positive/Celebratory", Some("Moods & Atmosphere/Positive"), None, "Celebratory sounds"),
    
    ("Moods & Atmosphere/Neutral", Some("Moods & Atmosphere"), Some("😐"), "Neutral mood sounds"),
    ("Moods & Atmosphere/Neutral/Mysterious", Some("Moods & Atmosphere/Neutral"), None, "Mysterious sounds"),
    ("Moods & Atmosphere/Neutral/Contemplative", Some("Moods & Atmosphere/Neutral"), None, "Contemplative sounds"),
    ("Moods & Atmosphere/Neutral/Ethereal", Some("Moods & Atmosphere/Neutral"), None, "Ethereal sounds"),
    ("Moods & Atmosphere/Neutral/Ceremonial", Some("Moods & Atmosphere/Neutral"), None, "Ceremonial sounds"),
    
    ("Moods & Atmosphere/Dark", Some("Moods & Atmosphere"), Some("😰"), "Dark mood sounds"),
    ("Moods & Atmosphere/Dark/Ominous & Foreboding", Some("Moods & Atmosphere/Dark"), None, "Ominous and foreboding sounds"),
    ("Moods & Atmosphere/Dark/Sinister & Evil", Some("Moods & Atmosphere/Dark"), None, "Sinister and evil sounds"),
    ("Moods & Atmosphere/Dark/Tense", Some("Moods & Atmosphere/Dark"), None, "Tense sounds"),
    ("Moods & Atmosphere/Dark/Gothic", Some("Moods & Atmosphere/Dark"), None, "Gothic sounds"),
    ("Moods & Atmosphere/Dark/Tragic & Sorrowful", Some("Moods & Atmosphere/Dark"), None, "Tragic and sorrowful sounds"),
    ("Moods & Atmosphere/Dark/Corrupted & Tainted", Some("Moods & Atmosphere/Dark"), None, "Corrupted and tainted sounds"),
    
    ("Moods & Atmosphere/Action", Some("Moods & Atmosphere"), Some("⚡"), "Action mood sounds"),
    ("Moods & Atmosphere/Action/Rising Tension", Some("Moods & Atmosphere/Action"), None, "Rising tension sounds"),
    ("Moods & Atmosphere/Action/Aggressive & Violent", Some("Moods & Atmosphere/Action"), None, "Aggressive and violent sounds"),
    ("Moods & Atmosphere/Action/Building", Some("Moods & Atmosphere/Action"), None, "Building action sounds"),
    ("Moods & Atmosphere/Action/Brooding Intensity", Some("Moods & Atmosphere/Action"), None, "Brooding intensity sounds"),

    // === ACTIVITIES & CRAFTS ===
    ("Activities & Crafts/Artisan Crafts", Some("Activities & Crafts"), Some("⚒️"), "Artisan craft sounds"),
    
    // Fine Arts
    ("Activities & Crafts/Artistic Crafts", Some("Activities & Crafts"), Some("🎨"), "Artistic craft sounds"),
    ("Activities & Crafts/Artistic Crafts/Fine Arts", Some("Activities & Crafts/Artistic Crafts"), Some("🖼️"), "Fine arts sounds"),
    ("Activities & Crafts/Artistic Crafts/Fine Arts/Calligraphy", Some("Activities & Crafts/Artistic Crafts/Fine Arts"), None, "Calligraphy sounds"),
    ("Activities & Crafts/Artistic Crafts/Fine Arts/Mosaics", Some("Activities & Crafts/Artistic Crafts/Fine Arts"), None, "Mosaic creation sounds"),
    ("Activities & Crafts/Artistic Crafts/Fine Arts/Painting", Some("Activities & Crafts/Artistic Crafts/Fine Arts"), None, "Painting sounds"),
    ("Activities & Crafts/Artistic Crafts/Fine Arts/Sculpting", Some("Activities & Crafts/Artistic Crafts/Fine Arts"), None, "Sculpting sounds"),
    
    // Performance Arts
    ("Activities & Crafts/Artistic Crafts/Performance Arts", Some("Activities & Crafts/Artistic Crafts"), Some("🎭"), "Performance arts sounds"),
    ("Activities & Crafts/Artistic Crafts/Performance Arts/Dance", Some("Activities & Crafts/Artistic Crafts/Performance Arts"), None, "Dance sounds"),
    ("Activities & Crafts/Artistic Crafts/Performance Arts/Singing", Some("Activities & Crafts/Artistic Crafts/Performance Arts"), None, "Singing sounds"),
    ("Activities & Crafts/Artistic Crafts/Performance Arts/Storytelling", Some("Activities & Crafts/Artistic Crafts/Performance Arts"), None, "Storytelling sounds"),
    ("Activities & Crafts/Artistic Crafts/Performance Arts/Theater", Some("Activities & Crafts/Artistic Crafts/Performance Arts"), None, "Theater sounds"),
    
    ("Activities & Crafts/Artisan Crafts/Blacksmithing", Some("Activities & Crafts/Artisan Crafts"), Some("🔥"), "Blacksmithing sounds"),
    ("Activities & Crafts/Artisan Crafts/Blacksmithing/Forges", Some("Activities & Crafts/Artisan Crafts/Blacksmithing"), None, "Forge sounds"),
    ("Activities & Crafts/Artisan Crafts/Blacksmithing/Anvil Work", Some("Activities & Crafts/Artisan Crafts/Blacksmithing"), None, "Anvil work sounds"),
    ("Activities & Crafts/Artisan Crafts/Blacksmithing/Weapon Making", Some("Activities & Crafts/Artisan Crafts/Blacksmithing"), None, "Weapon making sounds"),
    
    ("Activities & Crafts/Artisan Crafts/Alchemy", Some("Activities & Crafts/Artisan Crafts"), Some("⚗️"), "Alchemy sounds"),
    ("Activities & Crafts/Artisan Crafts/Alchemy/Brewing", Some("Activities & Crafts/Artisan Crafts/Alchemy"), None, "Brewing sounds"),
    ("Activities & Crafts/Artisan Crafts/Alchemy/Laboratories", Some("Activities & Crafts/Artisan Crafts/Alchemy"), None, "Laboratory sounds"),
    ("Activities & Crafts/Artisan Crafts/Alchemy/Experiments", Some("Activities & Crafts/Artisan Crafts/Alchemy"), None, "Experiment sounds"),
    
    ("Activities & Crafts/Artisan Crafts/Enchanting", Some("Activities & Crafts/Artisan Crafts"), Some("✨"), "Enchanting sounds"),
    ("Activities & Crafts/Artisan Crafts/Enchanting/Rituals", Some("Activities & Crafts/Artisan Crafts/Enchanting"), None, "Enchanting ritual sounds"),
    ("Activities & Crafts/Artisan Crafts/Enchanting/Rune Carving", Some("Activities & Crafts/Artisan Crafts/Enchanting"), None, "Rune carving sounds"),
    ("Activities & Crafts/Artisan Crafts/Enchanting/Magical Infusion", Some("Activities & Crafts/Artisan Crafts/Enchanting"), None, "Magical infusion sounds"),
    
    // Professional Crafts
    ("Activities & Crafts/Professional Crafts", Some("Activities & Crafts"), Some("🏭"), "Professional craft sounds"),
    ("Activities & Crafts/Professional Crafts/Alchemy", Some("Activities & Crafts/Professional Crafts"), Some("⚗️"), "Professional alchemy sounds"),
    ("Activities & Crafts/Professional Crafts/Alchemy/Elixir Creation", Some("Activities & Crafts/Professional Crafts/Alchemy"), None, "Elixir creation sounds"),
    ("Activities & Crafts/Professional Crafts/Alchemy/Herb Preparation", Some("Activities & Crafts/Professional Crafts/Alchemy"), None, "Herb preparation sounds"),
    ("Activities & Crafts/Professional Crafts/Alchemy/Potion Brewing", Some("Activities & Crafts/Professional Crafts/Alchemy"), None, "Potion brewing sounds"),
    ("Activities & Crafts/Professional Crafts/Alchemy/Transmutation", Some("Activities & Crafts/Professional Crafts/Alchemy"), None, "Transmutation sounds"),
    
    ("Activities & Crafts/Professional Crafts/Construction", Some("Activities & Crafts/Professional Crafts"), Some("🏗️"), "Construction sounds"),
    ("Activities & Crafts/Professional Crafts/Construction/Bridge Building", Some("Activities & Crafts/Professional Crafts/Construction"), None, "Bridge building sounds"),
    ("Activities & Crafts/Professional Crafts/Construction/Fortification", Some("Activities & Crafts/Professional Crafts/Construction"), None, "Fortification sounds"),
    ("Activities & Crafts/Professional Crafts/Construction/House Building", Some("Activities & Crafts/Professional Crafts/Construction"), None, "House building sounds"),
    ("Activities & Crafts/Professional Crafts/Construction/Shipbuilding", Some("Activities & Crafts/Professional Crafts/Construction"), None, "Shipbuilding sounds"),
    
    ("Activities & Crafts/Professional Crafts/Enchanting", Some("Activities & Crafts/Professional Crafts"), Some("✨"), "Professional enchanting sounds"),
    ("Activities & Crafts/Professional Crafts/Enchanting/Item Enchanting", Some("Activities & Crafts/Professional Crafts/Enchanting"), None, "Item enchanting sounds"),
    ("Activities & Crafts/Professional Crafts/Enchanting/Rune Carving", Some("Activities & Crafts/Professional Crafts/Enchanting"), None, "Rune carving sounds"),
    ("Activities & Crafts/Professional Crafts/Enchanting/Ward Creation", Some("Activities & Crafts/Professional Crafts/Enchanting"), None, "Ward creation sounds"),
    ("Activities & Crafts/Professional Crafts/Enchanting/Weapon Enchanting", Some("Activities & Crafts/Professional Crafts/Enchanting"), None, "Weapon enchanting sounds"),
    
    ("Activities & Crafts/Professional Crafts/Smithing", Some("Activities & Crafts/Professional Crafts"), Some("⚒️"), "Professional smithing sounds"),
    ("Activities & Crafts/Professional Crafts/Smithing/Armorsmithing", Some("Activities & Crafts/Professional Crafts/Smithing"), None, "Armorsmithing sounds"),
    ("Activities & Crafts/Professional Crafts/Smithing/Blacksmithing", Some("Activities & Crafts/Professional Crafts/Smithing"), None, "Professional blacksmithing sounds"),
    ("Activities & Crafts/Professional Crafts/Smithing/Jewelry Making", Some("Activities & Crafts/Professional Crafts/Smithing"), None, "Jewelry making sounds"),
    ("Activities & Crafts/Professional Crafts/Smithing/Weaponsmithing", Some("Activities & Crafts/Professional Crafts/Smithing"), None, "Weaponsmithing sounds"),
    
    // Mundane Activities
    ("Activities & Crafts/Mundane Activities", Some("Activities & Crafts"), Some("🏠"), "Mundane activity sounds"),
    ("Activities & Crafts/Mundane Activities/Daily Tasks", Some("Activities & Crafts/Mundane Activities"), Some("📋"), "Daily task sounds"),
    ("Activities & Crafts/Mundane Activities/Daily Tasks/Cleaning", Some("Activities & Crafts/Mundane Activities/Daily Tasks"), None, "Cleaning sounds"),
    ("Activities & Crafts/Mundane Activities/Daily Tasks/Cooking", Some("Activities & Crafts/Mundane Activities/Daily Tasks"), None, "Cooking sounds"),
    ("Activities & Crafts/Mundane Activities/Daily Tasks/Farming", Some("Activities & Crafts/Mundane Activities/Daily Tasks"), None, "Farming sounds"),
    ("Activities & Crafts/Mundane Activities/Daily Tasks/Shopping", Some("Activities & Crafts/Mundane Activities/Daily Tasks"), None, "Shopping sounds"),
    ("Activities & Crafts/Mundane Activities/Daily Tasks/Traveling", Some("Activities & Crafts/Mundane Activities/Daily Tasks"), None, "Traveling sounds"),
    
    ("Activities & Crafts/Mundane Activities/Hobbies", Some("Activities & Crafts/Mundane Activities"), Some("🎯"), "Hobby sounds"),
    ("Activities & Crafts/Mundane Activities/Hobbies/Exercise", Some("Activities & Crafts/Mundane Activities/Hobbies"), None, "Exercise sounds"),
    ("Activities & Crafts/Mundane Activities/Hobbies/Gardening", Some("Activities & Crafts/Mundane Activities/Hobbies"), None, "Gardening sounds"),
    ("Activities & Crafts/Mundane Activities/Hobbies/Music Practice", Some("Activities & Crafts/Mundane Activities/Hobbies"), None, "Music practice sounds"),
    ("Activities & Crafts/Mundane Activities/Hobbies/Painting", Some("Activities & Crafts/Mundane Activities/Hobbies"), None, "Painting sounds"),
    ("Activities & Crafts/Mundane Activities/Hobbies/Reading", Some("Activities & Crafts/Mundane Activities/Hobbies"), None, "Reading sounds"),

    ("Activities & Crafts/Daily Life", Some("Activities & Crafts"), Some("🏠"), "Daily life sounds"),
    ("Activities & Crafts/Daily Life/Cooking", Some("Activities & Crafts/Daily Life"), Some("🍳"), "Cooking sounds"),
    ("Activities & Crafts/Daily Life/Cooking/Kitchens", Some("Activities & Crafts/Daily Life/Cooking"), None, "Kitchen sounds"),
    ("Activities & Crafts/Daily Life/Cooking/Hearths", Some("Activities & Crafts/Daily Life/Cooking"), None, "Hearth sounds"),
    ("Activities & Crafts/Daily Life/Cooking/Feasts", Some("Activities & Crafts/Daily Life/Cooking"), None, "Feast sounds"),
    
    ("Activities & Crafts/Daily Life/Sleep & Rest", Some("Activities & Crafts/Daily Life"), Some("🛏️"), "Sleep and rest sounds"),
    ("Activities & Crafts/Daily Life/Sleep & Rest/Bedrooms", Some("Activities & Crafts/Daily Life/Sleep & Rest"), None, "Bedroom sounds"),
    ("Activities & Crafts/Daily Life/Sleep & Rest/Campfires", Some("Activities & Crafts/Daily Life/Sleep & Rest"), None, "Campfire sounds"),
    ("Activities & Crafts/Daily Life/Sleep & Rest/Night Watches", Some("Activities & Crafts/Daily Life/Sleep & Rest"), None, "Night watch sounds"),

    // === CULTURAL STYLES ===
    ("Cultural Styles/Ancient Civilizations", Some("Cultural Styles"), Some("🏛️"), "Ancient civilization sounds"),
    ("Cultural Styles/Ancient Civilizations/Ancient Greek", Some("Cultural Styles/Ancient Civilizations"), Some("🏛️"), "Ancient Greek sounds"),
    ("Cultural Styles/Ancient Civilizations/Ancient Greek/Temples", Some("Cultural Styles/Ancient Civilizations/Ancient Greek"), None, "Greek temple sounds"),
    ("Cultural Styles/Ancient Civilizations/Ancient Greek/Amphitheaters", Some("Cultural Styles/Ancient Civilizations/Ancient Greek"), None, "Greek amphitheater sounds"),
    ("Cultural Styles/Ancient Civilizations/Ancient Greek/Marketplaces", Some("Cultural Styles/Ancient Civilizations/Ancient Greek"), None, "Greek marketplace sounds"),
    
    ("Cultural Styles/Ancient Civilizations/Ancient Roman", Some("Cultural Styles/Ancient Civilizations"), Some("🏛️"), "Ancient Roman sounds"),
    ("Cultural Styles/Ancient Civilizations/Ancient Roman/Forums", Some("Cultural Styles/Ancient Civilizations/Ancient Roman"), None, "Roman forum sounds"),
    ("Cultural Styles/Ancient Civilizations/Ancient Roman/Colosseum", Some("Cultural Styles/Ancient Civilizations/Ancient Roman"), None, "Colosseum sounds"),
    ("Cultural Styles/Ancient Civilizations/Ancient Roman/Bath Houses", Some("Cultural Styles/Ancient Civilizations/Ancient Roman"), None, "Roman bath house sounds"),
    
    ("Cultural Styles/Ancient Civilizations/Egyptian", Some("Cultural Styles/Ancient Civilizations"), Some("🏜️"), "Ancient Egyptian sounds"),
    ("Cultural Styles/Ancient Civilizations/Egyptian/Temples", Some("Cultural Styles/Ancient Civilizations/Egyptian"), None, "Egyptian temple sounds"),
    ("Cultural Styles/Ancient Civilizations/Egyptian/Pyramids", Some("Cultural Styles/Ancient Civilizations/Egyptian"), None, "Pyramid sounds"),
    ("Cultural Styles/Ancient Civilizations/Egyptian/Tombs", Some("Cultural Styles/Ancient Civilizations/Egyptian"), None, "Egyptian tomb sounds"),
    
    ("Cultural Styles/Ancient Civilizations/Norse", Some("Cultural Styles/Ancient Civilizations"), Some("⚔️"), "Norse sounds"),
    ("Cultural Styles/Ancient Civilizations/Norse/Battlefields", Some("Cultural Styles/Ancient Civilizations/Norse"), None, "Norse battlefield sounds"),
    ("Cultural Styles/Ancient Civilizations/Norse/Mead Halls", Some("Cultural Styles/Ancient Civilizations/Norse"), None, "Mead hall sounds"),
    ("Cultural Styles/Ancient Civilizations/Norse/Longships", Some("Cultural Styles/Ancient Civilizations/Norse"), None, "Longship sounds"),
    
    ("Cultural Styles/Medieval & Renaissance", Some("Cultural Styles"), Some("🏰"), "Medieval and Renaissance sounds"),
    ("Cultural Styles/Medieval & Renaissance/Medieval European", Some("Cultural Styles/Medieval & Renaissance"), Some("🏰"), "Medieval European sounds"),
    ("Cultural Styles/Medieval & Renaissance/Medieval European/Castles", Some("Cultural Styles/Medieval & Renaissance/Medieval European"), None, "Medieval castle sounds"),
    ("Cultural Styles/Medieval & Renaissance/Medieval European/Monasteries", Some("Cultural Styles/Medieval & Renaissance/Medieval European"), None, "Monastery sounds"),
    ("Cultural Styles/Medieval & Renaissance/Medieval European/Tournament Grounds", Some("Cultural Styles/Medieval & Renaissance/Medieval European"), None, "Tournament ground sounds"),
    
    ("Cultural Styles/Eastern Traditions", Some("Cultural Styles"), Some("🏯"), "Eastern tradition sounds"),
    ("Cultural Styles/Eastern Traditions/Japanese Traditional", Some("Cultural Styles/Eastern Traditions"), Some("🏯"), "Japanese traditional sounds"),
    ("Cultural Styles/Eastern Traditions/Japanese Traditional/Temples", Some("Cultural Styles/Eastern Traditions/Japanese Traditional"), None, "Japanese temple sounds"),
    ("Cultural Styles/Eastern Traditions/Japanese Traditional/Dojos", Some("Cultural Styles/Eastern Traditions/Japanese Traditional"), None, "Dojo sounds"),
    ("Cultural Styles/Eastern Traditions/Japanese Traditional/Gardens", Some("Cultural Styles/Eastern Traditions/Japanese Traditional"), None, "Japanese garden sounds"),
    
    ("Cultural Styles/Eastern Traditions/Chinese Traditional", Some("Cultural Styles/Eastern Traditions"), Some("🏯"), "Chinese traditional sounds"),
    ("Cultural Styles/Eastern Traditions/Chinese Traditional/Palaces", Some("Cultural Styles/Eastern Traditions/Chinese Traditional"), None, "Chinese palace sounds"),
    ("Cultural Styles/Eastern Traditions/Chinese Traditional/Tea Houses", Some("Cultural Styles/Eastern Traditions/Chinese Traditional"), None, "Tea house sounds"),
    ("Cultural Styles/Eastern Traditions/Chinese Traditional/Martial Schools", Some("Cultural Styles/Eastern Traditions/Chinese Traditional"), None, "Martial school sounds"),

    // === FANTASY GENRES ===
    ("Fantasy Genres/High Fantasy", Some("Fantasy Genres"), Some("⚔️"), "High fantasy sounds"),
    ("Fantasy Genres/High Fantasy/Epic Quests", Some("Fantasy Genres/High Fantasy"), None, "Epic quest sounds"),
    ("Fantasy Genres/High Fantasy/Chosen Ones", Some("Fantasy Genres/High Fantasy"), None, "Chosen one sounds"),
    ("Fantasy Genres/High Fantasy/Ancient Prophecies", Some("Fantasy Genres/High Fantasy"), None, "Ancient prophecy sounds"),
    
    ("Fantasy Genres/Dark Fantasy", Some("Fantasy Genres"), Some("☠️"), "Dark fantasy sounds"),
    ("Fantasy Genres/Dark Fantasy/Gothic Horror", Some("Fantasy Genres/Dark Fantasy"), None, "Gothic horror sounds"),
    ("Fantasy Genres/Dark Fantasy/Corruption & Evil", Some("Fantasy Genres/Dark Fantasy"), None, "Corruption and evil sounds"),
    ("Fantasy Genres/Dark Fantasy/Moral Ambiguity", Some("Fantasy Genres/Dark Fantasy"), None, "Moral ambiguity sounds"),
    
    ("Fantasy Genres/Urban Fantasy", Some("Fantasy Genres"), Some("🌃"), "Urban fantasy sounds"),
    ("Fantasy Genres/Urban Fantasy/Modern Magic", Some("Fantasy Genres/Urban Fantasy"), None, "Modern magic sounds"),
    ("Fantasy Genres/Urban Fantasy/Hidden Worlds", Some("Fantasy Genres/Urban Fantasy"), None, "Hidden world sounds"),
    ("Fantasy Genres/Urban Fantasy/Supernatural Detective", Some("Fantasy Genres/Urban Fantasy"), None, "Supernatural detective sounds"),
    
    ("Fantasy Genres/Sword & Sorcery", Some("Fantasy Genres"), Some("🗡️"), "Sword and sorcery sounds"),
    ("Fantasy Genres/Sword & Sorcery/Barbarian Heroes", Some("Fantasy Genres/Sword & Sorcery"), None, "Barbarian hero sounds"),
    ("Fantasy Genres/Sword & Sorcery/Lost Civilizations", Some("Fantasy Genres/Sword & Sorcery"), None, "Lost civilization sounds"),
    ("Fantasy Genres/Sword & Sorcery/Pulp Adventure", Some("Fantasy Genres/Sword & Sorcery"), None, "Pulp adventure sounds"),

    // === SFX & FOLEY ===
    ("SFX & Foley/Combat Sounds", Some("SFX & Foley"), Some("⚔️"), "Combat sound effects"),
    ("SFX & Foley/Combat Sounds/Weapon Impacts", Some("SFX & Foley/Combat Sounds"), Some("💥"), "Weapon impact sounds"),
    ("SFX & Foley/Combat Sounds/Weapon Impacts/Sword Clashing", Some("SFX & Foley/Combat Sounds/Weapon Impacts"), None, "Sword clashing sounds"),
    ("SFX & Foley/Combat Sounds/Weapon Impacts/Arrow Hits", Some("SFX & Foley/Combat Sounds/Weapon Impacts"), None, "Arrow hit sounds"),
    ("SFX & Foley/Combat Sounds/Weapon Impacts/Blunt Impacts", Some("SFX & Foley/Combat Sounds/Weapon Impacts"), None, "Blunt impact sounds"),
    
    ("SFX & Foley/Combat Sounds/Magic Effects", Some("SFX & Foley/Combat Sounds"), Some("✨"), "Magic effect sounds"),
    ("SFX & Foley/Combat Sounds/Magic Effects/Spell Casting", Some("SFX & Foley/Combat Sounds/Magic Effects"), None, "Spell casting sounds"),
    ("SFX & Foley/Combat Sounds/Magic Effects/Energy Shields", Some("SFX & Foley/Combat Sounds/Magic Effects"), None, "Energy shield sounds"),
    ("SFX & Foley/Combat Sounds/Magic Effects/Explosions", Some("SFX & Foley/Combat Sounds/Magic Effects"), None, "Magic explosion sounds"),
    
    ("SFX & Foley/Combat Sounds/Armor & Movement", Some("SFX & Foley/Combat Sounds"), Some("🛡️"), "Armor and movement sounds"),
    ("SFX & Foley/Combat Sounds/Armor & Movement/Footsteps", Some("SFX & Foley/Combat Sounds/Armor & Movement"), None, "Footstep sounds"),
    ("SFX & Foley/Combat Sounds/Armor & Movement/Armor Clanking", Some("SFX & Foley/Combat Sounds/Armor & Movement"), None, "Armor clanking sounds"),
    ("SFX & Foley/Combat Sounds/Armor & Movement/Combat Grunts", Some("SFX & Foley/Combat Sounds/Armor & Movement"), None, "Combat grunt sounds"),
    
    ("SFX & Foley/Creature Sounds", Some("SFX & Foley"), Some("🐲"), "Creature sound effects"),
    ("SFX & Foley/Creature Sounds/Dragon Breath", Some("SFX & Foley/Creature Sounds"), None, "Dragon breath sounds"),
    ("SFX & Foley/Creature Sounds/Monster Roars", Some("SFX & Foley/Creature Sounds"), None, "Monster roar sounds"),
    ("SFX & Foley/Creature Sounds/Beast Growls", Some("SFX & Foley/Creature Sounds"), None, "Beast growl sounds"),
    ("SFX & Foley/Creature Sounds/Creature Movement", Some("SFX & Foley/Creature Sounds"), None, "Creature movement sounds"),
    
    ("SFX & Foley/Environment Foley", Some("SFX & Foley"), Some("🌲"), "Environment foley sounds"),
    ("SFX & Foley/Environment Foley/Urban Sounds", Some("SFX & Foley/Environment Foley"), Some("🏙️"), "Urban foley sounds"),
    ("SFX & Foley/Environment Foley/Urban Sounds/Tavern Murmurs", Some("SFX & Foley/Environment Foley/Urban Sounds"), None, "Tavern murmur sounds"),
    ("SFX & Foley/Environment Foley/Urban Sounds/Market Bustle", Some("SFX & Foley/Environment Foley/Urban Sounds"), None, "Market bustle sounds"),
    ("SFX & Foley/Environment Foley/Urban Sounds/Street Noise", Some("SFX & Foley/Environment Foley/Urban Sounds"), None, "Street noise sounds"),
    
    ("SFX & Foley/Environment Foley/Nature Sounds", Some("SFX & Foley/Environment Foley"), Some("🌳"), "Nature foley sounds"),
    ("SFX & Foley/Environment Foley/Nature Sounds/Forest Ambience", Some("SFX & Foley/Environment Foley/Nature Sounds"), None, "Forest ambience sounds"),
    ("SFX & Foley/Environment Foley/Nature Sounds/Ocean Waves", Some("SFX & Foley/Environment Foley/Nature Sounds"), None, "Ocean wave sounds"),
    ("SFX & Foley/Environment Foley/Nature Sounds/Wind & Weather", Some("SFX & Foley/Environment Foley/Nature Sounds"), None, "Wind and weather sounds"),

    // === SESSION STRUCTURE ===
    ("Session Structure/Opening", Some("Session Structure"), Some("🎬"), "Session opening sounds"),
    ("Session Structure/Opening/Recap", Some("Session Structure/Opening"), None, "Session recap sounds"),
    ("Session Structure/Opening/Setting Scene", Some("Session Structure/Opening"), None, "Setting scene sounds"),
    ("Session Structure/Opening/Call to Adventure", Some("Session Structure/Opening"), None, "Call to adventure sounds"),
    
    ("Session Structure/Exploration", Some("Session Structure"), Some("🗺️"), "Session exploration sounds"),
    ("Session Structure/Exploration/Travel", Some("Session Structure/Exploration"), None, "Travel sounds"),
    ("Session Structure/Exploration/Discovery", Some("Session Structure/Exploration"), None, "Discovery sounds"),
    ("Session Structure/Exploration/Mapping", Some("Session Structure/Exploration"), None, "Mapping sounds"),
    
    ("Session Structure/Challenges", Some("Session Structure"), Some("🧩"), "Session challenge sounds"),
    ("Session Structure/Challenges/Puzzles", Some("Session Structure/Challenges"), None, "Puzzle sounds"),
    ("Session Structure/Challenges/Traps", Some("Session Structure/Challenges"), None, "Trap sounds"),
    ("Session Structure/Challenges/Social", Some("Session Structure/Challenges"), None, "Social challenge sounds"),
    ("Session Structure/Challenges/Physical", Some("Session Structure/Challenges"), None, "Physical challenge sounds"),
    
    ("Session Structure/Climax", Some("Session Structure"), Some("⚔️"), "Session climax sounds"),
    ("Session Structure/Climax/Boss Encounters", Some("Session Structure/Climax"), None, "Boss encounter sounds"),
    ("Session Structure/Climax/Major Revelations", Some("Session Structure/Climax"), None, "Major revelation sounds"),
    ("Session Structure/Climax/Key Decisions", Some("Session Structure/Climax"), None, "Key decision sounds"),
    
    ("Session Structure/Resolution", Some("Session Structure"), Some("🏆"), "Session resolution sounds"),
    ("Session Structure/Resolution/Victory Celebration", Some("Session Structure/Resolution"), None, "Victory celebration sounds"),
    ("Session Structure/Resolution/Character Development", Some("Session Structure/Resolution"), None, "Character development sounds"),
    ("Session Structure/Resolution/Next Steps", Some("Session Structure/Resolution"), None, "Next steps sounds"),

    // === SCI-FI GENRES ===
    ("Sci-Fi Genres", None, Some("🚀"), "Sci-Fi genre specific sounds"),
    ("Sci-Fi Genres/Space Opera", Some("Sci-Fi Genres"), Some("⭐"), "Space opera sounds"),
    ("Sci-Fi Genres/Space Opera/Epic Battles", Some("Sci-Fi Genres/Space Opera"), None, "Epic space battle sounds"),
    ("Sci-Fi Genres/Space Opera/Alien Worlds", Some("Sci-Fi Genres/Space Opera"), None, "Alien world sounds"),
    ("Sci-Fi Genres/Space Opera/Galactic Politics", Some("Sci-Fi Genres/Space Opera"), None, "Galactic politics sounds"),
    
    ("Sci-Fi Genres/Cyberpunk", Some("Sci-Fi Genres"), Some("💻"), "Cyberpunk sounds"),
    ("Sci-Fi Genres/Cyberpunk/Neon Cities", Some("Sci-Fi Genres/Cyberpunk"), None, "Neon city sounds"),
    ("Sci-Fi Genres/Cyberpunk/Corporate Control", Some("Sci-Fi Genres/Cyberpunk"), None, "Corporate control sounds"),
    ("Sci-Fi Genres/Cyberpunk/Digital Rebellion", Some("Sci-Fi Genres/Cyberpunk"), None, "Digital rebellion sounds"),
    
    ("Sci-Fi Genres/Post-Apocalyptic", Some("Sci-Fi Genres"), Some("☢️"), "Post-apocalyptic sounds"),
    ("Sci-Fi Genres/Post-Apocalyptic/Wasteland", Some("Sci-Fi Genres/Post-Apocalyptic"), None, "Wasteland sounds"),
    ("Sci-Fi Genres/Post-Apocalyptic/Survival", Some("Sci-Fi Genres/Post-Apocalyptic"), None, "Survival sounds"),
    
    // === AUDIO STRUCTURE - MAIN BODY ===
    ("Audio Structure/Main Body/Loops", Some("Audio Structure/Main Body"), Some("🔄"), "Loop segments"),
    ("Audio Structure/Main Body/Loops/Ambient Loop", Some("Audio Structure/Main Body/Loops"), None, "Ambient loop segments"),
    ("Audio Structure/Main Body/Loops/Evolving Loop", Some("Audio Structure/Main Body/Loops"), None, "Evolving loop segments"),
    ("Audio Structure/Main Body/Loops/Layered Loop", Some("Audio Structure/Main Body/Loops"), None, "Layered loop segments"),
    ("Audio Structure/Main Body/Loops/Rhythmic Loop", Some("Audio Structure/Main Body/Loops"), None, "Rhythmic loop segments"),
    ("Audio Structure/Main Body/Loops/Seamless Loop", Some("Audio Structure/Main Body/Loops"), None, "Seamless loop segments"),
    
    ("Audio Structure/Main Body/Variations", Some("Audio Structure/Main Body"), Some("🎵"), "Variation segments"),
    ("Audio Structure/Main Body/Variations/Dynamic Variations", Some("Audio Structure/Main Body/Variations"), None, "Dynamic variation segments"),
    ("Audio Structure/Main Body/Variations/Harmonic Variations", Some("Audio Structure/Main Body/Variations"), None, "Harmonic variation segments"),
    ("Audio Structure/Main Body/Variations/Tempo Variations", Some("Audio Structure/Main Body/Variations"), None, "Tempo variation segments"),
    ("Audio Structure/Main Body/Variations/Textural Variations", Some("Audio Structure/Main Body/Variations"), None, "Textural variation segments"),
    ("Audio Structure/Main Body/Variations/Theme Variations", Some("Audio Structure/Main Body/Variations"), None, "Theme variation segments"),
    
    // === AUDIO STRUCTURE - STRUCTURAL ELEMENTS ===
    ("Audio Structure/Structural Elements/Phrases", Some("Audio Structure/Structural Elements"), Some("🎼"), "Musical phrase segments"),
    ("Audio Structure/Structural Elements/Phrases/Call and Response", Some("Audio Structure/Structural Elements/Phrases"), None, "Call and response phrase segments"),
    ("Audio Structure/Structural Elements/Phrases/Contrapuntal Phrases", Some("Audio Structure/Structural Elements/Phrases"), None, "Contrapuntal phrase segments"),
    ("Audio Structure/Structural Elements/Phrases/Extended Phrases", Some("Audio Structure/Structural Elements/Phrases"), None, "Extended phrase segments"),
    ("Audio Structure/Structural Elements/Phrases/Musical Phrase", Some("Audio Structure/Structural Elements/Phrases"), None, "Musical phrase segments"),
    ("Audio Structure/Structural Elements/Phrases/Sequential Phrases", Some("Audio Structure/Structural Elements/Phrases"), None, "Sequential phrase segments"),
    
    // === AUDIO STRUCTURE - TRANSITION SEGMENTS ===
    ("Audio Structure/Transition Segments/Bridges", Some("Audio Structure/Transition Segments"), Some("🌉"), "Bridge segments"),
    ("Audio Structure/Transition Segments/Bridges/Breakdown", Some("Audio Structure/Transition Segments/Bridges"), None, "Breakdown bridge segments"),
    ("Audio Structure/Transition Segments/Bridges/Build Section", Some("Audio Structure/Transition Segments/Bridges"), None, "Build section bridge segments"),
    ("Audio Structure/Transition Segments/Bridges/Key Modulation", Some("Audio Structure/Transition Segments/Bridges"), None, "Key modulation bridge segments"),
    ("Audio Structure/Transition Segments/Bridges/Musical Bridge", Some("Audio Structure/Transition Segments/Bridges"), None, "Musical bridge segments"),
    ("Audio Structure/Transition Segments/Bridges/Tempo Change", Some("Audio Structure/Transition Segments/Bridges"), None, "Tempo change bridge segments"),
    
    ("Audio Structure/Transition Segments/Crossfades", Some("Audio Structure/Transition Segments"), Some("↔️"), "Crossfade segments"),
    ("Audio Structure/Transition Segments/Crossfades/Creative Crossfade", Some("Audio Structure/Transition Segments/Crossfades"), None, "Creative crossfade segments"),
    ("Audio Structure/Transition Segments/Crossfades/Extended Fade", Some("Audio Structure/Transition Segments/Crossfades"), None, "Extended fade segments"),
    ("Audio Structure/Transition Segments/Crossfades/Gated Crossfade", Some("Audio Structure/Transition Segments/Crossfades"), None, "Gated crossfade segments"),
    ("Audio Structure/Transition Segments/Crossfades/Quick Fade", Some("Audio Structure/Transition Segments/Crossfades"), None, "Quick fade segments"),
    ("Audio Structure/Transition Segments/Crossfades/Smooth Crossfade", Some("Audio Structure/Transition Segments/Crossfades"), None, "Smooth crossfade segments"),
    
    // === CREATURES - BEASTS ===
    ("Creatures/Beasts/Aquatic/Sea Creatures", Some("Creatures/Beasts/Aquatic"), None, "Sea creature sounds"),
    
    ("Creatures/Beasts/Companions/Dogs", Some("Creatures/Beasts/Companions"), None, "Dog companion sounds"),
    ("Creatures/Beasts/Companions/Farm Animals", Some("Creatures/Beasts/Companions"), None, "Farm animal companion sounds"),
    ("Creatures/Beasts/Companions/Horses", Some("Creatures/Beasts/Companions"), None, "Horse companion sounds"),
    
    ("Creatures/Beasts/Flying/Winged", Some("Creatures/Beasts/Flying"), None, "Winged creature sounds"),
    
    ("Creatures/Beasts/Hostile/Giant Spiders", Some("Creatures/Beasts/Hostile"), None, "Giant spider sounds"),
    
    ("Creatures/Beasts/Predators/Prehistoric", Some("Creatures/Beasts/Predators"), None, "Prehistoric predator sounds"),
    ("Creatures/Beasts/Predators/Serpents", Some("Creatures/Beasts/Predators"), None, "Serpent predator sounds"),
    
    ("Creatures/Beasts/Vermin/Insects", Some("Creatures/Beasts/Vermin"), None, "Insect vermin sounds"),
    ("Creatures/Beasts/Vermin/Rats", Some("Creatures/Beasts/Vermin"), None, "Rat vermin sounds"),
    
    // === CREATURES - HUMANOIDS ===
    ("Creatures/Humanoids/Hostile/Assassins", Some("Creatures/Humanoids/Hostile"), None, "Assassin sounds"),
    ("Creatures/Humanoids/Hostile/Giants", Some("Creatures/Humanoids/Hostile"), None, "Giant humanoid sounds"),
    ("Creatures/Humanoids/Hostile/Ogres", Some("Creatures/Humanoids/Hostile"), None, "Ogre sounds"),
    ("Creatures/Humanoids/Hostile/Trolls", Some("Creatures/Humanoids/Hostile"), None, "Troll sounds"),
    
    ("Creatures/Humanoids/Neutral/Civilians", Some("Creatures/Humanoids/Neutral"), None, "Civilian sounds"),
    
    // === CREATURES - SUPERNATURAL & UNDEAD ===
    ("Creatures/Supernatural/Aberrations", Some("Creatures/Supernatural"), None, "Aberration creature sounds"),
    ("Creatures/Supernatural/Constructs", Some("Creatures/Supernatural"), None, "Construct creature sounds"),
    ("Creatures/Supernatural/Spirits", Some("Creatures/Supernatural"), None, "Spirit creature sounds"),
    ("Creatures/Supernatural/Witches", Some("Creatures/Supernatural"), None, "Witch creature sounds"),
    
    ("Creatures/Undead/Lesser/Ghouls", Some("Creatures/Undead/Lesser"), None, "Ghoul sounds"),
    
    // === CULTURAL STYLES - AGE OF EXPLORATION ===
    ("Cultural Styles/Age of Exploration/Pirate Era", Some("Cultural Styles/Age of Exploration"), None, "Pirate era sounds"),
    
    // === CULTURAL STYLES - ANCIENT CIVILIZATIONS ===
    ("Cultural Styles/Ancient Civilizations/Arabian Nights", Some("Cultural Styles/Ancient Civilizations"), None, "Arabian Nights sounds"),
    ("Cultural Styles/Ancient Civilizations/Asian Dynasties", Some("Cultural Styles/Ancient Civilizations"), None, "Asian dynasties sounds"),
    ("Cultural Styles/Ancient Civilizations/Aztec & Mayan", Some("Cultural Styles/Ancient Civilizations"), None, "Aztec & Mayan sounds"),
    ("Cultural Styles/Ancient Civilizations/Egyptian Pharaohs", Some("Cultural Styles/Ancient Civilizations"), None, "Egyptian pharaoh sounds"),
    ("Cultural Styles/Ancient Civilizations/Greek & Roman", Some("Cultural Styles/Ancient Civilizations"), None, "Greek & Roman sounds"),
    ("Cultural Styles/Ancient Civilizations/Indian Subcontinent", Some("Cultural Styles/Ancient Civilizations"), None, "Indian subcontinent sounds"),
    ("Cultural Styles/Ancient Civilizations/Norse Culture", Some("Cultural Styles/Ancient Civilizations"), None, "Norse culture sounds"),
    ("Cultural Styles/Ancient Civilizations/South American", Some("Cultural Styles/Ancient Civilizations"), None, "South American sounds"),
    
    // === CULTURAL STYLES - FANTASY CULTURES - DWARVEN ===
    ("Cultural Styles/Fantasy Cultures/Dwarven/Deep Dwarf", Some("Cultural Styles/Fantasy Cultures/Dwarven"), None, "Deep dwarf sounds"),
    ("Cultural Styles/Fantasy Cultures/Dwarven/Forge Dwarf", Some("Cultural Styles/Fantasy Cultures/Dwarven"), None, "Forge dwarf sounds"),
    ("Cultural Styles/Fantasy Cultures/Dwarven/Hill Dwarf", Some("Cultural Styles/Fantasy Cultures/Dwarven"), None, "Hill dwarf sounds"),
    ("Cultural Styles/Fantasy Cultures/Dwarven/Mountain Dwarf", Some("Cultural Styles/Fantasy Cultures/Dwarven"), None, "Mountain dwarf sounds"),
    
    // === CULTURAL STYLES - FANTASY CULTURES - ELVISH ===
    ("Cultural Styles/Fantasy Cultures/Elvish/Dark Elf", Some("Cultural Styles/Fantasy Cultures/Elvish"), None, "Dark elf sounds"),
    ("Cultural Styles/Fantasy Cultures/Elvish/High Elf", Some("Cultural Styles/Fantasy Cultures/Elvish"), None, "High elf sounds"),
    ("Cultural Styles/Fantasy Cultures/Elvish/Sea Elf", Some("Cultural Styles/Fantasy Cultures/Elvish"), None, "Sea elf sounds"),
    ("Cultural Styles/Fantasy Cultures/Elvish/Wood Elf", Some("Cultural Styles/Fantasy Cultures/Elvish"), None, "Wood elf sounds"),
    
    // === CULTURAL STYLES - FANTASY CULTURES - ORCISH ===
    ("Cultural Styles/Fantasy Cultures/Orcish/Civilized Orc", Some("Cultural Styles/Fantasy Cultures/Orcish"), None, "Civilized orc sounds"),
    ("Cultural Styles/Fantasy Cultures/Orcish/Half-Orc", Some("Cultural Styles/Fantasy Cultures/Orcish"), None, "Half-orc sounds"),
    ("Cultural Styles/Fantasy Cultures/Orcish/Tribal Orc", Some("Cultural Styles/Fantasy Cultures/Orcish"), None, "Tribal orc sounds"),
    
    // === CULTURAL STYLES - HISTORICAL PERIODS - ANCIENT ===
    ("Cultural Styles/Historical Periods/Ancient/Egyptian", Some("Cultural Styles/Historical Periods/Ancient"), None, "Ancient Egyptian sounds"),
    ("Cultural Styles/Historical Periods/Ancient/Greek", Some("Cultural Styles/Historical Periods/Ancient"), None, "Ancient Greek sounds"),
    ("Cultural Styles/Historical Periods/Ancient/Mesopotamian", Some("Cultural Styles/Historical Periods/Ancient"), None, "Ancient Mesopotamian sounds"),
    ("Cultural Styles/Historical Periods/Ancient/Roman", Some("Cultural Styles/Historical Periods/Ancient"), None, "Ancient Roman sounds"),
    
    // === CULTURAL STYLES - HISTORICAL PERIODS - MEDIEVAL ===
    ("Cultural Styles/Historical Periods/Medieval/Byzantine", Some("Cultural Styles/Historical Periods/Medieval"), None, "Byzantine sounds"),
    ("Cultural Styles/Historical Periods/Medieval/European", Some("Cultural Styles/Historical Periods/Medieval"), None, "Medieval European sounds"),
    ("Cultural Styles/Historical Periods/Medieval/Islamic Golden Age", Some("Cultural Styles/Historical Periods/Medieval"), None, "Islamic Golden Age sounds"),
    ("Cultural Styles/Historical Periods/Medieval/Mongol Empire", Some("Cultural Styles/Historical Periods/Medieval"), None, "Mongol Empire sounds"),
    
    // === CULTURAL STYLES - HISTORICAL PERIODS - RENAISSANCE ===
    ("Cultural Styles/Historical Periods/Renaissance/Elizabethan", Some("Cultural Styles/Historical Periods/Renaissance"), None, "Elizabethan sounds"),
    ("Cultural Styles/Historical Periods/Renaissance/Italian Renaissance", Some("Cultural Styles/Historical Periods/Renaissance"), None, "Italian Renaissance sounds"),
    ("Cultural Styles/Historical Periods/Renaissance/Northern Renaissance", Some("Cultural Styles/Historical Periods/Renaissance"), None, "Northern Renaissance sounds"),
    ("Cultural Styles/Historical Periods/Renaissance/Spanish Golden Age", Some("Cultural Styles/Historical Periods/Renaissance"), None, "Spanish Golden Age sounds"),
    
    // === CULTURAL STYLES - MEDIEVAL & RENAISSANCE ===
    ("Cultural Styles/Medieval & Renaissance/Celtic Influences", Some("Cultural Styles/Medieval & Renaissance"), None, "Celtic influences sounds"),
    ("Cultural Styles/Medieval & Renaissance/Eastern Europe", Some("Cultural Styles/Medieval & Renaissance"), None, "Eastern Europe sounds"),
    ("Cultural Styles/Medieval & Renaissance/Medieval Courts", Some("Cultural Styles/Medieval & Renaissance"), None, "Medieval court sounds"),
    ("Cultural Styles/Medieval & Renaissance/Medieval Europe", Some("Cultural Styles/Medieval & Renaissance"), None, "Medieval Europe sounds"),
    ("Cultural Styles/Medieval & Renaissance/Renaissance Italy", Some("Cultural Styles/Medieval & Renaissance"), None, "Renaissance Italy sounds"),
    
    // === CULTURAL STYLES - MODERN ERA ===
    ("Cultural Styles/Modern Era/Cinema", Some("Cultural Styles/Modern Era"), None, "Cinema sounds"),
    ("Cultural Styles/Modern Era/Contemporary", Some("Cultural Styles/Modern Era"), None, "Contemporary sounds"),
    ("Cultural Styles/Modern Era/Film Noir", Some("Cultural Styles/Modern Era"), None, "Film noir sounds"),
    ("Cultural Styles/Modern Era/Industrial Age", Some("Cultural Styles/Modern Era"), None, "Industrial age sounds"),
    ("Cultural Styles/Modern Era/Jazz Age", Some("Cultural Styles/Modern Era"), None, "Jazz age sounds"),
    ("Cultural Styles/Modern Era/Romantic Period", Some("Cultural Styles/Modern Era"), None, "Romantic period sounds"),
    ("Cultural Styles/Modern Era/Urban Culture", Some("Cultural Styles/Modern Era"), None, "Urban culture sounds"),
    
    // === CULTURAL STYLES - REGIONAL CULTURES - AFRICAN ===
    ("Cultural Styles/Regional Cultures/African/Egyptian", Some("Cultural Styles/Regional Cultures/African"), None, "Egyptian regional culture sounds"),
    ("Cultural Styles/Regional Cultures/African/Ethiopian", Some("Cultural Styles/Regional Cultures/African"), None, "Ethiopian regional culture sounds"),
    ("Cultural Styles/Regional Cultures/African/Nubian", Some("Cultural Styles/Regional Cultures/African"), None, "Nubian regional culture sounds"),
    ("Cultural Styles/Regional Cultures/African/West African", Some("Cultural Styles/Regional Cultures/African"), None, "West African regional culture sounds"),
    
    // === CULTURAL STYLES - REGIONAL CULTURES - AMERICAN ===
    ("Cultural Styles/Regional Cultures/American/Aztec", Some("Cultural Styles/Regional Cultures/American"), None, "Aztec regional culture sounds"),
    ("Cultural Styles/Regional Cultures/American/Incan", Some("Cultural Styles/Regional Cultures/American"), None, "Incan regional culture sounds"),
    ("Cultural Styles/Regional Cultures/American/Mayan", Some("Cultural Styles/Regional Cultures/American"), None, "Mayan regional culture sounds"),
    ("Cultural Styles/Regional Cultures/American/Native American", Some("Cultural Styles/Regional Cultures/American"), None, "Native American regional culture sounds"),
    
    // === CULTURAL STYLES - REGIONAL CULTURES - ASIAN ===
    ("Cultural Styles/Regional Cultures/Asian/Chinese", Some("Cultural Styles/Regional Cultures/Asian"), None, "Chinese regional culture sounds"),
    ("Cultural Styles/Regional Cultures/Asian/Indian", Some("Cultural Styles/Regional Cultures/Asian"), None, "Indian regional culture sounds"),
    ("Cultural Styles/Regional Cultures/Asian/Japanese", Some("Cultural Styles/Regional Cultures/Asian"), None, "Japanese regional culture sounds"),
    ("Cultural Styles/Regional Cultures/Asian/Korean", Some("Cultural Styles/Regional Cultures/Asian"), None, "Korean regional culture sounds"),
    ("Cultural Styles/Regional Cultures/Asian/Persian", Some("Cultural Styles/Regional Cultures/Asian"), None, "Persian regional culture sounds"),
    
    // === CULTURAL STYLES - REGIONAL CULTURES - EUROPEAN ===
    ("Cultural Styles/Regional Cultures/European/Celtic", Some("Cultural Styles/Regional Cultures/European"), None, "Celtic regional culture sounds"),
    ("Cultural Styles/Regional Cultures/European/Germanic", Some("Cultural Styles/Regional Cultures/European"), None, "Germanic regional culture sounds"),
    ("Cultural Styles/Regional Cultures/European/Nordic/Norse", Some("Cultural Styles/Regional Cultures/European/Nordic"), None, "Norse regional culture sounds"),
    ("Cultural Styles/Regional Cultures/European/Nordic/Scandinavian", Some("Cultural Styles/Regional Cultures/European/Nordic"), None, "Scandinavian regional culture sounds"),
    ("Cultural Styles/Regional Cultures/European/Slavic", Some("Cultural Styles/Regional Cultures/European"), None, "Slavic regional culture sounds"),
    
    // === CULTURAL STYLES - STEAMPUNK & VICTORIAN ===
    ("Cultural Styles/Steampunk & Victorian/Classic Steampunk", Some("Cultural Styles/Steampunk & Victorian"), None, "Classic steampunk sounds"),
    ("Cultural Styles/Steampunk & Victorian/Dieselpunk", Some("Cultural Styles/Steampunk & Victorian"), None, "Dieselpunk sounds"),
    ("Cultural Styles/Steampunk & Victorian/Steampunk", Some("Cultural Styles/Steampunk & Victorian"), None, "Steampunk sounds"),
    ("Cultural Styles/Steampunk & Victorian/Victorian England", Some("Cultural Styles/Steampunk & Victorian"), None, "Victorian England sounds"),
    
    // === CULTURAL STYLES - WILD WEST ===
    ("Cultural Styles/Wild West/Frontier Towns", Some("Cultural Styles/Wild West"), None, "Frontier town sounds"),
    ("Cultural Styles/Wild West/Weird West", Some("Cultural Styles/Wild West"), None, "Weird west sounds"),
    
    // === ENVIRONMENTS - CAVES ===
    ("Environments/Caves/Deep Caverns", Some("Environments/Caves"), None, "Deep cavern sounds"),
    
    // === ENVIRONMENTS - DUNGEONS ===
    ("Environments/Dungeons/Chambers", Some("Environments/Dungeons"), None, "Dungeon chamber sounds"),
    ("Environments/Dungeons/Deep Underground", Some("Environments/Dungeons"), None, "Deep underground dungeon sounds"),
    ("Environments/Dungeons/Mines/Abandoned Mine", Some("Environments/Dungeons/Mines"), None, "Abandoned mine sounds"),
    ("Environments/Dungeons/Secret Passages", Some("Environments/Dungeons"), None, "Secret passage sounds"),
    ("Environments/Dungeons/Sewers", Some("Environments/Dungeons"), None, "Sewer dungeon sounds"),
    
    // === ENVIRONMENTS - MOUNTAINS ===
    ("Environments/Mountains/High Peaks", Some("Environments/Mountains"), None, "High mountain peak sounds"),
    
    // === ENVIRONMENTS - MYSTICAL ===
    ("Environments/Mystical/Astral Plane", Some("Environments/Mystical"), None, "Astral plane sounds"),
    ("Environments/Mystical/Hell", Some("Environments/Mystical"), None, "Hell environment sounds"),
    
    // === ENVIRONMENTS - NATURAL - ARCTIC ===
    ("Environments/Natural/Arctic/Frozen Wastes", Some("Environments/Natural/Arctic"), None, "Frozen wasteland sounds"),
    
    // === ENVIRONMENTS - NATURAL - DESERT ===
    ("Environments/Natural/Desert/Arid Wastes", Some("Environments/Natural/Desert"), None, "Arid desert wasteland sounds"),
    ("Environments/Natural/Desert/Rocky Canyons", Some("Environments/Natural/Desert"), None, "Rocky canyon sounds"),
    
    // === ENVIRONMENTS - NATURAL - FARMLAND ===
    ("Environments/Natural/Farmland/Crop Fields", Some("Environments/Natural/Farmland"), None, "Crop field sounds"),
    ("Environments/Natural/Farmland/Rural Areas", Some("Environments/Natural/Farmland"), None, "Rural farmland sounds"),
    
    // === ENVIRONMENTS - NATURAL - GARDENS ===
    ("Environments/Natural/Gardens/Parks", Some("Environments/Natural/Gardens"), None, "Park garden sounds"),
    
    // === ENVIRONMENTS - NATURAL - GRASSLANDS ===
    ("Environments/Natural/Grasslands/Open Plains", Some("Environments/Natural/Grasslands"), None, "Open plains sounds"),
    ("Environments/Natural/Grasslands/Rolling Hills", Some("Environments/Natural/Grasslands"), None, "Rolling hills sounds"),
    ("Environments/Natural/Grasslands/Steppes", Some("Environments/Natural/Grasslands"), None, "Steppe grasslands sounds"),
    
    // === ENVIRONMENTS - NATURAL - WATER ===
    ("Environments/Natural/Water/Bogs", Some("Environments/Natural/Water"), None, "Bog water sounds"),
    ("Environments/Natural/Water/Coastal", Some("Environments/Natural/Water"), None, "Coastal water sounds"),
    ("Environments/Natural/Water/Lakes", Some("Environments/Natural/Water"), None, "Lake water sounds"),
    ("Environments/Natural/Water/Marshlands", Some("Environments/Natural/Water"), None, "Marshland water sounds"),
    ("Environments/Natural/Water/Open Ocean", Some("Environments/Natural/Water"), None, "Open ocean sounds"),
    
    // === ENVIRONMENTS - NATURAL - WEATHER ===
    ("Environments/Natural/Weather/Clear Night", Some("Environments/Natural/Weather"), None, "Clear night weather sounds"),
    ("Environments/Natural/Weather/Misty", Some("Environments/Natural/Weather"), None, "Misty weather sounds"),
    ("Environments/Natural/Weather/Rainy", Some("Environments/Natural/Weather"), None, "Rainy weather sounds"),
    ("Environments/Natural/Weather/Stormy", Some("Environments/Natural/Weather"), None, "Stormy weather sounds"),
    
    // === ENVIRONMENTS - SCI-FI ===
    ("Environments/Sci-Fi/Spaceship Interior", Some("Environments/Sci-Fi"), None, "Spaceship interior sounds"),
    
    // === ENVIRONMENTS - URBAN - BUILDINGS ===
    ("Environments/Urban/Buildings/Abandoned", Some("Environments/Urban/Buildings"), None, "Abandoned building sounds"),
    ("Environments/Urban/Buildings/Asylum", Some("Environments/Urban/Buildings"), None, "Asylum building sounds"),
    ("Environments/Urban/Buildings/Castles", Some("Environments/Urban/Buildings"), None, "Castle building sounds"),
    ("Environments/Urban/Buildings/Cemetery", Some("Environments/Urban/Buildings"), None, "Cemetery sounds"),
    ("Environments/Urban/Buildings/Control Room", Some("Environments/Urban/Buildings"), None, "Control room sounds"),
    ("Environments/Urban/Buildings/Corporate", Some("Environments/Urban/Buildings"), None, "Corporate building sounds"),
    ("Environments/Urban/Buildings/Cultural", Some("Environments/Urban/Buildings"), None, "Cultural building sounds"),
    ("Environments/Urban/Buildings/Factories", Some("Environments/Urban/Buildings"), None, "Factory building sounds"),
    ("Environments/Urban/Buildings/Government", Some("Environments/Urban/Buildings"), None, "Government building sounds"),
    ("Environments/Urban/Buildings/Haunted", Some("Environments/Urban/Buildings"), None, "Haunted building sounds"),
    ("Environments/Urban/Buildings/Hospital", Some("Environments/Urban/Buildings"), None, "Hospital building sounds"),
    ("Environments/Urban/Buildings/Industrial", Some("Environments/Urban/Buildings"), None, "Industrial building sounds"),
    ("Environments/Urban/Buildings/Inns", Some("Environments/Urban/Buildings"), None, "Inn building sounds"),
    ("Environments/Urban/Buildings/Laboratory", Some("Environments/Urban/Buildings"), None, "Laboratory building sounds"),
    ("Environments/Urban/Buildings/Office", Some("Environments/Urban/Buildings"), None, "Office building sounds"),
    ("Environments/Urban/Buildings/Recreational", Some("Environments/Urban/Buildings"), None, "Recreational building sounds"),
    ("Environments/Urban/Buildings/Residential", Some("Environments/Urban/Buildings"), None, "Residential building sounds"),
    ("Environments/Urban/Buildings/Royal", Some("Environments/Urban/Buildings"), None, "Royal building sounds"),
    ("Environments/Urban/Buildings/Storage", Some("Environments/Urban/Buildings"), None, "Storage building sounds"),
    ("Environments/Urban/Buildings/Temple", Some("Environments/Urban/Buildings"), None, "Temple building sounds"),
    ("Environments/Urban/Buildings/Tower", Some("Environments/Urban/Buildings"), None, "Tower building sounds"),
    ("Environments/Urban/Buildings/Warehouse", Some("Environments/Urban/Buildings"), None, "Warehouse building sounds"),
    
    // === ENVIRONMENTS - URBAN - CITIES ===
    ("Environments/Urban/Cities/Ancient Cities", Some("Environments/Urban/Cities"), None, "Ancient city sounds"),
    ("Environments/Urban/Cities/Back Alleys", Some("Environments/Urban/Cities"), None, "Back alley sounds"),
    ("Environments/Urban/Cities/Great Bridge", Some("Environments/Urban/Cities"), None, "Great bridge sounds"),
    ("Environments/Urban/Cities/Streets", Some("Environments/Urban/Cities"), None, "City street sounds"),
    ("Environments/Urban/Cities/Suburbs", Some("Environments/Urban/Cities"), None, "Suburban sounds"),
    
    // === ENVIRONMENTS - URBAN - FORTIFICATIONS ===
    ("Environments/Urban/Fortifications/Stone Keep", Some("Environments/Urban/Fortifications"), None, "Stone keep sounds"),
    
    // === ENVIRONMENTS - VEHICLES ===
    ("Environments/Vehicles/Ships", Some("Environments/Vehicles"), None, "Ship vehicle sounds"),
    
    // === FANTASY GENRES - DARK FANTASY - COSMIC HORROR ===
    ("Fantasy Genres/Dark Fantasy/Cosmic Horror/Cultist Ritual", Some("Fantasy Genres/Dark Fantasy/Cosmic Horror"), None, "Cultist ritual sounds"),
    ("Fantasy Genres/Dark Fantasy/Cosmic Horror/Eldritch Awakening", Some("Fantasy Genres/Dark Fantasy/Cosmic Horror"), None, "Eldritch awakening sounds"),
    ("Fantasy Genres/Dark Fantasy/Cosmic Horror/Forbidden Knowledge", Some("Fantasy Genres/Dark Fantasy/Cosmic Horror"), None, "Forbidden knowledge sounds"),
    ("Fantasy Genres/Dark Fantasy/Cosmic Horror/Void Whispers", Some("Fantasy Genres/Dark Fantasy/Cosmic Horror"), None, "Void whispers sounds"),
    
    // === FANTASY GENRES - DARK FANTASY - GOTHIC HORROR ===
    ("Fantasy Genres/Dark Fantasy/Gothic Horror/Cursed Forest", Some("Fantasy Genres/Dark Fantasy/Gothic Horror"), None, "Cursed forest sounds"),
    ("Fantasy Genres/Dark Fantasy/Gothic Horror/Haunted Manor", Some("Fantasy Genres/Dark Fantasy/Gothic Horror"), None, "Haunted manor sounds"),
    ("Fantasy Genres/Dark Fantasy/Gothic Horror/Necromancer", Some("Fantasy Genres/Dark Fantasy/Gothic Horror"), None, "Necromancer sounds"),
    ("Fantasy Genres/Dark Fantasy/Gothic Horror/Vampire Castle", Some("Fantasy Genres/Dark Fantasy/Gothic Horror"), None, "Vampire castle sounds"),
    
    // === FANTASY GENRES - FAIRY TALE FANTASY - CLASSIC TALES ===
    ("Fantasy Genres/Fairy Tale Fantasy/Classic Tales/Enchanted Forest", Some("Fantasy Genres/Fairy Tale Fantasy/Classic Tales"), None, "Enchanted forest sounds"),
    ("Fantasy Genres/Fairy Tale Fantasy/Classic Tales/Fairy Godmother", Some("Fantasy Genres/Fairy Tale Fantasy/Classic Tales"), None, "Fairy godmother sounds"),
    ("Fantasy Genres/Fairy Tale Fantasy/Classic Tales/Magic Castle", Some("Fantasy Genres/Fairy Tale Fantasy/Classic Tales"), None, "Magic castle sounds"),
    ("Fantasy Genres/Fairy Tale Fantasy/Classic Tales/Wicked Witch", Some("Fantasy Genres/Fairy Tale Fantasy/Classic Tales"), None, "Wicked witch sounds"),
    ("Fantasy Genres/Fairy Tale Fantasy/Classic Tales/Winter Wonderland", Some("Fantasy Genres/Fairy Tale Fantasy/Classic Tales"), None, "Winter wonderland sounds"),
    
    // === FANTASY GENRES - FAIRY TALE FANTASY - DARK TALES ===
    ("Fantasy Genres/Fairy Tale Fantasy/Dark Tales/Grimm Brothers", Some("Fantasy Genres/Fairy Tale Fantasy/Dark Tales"), None, "Grimm Brothers sounds"),
    ("Fantasy Genres/Fairy Tale Fantasy/Dark Tales/Lost Children", Some("Fantasy Genres/Fairy Tale Fantasy/Dark Tales"), None, "Lost children sounds"),
    ("Fantasy Genres/Fairy Tale Fantasy/Dark Tales/Nightmare Tale", Some("Fantasy Genres/Fairy Tale Fantasy/Dark Tales"), None, "Nightmare tale sounds"),
    ("Fantasy Genres/Fairy Tale Fantasy/Dark Tales/Twisted Fairy Tale", Some("Fantasy Genres/Fairy Tale Fantasy/Dark Tales"), None, "Twisted fairy tale sounds"),
    
    // === FANTASY GENRES - HIGH FANTASY - EPIC FANTASY ===
    ("Fantasy Genres/High Fantasy/Epic Fantasy/Ancient Prophecy", Some("Fantasy Genres/High Fantasy/Epic Fantasy"), None, "Ancient prophecy sounds"),
    ("Fantasy Genres/High Fantasy/Epic Fantasy/Divine Intervention", Some("Fantasy Genres/High Fantasy/Epic Fantasy"), None, "Divine intervention sounds"),
    ("Fantasy Genres/High Fantasy/Epic Fantasy/Dragon Battle", Some("Fantasy Genres/High Fantasy/Epic Fantasy"), None, "Dragon battle sounds"),
    ("Fantasy Genres/High Fantasy/Epic Fantasy/Heroic Orchestral", Some("Fantasy Genres/High Fantasy/Epic Fantasy"), None, "Heroic orchestral sounds"),
    ("Fantasy Genres/High Fantasy/Epic Fantasy/Mythic Choir", Some("Fantasy Genres/High Fantasy/Epic Fantasy"), None, "Mythic choir sounds"),
    
    // === FANTASY GENRES - HIGH FANTASY - SWORD & SORCERY ===
    ("Fantasy Genres/High Fantasy/Sword & Sorcery/Barbarian", Some("Fantasy Genres/High Fantasy/Sword & Sorcery"), None, "Barbarian sounds"),
    ("Fantasy Genres/High Fantasy/Sword & Sorcery/Beast Master", Some("Fantasy Genres/High Fantasy/Sword & Sorcery"), None, "Beast master sounds"),
    ("Fantasy Genres/High Fantasy/Sword & Sorcery/Dungeon Crawl", Some("Fantasy Genres/High Fantasy/Sword & Sorcery"), None, "Dungeon crawl sounds"),
    ("Fantasy Genres/High Fantasy/Sword & Sorcery/Treasure Hunt", Some("Fantasy Genres/High Fantasy/Sword & Sorcery"), None, "Treasure hunt sounds"),
    ("Fantasy Genres/High Fantasy/Sword & Sorcery/Wizard Duel", Some("Fantasy Genres/High Fantasy/Sword & Sorcery"), None, "Wizard duel sounds"),
    
    // === FANTASY GENRES - MEDIEVAL FANTASY ===
    ("Fantasy Genres/Medieval Fantasy/Chivalric Romance/Noble Quest", Some("Fantasy Genres/Medieval Fantasy/Chivalric Romance"), None, "Noble quest sounds"),
    ("Fantasy Genres/Medieval Fantasy/Historic Villages/Caravan Travel", Some("Fantasy Genres/Medieval Fantasy/Historic Villages"), None, "Caravan travel sounds"),
    
    // === FANTASY GENRES - SWASHBUCKLING FANTASY ===
    ("Fantasy Genres/Swashbuckling Fantasy/High Seas Adventure/Merchant Ship", Some("Fantasy Genres/Swashbuckling Fantasy/High Seas Adventure"), None, "Merchant ship sounds"),
    ("Fantasy Genres/Swashbuckling Fantasy/High Seas Adventure/Naval Battle", Some("Fantasy Genres/Swashbuckling Fantasy/High Seas Adventure"), None, "Naval battle sounds"),
    ("Fantasy Genres/Swashbuckling Fantasy/High Seas Adventure/Pirate Ship", Some("Fantasy Genres/Swashbuckling Fantasy/High Seas Adventure"), None, "Pirate ship sounds"),
    
    // === FANTASY GENRES - URBAN FANTASY - MODERN MAGIC ===
    ("Fantasy Genres/Urban Fantasy/Modern Magic/City Wizard", Some("Fantasy Genres/Urban Fantasy/Modern Magic"), None, "City wizard sounds"),
    ("Fantasy Genres/Urban Fantasy/Modern Magic/Corporate Demon", Some("Fantasy Genres/Urban Fantasy/Modern Magic"), None, "Corporate demon sounds"),
    ("Fantasy Genres/Urban Fantasy/Modern Magic/Neon Sorcery", Some("Fantasy Genres/Urban Fantasy/Modern Magic"), None, "Neon sorcery sounds"),
    ("Fantasy Genres/Urban Fantasy/Modern Magic/Subway Dragon", Some("Fantasy Genres/Urban Fantasy/Modern Magic"), None, "Subway dragon sounds"),
    
    // === FANTASY GENRES - URBAN FANTASY - PARANORMAL INVESTIGATION ===
    ("Fantasy Genres/Urban Fantasy/Paranormal Investigation/Ghost Detective", Some("Fantasy Genres/Urban Fantasy/Paranormal Investigation"), None, "Ghost detective sounds"),
    ("Fantasy Genres/Urban Fantasy/Paranormal Investigation/Monster Hunter", Some("Fantasy Genres/Urban Fantasy/Paranormal Investigation"), None, "Monster hunter sounds"),
    ("Fantasy Genres/Urban Fantasy/Paranormal Investigation/Occult Police", Some("Fantasy Genres/Urban Fantasy/Paranormal Investigation"), None, "Occult police sounds"),
    ("Fantasy Genres/Urban Fantasy/Paranormal Investigation/Supernatural Mystery", Some("Fantasy Genres/Urban Fantasy/Paranormal Investigation"), None, "Supernatural mystery sounds"),
    
    // === FANTASY GENRES - URBAN FANTASY - STEAMPUNK ===
    ("Fantasy Genres/Urban Fantasy/Steampunk/Airship", Some("Fantasy Genres/Urban Fantasy/Steampunk"), None, "Steampunk airship sounds"),
    ("Fantasy Genres/Urban Fantasy/Steampunk/Clockwork", Some("Fantasy Genres/Urban Fantasy/Steampunk"), None, "Steampunk clockwork sounds"),
    
    // === MAGIC & POWERS - ELEMENTAL MAGIC ===
    ("Magic & Powers/Elemental Magic/Air/Wind", Some("Magic & Powers/Elemental Magic/Air"), None, "Wind elemental magic sounds"),
    
    // === MAGIC & POWERS - SCHOOLS OF MAGIC - ABJURATION ===
    ("Magic & Powers/Schools of Magic/Abjuration/Protection", Some("Magic & Powers/Schools of Magic/Abjuration"), None, "Protection abjuration magic sounds"),
    
    // === MAGIC & POWERS - SCHOOLS OF MAGIC - DIVINATION ===
    ("Magic & Powers/Schools of Magic/Divination/Mind Reading", Some("Magic & Powers/Schools of Magic/Divination"), None, "Mind reading divination magic sounds"),
    
    // === MAGIC & POWERS - SCHOOLS OF MAGIC - ENCHANTMENT ===
    ("Magic & Powers/Schools of Magic/Enchantment/Charm", Some("Magic & Powers/Schools of Magic/Enchantment"), None, "Charm enchantment magic sounds"),
    ("Magic & Powers/Schools of Magic/Enchantment/Mind Control", Some("Magic & Powers/Schools of Magic/Enchantment"), None, "Mind control enchantment magic sounds"),
    ("Magic & Powers/Schools of Magic/Enchantment/Sleep", Some("Magic & Powers/Schools of Magic/Enchantment"), None, "Sleep enchantment magic sounds"),
    
    // === MAGIC & POWERS - SCHOOLS OF MAGIC - EVOCATION ===
    ("Magic & Powers/Schools of Magic/Evocation/Shadow", Some("Magic & Powers/Schools of Magic/Evocation"), None, "Shadow evocation magic sounds"),
    
    // === MAGIC & POWERS - SCHOOLS OF MAGIC - ILLUSION ===
    ("Magic & Powers/Schools of Magic/Illusion/Disguise", Some("Magic & Powers/Schools of Magic/Illusion"), None, "Disguise illusion magic sounds"),
    ("Magic & Powers/Schools of Magic/Illusion/Illusion Creation", Some("Magic & Powers/Schools of Magic/Illusion"), None, "Illusion creation magic sounds"),
    
    // === MAGIC & POWERS - SCHOOLS OF MAGIC - NECROMANCY ===
    ("Magic & Powers/Schools of Magic/Necromancy/Soul Magic", Some("Magic & Powers/Schools of Magic/Necromancy"), None, "Soul magic necromancy sounds"),
    ("Magic & Powers/Schools of Magic/Necromancy/Undead Animation", Some("Magic & Powers/Schools of Magic/Necromancy"), None, "Undead animation necromancy sounds"),
    ("Magic & Powers/Schools of Magic/Necromancy/Void", Some("Magic & Powers/Schools of Magic/Necromancy"), None, "Void necromancy magic sounds"),
    
    // === MAGIC & POWERS - SCHOOLS OF MAGIC - TRANSMUTATION ===
    ("Magic & Powers/Schools of Magic/Transmutation/Shape Change", Some("Magic & Powers/Schools of Magic/Transmutation"), None, "Shape change transmutation magic sounds"),
    
    // === MAGIC & POWERS - TECHNOLOGY - ANCIENT ===
    ("Magic & Powers/Technology/Ancient/Stone", Some("Magic & Powers/Technology/Ancient"), None, "Ancient stone technology sounds"),
    ("Magic & Powers/Technology/Ancient/Wood", Some("Magic & Powers/Technology/Ancient"), None, "Ancient wood technology sounds"),
    
    // === MAGIC & POWERS - TECHNOLOGY - MODERN ===
    ("Magic & Powers/Technology/Modern/Communication", Some("Magic & Powers/Technology/Modern"), None, "Modern communication technology sounds"),
    ("Magic & Powers/Technology/Modern/Electrical", Some("Magic & Powers/Technology/Modern"), None, "Modern electrical technology sounds"),
    ("Magic & Powers/Technology/Modern/Electronics", Some("Magic & Powers/Technology/Modern"), None, "Modern electronics technology sounds"),
    ("Magic & Powers/Technology/Modern/Machinery", Some("Magic & Powers/Technology/Modern"), None, "Modern machinery technology sounds"),
    ("Magic & Powers/Technology/Modern/Vehicles", Some("Magic & Powers/Technology/Modern"), None, "Modern vehicle technology sounds"),
    
    // === MAGIC & POWERS - TECHNOLOGY - SCI-FI ===
    ("Magic & Powers/Technology/Sci-Fi/Alien", Some("Magic & Powers/Technology/Sci-Fi"), None, "Alien sci-fi technology sounds"),
    ("Magic & Powers/Technology/Sci-Fi/Anti-Gravity", Some("Magic & Powers/Technology/Sci-Fi"), None, "Anti-gravity sci-fi technology sounds"),
    ("Magic & Powers/Technology/Sci-Fi/Communication", Some("Magic & Powers/Technology/Sci-Fi"), None, "Sci-fi communication technology sounds"),
    ("Magic & Powers/Technology/Sci-Fi/Corporate", Some("Magic & Powers/Technology/Sci-Fi"), None, "Corporate sci-fi technology sounds"),
    ("Magic & Powers/Technology/Sci-Fi/Energy", Some("Magic & Powers/Technology/Sci-Fi"), None, "Energy sci-fi technology sounds"),
    ("Magic & Powers/Technology/Sci-Fi/Mecha", Some("Magic & Powers/Technology/Sci-Fi"), None, "Mecha sci-fi technology sounds"),
    ("Magic & Powers/Technology/Sci-Fi/Nanotechnology", Some("Magic & Powers/Technology/Sci-Fi"), None, "Nanotechnology sci-fi sounds"),
    ("Magic & Powers/Technology/Sci-Fi/Quantum", Some("Magic & Powers/Technology/Sci-Fi"), None, "Quantum sci-fi technology sounds"),
    
    // === MAGIC & POWERS - TECHNOLOGY - STEAMPUNK ===
    ("Magic & Powers/Technology/Steampunk/Clockwork", Some("Magic & Powers/Technology/Steampunk"), None, "Steampunk clockwork technology sounds"),
    ("Magic & Powers/Technology/Steampunk/Steam", Some("Magic & Powers/Technology/Steampunk"), None, "Steampunk steam technology sounds"),
    
    // === MENTAL STATES - ALTERED STATES - DREAMS ===
    ("Mental States/Altered States/Dreams/Lucid Dreaming", Some("Mental States/Altered States/Dreams"), None, "Lucid dreaming sounds"),
    ("Mental States/Altered States/Dreams/Nightmares", Some("Mental States/Altered States/Dreams"), None, "Nightmare sounds"),
    ("Mental States/Altered States/Dreams/Prophetic Visions", Some("Mental States/Altered States/Dreams"), None, "Prophetic vision sounds"),
    ("Mental States/Altered States/Dreams/Recurring Dreams", Some("Mental States/Altered States/Dreams"), None, "Recurring dream sounds"),
    ("Mental States/Altered States/Dreams/Surreal Dreams", Some("Mental States/Altered States/Dreams"), None, "Surreal dream sounds"),
    
    // === MENTAL STATES - ALTERED STATES - TRANCE ===
    ("Mental States/Altered States/Trance/Dance Trance", Some("Mental States/Altered States/Trance"), None, "Dance trance sounds"),
    ("Mental States/Altered States/Trance/Ecstatic State", Some("Mental States/Altered States/Trance"), None, "Ecstatic state sounds"),
    ("Mental States/Altered States/Trance/Hypnotic State", Some("Mental States/Altered States/Trance"), None, "Hypnotic state sounds"),
    ("Mental States/Altered States/Trance/Ritual Trance", Some("Mental States/Altered States/Trance"), None, "Ritual trance sounds"),
    ("Mental States/Altered States/Trance/Shamanic Journey", Some("Mental States/Altered States/Trance"), None, "Shamanic journey sounds"),
    
    // === MENTAL STATES - COGNITIVE STATES - CONFUSED ===
    ("Mental States/Cognitive States/Confused/Decision Paralysis", Some("Mental States/Cognitive States/Confused"), None, "Decision paralysis sounds"),
    ("Mental States/Cognitive States/Confused/Information Overload", Some("Mental States/Cognitive States/Confused"), None, "Information overload sounds"),
    ("Mental States/Cognitive States/Confused/Memory Lapse", Some("Mental States/Cognitive States/Confused"), None, "Memory lapse sounds"),
    ("Mental States/Cognitive States/Confused/Mental Fog", Some("Mental States/Cognitive States/Confused"), None, "Mental fog sounds"),
    ("Mental States/Cognitive States/Confused/Uncertainty", Some("Mental States/Cognitive States/Confused"), None, "Uncertainty sounds"),
    
    // === MENTAL STATES - COGNITIVE STATES - FOCUSED ===
    ("Mental States/Cognitive States/Focused/Creative Flow", Some("Mental States/Cognitive States/Focused"), None, "Creative flow sounds"),
    ("Mental States/Cognitive States/Focused/Deep Concentration", Some("Mental States/Cognitive States/Focused"), None, "Deep concentration sounds"),
    ("Mental States/Cognitive States/Focused/Learning", Some("Mental States/Cognitive States/Focused"), None, "Learning sounds"),
    ("Mental States/Cognitive States/Focused/Meditation", Some("Mental States/Cognitive States/Focused"), None, "Meditation sounds"),
    ("Mental States/Cognitive States/Focused/Problem Solving", Some("Mental States/Cognitive States/Focused"), None, "Problem solving sounds"),
    
    // === MENTAL STATES - CONSCIOUSNESS LEVELS - AWAKENING ===
    ("Mental States/Consciousness Levels/Awakening/Deep Understanding", Some("Mental States/Consciousness Levels/Awakening"), None, "Deep understanding sounds"),
    ("Mental States/Consciousness Levels/Awakening/Higher Awareness", Some("Mental States/Consciousness Levels/Awakening"), None, "Higher awareness sounds"),
    ("Mental States/Consciousness Levels/Awakening/Mental Clarity", Some("Mental States/Consciousness Levels/Awakening"), None, "Mental clarity sounds"),
    ("Mental States/Consciousness Levels/Awakening/Spiritual Awakening", Some("Mental States/Consciousness Levels/Awakening"), None, "Spiritual awakening sounds"),
    ("Mental States/Consciousness Levels/Awakening/Unity Consciousness", Some("Mental States/Consciousness Levels/Awakening"), None, "Unity consciousness sounds"),
    
    // === MENTAL STATES - CONSCIOUSNESS LEVELS - DIMINISHED ===
    ("Mental States/Consciousness Levels/Diminished/Coma", Some("Mental States/Consciousness Levels/Diminished"), None, "Coma consciousness sounds"),
    ("Mental States/Consciousness Levels/Diminished/Delusion", Some("Mental States/Consciousness Levels/Diminished"), None, "Delusion consciousness sounds"),
    ("Mental States/Consciousness Levels/Diminished/Dissociation", Some("Mental States/Consciousness Levels/Diminished"), None, "Dissociation consciousness sounds"),
    ("Mental States/Consciousness Levels/Diminished/Drowsiness", Some("Mental States/Consciousness Levels/Diminished"), None, "Drowsiness consciousness sounds"),
    ("Mental States/Consciousness Levels/Diminished/Unconsciousness", Some("Mental States/Consciousness Levels/Diminished"), None, "Unconsciousness sounds"),
    
    // === MENTAL STATES - EMOTIONAL STATES - NEGATIVE ===
    ("Mental States/Emotional States/Negative/Anger", Some("Mental States/Emotional States/Negative"), None, "Anger emotional sounds"),
    ("Mental States/Emotional States/Negative/Disgust", Some("Mental States/Emotional States/Negative"), None, "Disgust emotional sounds"),
    ("Mental States/Emotional States/Negative/Fear", Some("Mental States/Emotional States/Negative"), None, "Fear emotional sounds"),
    ("Mental States/Emotional States/Negative/Jealousy", Some("Mental States/Emotional States/Negative"), None, "Jealousy emotional sounds"),
    ("Mental States/Emotional States/Negative/Sadness", Some("Mental States/Emotional States/Negative"), None, "Sadness emotional sounds"),
    
    // === MENTAL STATES - EMOTIONAL STATES - POSITIVE ===
    ("Mental States/Emotional States/Positive/Confidence", Some("Mental States/Emotional States/Positive"), None, "Confidence emotional sounds"),
    ("Mental States/Emotional States/Positive/Hope", Some("Mental States/Emotional States/Positive"), None, "Hope emotional sounds"),
    ("Mental States/Emotional States/Positive/Joy", Some("Mental States/Emotional States/Positive"), None, "Joy emotional sounds"),
    ("Mental States/Emotional States/Positive/Love", Some("Mental States/Emotional States/Positive"), None, "Love emotional sounds"),
    ("Mental States/Emotional States/Positive/Serenity", Some("Mental States/Emotional States/Positive"), None, "Serenity emotional sounds"),
    
    // === MOODS & ATMOSPHERE - ACTION & ENERGY ===
    ("Moods & Atmosphere/Action & Energy/Aggressive", Some("Moods & Atmosphere/Action & Energy"), None, "Aggressive action sounds"),
    ("Moods & Atmosphere/Action & Energy/Brutal", Some("Moods & Atmosphere/Action & Energy"), None, "Brutal action sounds"),
    ("Moods & Atmosphere/Action & Energy/Bustling", Some("Moods & Atmosphere/Action & Energy"), None, "Bustling action sounds"),
    ("Moods & Atmosphere/Action & Energy/Busy", Some("Moods & Atmosphere/Action & Energy"), None, "Busy action sounds"),
    ("Moods & Atmosphere/Action & Energy/Chaotic", Some("Moods & Atmosphere/Action & Energy"), None, "Chaotic action sounds"),
    ("Moods & Atmosphere/Action & Energy/Charged", Some("Moods & Atmosphere/Action & Energy"), None, "Charged action sounds"),
    ("Moods & Atmosphere/Action & Energy/Driving", Some("Moods & Atmosphere/Action & Energy"), None, "Driving action sounds"),
    ("Moods & Atmosphere/Action & Energy/Energetic", Some("Moods & Atmosphere/Action & Energy"), None, "Energetic action sounds"),
    ("Moods & Atmosphere/Action & Energy/Explosive", Some("Moods & Atmosphere/Action & Energy"), None, "Explosive action sounds"),
    ("Moods & Atmosphere/Action & Energy/Frenetic", Some("Moods & Atmosphere/Action & Energy"), None, "Frenetic action sounds"),
    ("Moods & Atmosphere/Action & Energy/Furious", Some("Moods & Atmosphere/Action & Energy"), None, "Furious action sounds"),
    ("Moods & Atmosphere/Action & Energy/Intense", Some("Moods & Atmosphere/Action & Energy"), None, "Intense action sounds"),
    ("Moods & Atmosphere/Action & Energy/Noisy", Some("Moods & Atmosphere/Action & Energy"), None, "Noisy action sounds"),
    ("Moods & Atmosphere/Action & Energy/Percussive", Some("Moods & Atmosphere/Action & Energy"), None, "Percussive action sounds"),
    ("Moods & Atmosphere/Action & Energy/Relentless", Some("Moods & Atmosphere/Action & Energy"), None, "Relentless action sounds"),
    ("Moods & Atmosphere/Action & Energy/Urgent", Some("Moods & Atmosphere/Action & Energy"), None, "Urgent action sounds"),
    ("Moods & Atmosphere/Action & Energy/Violent", Some("Moods & Atmosphere/Action & Energy"), None, "Violent action sounds"),
    
    // === MOODS & ATMOSPHERE - ACTION ===
    ("Moods & Atmosphere/Action/Chaotic & Frenzied", Some("Moods & Atmosphere/Action"), None, "Chaotic and frenzied action atmosphere"),
    ("Moods & Atmosphere/Action/Urgent & Pressing", Some("Moods & Atmosphere/Action"), None, "Urgent and pressing action atmosphere"),
    
    // === MOODS & ATMOSPHERE - ADVENTURE & EXCITEMENT ===
    ("Moods & Atmosphere/Adventure & Excitement/Adventurous", Some("Moods & Atmosphere/Adventure & Excitement"), None, "Adventurous excitement sounds"),
    
    // === MOODS & ATMOSPHERE - AMBIENT & ATMOSPHERIC ===
    ("Moods & Atmosphere/Ambient & Atmospheric/Atmospheric", Some("Moods & Atmosphere/Ambient & Atmospheric"), None, "Atmospheric ambient sounds"),
    
    // === MOODS & ATMOSPHERE - CALM & PEACEFUL ===
    ("Moods & Atmosphere/Calm & Peaceful/Comforting", Some("Moods & Atmosphere/Calm & Peaceful"), None, "Comforting peaceful sounds"),
    ("Moods & Atmosphere/Calm & Peaceful/Neutral", Some("Moods & Atmosphere/Calm & Peaceful"), None, "Neutral peaceful sounds"),
    ("Moods & Atmosphere/Calm & Peaceful/Pastoral", Some("Moods & Atmosphere/Calm & Peaceful"), None, "Pastoral peaceful sounds"),
    ("Moods & Atmosphere/Calm & Peaceful/Quiet", Some("Moods & Atmosphere/Calm & Peaceful"), None, "Quiet peaceful sounds"),
    ("Moods & Atmosphere/Calm & Peaceful/Serene", Some("Moods & Atmosphere/Calm & Peaceful"), None, "Serene peaceful sounds"),
    ("Moods & Atmosphere/Calm & Peaceful/Tranquil", Some("Moods & Atmosphere/Calm & Peaceful"), None, "Tranquil peaceful sounds"),
    
    // === MOODS & ATMOSPHERE - CONTEMPLATIVE ===
    ("Moods & Atmosphere/Contemplative/Austere", Some("Moods & Atmosphere/Contemplative"), None, "Austere contemplative sounds"),
    ("Moods & Atmosphere/Contemplative/Dreamlike", Some("Moods & Atmosphere/Contemplative"), None, "Dreamlike contemplative sounds"),
    ("Moods & Atmosphere/Contemplative/Dreamlike & Surreal", Some("Moods & Atmosphere/Contemplative"), None, "Dreamlike and surreal contemplative sounds"),
    ("Moods & Atmosphere/Contemplative/Dreamy", Some("Moods & Atmosphere/Contemplative"), None, "Dreamy contemplative sounds"),
    ("Moods & Atmosphere/Contemplative/Hazy", Some("Moods & Atmosphere/Contemplative"), None, "Hazy contemplative sounds"),
    ("Moods & Atmosphere/Contemplative/Hypnotic", Some("Moods & Atmosphere/Contemplative"), None, "Hypnotic contemplative sounds"),
    ("Moods & Atmosphere/Contemplative/Lonely", Some("Moods & Atmosphere/Contemplative"), None, "Lonely contemplative sounds"),
    ("Moods & Atmosphere/Contemplative/Nostalgic & Remembering", Some("Moods & Atmosphere/Contemplative"), None, "Nostalgic and remembering contemplative sounds"),
    ("Moods & Atmosphere/Contemplative/Reflective", Some("Moods & Atmosphere/Contemplative"), None, "Reflective contemplative sounds"),
    ("Moods & Atmosphere/Contemplative/Sacred", Some("Moods & Atmosphere/Contemplative"), None, "Sacred contemplative sounds"),
    ("Moods & Atmosphere/Contemplative/Solemn", Some("Moods & Atmosphere/Contemplative"), None, "Solemn contemplative sounds"),
    ("Moods & Atmosphere/Contemplative/Solemn & Ceremonial", Some("Moods & Atmosphere/Contemplative"), None, "Solemn and ceremonial contemplative sounds"),
    ("Moods & Atmosphere/Contemplative/Stoic", Some("Moods & Atmosphere/Contemplative"), None, "Stoic contemplative sounds"),
    ("Moods & Atmosphere/Contemplative/Swell", Some("Moods & Atmosphere/Contemplative"), None, "Swell contemplative sounds"),
    ("Moods & Atmosphere/Contemplative/Thoughtful & Reflective", Some("Moods & Atmosphere/Contemplative"), None, "Thoughtful and reflective contemplative sounds"),
    
    // === MOODS & ATMOSPHERE - DARK & FOREBODING ===
    ("Moods & Atmosphere/Dark & Foreboding/Abhorrent", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Abhorrent dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Abyss", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Abyss dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Bleak", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Bleak dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Body Horror", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Body horror dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Brooding", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Brooding dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Chilling", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Chilling dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Claustrophobic", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Claustrophobic dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Cosmic", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Cosmic dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Creepy", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Creepy dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Dark", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Dark atmosphere sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Demonic", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Demonic dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Desolate", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Desolate dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Dissonant", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Dissonant dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Dread", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Dread dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Eerie", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Eerie dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Eldritch", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Eldritch dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Fatalistic", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Fatalistic dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Fearful", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Fearful dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Foreboding", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Foreboding dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Ghostly", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Ghostly dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Gory", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Gory dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Gothic", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Gothic dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Grim", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Grim dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Gruesome", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Gruesome dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Haunting", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Haunting dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Menacing", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Menacing dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Nihilistic", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Nihilistic dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Ominous", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Ominous dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Oppressive", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Oppressive dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Pain", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Pain dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Painful", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Painful dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Ritual Fear", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Ritual fear dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Scary", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Scary dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Sinister", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Sinister dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Terrifying", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Terrifying dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Terror", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Terror dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Uncanny", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Uncanny dark sounds"),
    ("Moods & Atmosphere/Dark & Foreboding/Unsettling", Some("Moods & Atmosphere/Dark & Foreboding"), None, "Unsettling dark sounds"),
    
    // === MOODS & ATMOSPHERE - DARK ===
    ("Moods & Atmosphere/Dark/Desolate & Abandoned", Some("Moods & Atmosphere/Dark"), None, "Desolate and abandoned dark atmosphere"),
    
    // === MOODS & ATMOSPHERE - GENTLE & ROMANTIC ===
    ("Moods & Atmosphere/Gentle & Romantic/Bright", Some("Moods & Atmosphere/Gentle & Romantic"), None, "Bright gentle sounds"),
    ("Moods & Atmosphere/Gentle & Romantic/Festive", Some("Moods & Atmosphere/Gentle & Romantic"), None, "Festive gentle sounds"),
    ("Moods & Atmosphere/Gentle & Romantic/Lighthearted", Some("Moods & Atmosphere/Gentle & Romantic"), None, "Lighthearted gentle sounds"),
    ("Moods & Atmosphere/Gentle & Romantic/Merry", Some("Moods & Atmosphere/Gentle & Romantic"), None, "Merry gentle sounds"),
    ("Moods & Atmosphere/Gentle & Romantic/Romantic", Some("Moods & Atmosphere/Gentle & Romantic"), None, "Romantic gentle sounds"),
    ("Moods & Atmosphere/Gentle & Romantic/Tender", Some("Moods & Atmosphere/Gentle & Romantic"), None, "Tender gentle sounds"),
    ("Moods & Atmosphere/Gentle & Romantic/Warm", Some("Moods & Atmosphere/Gentle & Romantic"), None, "Warm gentle sounds"),
    
    // === MOODS & ATMOSPHERE - HEROIC & TRIUMPHANT ===
    ("Moods & Atmosphere/Heroic & Triumphant/Confident", Some("Moods & Atmosphere/Heroic & Triumphant"), None, "Confident heroic sounds"),
    ("Moods & Atmosphere/Heroic & Triumphant/Epic", Some("Moods & Atmosphere/Heroic & Triumphant"), None, "Epic heroic sounds"),
    ("Moods & Atmosphere/Heroic & Triumphant/Hopeful", Some("Moods & Atmosphere/Heroic & Triumphant"), None, "Hopeful heroic sounds"),
    ("Moods & Atmosphere/Heroic & Triumphant/Inspiring", Some("Moods & Atmosphere/Heroic & Triumphant"), None, "Inspiring heroic sounds"),
    ("Moods & Atmosphere/Heroic & Triumphant/Noble", Some("Moods & Atmosphere/Heroic & Triumphant"), None, "Noble heroic sounds"),
    ("Moods & Atmosphere/Heroic & Triumphant/Uplifting", Some("Moods & Atmosphere/Heroic & Triumphant"), None, "Uplifting heroic sounds"),
    ("Moods & Atmosphere/Heroic & Triumphant/Victorious", Some("Moods & Atmosphere/Heroic & Triumphant"), None, "Victorious heroic sounds"),
    ("Moods & Atmosphere/Heroic & Triumphant/Victory", Some("Moods & Atmosphere/Heroic & Triumphant"), None, "Victory heroic sounds"),
    
    // === MOODS & ATMOSPHERE - LIGHT & SERENE ===
    ("Moods & Atmosphere/Light & Serene", Some("Moods & Atmosphere"), Some("☀️"), "Light and serene mood sounds"),
    ("Moods & Atmosphere/Light & Serene/Airy", Some("Moods & Atmosphere/Light & Serene"), None, "Airy light sounds"),
    ("Moods & Atmosphere/Light & Serene/Calm", Some("Moods & Atmosphere/Light & Serene"), None, "Calm light sounds"),
    
    // === MOODS & ATMOSPHERE - MELANCHOLY & LOSS ===
    ("Moods & Atmosphere/Melancholy & Loss/Bittersweet", Some("Moods & Atmosphere/Melancholy & Loss"), None, "Bittersweet melancholy sounds"),
    ("Moods & Atmosphere/Melancholy & Loss/Sorrowful", Some("Moods & Atmosphere/Melancholy & Loss"), None, "Sorrowful melancholy sounds"),
    ("Moods & Atmosphere/Melancholy & Loss/Tragic", Some("Moods & Atmosphere/Melancholy & Loss"), None, "Tragic melancholy sounds"),
    
    // === MOODS & ATMOSPHERE - MELANCHOLY & REFLECTION ===
    ("Moods & Atmosphere/Melancholy & Reflection/Nostalgic", Some("Moods & Atmosphere/Melancholy & Reflection"), None, "Nostalgic melancholy sounds"),
    ("Moods & Atmosphere/Melancholy & Reflection/Somber", Some("Moods & Atmosphere/Melancholy & Reflection"), None, "Somber melancholy sounds"),
    
    // === MOODS & ATMOSPHERE - MODERN ===
    ("Moods & Atmosphere/Modern/Digital", Some("Moods & Atmosphere/Modern"), None, "Digital modern sounds"),
    ("Moods & Atmosphere/Modern/Industrial", Some("Moods & Atmosphere/Modern"), None, "Industrial modern sounds"),
    
    // === MOODS & ATMOSPHERE - MYSTERY & WONDER ===
    ("Moods & Atmosphere/Mystery & Wonder/Arcane", Some("Moods & Atmosphere/Mystery & Wonder"), None, "Arcane mystery sounds"),
    ("Moods & Atmosphere/Mystery & Wonder/Ceremonial", Some("Moods & Atmosphere/Mystery & Wonder"), None, "Ceremonial mystery sounds"),
    ("Moods & Atmosphere/Mystery & Wonder/Curious", Some("Moods & Atmosphere/Mystery & Wonder"), None, "Curious mystery sounds"),
    ("Moods & Atmosphere/Mystery & Wonder/Enigmatic", Some("Moods & Atmosphere/Mystery & Wonder"), None, "Enigmatic mystery sounds"),
    ("Moods & Atmosphere/Mystery & Wonder/Ethereal", Some("Moods & Atmosphere/Mystery & Wonder"), None, "Ethereal mystery sounds"),
    ("Moods & Atmosphere/Mystery & Wonder/Liminal", Some("Moods & Atmosphere/Mystery & Wonder"), None, "Liminal mystery sounds"),
    ("Moods & Atmosphere/Mystery & Wonder/Magical", Some("Moods & Atmosphere/Mystery & Wonder"), None, "Magical mystery sounds"),
    ("Moods & Atmosphere/Mystery & Wonder/Mysterious", Some("Moods & Atmosphere/Mystery & Wonder"), None, "Mysterious sounds"),
    ("Moods & Atmosphere/Mystery & Wonder/Mystical", Some("Moods & Atmosphere/Mystery & Wonder"), None, "Mystical mystery sounds"),
    ("Moods & Atmosphere/Mystery & Wonder/Otherworldly", Some("Moods & Atmosphere/Mystery & Wonder"), None, "Otherworldly mystery sounds"),
    ("Moods & Atmosphere/Mystery & Wonder/Ritualistic", Some("Moods & Atmosphere/Mystery & Wonder"), None, "Ritualistic mystery sounds"),
    
    // === MOODS & ATMOSPHERE - MYSTICAL ===
    ("Moods & Atmosphere/Mystical/Arcane & Magical", Some("Moods & Atmosphere/Mystical"), None, "Arcane and magical mystical sounds"),
    ("Moods & Atmosphere/Mystical/Ethereal & Otherworldly", Some("Moods & Atmosphere/Mystical"), None, "Ethereal and otherworldly mystical sounds"),
    ("Moods & Atmosphere/Mystical/Mysterious & Enigmatic", Some("Moods & Atmosphere/Mystical"), None, "Mysterious and enigmatic mystical sounds"),
    ("Moods & Atmosphere/Mystical/Sacred & Divine", Some("Moods & Atmosphere/Mystical"), None, "Sacred and divine mystical sounds"),
    
    // === MOODS & ATMOSPHERE - NATURAL ===
    ("Moods & Atmosphere/Natural/Airy", Some("Moods & Atmosphere/Natural"), None, "Airy natural sounds"),
    ("Moods & Atmosphere/Natural/Bio-Organic", Some("Moods & Atmosphere/Natural"), None, "Bio-organic natural sounds"),
    ("Moods & Atmosphere/Natural/Cold", Some("Moods & Atmosphere/Natural"), None, "Cold natural sounds"),
    ("Moods & Atmosphere/Natural/Dry", Some("Moods & Atmosphere/Natural"), None, "Dry natural sounds"),
    ("Moods & Atmosphere/Natural/Glacial", Some("Moods & Atmosphere/Natural"), None, "Glacial natural sounds"),
    ("Moods & Atmosphere/Natural/Organic", Some("Moods & Atmosphere/Natural"), None, "Organic natural sounds"),
    ("Moods & Atmosphere/Natural/Wet", Some("Moods & Atmosphere/Natural"), None, "Wet natural sounds"),
    ("Moods & Atmosphere/Natural/Windy", Some("Moods & Atmosphere/Natural"), None, "Windy natural sounds"),
    
    // === MOODS & ATMOSPHERE - PLAYFUL & LIGHT ===
    ("Moods & Atmosphere/Playful & Light/Playful", Some("Moods & Atmosphere/Playful & Light"), None, "Playful light sounds"),
    ("Moods & Atmosphere/Playful & Light/Whimsical", Some("Moods & Atmosphere/Playful & Light"), None, "Whimsical light sounds"),
    
    // === MOODS & ATMOSPHERE - POSITIVE ===
    ("Moods & Atmosphere/Positive/Adventurous & Bold", Some("Moods & Atmosphere/Positive"), None, "Adventurous and bold positive sounds"),
    
    // === MOODS & ATMOSPHERE - SAD & MELANCHOLY ===
    ("Moods & Atmosphere/Sad & Melancholy/Melancholic", Some("Moods & Atmosphere/Sad & Melancholy"), None, "Melancholic sad sounds"),
    
    // === MOODS & ATMOSPHERE - SUSPENSE & UNCERTAINTY ===
    ("Moods & Atmosphere/Suspense & Uncertainty/Building", Some("Moods & Atmosphere/Suspense & Uncertainty"), None, "Building suspense sounds"),
    ("Moods & Atmosphere/Suspense & Uncertainty/Calm Before Storm", Some("Moods & Atmosphere/Suspense & Uncertainty"), None, "Calm before storm suspense sounds"),
    ("Moods & Atmosphere/Suspense & Uncertainty/High Stakes", Some("Moods & Atmosphere/Suspense & Uncertainty"), None, "High stakes suspense sounds"),
    ("Moods & Atmosphere/Suspense & Uncertainty/Rising", Some("Moods & Atmosphere/Suspense & Uncertainty"), None, "Rising suspense sounds"),
    ("Moods & Atmosphere/Suspense & Uncertainty/Suspense", Some("Moods & Atmosphere/Suspense & Uncertainty"), None, "Suspense sounds"),
    ("Moods & Atmosphere/Suspense & Uncertainty/Tense", Some("Moods & Atmosphere/Suspense & Uncertainty"), None, "Tense suspense sounds"),
    ("Moods & Atmosphere/Suspense & Uncertainty/Uneasy", Some("Moods & Atmosphere/Suspense & Uncertainty"), None, "Uneasy suspense sounds"),
    ("Moods & Atmosphere/Suspense & Uncertainty/Volatile", Some("Moods & Atmosphere/Suspense & Uncertainty"), None, "Volatile suspense sounds"),
    
    // === MUSICAL INSTRUMENTS - BRASS ===
    ("Musical Instruments/Brass/French Horn", Some("Musical Instruments/Brass"), None, "French horn sounds"),
    ("Musical Instruments/Brass/Horn", Some("Musical Instruments/Brass"), None, "Horn sounds"),
    ("Musical Instruments/Brass/Trumpet", Some("Musical Instruments/Brass"), None, "Trumpet sounds"),
    
    // === MUSICAL INSTRUMENTS - ELECTRONIC ===
    ("Musical Instruments/Electronic/Effects", Some("Musical Instruments/Electronic"), None, "Electronic effects sounds"),
    ("Musical Instruments/Electronic/Synthesizers", Some("Musical Instruments/Electronic"), None, "Synthesizer sounds"),
    ("Musical Instruments/Electronic/Vocals", Some("Musical Instruments/Electronic"), None, "Electronic vocal sounds"),
    
    // === MUSICAL INSTRUMENTS - KEYBOARD - ORGAN ===
    ("Musical Instruments/Keyboard/Organ/Accordion", Some("Musical Instruments/Keyboard/Organ"), None, "Accordion sounds"),
    ("Musical Instruments/Keyboard/Organ/Hammond Organ", Some("Musical Instruments/Keyboard/Organ"), None, "Hammond organ sounds"),
    ("Musical Instruments/Keyboard/Organ/Pipe Organ", Some("Musical Instruments/Keyboard/Organ"), None, "Pipe organ sounds"),
    
    // === MUSICAL INSTRUMENTS - KEYBOARD - PIANO ===
    ("Musical Instruments/Keyboard/Piano/Celesta", Some("Musical Instruments/Keyboard/Piano"), None, "Celesta sounds"),
    ("Musical Instruments/Keyboard/Piano/Grand Piano", Some("Musical Instruments/Keyboard/Piano"), None, "Grand piano sounds"),
    ("Musical Instruments/Keyboard/Piano/Harpsichord", Some("Musical Instruments/Keyboard/Piano"), None, "Harpsichord sounds"),
    ("Musical Instruments/Keyboard/Piano/Upright Piano", Some("Musical Instruments/Keyboard/Piano"), None, "Upright piano sounds"),
    
    // === MUSICAL INSTRUMENTS - ORCHESTRAL ===
    ("Musical Instruments/Orchestral/Strings", Some("Musical Instruments/Orchestral"), None, "Orchestral string sounds"),
    
    // === MUSICAL INSTRUMENTS - PERCUSSION - BELLS & CHIMES ===
    ("Musical Instruments/Percussion/Bells & Chimes/Church Bells", Some("Musical Instruments/Percussion/Bells & Chimes"), None, "Church bell sounds"),
    ("Musical Instruments/Percussion/Bells & Chimes/Hand Bells", Some("Musical Instruments/Percussion/Bells & Chimes"), None, "Hand bell sounds"),
    ("Musical Instruments/Percussion/Bells & Chimes/Sleigh Bells", Some("Musical Instruments/Percussion/Bells & Chimes"), None, "Sleigh bell sounds"),
    ("Musical Instruments/Percussion/Bells & Chimes/Temple Bells", Some("Musical Instruments/Percussion/Bells & Chimes"), None, "Temple bell sounds"),
    ("Musical Instruments/Percussion/Bells & Chimes/Wind Chimes", Some("Musical Instruments/Percussion/Bells & Chimes"), None, "Wind chime sounds"),
    
    // === MUSICAL INSTRUMENTS - PERCUSSION - DRUMS ===
    ("Musical Instruments/Percussion/Drums/Drum Kit", Some("Musical Instruments/Percussion/Drums"), None, "Drum kit sounds"),
    ("Musical Instruments/Percussion/Drums/Electronic", Some("Musical Instruments/Percussion/Drums"), None, "Electronic drum sounds"),
    ("Musical Instruments/Percussion/Drums/Hand Drums", Some("Musical Instruments/Percussion/Drums"), None, "Hand drum sounds"),
    ("Musical Instruments/Percussion/Drums/Orchestral", Some("Musical Instruments/Percussion/Drums"), None, "Orchestral drum sounds"),
    ("Musical Instruments/Percussion/Drums/Timpani", Some("Musical Instruments/Percussion/Drums"), None, "Timpani sounds"),
    ("Musical Instruments/Percussion/Drums/Tribal Drums", Some("Musical Instruments/Percussion/Drums"), None, "Tribal drum sounds"),
    ("Musical Instruments/Percussion/Drums/War Drums", Some("Musical Instruments/Percussion/Drums"), None, "War drum sounds"),
    
    // === MUSICAL INSTRUMENTS - PERCUSSION - OTHER ===
    ("Musical Instruments/Percussion/Impacts", Some("Musical Instruments/Percussion"), None, "Percussion impact sounds"),
    
    // === MUSICAL INSTRUMENTS - PERCUSSION - MALLET ===
    ("Musical Instruments/Percussion/Mallet/Bells", Some("Musical Instruments/Percussion/Mallet"), None, "Mallet bell sounds"),
    ("Musical Instruments/Percussion/Mallet/Chimes", Some("Musical Instruments/Percussion/Mallet"), None, "Mallet chime sounds"),
    ("Musical Instruments/Percussion/Mallet/Gongs", Some("Musical Instruments/Percussion/Mallet"), None, "Mallet gong sounds"),
    ("Musical Instruments/Percussion/Mallet/Marimba", Some("Musical Instruments/Percussion/Mallet"), None, "Marimba sounds"),
    
    // === MUSICAL INSTRUMENTS - PERCUSSION - METALLIC ===
    ("Musical Instruments/Percussion/Metallic/Clavaves", Some("Musical Instruments/Percussion/Metallic"), None, "Clavaves sounds"),
    ("Musical Instruments/Percussion/Metallic/Cymbals", Some("Musical Instruments/Percussion/Metallic"), None, "Cymbal sounds"),
    ("Musical Instruments/Percussion/Metallic/Hi-Hats", Some("Musical Instruments/Percussion/Metallic"), None, "Hi-hat sounds"),
    
    // === MUSICAL INSTRUMENTS - PERCUSSION - OTHER ===
    ("Musical Instruments/Percussion/Shakers", Some("Musical Instruments/Percussion"), None, "Shaker percussion sounds"),
    
    // === MUSICAL INSTRUMENTS - PERCUSSION - WOOD ===
    ("Musical Instruments/Percussion/Wood/Xylophone", Some("Musical Instruments/Percussion/Wood"), None, "Xylophone sounds"),
    
    // === MUSICAL INSTRUMENTS - STRING INSTRUMENTS - BOWED ===
    ("Musical Instruments/String Instruments/Bowed/Cello", Some("Musical Instruments/String Instruments/Bowed"), None, "Cello sounds"),
    ("Musical Instruments/String Instruments/Bowed/Double Bass", Some("Musical Instruments/String Instruments/Bowed"), None, "Double bass sounds"),
    ("Musical Instruments/String Instruments/Bowed/Fiddle", Some("Musical Instruments/String Instruments/Bowed"), None, "Fiddle sounds"),
    ("Musical Instruments/String Instruments/Bowed/Viola", Some("Musical Instruments/String Instruments/Bowed"), None, "Viola sounds"),
    ("Musical Instruments/String Instruments/Bowed/Violin", Some("Musical Instruments/String Instruments/Bowed"), None, "Violin sounds"),
    
    // === MUSICAL INSTRUMENTS - STRING INSTRUMENTS - PLUCKED ===
    ("Musical Instruments/String Instruments/Plucked/Acoustic Guitar", Some("Musical Instruments/String Instruments/Plucked"), None, "Acoustic guitar sounds"),
    ("Musical Instruments/String Instruments/Plucked/Banjo", Some("Musical Instruments/String Instruments/Plucked"), None, "Banjo sounds"),
    ("Musical Instruments/String Instruments/Plucked/Electric Guitar", Some("Musical Instruments/String Instruments/Plucked"), None, "Electric guitar sounds"),
    ("Musical Instruments/String Instruments/Plucked/Harp", Some("Musical Instruments/String Instruments/Plucked"), None, "Harp sounds"),
    ("Musical Instruments/String Instruments/Plucked/Lute", Some("Musical Instruments/String Instruments/Plucked"), None, "Lute sounds"),
    
    // === MUSICAL INSTRUMENTS - STRINGS ===
    ("Musical Instruments/Strings/Bowed/Violin", Some("Musical Instruments/Strings/Bowed"), None, "Violin string sounds"),
    ("Musical Instruments/Strings/Orchestral/Cello", Some("Musical Instruments/Strings/Orchestral"), None, "Cello orchestral sounds"),
    ("Musical Instruments/Strings/Orchestral/Violin", Some("Musical Instruments/Strings/Orchestral"), None, "Violin orchestral sounds"),
    ("Musical Instruments/Strings/Plucked/Bass", Some("Musical Instruments/Strings/Plucked"), None, "Bass string sounds"),
    ("Musical Instruments/Strings/Plucked/Guitar", Some("Musical Instruments/Strings/Plucked"), None, "Guitar string sounds"),
    ("Musical Instruments/Strings/Plucked/Harp", Some("Musical Instruments/Strings/Plucked"), None, "Harp string sounds"),
    
    // === MUSICAL INSTRUMENTS - VOCALS ===
    ("Musical Instruments/Vocals/Choir", Some("Musical Instruments/Vocals"), None, "Choir vocal sounds"),
    
    // === MUSICAL INSTRUMENTS - WIND INSTRUMENTS - BRASS ===
    ("Musical Instruments/Wind Instruments/Brass/Bagpipes", Some("Musical Instruments/Wind Instruments/Brass"), None, "Bagpipe sounds"),
    ("Musical Instruments/Wind Instruments/Brass/French Horn", Some("Musical Instruments/Wind Instruments/Brass"), None, "French horn wind sounds"),
    ("Musical Instruments/Wind Instruments/Brass/Trombone", Some("Musical Instruments/Wind Instruments/Brass"), None, "Trombone sounds"),
    ("Musical Instruments/Wind Instruments/Brass/Trumpet", Some("Musical Instruments/Wind Instruments/Brass"), None, "Trumpet wind sounds"),
    ("Musical Instruments/Wind Instruments/Brass/Tuba", Some("Musical Instruments/Wind Instruments/Brass"), None, "Tuba sounds"),
    
    // === MUSICAL INSTRUMENTS - WIND INSTRUMENTS - WOODWINDS ===
    ("Musical Instruments/Wind Instruments/Woodwinds/Clarinet", Some("Musical Instruments/Wind Instruments/Woodwinds"), None, "Clarinet sounds"),
    ("Musical Instruments/Wind Instruments/Woodwinds/Flute", Some("Musical Instruments/Wind Instruments/Woodwinds"), None, "Flute woodwind sounds"),
    ("Musical Instruments/Wind Instruments/Woodwinds/Oboe", Some("Musical Instruments/Wind Instruments/Woodwinds"), None, "Oboe sounds"),
    ("Musical Instruments/Wind Instruments/Woodwinds/Pan Flute", Some("Musical Instruments/Wind Instruments/Woodwinds"), None, "Pan flute sounds"),
    ("Musical Instruments/Wind Instruments/Woodwinds/Recorder", Some("Musical Instruments/Wind Instruments/Woodwinds"), None, "Recorder sounds"),
    
    // === MUSICAL INSTRUMENTS - WINDS ===
    ("Musical Instruments/Winds/Clarinet", Some("Musical Instruments/Winds"), None, "Clarinet wind sounds"),
    ("Musical Instruments/Winds/Flute", Some("Musical Instruments/Winds"), None, "Flute wind sounds"),
    ("Musical Instruments/Winds/Organ", Some("Musical Instruments/Winds"), None, "Organ wind sounds"),
    ("Musical Instruments/Winds/Piccolo", Some("Musical Instruments/Winds"), None, "Piccolo sounds"),
    
    // === MUSICAL INSTRUMENTS - WOODWINDS ===
    ("Musical Instruments/Woodwinds/Flute", Some("Musical Instruments/Woodwinds"), None, "Flute woodwind sounds"),
    
    // === ORGANIZATIONS - GOVERNMENT - EMPIRE ===
    ("Organizations/Government/Empire/Imperial Bureaucracy", Some("Organizations/Government/Empire"), None, "Imperial bureaucracy sounds"),
    ("Organizations/Government/Empire/Imperial Court", Some("Organizations/Government/Empire"), None, "Imperial court sounds"),
    ("Organizations/Government/Empire/Provincial Governors", Some("Organizations/Government/Empire"), None, "Provincial governor sounds"),
    ("Organizations/Government/Empire/War Council", Some("Organizations/Government/Empire"), None, "War council sounds"),
    
    // === ORGANIZATIONS - GOVERNMENT - MONARCHY ===
    ("Organizations/Government/Monarchy/Palace Guards", Some("Organizations/Government/Monarchy"), None, "Palace guard sounds"),
    ("Organizations/Government/Monarchy/Royal Court", Some("Organizations/Government/Monarchy"), None, "Royal court sounds"),
    ("Organizations/Government/Monarchy/Royal Dynasty", Some("Organizations/Government/Monarchy"), None, "Royal dynasty sounds"),
    ("Organizations/Government/Monarchy/Throne Room", Some("Organizations/Government/Monarchy"), None, "Throne room sounds"),
    
    // === ORGANIZATIONS - GOVERNMENT - REPUBLIC ===
    ("Organizations/Government/Republic/Civic Assembly", Some("Organizations/Government/Republic"), None, "Civic assembly sounds"),
    ("Organizations/Government/Republic/Council", Some("Organizations/Government/Republic"), None, "Republic council sounds"),
    ("Organizations/Government/Republic/Election Committee", Some("Organizations/Government/Republic"), None, "Election committee sounds"),
    ("Organizations/Government/Republic/Senate", Some("Organizations/Government/Republic"), None, "Senate sounds"),
    ("Sci-Fi Genres/Post-Apocalyptic/Rebuilding", Some("Sci-Fi Genres/Post-Apocalyptic"), None, "Rebuilding sounds"),
    
    ("Sci-Fi Genres/Horror Sci-Fi", Some("Sci-Fi Genres"), Some("👽"), "Horror sci-fi sounds"),
    ("Sci-Fi Genres/Horror Sci-Fi/Alien Threats", Some("Sci-Fi Genres/Horror Sci-Fi"), None, "Alien threat sounds"),
    ("Sci-Fi Genres/Horror Sci-Fi/Body Horror", Some("Sci-Fi Genres/Horror Sci-Fi"), None, "Body horror sounds"),
    ("Sci-Fi Genres/Horror Sci-Fi/Cosmic Dread", Some("Sci-Fi Genres/Horror Sci-Fi"), None, "Cosmic dread sounds"),

    // === MUSICAL INSTRUMENTS ===
    ("Musical Instruments", None, Some("🎵"), "Musical instrument sounds"),
    ("Musical Instruments/String Instruments", Some("Musical Instruments"), Some("🎻"), "String instrument sounds"),
    ("Musical Instruments/String Instruments/Orchestral", Some("Musical Instruments/String Instruments"), Some("🎼"), "Orchestral string sounds"),
    ("Musical Instruments/String Instruments/Orchestral/Warm Strings", Some("Musical Instruments/String Instruments/Orchestral"), None, "Warm string sounds"),
    ("Musical Instruments/String Instruments/Orchestral/Dissonant Strings", Some("Musical Instruments/String Instruments/Orchestral"), None, "Dissonant string sounds"),
    ("Musical Instruments/String Instruments/Orchestral/Solo Violin", Some("Musical Instruments/String Instruments/Orchestral"), None, "Solo violin sounds"),
    ("Musical Instruments/String Instruments/Orchestral/Solo Cello", Some("Musical Instruments/String Instruments/Orchestral"), None, "Solo cello sounds"),
    
    ("Musical Instruments/String Instruments/Folk Strings", Some("Musical Instruments/String Instruments"), Some("🪕"), "Folk string sounds"),
    ("Musical Instruments/String Instruments/Folk Strings/Harp", Some("Musical Instruments/String Instruments/Folk Strings"), None, "Harp sounds"),
    ("Musical Instruments/String Instruments/Folk Strings/Lute", Some("Musical Instruments/String Instruments/Folk Strings"), None, "Lute sounds"),
    ("Musical Instruments/String Instruments/Folk Strings/Hurdy-Gurdy", Some("Musical Instruments/String Instruments/Folk Strings"), None, "Hurdy-gurdy sounds"),
    ("Musical Instruments/String Instruments/Folk Strings/Nyckelharpa", Some("Musical Instruments/String Instruments/Folk Strings"), None, "Nyckelharpa sounds"),
    
    ("Musical Instruments/String Instruments/World Strings", Some("Musical Instruments/String Instruments"), Some("🌏"), "World string sounds"),
    ("Musical Instruments/String Instruments/World Strings/Oud", Some("Musical Instruments/String Instruments/World Strings"), None, "Oud sounds"),
    ("Musical Instruments/String Instruments/World Strings/Sitar", Some("Musical Instruments/String Instruments/World Strings"), None, "Sitar sounds"),
    ("Musical Instruments/String Instruments/World Strings/Erhu", Some("Musical Instruments/String Instruments/World Strings"), None, "Erhu sounds"),
    ("Musical Instruments/String Instruments/World Strings/Guzheng", Some("Musical Instruments/String Instruments/World Strings"), None, "Guzheng sounds"),
    ("Musical Instruments/String Instruments/World Strings/Koto", Some("Musical Instruments/String Instruments/World Strings"), None, "Koto sounds"),
    
    ("Musical Instruments/Wind Instruments", Some("Musical Instruments"), Some("🎷"), "Wind instrument sounds"),
    ("Musical Instruments/Wind Instruments/Orchestral Winds", Some("Musical Instruments/Wind Instruments"), Some("🎺"), "Orchestral wind sounds"),
    ("Musical Instruments/Wind Instruments/Orchestral Winds/Flute", Some("Musical Instruments/Wind Instruments/Orchestral Winds"), None, "Flute sounds"),
    ("Musical Instruments/Wind Instruments/Orchestral Winds/Whistle", Some("Musical Instruments/Wind Instruments/Orchestral Winds"), None, "Whistle sounds"),
    ("Musical Instruments/Wind Instruments/Orchestral Winds/Low Brass", Some("Musical Instruments/Wind Instruments/Orchestral Winds"), None, "Low brass sounds"),
    
    ("Musical Instruments/Wind Instruments/Folk Winds", Some("Musical Instruments/Wind Instruments"), Some("🎭"), "Folk wind sounds"),
    ("Musical Instruments/Wind Instruments/Folk Winds/Bagpipes", Some("Musical Instruments/Wind Instruments/Folk Winds"), None, "Bagpipe sounds"),
    ("Musical Instruments/Wind Instruments/Folk Winds/Recorder", Some("Musical Instruments/Wind Instruments/Folk Winds"), None, "Recorder sounds"),
    
    ("Musical Instruments/Wind Instruments/World Winds", Some("Musical Instruments/Wind Instruments"), Some("🌬️"), "World wind sounds"),
    ("Musical Instruments/Wind Instruments/World Winds/Shakuhachi", Some("Musical Instruments/Wind Instruments/World Winds"), None, "Shakuhachi sounds"),
    
    ("Musical Instruments/Percussion", Some("Musical Instruments"), Some("🥁"), "Percussion sounds"),
    ("Musical Instruments/Percussion/Orchestral", Some("Musical Instruments/Percussion"), Some("🎼"), "Orchestral percussion sounds"),
    ("Musical Instruments/Percussion/Orchestral/Timpani", Some("Musical Instruments/Percussion/Orchestral"), None, "Timpani sounds"),
    ("Musical Instruments/Percussion/Orchestral/Metallic Hits", Some("Musical Instruments/Percussion/Orchestral"), None, "Metallic hit sounds"),
    
    ("Musical Instruments/Percussion/Folk", Some("Musical Instruments/Percussion"), Some("🪘"), "Folk percussion sounds"),
    ("Musical Instruments/Percussion/Folk/Bodhran", Some("Musical Instruments/Percussion/Folk"), None, "Bodhran sounds"),
    ("Musical Instruments/Percussion/Folk/Frame Drums", Some("Musical Instruments/Percussion/Folk"), None, "Frame drum sounds"),
    
    ("Musical Instruments/Percussion/World", Some("Musical Instruments/Percussion"), Some("🌍"), "World percussion sounds"),
    ("Musical Instruments/Percussion/World/Taiko", Some("Musical Instruments/Percussion/World"), None, "Taiko sounds"),
    ("Musical Instruments/Percussion/World/Gamelan", Some("Musical Instruments/Percussion/World"), None, "Gamelan sounds"),
    ("Musical Instruments/Percussion/World/Djembe", Some("Musical Instruments/Percussion/World"), None, "Djembe sounds"),
    
    ("Musical Instruments/Keyboard & Organ", Some("Musical Instruments"), Some("🎹"), "Keyboard and organ sounds"),
    ("Musical Instruments/Keyboard & Organ/Church Organ", Some("Musical Instruments/Keyboard & Organ"), None, "Church organ sounds"),
    ("Musical Instruments/Keyboard & Organ/Harpsichord", Some("Musical Instruments/Keyboard & Organ"), None, "Harpsichord sounds"),
    ("Musical Instruments/Keyboard & Organ/Piano", Some("Musical Instruments/Keyboard & Organ"), None, "Piano sounds"),
    
    ("Musical Instruments/Electronic", Some("Musical Instruments"), Some("🎛️"), "Electronic instrument sounds"),
    ("Musical Instruments/Electronic/Analog Synth", Some("Musical Instruments/Electronic"), None, "Analog synth sounds"),
    ("Musical Instruments/Electronic/FM Synth", Some("Musical Instruments/Electronic"), None, "FM synth sounds"),
    ("Musical Instruments/Electronic/Granular", Some("Musical Instruments/Electronic"), None, "Granular synth sounds"),
    ("Musical Instruments/Electronic/Noise Texture", Some("Musical Instruments/Electronic"), None, "Noise texture sounds"),
    
    ("Musical Instruments/Vocal", Some("Musical Instruments"), Some("🎤"), "Vocal sounds"),
    ("Musical Instruments/Vocal/Church Choir", Some("Musical Instruments/Vocal"), None, "Church choir sounds"),
    ("Musical Instruments/Vocal/Male Chant", Some("Musical Instruments/Vocal"), None, "Male chant sounds"),
    ("Musical Instruments/Vocal/Female Vocalise", Some("Musical Instruments/Vocal"), None, "Female vocalise sounds"),
    ("Musical Instruments/Vocal/Child Choir", Some("Musical Instruments/Vocal"), None, "Child choir sounds"),
    ("Musical Instruments/Vocal/Throat Singing", Some("Musical Instruments/Vocal"), None, "Throat singing sounds"),

    // === AUDIO STRUCTURE ===
    ("Audio Structure", None, Some("🎵"), "Audio structure sounds"),
    
    // Ending Segments
    ("Audio Structure/Ending Segments", Some("Audio Structure"), Some("⏹️"), "Audio ending segment sounds"),
    ("Audio Structure/Ending Segments/Conclusions", Some("Audio Structure/Ending Segments"), Some("🏁"), "Conclusion sounds"),
    ("Audio Structure/Ending Segments/Conclusions/Abrupt Stop", Some("Audio Structure/Ending Segments/Conclusions"), None, "Abrupt stop sounds"),
    ("Audio Structure/Ending Segments/Conclusions/Echo Fade", Some("Audio Structure/Ending Segments/Conclusions"), None, "Echo fade sounds"),
    ("Audio Structure/Ending Segments/Conclusions/Fade Out", Some("Audio Structure/Ending Segments/Conclusions"), None, "Fade out sounds"),
    ("Audio Structure/Ending Segments/Conclusions/Musical Resolution", Some("Audio Structure/Ending Segments/Conclusions"), None, "Musical resolution sounds"),
    ("Audio Structure/Ending Segments/Conclusions/Ritual Ending", Some("Audio Structure/Ending Segments/Conclusions"), None, "Ritual ending sounds"),
    
    // Intro Segments  
    ("Audio Structure/Intro Segments", Some("Audio Structure"), Some("▶️"), "Audio intro segment sounds"),
    ("Audio Structure/Intro Segments/Opening", Some("Audio Structure/Intro Segments"), Some("🎬"), "Opening sounds"),
    ("Audio Structure/Intro Segments/Opening/Build Up", Some("Audio Structure/Intro Segments/Opening"), None, "Build up sounds"),
    ("Audio Structure/Intro Segments/Opening/Cold Open", Some("Audio Structure/Intro Segments/Opening"), None, "Cold open sounds"),
    ("Audio Structure/Intro Segments/Opening/Fade In", Some("Audio Structure/Intro Segments/Opening"), None, "Fade in sounds"),
    ("Audio Structure/Intro Segments/Opening/Musical Stinger", Some("Audio Structure/Intro Segments/Opening"), None, "Musical stinger sounds"),
    ("Audio Structure/Intro Segments/Opening/Theme Introduction", Some("Audio Structure/Intro Segments/Opening"), None, "Theme introduction sounds"),

    ("Audio Structure/Song Structure", Some("Audio Structure"), Some("🎼"), "Song structure sounds"),
    ("Audio Structure/Song Structure/Intros", Some("Audio Structure/Song Structure"), None, "Introduction sounds"),
    ("Audio Structure/Song Structure/Outros", Some("Audio Structure/Song Structure"), None, "Outro sounds"),
    ("Audio Structure/Song Structure/Transitions", Some("Audio Structure/Song Structure"), None, "Transition sounds"),
    ("Audio Structure/Song Structure/Stingers", Some("Audio Structure/Song Structure"), None, "Stinger sounds"),
    ("Audio Structure/Song Structure/Loops", Some("Audio Structure/Song Structure"), None, "Loop sounds"),
    
    ("Audio Structure/Mix Types", Some("Audio Structure"), Some("🎚️"), "Mix type sounds"),
    ("Audio Structure/Mix Types/Full Mix", Some("Audio Structure/Mix Types"), None, "Full mix sounds"),
    ("Audio Structure/Mix Types/Instrumental", Some("Audio Structure/Mix Types"), None, "Instrumental sounds"),
    ("Audio Structure/Mix Types/With Vocals", Some("Audio Structure/Mix Types"), None, "With vocals sounds"),
    ("Audio Structure/Mix Types/Alternative Mix", Some("Audio Structure/Mix Types"), None, "Alternative mix sounds"),
    ("Audio Structure/Mix Types/Extended Mix", Some("Audio Structure/Mix Types"), None, "Extended mix sounds"),
    
    ("Audio Structure/Stems & Layers", Some("Audio Structure"), Some("🎛️"), "Stems and layer sounds"),
    ("Audio Structure/Stems & Layers/Percussion Stems", Some("Audio Structure/Stems & Layers"), None, "Percussion stem sounds"),
    ("Audio Structure/Stems & Layers/Ambient Stems", Some("Audio Structure/Stems & Layers"), None, "Ambient stem sounds"),
    ("Audio Structure/Stems & Layers/Melody Stems", Some("Audio Structure/Stems & Layers"), None, "Melody stem sounds"),
    ("Audio Structure/Stems & Layers/Bass Stems", Some("Audio Structure/Stems & Layers"), None, "Bass stem sounds"),
    ("Audio Structure/Stems & Layers/Harmony Stems", Some("Audio Structure/Stems & Layers"), None, "Harmony stem sounds"),
    
    ("Audio Structure/Diegetic vs Non-Diegetic", Some("Audio Structure"), Some("📻"), "Diegetic classification sounds"),
    ("Audio Structure/Diegetic vs Non-Diegetic/Diegetic", Some("Audio Structure/Diegetic vs Non-Diegetic"), Some("📺"), "Diegetic sounds"),
    ("Audio Structure/Diegetic vs Non-Diegetic/Diegetic/In-World Music", Some("Audio Structure/Diegetic vs Non-Diegetic/Diegetic"), None, "In-world music sounds"),
    ("Audio Structure/Diegetic vs Non-Diegetic/Diegetic/Radio", Some("Audio Structure/Diegetic vs Non-Diegetic/Diegetic"), None, "Radio sounds"),
    ("Audio Structure/Diegetic vs Non-Diegetic/Diegetic/Live Performance", Some("Audio Structure/Diegetic vs Non-Diegetic/Diegetic"), None, "Live performance sounds"),
    
    ("Audio Structure/Diegetic vs Non-Diegetic/Non-Diegetic", Some("Audio Structure/Diegetic vs Non-Diegetic"), Some("🎬"), "Non-diegetic sounds"),
    ("Audio Structure/Diegetic vs Non-Diegetic/Non-Diegetic/Background Score", Some("Audio Structure/Diegetic vs Non-Diegetic/Non-Diegetic"), None, "Background score sounds"),
    ("Audio Structure/Diegetic vs Non-Diegetic/Non-Diegetic/Emotional Underscore", Some("Audio Structure/Diegetic vs Non-Diegetic/Non-Diegetic"), None, "Emotional underscore sounds"),
    ("Audio Structure/Diegetic vs Non-Diegetic/Non-Diegetic/Atmospheric", Some("Audio Structure/Diegetic vs Non-Diegetic/Non-Diegetic"), None, "Atmospheric sounds"),
    
    ("Audio Structure/Audio Quality", Some("Audio Structure"), Some("🎧"), "Audio quality sounds"),
    ("Audio Structure/Audio Quality/Bed Tracks", Some("Audio Structure/Audio Quality"), None, "Bed track sounds"),
    ("Audio Structure/Audio Quality/Drone Layers", Some("Audio Structure/Audio Quality"), None, "Drone layer sounds"),
    ("Audio Structure/Audio Quality/Motifs", Some("Audio Structure/Audio Quality"), None, "Motif sounds"),
    ("Audio Structure/Audio Quality/Themes", Some("Audio Structure/Audio Quality"), None, "Theme sounds"),
    ("Audio Structure/Audio Quality/Sub-Boom", Some("Audio Structure/Audio Quality"), None, "Sub-boom sounds"),

    // === ORGANIZATIONS ===
    ("Organizations", None, Some("🏛️"), "Organization sounds"),
    ("Organizations/Criminal", Some("Organizations"), Some("🔪"), "Criminal organization sounds"),
    ("Organizations/Criminal/Thieves Guilds", Some("Organizations/Criminal"), None, "Thieves guild sounds"),
    ("Organizations/Criminal/Cartels", Some("Organizations/Criminal"), None, "Cartel sounds"),
    ("Organizations/Criminal/Smuggler Networks", Some("Organizations/Criminal"), None, "Smuggler network sounds"),
    ("Organizations/Criminal/Pirate Crews", Some("Organizations/Criminal"), None, "Pirate crew sounds"),
    
    ("Organizations/Academic", Some("Organizations"), Some("📚"), "Academic organization sounds"),
    ("Organizations/Academic/Mages Guilds", Some("Organizations/Academic"), None, "Mages guild sounds"),
    ("Organizations/Academic/Universities", Some("Organizations/Academic"), None, "University sounds"),
    ("Organizations/Academic/Research Institutes", Some("Organizations/Academic"), None, "Research institute sounds"),
    ("Organizations/Academic/Scholarly Orders", Some("Organizations/Academic"), None, "Scholarly order sounds"),
    
    ("Organizations/Religious", Some("Organizations"), Some("⛪"), "Religious organization sounds"),
    ("Organizations/Religious/Churches", Some("Organizations/Religious"), None, "Church sounds"),
    ("Organizations/Religious/Cults", Some("Organizations/Religious"), None, "Cult sounds"),
    ("Organizations/Religious/Monastic Orders", Some("Organizations/Religious"), None, "Monastic order sounds"),
    ("Organizations/Religious/Divine Orders", Some("Organizations/Religious"), None, "Divine order sounds"),
    
    ("Organizations/Political", Some("Organizations"), Some("🏛️"), "Political organization sounds"),
    ("Organizations/Political/Empires", Some("Organizations/Political"), None, "Empire sounds"),
    ("Organizations/Political/Rebel Groups", Some("Organizations/Political"), None, "Rebel group sounds"),
    ("Organizations/Political/Noble Houses", Some("Organizations/Political"), None, "Noble house sounds"),
    ("Organizations/Political/City States", Some("Organizations/Political"), None, "City state sounds"),
    
    ("Organizations/Military", Some("Organizations"), Some("⚔️"), "Military organization sounds"),
    ("Organizations/Military/Knightly Orders", Some("Organizations/Military"), None, "Knightly order sounds"),
    ("Organizations/Military/Mercenary Companies", Some("Organizations/Military"), None, "Mercenary company sounds"),
    ("Organizations/Military/Royal Guards", Some("Organizations/Military"), None, "Royal guard sounds"),
    ("Organizations/Military/Elite Units", Some("Organizations/Military"), None, "Elite unit sounds"),
    
    ("Organizations/Economic", Some("Organizations"), Some("💰"), "Economic organization sounds"),
    ("Organizations/Economic/Merchant Guilds", Some("Organizations/Economic"), None, "Merchant guild sounds"),
    ("Organizations/Economic/Trade Consortiums", Some("Organizations/Economic"), None, "Trade consortium sounds"),
    ("Organizations/Economic/Banking Houses", Some("Organizations/Economic"), None, "Banking house sounds"),
    ("Organizations/Economic/Crafting Guilds", Some("Organizations/Economic"), None, "Crafting guild sounds"),

    // === TEMPORAL EVENTS ===
    ("Temporal Events", None, Some("⏰"), "Temporal event sounds"),
    ("Temporal Events/Daily Cycles", Some("Temporal Events"), Some("🌅"), "Daily cycle sounds"),
    ("Temporal Events/Daily Cycles/Dawn", Some("Temporal Events/Daily Cycles"), None, "Dawn sounds"),
    ("Temporal Events/Daily Cycles/Midday", Some("Temporal Events/Daily Cycles"), None, "Midday sounds"),
    ("Temporal Events/Daily Cycles/Dusk", Some("Temporal Events/Daily Cycles"), None, "Dusk sounds"),
    ("Temporal Events/Daily Cycles/Midnight", Some("Temporal Events/Daily Cycles"), None, "Midnight sounds"),
    
    ("Temporal Events/Seasonal", Some("Temporal Events"), Some("🍂"), "Seasonal sounds"),
    ("Temporal Events/Seasonal/Spring", Some("Temporal Events/Seasonal"), None, "Spring sounds"),
    ("Temporal Events/Seasonal/Summer", Some("Temporal Events/Seasonal"), None, "Summer sounds"),
    ("Temporal Events/Seasonal/Autumn", Some("Temporal Events/Seasonal"), None, "Autumn sounds"),
    ("Temporal Events/Seasonal/Winter", Some("Temporal Events/Seasonal"), None, "Winter sounds"),
    
    ("Temporal Events/Festivals & Holidays", Some("Temporal Events"), Some("🎉"), "Festival and holiday sounds"),
    ("Temporal Events/Festivals & Holidays/Harvest Festivals", Some("Temporal Events/Festivals & Holidays"), None, "Harvest festival sounds"),
    ("Temporal Events/Festivals & Holidays/Religious Holidays", Some("Temporal Events/Festivals & Holidays"), None, "Religious holiday sounds"),
    ("Temporal Events/Festivals & Holidays/Royal Celebrations", Some("Temporal Events/Festivals & Holidays"), None, "Royal celebration sounds"),
    ("Temporal Events/Festivals & Holidays/Cultural Events", Some("Temporal Events/Festivals & Holidays"), None, "Cultural event sounds"),
    
    ("Temporal Events/Magical Time", Some("Temporal Events"), Some("🌀"), "Magical time sounds"),
    ("Temporal Events/Magical Time/Time Loops", Some("Temporal Events/Magical Time"), None, "Time loop sounds"),
    ("Temporal Events/Magical Time/Time Warps", Some("Temporal Events/Magical Time"), None, "Time warp sounds"),
    ("Temporal Events/Magical Time/Temporal Rifts", Some("Temporal Events/Magical Time"), None, "Temporal rift sounds"),
    ("Temporal Events/Magical Time/Chrono Distortions", Some("Temporal Events/Magical Time"), None, "Chrono distortion sounds"),

    // === MENTAL STATES ===
    ("Mental States", None, Some("🧠"), "Mental state sounds"),
    ("Mental States/Madness & Insanity", Some("Mental States"), Some("😵‍💫"), "Madness and insanity sounds"),
    ("Mental States/Madness & Insanity/Slow Descent", Some("Mental States/Madness & Insanity"), None, "Slow descent sounds"),
    ("Mental States/Madness & Insanity/Sudden Break", Some("Mental States/Madness & Insanity"), None, "Sudden break sounds"),
    ("Mental States/Madness & Insanity/Paranoia", Some("Mental States/Madness & Insanity"), None, "Paranoia sounds"),
    ("Mental States/Madness & Insanity/Delusions", Some("Mental States/Madness & Insanity"), None, "Delusion sounds"),
    
    ("Mental States/Memory & Past", Some("Mental States"), Some("💭"), "Memory and past sounds"),
    ("Mental States/Memory & Past/Nostalgia", Some("Mental States/Memory & Past"), None, "Nostalgia sounds"),
    ("Mental States/Memory & Past/Lost Memories", Some("Mental States/Memory & Past"), None, "Lost memory sounds"),
    ("Mental States/Memory & Past/Repressed Trauma", Some("Mental States/Memory & Past"), None, "Repressed trauma sounds"),
    ("Mental States/Memory & Past/False Memories", Some("Mental States/Memory & Past"), None, "False memory sounds"),
    
    ("Mental States/Dreams & Visions", Some("Mental States"), Some("💫"), "Dreams and vision sounds"),
    ("Mental States/Dreams & Visions/Prophetic Dreams", Some("Mental States/Dreams & Visions"), None, "Prophetic dream sounds"),
    ("Mental States/Dreams & Visions/Nightmares", Some("Mental States/Dreams & Visions"), None, "Nightmare sounds"),
    ("Mental States/Dreams & Visions/Lucid Dreams", Some("Mental States/Dreams & Visions"), None, "Lucid dream sounds"),
    ("Mental States/Dreams & Visions/Shared Visions", Some("Mental States/Dreams & Visions"), None, "Shared vision sounds"),
    
    ("Mental States/Consciousness", Some("Mental States"), Some("🌟"), "Consciousness sounds"),
    ("Mental States/Consciousness/Telepathy", Some("Mental States/Consciousness"), None, "Telepathy sounds"),
    ("Mental States/Consciousness/Mind Control", Some("Mental States/Consciousness"), None, "Mind control sounds"),
    ("Mental States/Consciousness/Possession", Some("Mental States/Consciousness"), None, "Possession sounds"),
    ("Mental States/Consciousness/Soul Transfer", Some("Mental States/Consciousness"), None, "Soul transfer sounds"),
    
    // === ORGANIZATIONS - GUILDS - PROFESSIONAL GUILDS ===
    ("Organizations/Guilds/Professional Guilds/Bards Guild", Some("Organizations/Guilds/Professional Guilds"), None, "Bards guild sounds"),
    ("Organizations/Guilds/Professional Guilds/Healers Guild", Some("Organizations/Guilds/Professional Guilds"), None, "Healers guild sounds"),
    ("Organizations/Guilds/Professional Guilds/Thieves Guild", Some("Organizations/Guilds/Professional Guilds"), None, "Thieves guild sounds"),
    ("Organizations/Guilds/Professional Guilds/Wizards Guild", Some("Organizations/Guilds/Professional Guilds"), None, "Wizards guild sounds"),
    
    // === ORGANIZATIONS - GUILDS - TRADE GUILDS ===
    ("Organizations/Guilds/Trade Guilds/Caravan Masters", Some("Organizations/Guilds/Trade Guilds"), None, "Caravan masters sounds"),
    ("Organizations/Guilds/Trade Guilds/Crafters Guild", Some("Organizations/Guilds/Trade Guilds"), None, "Crafters guild sounds"),
    ("Organizations/Guilds/Trade Guilds/Merchants Guild", Some("Organizations/Guilds/Trade Guilds"), None, "Merchants guild sounds"),
    ("Organizations/Guilds/Trade Guilds/Shipwrights Guild", Some("Organizations/Guilds/Trade Guilds"), None, "Shipwrights guild sounds"),
    
    // === ORGANIZATIONS - MILITARY - ARMIES ===
    ("Organizations/Military/Armies/Elite Guard", Some("Organizations/Military/Armies"), None, "Elite guard sounds"),
    ("Organizations/Military/Armies/Local Militia", Some("Organizations/Military/Armies"), None, "Local militia sounds"),
    ("Organizations/Military/Armies/Mercenary Company", Some("Organizations/Military/Armies"), None, "Mercenary company sounds"),
    ("Organizations/Military/Armies/Standing Army", Some("Organizations/Military/Armies"), None, "Standing army sounds"),
    
    // === ORGANIZATIONS - MILITARY - NAVY ===
    ("Organizations/Military/Navy/Coastal Guard", Some("Organizations/Military/Navy"), None, "Coastal guard sounds"),
    ("Organizations/Military/Navy/Fleet Command", Some("Organizations/Military/Navy"), None, "Fleet command sounds"),
    ("Organizations/Military/Navy/Naval Explorers", Some("Organizations/Military/Navy"), None, "Naval explorers sounds"),
    ("Organizations/Military/Navy/Pirate Crew", Some("Organizations/Military/Navy"), None, "Pirate crew sounds"),
    
    // === ORGANIZATIONS - RELIGIOUS - MONASTERIES ===
    ("Organizations/Religious/Monasteries/Monastic Gardens", Some("Organizations/Religious/Monasteries"), None, "Monastic garden sounds"),
    ("Organizations/Religious/Monasteries/Monastic Library", Some("Organizations/Religious/Monasteries"), None, "Monastic library sounds"),
    ("Organizations/Religious/Monasteries/Monastic Workshop", Some("Organizations/Religious/Monasteries"), None, "Monastic workshop sounds"),
    ("Organizations/Religious/Monasteries/Monks", Some("Organizations/Religious/Monasteries"), None, "Monks sounds"),
    
    // === ORGANIZATIONS - RELIGIOUS - TEMPLES ===
    ("Organizations/Religious/Temples/Temple Guardians", Some("Organizations/Religious/Temples"), None, "Temple guardian sounds"),
    ("Organizations/Religious/Temples/Temple Healers", Some("Organizations/Religious/Temples"), None, "Temple healer sounds"),
    ("Organizations/Religious/Temples/Temple Priests", Some("Organizations/Religious/Temples"), None, "Temple priest sounds"),
    ("Organizations/Religious/Temples/Temple Scribes", Some("Organizations/Religious/Temples"), None, "Temple scribe sounds"),
    
    // === ORGANIZATIONS - SECRET SOCIETIES - CONSPIRACIES ===
    ("Organizations/Secret Societies/Conspiracies/Death Cult", Some("Organizations/Secret Societies/Conspiracies"), None, "Death cult sounds"),
    ("Organizations/Secret Societies/Conspiracies/Doomsday Cult", Some("Organizations/Secret Societies/Conspiracies"), None, "Doomsday cult sounds"),
    ("Organizations/Secret Societies/Conspiracies/Illuminati", Some("Organizations/Secret Societies/Conspiracies"), None, "Illuminati sounds"),
    ("Organizations/Secret Societies/Conspiracies/Shadow Council", Some("Organizations/Secret Societies/Conspiracies"), None, "Shadow council sounds"),
    
    // === ORGANIZATIONS - SECRET SOCIETIES - MYSTERY SCHOOLS ===
    ("Organizations/Secret Societies/Mystery Schools/Ancient Order", Some("Organizations/Secret Societies/Mystery Schools"), None, "Ancient order sounds"),
    ("Organizations/Secret Societies/Mystery Schools/Cabalistic Circle", Some("Organizations/Secret Societies/Mystery Schools"), None, "Cabalistic circle sounds"),
    ("Organizations/Secret Societies/Mystery Schools/Gnostic Brotherhood", Some("Organizations/Secret Societies/Mystery Schools"), None, "Gnostic brotherhood sounds"),
    ("Organizations/Secret Societies/Mystery Schools/Hermetic Order", Some("Organizations/Secret Societies/Mystery Schools"), None, "Hermetic order sounds"),
    
    // === SFX & FOLEY - COMBAT SOUNDS ===
    ("SFX & Foley/Combat Sounds/Impacts", Some("SFX & Foley/Combat Sounds"), None, "Combat impact sounds"),
    ("SFX & Foley/Combat Sounds/Weapons", Some("SFX & Foley/Combat Sounds"), None, "Combat weapon sounds"),
    
    // === SFX & FOLEY - COMBAT SOUNDS - WEAPON IMPACTS ===
    ("SFX & Foley/Combat Sounds/Weapon Impacts/Arrow Shots", Some("SFX & Foley/Combat Sounds/Weapon Impacts"), None, "Arrow shot sounds"),
    ("SFX & Foley/Combat Sounds/Weapon Impacts/Explosions", Some("SFX & Foley/Combat Sounds/Weapon Impacts"), None, "Explosion sounds"),
    ("SFX & Foley/Combat Sounds/Weapon Impacts/Gunfire", Some("SFX & Foley/Combat Sounds/Weapon Impacts"), None, "Gunfire sounds"),
    
    // === SFX & FOLEY - CREATURE SOUNDS ===
    ("SFX & Foley/Creature Sounds/Animal Calls", Some("SFX & Foley/Creature Sounds"), None, "Animal call sounds"),
    ("SFX & Foley/Creature Sounds/Animals", Some("SFX & Foley/Creature Sounds"), None, "Animal sounds"),
    ("SFX & Foley/Creature Sounds/Breathing", Some("SFX & Foley/Creature Sounds"), None, "Breathing sounds"),
    ("SFX & Foley/Creature Sounds/Crowd", Some("SFX & Foley/Creature Sounds"), None, "Crowd sounds"),
    ("SFX & Foley/Creature Sounds/Growls", Some("SFX & Foley/Creature Sounds"), None, "Growl sounds"),
    ("SFX & Foley/Creature Sounds/Heartbeat", Some("SFX & Foley/Creature Sounds"), None, "Heartbeat sounds"),
    ("SFX & Foley/Creature Sounds/Human Voice", Some("SFX & Foley/Creature Sounds"), None, "Human voice sounds"),
    ("SFX & Foley/Creature Sounds/Insects", Some("SFX & Foley/Creature Sounds"), None, "Insect sounds"),
    ("SFX & Foley/Creature Sounds/Laughter", Some("SFX & Foley/Creature Sounds"), None, "Laughter sounds"),
    ("SFX & Foley/Creature Sounds/Roars", Some("SFX & Foley/Creature Sounds"), None, "Roar sounds"),
    ("SFX & Foley/Creature Sounds/Screams", Some("SFX & Foley/Creature Sounds"), None, "Scream sounds"),
    ("SFX & Foley/Creature Sounds/Supernatural Sounds", Some("SFX & Foley/Creature Sounds"), None, "Supernatural creature sounds"),
    ("SFX & Foley/Creature Sounds/Whispers", Some("SFX & Foley/Creature Sounds"), None, "Whisper sounds"),
    
    // === SFX & FOLEY - ENVIRONMENT FOLEY - URBAN SOUNDS ===
    ("SFX & Foley/Environment Foley/Urban Sounds/City Ambience", Some("SFX & Foley/Environment Foley/Urban Sounds"), None, "City ambience sounds"),
    ("SFX & Foley/Environment Foley/Urban Sounds/Street Movement", Some("SFX & Foley/Environment Foley/Urban Sounds"), None, "Street movement sounds"),
    
    // === SFX & FOLEY - ENVIRONMENTAL SOUNDS ===
    ("SFX & Foley/Environmental Sounds/Acoustics", Some("SFX & Foley/Environmental Sounds"), None, "Acoustic environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Atmospheric", Some("SFX & Foley/Environmental Sounds"), None, "Atmospheric environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Campfire", Some("SFX & Foley/Environmental Sounds"), None, "Campfire sounds"),
    ("SFX & Foley/Environmental Sounds/Cave Echoes", Some("SFX & Foley/Environmental Sounds"), None, "Cave echo sounds"),
    ("SFX & Foley/Environmental Sounds/Chemical", Some("SFX & Foley/Environmental Sounds"), None, "Chemical environmental sounds"),
    ("SFX & Foley/Environmental Sounds/City Traffic", Some("SFX & Foley/Environmental Sounds"), None, "City traffic sounds"),
    ("SFX & Foley/Environmental Sounds/Cleaning", Some("SFX & Foley/Environmental Sounds"), None, "Cleaning sounds"),
    ("SFX & Foley/Environmental Sounds/Destruction", Some("SFX & Foley/Environmental Sounds"), None, "Destruction sounds"),
    ("SFX & Foley/Environmental Sounds/Doors", Some("SFX & Foley/Environmental Sounds"), None, "Door environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Earth", Some("SFX & Foley/Environmental Sounds"), None, "Earth environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Fire", Some("SFX & Foley/Environmental Sounds"), None, "Fire environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Kitchen", Some("SFX & Foley/Environmental Sounds"), None, "Kitchen environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Lightning", Some("SFX & Foley/Environmental Sounds"), None, "Lightning environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Magical", Some("SFX & Foley/Environmental Sounds"), None, "Magical environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Musical", Some("SFX & Foley/Environmental Sounds"), None, "Musical environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Nature", Some("SFX & Foley/Environmental Sounds"), None, "Nature environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Physics", Some("SFX & Foley/Environmental Sounds"), None, "Physics environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Rain", Some("SFX & Foley/Environmental Sounds"), None, "Rain environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Stone", Some("SFX & Foley/Environmental Sounds"), None, "Stone environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Supernatural", Some("SFX & Foley/Environmental Sounds"), None, "Supernatural environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Tension", Some("SFX & Foley/Environmental Sounds"), None, "Tension environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Thunder", Some("SFX & Foley/Environmental Sounds"), None, "Thunder environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Urban", Some("SFX & Foley/Environmental Sounds"), None, "Urban environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Water", Some("SFX & Foley/Environmental Sounds"), None, "Water environmental sounds"),
    ("SFX & Foley/Environmental Sounds/Wind", Some("SFX & Foley/Environmental Sounds"), None, "Wind environmental sounds"),
    
    // === SFX & FOLEY - MAGICAL SOUNDS ===
    ("SFX & Foley/Magical Sounds/Enchantments", Some("SFX & Foley/Magical Sounds"), None, "Enchantment sounds"),
    ("SFX & Foley/Magical Sounds/Portals", Some("SFX & Foley/Magical Sounds"), None, "Portal magical sounds"),
    ("SFX & Foley/Magical Sounds/Spellcasting", Some("SFX & Foley/Magical Sounds"), None, "Spellcasting sounds"),
    ("SFX & Foley/Magical Sounds/Teleportation", Some("SFX & Foley/Magical Sounds"), None, "Teleportation sounds"),
    
    // === SFX & FOLEY - MATERIAL SOUNDS ===
    ("SFX & Foley/Material Sounds/Cardboard", Some("SFX & Foley/Material Sounds"), None, "Cardboard material sounds"),
    ("SFX & Foley/Material Sounds/Ceramic", Some("SFX & Foley/Material Sounds"), None, "Ceramic material sounds"),
    ("SFX & Foley/Material Sounds/Cloth", Some("SFX & Foley/Material Sounds"), None, "Cloth material sounds"),
    ("SFX & Foley/Material Sounds/Drum", Some("SFX & Foley/Material Sounds"), None, "Drum material sounds"),
    ("SFX & Foley/Material Sounds/Fabric", Some("SFX & Foley/Material Sounds"), None, "Fabric material sounds"),
    ("SFX & Foley/Material Sounds/General", Some("SFX & Foley/Material Sounds"), None, "General material sounds"),
    ("SFX & Foley/Material Sounds/Glass", Some("SFX & Foley/Material Sounds"), None, "Glass material sounds"),
    ("SFX & Foley/Material Sounds/Granular", Some("SFX & Foley/Material Sounds"), None, "Granular material sounds"),
    ("SFX & Foley/Material Sounds/Leather", Some("SFX & Foley/Material Sounds"), None, "Leather material sounds"),
    ("SFX & Foley/Material Sounds/Metal", Some("SFX & Foley/Material Sounds"), None, "Metal material sounds"),
    ("SFX & Foley/Material Sounds/Organic", Some("SFX & Foley/Material Sounds"), None, "Organic material sounds"),
    ("SFX & Foley/Material Sounds/Paper", Some("SFX & Foley/Material Sounds"), None, "Paper material sounds"),
    ("SFX & Foley/Material Sounds/Plastic", Some("SFX & Foley/Material Sounds"), None, "Plastic material sounds"),
    ("SFX & Foley/Material Sounds/Rubber", Some("SFX & Foley/Material Sounds"), None, "Rubber material sounds"),
    ("SFX & Foley/Material Sounds/Stone", Some("SFX & Foley/Material Sounds"), None, "Stone material sounds"),
    ("SFX & Foley/Material Sounds/Wood", Some("SFX & Foley/Material Sounds"), None, "Wood material sounds"),
    
    // === SFX & FOLEY - MECHANICAL SOUNDS ===
    ("SFX & Foley/Mechanical Sounds/Chains & Pulleys", Some("SFX & Foley/Mechanical Sounds"), None, "Chains and pulleys sounds"),
    ("SFX & Foley/Mechanical Sounds/Clockwork", Some("SFX & Foley/Mechanical Sounds"), None, "Clockwork mechanical sounds"),
    ("SFX & Foley/Mechanical Sounds/Doors & Gates", Some("SFX & Foley/Mechanical Sounds"), None, "Doors and gates sounds"),
    ("SFX & Foley/Mechanical Sounds/Steam & Hydraulics", Some("SFX & Foley/Mechanical Sounds"), None, "Steam and hydraulics sounds"),
    
    // === SFX & FOLEY - MOVEMENT SOUNDS ===
    ("SFX & Foley/Movement Sounds/Cloth", Some("SFX & Foley/Movement Sounds"), None, "Cloth movement sounds"),
    ("SFX & Foley/Movement Sounds/Footsteps", Some("SFX & Foley/Movement Sounds"), None, "Footstep movement sounds"),
    
    // === SFX & FOLEY - ORCHESTRAL ACCENTS ===
    ("SFX & Foley/Orchestral Accents/Brass Impacts", Some("SFX & Foley/Orchestral Accents"), None, "Brass impact accents"),
    ("SFX & Foley/Orchestral Accents/Build Ups", Some("SFX & Foley/Orchestral Accents"), None, "Orchestral build up accents"),
    
    // === SFX & FOLEY - TECHNOLOGY SOUNDS ===
    ("SFX & Foley/Technology Sounds/Alarms", Some("SFX & Foley/Technology Sounds"), None, "Alarm technology sounds"),
    ("SFX & Foley/Technology Sounds/Computer Interface", Some("SFX & Foley/Technology Sounds"), None, "Computer interface sounds"),
    ("SFX & Foley/Technology Sounds/Electrical", Some("SFX & Foley/Technology Sounds"), None, "Electrical technology sounds"),
    ("SFX & Foley/Technology Sounds/Electronic", Some("SFX & Foley/Technology Sounds"), None, "Electronic technology sounds"),
    ("SFX & Foley/Technology Sounds/Electronic Beeps", Some("SFX & Foley/Technology Sounds"), None, "Electronic beep sounds"),
    ("SFX & Foley/Technology Sounds/Emergency", Some("SFX & Foley/Technology Sounds"), None, "Emergency technology sounds"),
    ("SFX & Foley/Technology Sounds/Engines", Some("SFX & Foley/Technology Sounds"), None, "Engine technology sounds"),
    ("SFX & Foley/Technology Sounds/Equipment", Some("SFX & Foley/Technology Sounds"), None, "Equipment technology sounds"),
    ("SFX & Foley/Technology Sounds/Machinery", Some("SFX & Foley/Technology Sounds"), None, "Machinery technology sounds"),
    ("SFX & Foley/Technology Sounds/Phone", Some("SFX & Foley/Technology Sounds"), None, "Phone technology sounds"),
    ("SFX & Foley/Technology Sounds/Police Radio", Some("SFX & Foley/Technology Sounds"), None, "Police radio sounds"),
    ("SFX & Foley/Technology Sounds/Radio", Some("SFX & Foley/Technology Sounds"), None, "Radio technology sounds"),
    ("SFX & Foley/Technology Sounds/Tools", Some("SFX & Foley/Technology Sounds"), None, "Tool technology sounds"),
    ("SFX & Foley/Technology Sounds/Vehicles", Some("SFX & Foley/Technology Sounds"), None, "Vehicle technology sounds"),
    
    // === SCI-FI GENRES - CYBERPUNK - DYSTOPIAN CITIES ===
    ("Sci-Fi Genres/Cyberpunk/Dystopian Cities/Corporate Towers", Some("Sci-Fi Genres/Cyberpunk/Dystopian Cities"), None, "Corporate tower sounds"),
    ("Sci-Fi Genres/Cyberpunk/Dystopian Cities/Data Streams", Some("Sci-Fi Genres/Cyberpunk/Dystopian Cities"), None, "Data stream sounds"),
    ("Sci-Fi Genres/Cyberpunk/Dystopian Cities/Neon Streets", Some("Sci-Fi Genres/Cyberpunk/Dystopian Cities"), None, "Neon street sounds"),
    ("Sci-Fi Genres/Cyberpunk/Dystopian Cities/Underground", Some("Sci-Fi Genres/Cyberpunk/Dystopian Cities"), None, "Underground dystopian sounds"),
    
    // === SCI-FI GENRES - CYBERPUNK - TECH NOIR ===
    ("Sci-Fi Genres/Cyberpunk/Tech Noir/Digital Ghost", Some("Sci-Fi Genres/Cyberpunk/Tech Noir"), None, "Digital ghost sounds"),
    ("Sci-Fi Genres/Cyberpunk/Tech Noir/Memory Implants", Some("Sci-Fi Genres/Cyberpunk/Tech Noir"), None, "Memory implant sounds"),
    ("Sci-Fi Genres/Cyberpunk/Tech Noir/Private Eye", Some("Sci-Fi Genres/Cyberpunk/Tech Noir"), None, "Private eye sounds"),
    ("Sci-Fi Genres/Cyberpunk/Tech Noir/Rain-Soaked Streets", Some("Sci-Fi Genres/Cyberpunk/Tech Noir"), None, "Rain-soaked street sounds"),
    
    // === SCI-FI GENRES - HARD SCI-FI - MILITARY TECH ===
    ("Sci-Fi Genres/Hard Sci-Fi/Military Tech/Drones", Some("Sci-Fi Genres/Hard Sci-Fi/Military Tech"), None, "Military drone sounds"),
    ("Sci-Fi Genres/Hard Sci-Fi/Military Tech/Mechs", Some("Sci-Fi Genres/Hard Sci-Fi/Military Tech"), None, "Military mech sounds"),
    ("Sci-Fi Genres/Hard Sci-Fi/Military Tech/Weapons", Some("Sci-Fi Genres/Hard Sci-Fi/Military Tech"), None, "Military tech weapon sounds"),
    
    // === SCI-FI GENRES - HARD SCI-FI - SCIENTIFIC DISCOVERY ===
    ("Sci-Fi Genres/Hard Sci-Fi/Scientific Discovery/Breakthrough", Some("Sci-Fi Genres/Hard Sci-Fi/Scientific Discovery"), None, "Scientific breakthrough sounds"),
    ("Sci-Fi Genres/Hard Sci-Fi/Scientific Discovery/Energy Weapons", Some("Sci-Fi Genres/Hard Sci-Fi/Scientific Discovery"), None, "Energy weapon sounds"),
    ("Sci-Fi Genres/Hard Sci-Fi/Scientific Discovery/Genetic Engineering", Some("Sci-Fi Genres/Hard Sci-Fi/Scientific Discovery"), None, "Genetic engineering sounds"),
    ("Sci-Fi Genres/Hard Sci-Fi/Scientific Discovery/Laboratory", Some("Sci-Fi Genres/Hard Sci-Fi/Scientific Discovery"), None, "Laboratory sounds"),
    ("Sci-Fi Genres/Hard Sci-Fi/Scientific Discovery/Nuclear Physics", Some("Sci-Fi Genres/Hard Sci-Fi/Scientific Discovery"), None, "Nuclear physics sounds"),
    ("Sci-Fi Genres/Hard Sci-Fi/Scientific Discovery/Quantum Physics", Some("Sci-Fi Genres/Hard Sci-Fi/Scientific Discovery"), None, "Quantum physics sounds"),
    ("Sci-Fi Genres/Hard Sci-Fi/Scientific Discovery/Research", Some("Sci-Fi Genres/Hard Sci-Fi/Scientific Discovery"), None, "Research sounds"),
    
    // === SCI-FI GENRES - HARD SCI-FI - SPACE EXPLORATION ===
    ("Sci-Fi Genres/Hard Sci-Fi/Space Exploration/Alien World", Some("Sci-Fi Genres/Hard Sci-Fi/Space Exploration"), None, "Alien world sounds"),
    ("Sci-Fi Genres/Hard Sci-Fi/Space Exploration/Deep Space", Some("Sci-Fi Genres/Hard Sci-Fi/Space Exploration"), None, "Deep space sounds"),
    ("Sci-Fi Genres/Hard Sci-Fi/Space Exploration/Galactic Empire", Some("Sci-Fi Genres/Hard Sci-Fi/Space Exploration"), None, "Galactic empire sounds"),
    ("Sci-Fi Genres/Hard Sci-Fi/Space Exploration/Research Station", Some("Sci-Fi Genres/Hard Sci-Fi/Space Exploration"), None, "Research station sounds"),
    ("Sci-Fi Genres/Hard Sci-Fi/Space Exploration/Spacecraft", Some("Sci-Fi Genres/Hard Sci-Fi/Space Exploration"), None, "Spacecraft sounds"),
    
    // === SCI-FI GENRES - HARD SCI-FI - TRANSHUMANISM ===
    ("Sci-Fi Genres/Hard Sci-Fi/Transhumanism/Enhanced Humans", Some("Sci-Fi Genres/Hard Sci-Fi/Transhumanism"), None, "Enhanced human sounds"),
    
    // === SCI-FI GENRES - POST-APOCALYPTIC - SURVIVORS ===
    ("Sci-Fi Genres/Post-Apocalyptic/Survivors/New Civilization", Some("Sci-Fi Genres/Post-Apocalyptic/Survivors"), None, "New civilization sounds"),
    ("Sci-Fi Genres/Post-Apocalyptic/Survivors/Resistance Cell", Some("Sci-Fi Genres/Post-Apocalyptic/Survivors"), None, "Resistance cell sounds"),
    ("Sci-Fi Genres/Post-Apocalyptic/Survivors/Safe Haven", Some("Sci-Fi Genres/Post-Apocalyptic/Survivors"), None, "Safe haven sounds"),
    ("Sci-Fi Genres/Post-Apocalyptic/Survivors/Trading Caravan", Some("Sci-Fi Genres/Post-Apocalyptic/Survivors"), None, "Trading caravan sounds"),
    
    // === SCI-FI GENRES - POST-APOCALYPTIC - WASTELAND ===
    ("Sci-Fi Genres/Post-Apocalyptic/Wasteland/Desert Ruins", Some("Sci-Fi Genres/Post-Apocalyptic/Wasteland"), None, "Desert ruin sounds"),
    ("Sci-Fi Genres/Post-Apocalyptic/Wasteland/Mutant Creatures", Some("Sci-Fi Genres/Post-Apocalyptic/Wasteland"), None, "Mutant creature sounds"),
    ("Sci-Fi Genres/Post-Apocalyptic/Wasteland/Scavenger Camps", Some("Sci-Fi Genres/Post-Apocalyptic/Wasteland"), None, "Scavenger camp sounds"),
    ("Sci-Fi Genres/Post-Apocalyptic/Wasteland/Toxic Zones", Some("Sci-Fi Genres/Post-Apocalyptic/Wasteland"), None, "Toxic zone sounds"),
    
    // === SCI-FI GENRES - SPACE OPERA - GALACTIC EMPIRE ===
    ("Sci-Fi Genres/Space Opera/Galactic Empire/Death Star", Some("Sci-Fi Genres/Space Opera/Galactic Empire"), None, "Death star sounds"),
    ("Sci-Fi Genres/Space Opera/Galactic Empire/Fleet Battle", Some("Sci-Fi Genres/Space Opera/Galactic Empire"), None, "Fleet battle sounds"),
    ("Sci-Fi Genres/Space Opera/Galactic Empire/Imperial March", Some("Sci-Fi Genres/Space Opera/Galactic Empire"), None, "Imperial march sounds"),
    ("Sci-Fi Genres/Space Opera/Galactic Empire/Senate Politics", Some("Sci-Fi Genres/Space Opera/Galactic Empire"), None, "Senate politics sounds"),
    
    // === SCI-FI GENRES - SPACE OPERA - REBEL ALLIANCE ===
    ("Sci-Fi Genres/Space Opera/Rebel Alliance/Heroic Theme", Some("Sci-Fi Genres/Space Opera/Rebel Alliance"), None, "Heroic theme sounds"),
    ("Sci-Fi Genres/Space Opera/Rebel Alliance/Secret Base", Some("Sci-Fi Genres/Space Opera/Rebel Alliance"), None, "Secret base sounds"),
    ("Sci-Fi Genres/Space Opera/Rebel Alliance/Smuggler's Run", Some("Sci-Fi Genres/Space Opera/Rebel Alliance"), None, "Smuggler's run sounds"),
    ("Sci-Fi Genres/Space Opera/Rebel Alliance/Starfighter Assault", Some("Sci-Fi Genres/Space Opera/Rebel Alliance"), None, "Starfighter assault sounds"),
    
    // === SESSION STRUCTURE - ACTION SEQUENCES ===
    ("Session Structure/Action Sequences/Alert", Some("Session Structure/Action Sequences"), None, "Alert action sequence sounds"),
    ("Session Structure/Action Sequences/Ambush", Some("Session Structure/Action Sequences"), None, "Ambush action sequence sounds"),
    ("Session Structure/Action Sequences/Boss Battle", Some("Session Structure/Action Sequences"), None, "Boss battle action sounds"),
    ("Session Structure/Action Sequences/Chase", Some("Session Structure/Action Sequences"), None, "Chase action sequence sounds"),
    ("Session Structure/Action Sequences/Combat", Some("Session Structure/Action Sequences"), None, "Combat action sequence sounds"),
    ("Session Structure/Action Sequences/Emergency", Some("Session Structure/Action Sequences"), None, "Emergency action sequence sounds"),
    ("Session Structure/Action Sequences/Escape", Some("Session Structure/Action Sequences"), None, "Escape action sequence sounds"),
    ("Session Structure/Action Sequences/Hunt", Some("Session Structure/Action Sequences"), None, "Hunt action sequence sounds"),
    ("Session Structure/Action Sequences/Infiltration", Some("Session Structure/Action Sequences"), None, "Infiltration action sequence sounds"),
    ("Session Structure/Action Sequences/Siege", Some("Session Structure/Action Sequences"), None, "Siege action sequence sounds"),
    ("Session Structure/Action Sequences/Stealth", Some("Session Structure/Action Sequences"), None, "Stealth action sequence sounds"),
    ("Session Structure/Action Sequences/Trap", Some("Session Structure/Action Sequences"), None, "Trap action sequence sounds"),
    
    // === SESSION STRUCTURE - ADVENTURE SEQUENCES ===
    ("Session Structure/Adventure Sequences/Arrival", Some("Session Structure/Adventure Sequences"), None, "Arrival adventure sequence sounds"),
    ("Session Structure/Adventure Sequences/Cosmic", Some("Session Structure/Adventure Sequences"), None, "Cosmic adventure sequence sounds"),
    ("Session Structure/Adventure Sequences/Dungeon Exploration", Some("Session Structure/Adventure Sequences"), None, "Dungeon exploration sounds"),
    ("Session Structure/Adventure Sequences/Exploration", Some("Session Structure/Adventure Sequences"), None, "Exploration adventure sequence sounds"),
    ("Session Structure/Adventure Sequences/Facility", Some("Session Structure/Adventure Sequences"), None, "Facility adventure sequence sounds"),
    ("Session Structure/Adventure Sequences/General", Some("Session Structure/Adventure Sequences"), None, "General adventure sequence sounds"),
    ("Session Structure/Adventure Sequences/Hellscape", Some("Session Structure/Adventure Sequences"), None, "Hellscape adventure sequence sounds"),
    ("Session Structure/Adventure Sequences/Infiltration", Some("Session Structure/Adventure Sequences"), None, "Infiltration adventure sequence sounds"),
    ("Session Structure/Adventure Sequences/Lair", Some("Session Structure/Adventure Sequences"), None, "Lair adventure sequence sounds"),
    ("Session Structure/Adventure Sequences/Natural Disaster", Some("Session Structure/Adventure Sequences"), None, "Natural disaster adventure sounds"),
    ("Session Structure/Adventure Sequences/Time", Some("Session Structure/Adventure Sequences"), None, "Time adventure sequence sounds"),
    ("Session Structure/Adventure Sequences/Travel", Some("Session Structure/Adventure Sequences"), None, "Travel adventure sequence sounds"),
    ("Session Structure/Adventure Sequences/Underground", Some("Session Structure/Adventure Sequences"), None, "Underground adventure sequence sounds"),
    ("Session Structure/Adventure Sequences/Vehicle", Some("Session Structure/Adventure Sequences"), None, "Vehicle adventure sequence sounds"),
    ("Session Structure/Adventure Sequences/Weather", Some("Session Structure/Adventure Sequences"), None, "Weather adventure sequence sounds"),
    
    // === SESSION STRUCTURE - AMBIENT BACKGROUND ===
    ("Session Structure/Ambient Background/General", Some("Session Structure/Ambient Background"), None, "General ambient background sounds"),
    
    // === SESSION STRUCTURE - CHALLENGES ===
    ("Session Structure/Challenges/Puzzle", Some("Session Structure/Challenges"), None, "Puzzle challenge sounds"),
    ("Session Structure/Challenges/Research", Some("Session Structure/Challenges"), None, "Research challenge sounds"),
    ("Session Structure/Challenges/Ritual", Some("Session Structure/Challenges"), None, "Ritual challenge sounds"),
    ("Session Structure/Challenges/Survival", Some("Session Structure/Challenges"), None, "Survival challenge sounds"),
    
    // === SESSION STRUCTURE - CLIMAX & RESOLUTION - FINAL ENCOUNTERS ===
    ("Session Structure/Climax & Resolution/Final Encounters/Boss Fight", Some("Session Structure/Climax & Resolution/Final Encounters"), None, "Boss fight encounter sounds"),
    ("Session Structure/Climax & Resolution/Final Encounters/Last Stand", Some("Session Structure/Climax & Resolution/Final Encounters"), None, "Last stand encounter sounds"),
    ("Session Structure/Climax & Resolution/Final Encounters/Save the World", Some("Session Structure/Climax & Resolution/Final Encounters"), None, "Save the world encounter sounds"),
    ("Session Structure/Climax & Resolution/Final Encounters/Villain Showdown", Some("Session Structure/Climax & Resolution/Final Encounters"), None, "Villain showdown encounter sounds"),
    
    // === SESSION STRUCTURE - CLIMAX & RESOLUTION - REVELATIONS ===
    ("Session Structure/Climax & Resolution/Revelations/Betrayal Revealed", Some("Session Structure/Climax & Resolution/Revelations"), None, "Betrayal revelation sounds"),
    ("Session Structure/Climax & Resolution/Revelations/Mystery Solved", Some("Session Structure/Climax & Resolution/Revelations"), None, "Mystery solved revelation sounds"),
    ("Session Structure/Climax & Resolution/Revelations/Plot Twist", Some("Session Structure/Climax & Resolution/Revelations"), None, "Plot twist revelation sounds"),
    ("Session Structure/Climax & Resolution/Revelations/True Identity", Some("Session Structure/Climax & Resolution/Revelations"), None, "True identity revelation sounds"),
    
    // === SESSION STRUCTURE - DOWNTIME ===
    ("Session Structure/Downtime/Construction", Some("Session Structure/Downtime"), None, "Construction downtime sounds"),
    ("Session Structure/Downtime/General", Some("Session Structure/Downtime"), None, "General downtime sounds"),
    ("Session Structure/Downtime/Healing", Some("Session Structure/Downtime"), None, "Healing downtime sounds"),
    ("Session Structure/Downtime/Preparation", Some("Session Structure/Downtime"), None, "Preparation downtime sounds"),
    ("Session Structure/Downtime/Rest", Some("Session Structure/Downtime"), None, "Rest downtime sounds"),
    ("Session Structure/Downtime/Training", Some("Session Structure/Downtime"), None, "Training downtime sounds"),
    
    // === SESSION STRUCTURE - ENDINGS & CONCLUSIONS - SESSION END ===
    ("Session Structure/Endings & Conclusions/Session End/Bittersweet Ending", Some("Session Structure/Endings & Conclusions/Session End"), None, "Bittersweet ending sounds"),
    ("Session Structure/Endings & Conclusions/Session End/Cliffhanger", Some("Session Structure/Endings & Conclusions/Session End"), None, "Cliffhanger ending sounds"),
    ("Session Structure/Endings & Conclusions/Session End/Peaceful Resolution", Some("Session Structure/Endings & Conclusions/Session End"), None, "Peaceful resolution ending sounds"),
    ("Session Structure/Endings & Conclusions/Session End/Victory Celebration", Some("Session Structure/Endings & Conclusions/Session End"), None, "Victory celebration ending sounds"),
    
    // === SESSION STRUCTURE - HORROR ELEMENTS ===
    ("Session Structure/Horror Elements/Jump Scare", Some("Session Structure/Horror Elements"), None, "Jump scare horror element sounds"),
    
    // === SESSION STRUCTURE - MAIN CONTENT - EXPLORATION ===
    ("Session Structure/Main Content/Exploration/Ancient Ruins", Some("Session Structure/Main Content/Exploration"), None, "Ancient ruins exploration sounds"),
    ("Session Structure/Main Content/Exploration/Dungeon Exploration", Some("Session Structure/Main Content/Exploration"), None, "Dungeon exploration sounds"),
    ("Session Structure/Main Content/Exploration/Urban Investigation", Some("Session Structure/Main Content/Exploration"), None, "Urban investigation sounds"),
    ("Session Structure/Main Content/Exploration/Wilderness Travel", Some("Session Structure/Main Content/Exploration"), None, "Wilderness travel sounds"),
    
    // === SESSION STRUCTURE - MAIN CONTENT - PROBLEM SOLVING ===
    ("Session Structure/Main Content/Problem Solving/Investigation", Some("Session Structure/Main Content/Problem Solving"), None, "Investigation problem solving sounds"),
    ("Session Structure/Main Content/Problem Solving/Puzzles", Some("Session Structure/Main Content/Problem Solving"), None, "Puzzle problem solving sounds"),
    ("Session Structure/Main Content/Problem Solving/Riddles", Some("Session Structure/Main Content/Problem Solving"), None, "Riddle problem solving sounds"),
    ("Session Structure/Main Content/Problem Solving/Traps", Some("Session Structure/Main Content/Problem Solving"), None, "Trap problem solving sounds"),
    
    // === SESSION STRUCTURE - MAIN CONTENT - ROLE-PLAYING ===
    ("Session Structure/Main Content/Role-Playing/Character Development", Some("Session Structure/Main Content/Role-Playing"), None, "Character development sounds"),
    ("Session Structure/Main Content/Role-Playing/Dream Sequences", Some("Session Structure/Main Content/Role-Playing"), None, "Dream sequence sounds"),
    ("Session Structure/Main Content/Role-Playing/Flashbacks", Some("Session Structure/Main Content/Role-Playing"), None, "Flashback sounds"),
    ("Session Structure/Main Content/Role-Playing/Social Encounters", Some("Session Structure/Main Content/Role-Playing"), None, "Social encounter sounds"),
    
    // === SESSION STRUCTURE - OPENING & INTRODUCTIONS - SESSION START ===
    ("Session Structure/Opening & Introductions/Session Start/Character Introductions", Some("Session Structure/Opening & Introductions/Session Start"), None, "Character introduction sounds"),
    ("Session Structure/Opening & Introductions/Session Start/Pre-Game", Some("Session Structure/Opening & Introductions/Session Start"), None, "Pre-game sounds"),
    ("Session Structure/Opening & Introductions/Session Start/Recap", Some("Session Structure/Opening & Introductions/Session Start"), None, "Recap sounds"),
    ("Session Structure/Opening & Introductions/Session Start/Setting the Scene", Some("Session Structure/Opening & Introductions/Session Start"), None, "Setting the scene sounds"),
    
    // === SESSION STRUCTURE - OPENING & INTRODUCTIONS - STORY HOOKS ===
    ("Session Structure/Opening & Introductions/Story Hooks/Adventure Hook", Some("Session Structure/Opening & Introductions/Story Hooks"), None, "Adventure hook sounds"),
    ("Session Structure/Opening & Introductions/Story Hooks/Danger Hook", Some("Session Structure/Opening & Introductions/Story Hooks"), None, "Danger hook sounds"),
    ("Session Structure/Opening & Introductions/Story Hooks/Mystery Hook", Some("Session Structure/Opening & Introductions/Story Hooks"), None, "Mystery hook sounds"),
    ("Session Structure/Opening & Introductions/Story Hooks/Social Hook", Some("Session Structure/Opening & Introductions/Story Hooks"), None, "Social hook sounds"),
    
    // === SESSION STRUCTURE - RESULTS ===
    ("Session Structure/Results/Failure", Some("Session Structure/Results"), None, "Failure result sounds"),
    ("Session Structure/Results/Success", Some("Session Structure/Results"), None, "Success result sounds"),
    
    // === SESSION STRUCTURE - REVEALS ===
    ("Session Structure/Reveals/Mystery Reveal", Some("Session Structure/Reveals"), None, "Mystery reveal sounds"),
    
    // === SESSION STRUCTURE - SOCIAL ENCOUNTERS ===
    ("Session Structure/Social Encounters/Celebration", Some("Session Structure/Social Encounters"), None, "Celebration social encounter sounds"),
    ("Session Structure/Social Encounters/Ceremony", Some("Session Structure/Social Encounters"), None, "Ceremony social encounter sounds"),
    ("Session Structure/Social Encounters/Court", Some("Session Structure/Social Encounters"), None, "Court social encounter sounds"),
    ("Session Structure/Social Encounters/Cult", Some("Session Structure/Social Encounters"), None, "Cult social encounter sounds"),
    ("Session Structure/Social Encounters/General", Some("Session Structure/Social Encounters"), None, "General social encounter sounds"),
    ("Session Structure/Social Encounters/Hospital", Some("Session Structure/Social Encounters"), None, "Hospital social encounter sounds"),
    ("Session Structure/Social Encounters/Inn", Some("Session Structure/Social Encounters"), None, "Inn social encounter sounds"),
    ("Session Structure/Social Encounters/Investigation", Some("Session Structure/Social Encounters"), None, "Investigation social encounter sounds"),
    ("Session Structure/Social Encounters/Market", Some("Session Structure/Social Encounters"), None, "Market social encounter sounds"),
    ("Session Structure/Social Encounters/Negotiation", Some("Session Structure/Social Encounters"), None, "Negotiation social encounter sounds"),
    ("Session Structure/Social Encounters/Professional", Some("Session Structure/Social Encounters"), None, "Professional social encounter sounds"),
    ("Session Structure/Social Encounters/Shopping", Some("Session Structure/Social Encounters"), None, "Shopping social encounter sounds"),
    ("Session Structure/Social Encounters/Supernatural", Some("Session Structure/Social Encounters"), None, "Supernatural social encounter sounds"),
    ("Session Structure/Social Encounters/Tavern", Some("Session Structure/Social Encounters"), None, "Tavern social encounter sounds"),
    ("Session Structure/Social Encounters/Trial", Some("Session Structure/Social Encounters"), None, "Trial social encounter sounds"),
    
    // === SESSION STRUCTURE - STORY ELEMENTS ===
    ("Session Structure/Story Elements/Cliffhanger", Some("Session Structure/Story Elements"), None, "Cliffhanger story element sounds"),
    ("Session Structure/Story Elements/Closing", Some("Session Structure/Story Elements"), None, "Closing story element sounds"),
    ("Session Structure/Story Elements/Defeat", Some("Session Structure/Story Elements"), None, "Defeat story element sounds"),
    ("Session Structure/Story Elements/Flashback", Some("Session Structure/Story Elements"), None, "Flashback story element sounds"),
    ("Session Structure/Story Elements/Opening", Some("Session Structure/Story Elements"), None, "Opening story element sounds"),
    ("Session Structure/Story Elements/Recap", Some("Session Structure/Story Elements"), None, "Recap story element sounds"),
    ("Session Structure/Story Elements/Reveal", Some("Session Structure/Story Elements"), None, "Reveal story element sounds"),
    ("Session Structure/Story Elements/Tragedy", Some("Session Structure/Story Elements"), None, "Tragedy story element sounds"),
    ("Session Structure/Story Elements/Victory", Some("Session Structure/Story Elements"), None, "Victory story element sounds"),
    
    // === SESSION STRUCTURE - SYSTEM ===
    ("Session Structure/System/Error", Some("Session Structure/System"), None, "System error sounds"),
    ("Session Structure/System/Notification", Some("Session Structure/System"), None, "System notification sounds"),
    
    // === SESSION STRUCTURE - TRANSITIONS - SCENE CHANGES ===
    ("Session Structure/Transitions/Scene Changes/Cross Fade", Some("Session Structure/Transitions/Scene Changes"), None, "Cross fade transition sounds"),
    ("Session Structure/Transitions/Scene Changes/Fade In", Some("Session Structure/Transitions/Scene Changes"), None, "Fade in transition sounds"),
    ("Session Structure/Transitions/Scene Changes/Fade Out", Some("Session Structure/Transitions/Scene Changes"), None, "Fade out transition sounds"),
    ("Session Structure/Transitions/Scene Changes/Musical Stinger", Some("Session Structure/Transitions/Scene Changes"), None, "Musical stinger transition sounds"),
    ("Session Structure/Transitions/Scene Changes/Smooth Transition", Some("Session Structure/Transitions/Scene Changes"), None, "Smooth transition sounds"),
    
    // === SOCIAL ENCOUNTERS - ENTERTAINMENT ===
    ("Social Encounters/Entertainment/Dance Halls", Some("Social Encounters/Entertainment"), None, "Dance hall sounds"),
    ("Social Encounters/Entertainment/Diners", Some("Social Encounters/Entertainment"), None, "Diner sounds"),
    ("Social Encounters/Entertainment/Festivals", Some("Social Encounters/Entertainment"), None, "Festival entertainment sounds"),
    ("Social Encounters/Entertainment/Gambling Dens", Some("Social Encounters/Entertainment"), None, "Gambling den sounds"),
    ("Social Encounters/Entertainment/Jazz Clubs", Some("Social Encounters/Entertainment"), None, "Jazz club sounds"),
    ("Social Encounters/Entertainment/Lounges", Some("Social Encounters/Entertainment"), None, "Lounge sounds"),
    ("Social Encounters/Entertainment/Speakeasies", Some("Social Encounters/Entertainment"), None, "Speakeasy sounds"),
    ("Social Encounters/Entertainment/Theaters", Some("Social Encounters/Entertainment"), None, "Theater entertainment sounds"),
    
    // === SOCIAL ENCOUNTERS - GOVERNMENT & AUTHORITY ===
    ("Social Encounters/Government & Authority/Law Enforcement", Some("Social Encounters/Government & Authority"), None, "Law enforcement sounds"),
    
    // === SOCIAL ENCOUNTERS - GUILDS & ORGANIZATIONS ===
    ("Social Encounters/Guilds & Organizations/Mages Guilds", Some("Social Encounters/Guilds & Organizations"), None, "Mages guild sounds"),
    ("Social Encounters/Guilds & Organizations/Merchants Guilds", Some("Social Encounters/Guilds & Organizations"), None, "Merchants guild sounds"),
    ("Social Encounters/Guilds & Organizations/Thieves Guilds", Some("Social Encounters/Guilds & Organizations"), None, "Thieves guild sounds"),
    
    // === SOCIAL ENCOUNTERS - MARKETS & TRADE ===
    ("Social Encounters/Markets & Trade/International Markets", Some("Social Encounters/Markets & Trade"), None, "International market sounds"),
    ("Social Encounters/Markets & Trade/Merchant Negotiations", Some("Social Encounters/Markets & Trade"), None, "Merchant negotiation sounds"),
    
    // === SOCIAL ENCOUNTERS - NOBILITY & COURTS ===
    ("Social Encounters/Nobility & Courts/Court Trials", Some("Social Encounters/Nobility & Courts"), None, "Court trial sounds"),
    ("Social Encounters/Nobility & Courts/Political Intrigue", Some("Social Encounters/Nobility & Courts"), None, "Political intrigue sounds"),
    ("Social Encounters/Nobility & Courts/Royal Audiences", Some("Social Encounters/Nobility & Courts"), None, "Royal audience sounds"),
    
    // === SOCIAL ENCOUNTERS - TAVERNS & INNS ===
    ("Social Encounters/Taverns & Inns/Cozy", Some("Social Encounters/Taverns & Inns"), None, "Cozy tavern and inn sounds"),
    
    // === SUPERHERO & COMIC BOOK - HEROES ===
    ("Superhero & Comic Book/Heroes/Classic Heroes", Some("Superhero & Comic Book/Heroes"), None, "Classic hero sounds"),
    
    // === TEMPORAL EVENTS - CELESTIAL EVENTS - LUNAR PHASES ===
    ("Temporal Events/Celestial Events/Lunar Phases/Blood Moon", Some("Temporal Events/Celestial Events/Lunar Phases"), None, "Blood moon sounds"),
    ("Temporal Events/Celestial Events/Lunar Phases/Crescent Moon", Some("Temporal Events/Celestial Events/Lunar Phases"), None, "Crescent moon sounds"),
    ("Temporal Events/Celestial Events/Lunar Phases/Full Moon", Some("Temporal Events/Celestial Events/Lunar Phases"), None, "Full moon sounds"),
    ("Temporal Events/Celestial Events/Lunar Phases/New Moon", Some("Temporal Events/Celestial Events/Lunar Phases"), None, "New moon sounds"),
    
    // === TEMPORAL EVENTS - CELESTIAL EVENTS - SOLAR EVENTS ===
    ("Temporal Events/Celestial Events/Solar Events/Aurora Borealis", Some("Temporal Events/Celestial Events/Solar Events"), None, "Aurora borealis sounds"),
    ("Temporal Events/Celestial Events/Solar Events/Comet Passage", Some("Temporal Events/Celestial Events/Solar Events"), None, "Comet passage sounds"),
    ("Temporal Events/Celestial Events/Solar Events/Meteor Shower", Some("Temporal Events/Celestial Events/Solar Events"), None, "Meteor shower sounds"),
    ("Temporal Events/Celestial Events/Solar Events/Solar Eclipse", Some("Temporal Events/Celestial Events/Solar Events"), None, "Solar eclipse sounds"),
    
    // === TEMPORAL EVENTS - HISTORICAL EVENTS - BATTLES ===
    ("Temporal Events/Historical Events/Battles/Ancient War", Some("Temporal Events/Historical Events/Battles"), None, "Ancient war sounds"),
    ("Temporal Events/Historical Events/Battles/Civil War", Some("Temporal Events/Historical Events/Battles"), None, "Civil war sounds"),
    ("Temporal Events/Historical Events/Battles/Final Battle", Some("Temporal Events/Historical Events/Battles"), None, "Final battle sounds"),
    ("Temporal Events/Historical Events/Battles/Legendary Battle", Some("Temporal Events/Historical Events/Battles"), None, "Legendary battle sounds"),
    
    // === TEMPORAL EVENTS - HISTORICAL EVENTS - DISASTERS ===
    ("Temporal Events/Historical Events/Disasters/Great Earthquake", Some("Temporal Events/Historical Events/Disasters"), None, "Great earthquake sounds"),
    ("Temporal Events/Historical Events/Disasters/Great Flood", Some("Temporal Events/Historical Events/Disasters"), None, "Great flood sounds"),
    ("Temporal Events/Historical Events/Disasters/Great Plague", Some("Temporal Events/Historical Events/Disasters"), None, "Great plague sounds"),
    ("Temporal Events/Historical Events/Disasters/Volcanic Eruption", Some("Temporal Events/Historical Events/Disasters"), None, "Volcanic eruption sounds"),
    
    // === TEMPORAL EVENTS - SEASONS - AUTUMN ===
    ("Temporal Events/Seasons/Autumn/Early Autumn", Some("Temporal Events/Seasons/Autumn"), None, "Early autumn sounds"),
    ("Temporal Events/Seasons/Autumn/Full Autumn", Some("Temporal Events/Seasons/Autumn"), None, "Full autumn sounds"),
    ("Temporal Events/Seasons/Autumn/Harvest Time", Some("Temporal Events/Seasons/Autumn"), None, "Harvest time sounds"),
    ("Temporal Events/Seasons/Autumn/Late Autumn", Some("Temporal Events/Seasons/Autumn"), None, "Late autumn sounds"),
    
    // === TEMPORAL EVENTS - SEASONS - SPRING ===
    ("Temporal Events/Seasons/Spring/Early Spring", Some("Temporal Events/Seasons/Spring"), None, "Early spring sounds"),
    ("Temporal Events/Seasons/Spring/Full Spring", Some("Temporal Events/Seasons/Spring"), None, "Full spring sounds"),
    ("Temporal Events/Seasons/Spring/Late Spring", Some("Temporal Events/Seasons/Spring"), None, "Late spring sounds"),
    ("Temporal Events/Seasons/Spring/Spring Rain", Some("Temporal Events/Seasons/Spring"), None, "Spring rain sounds"),
    
    // === TEMPORAL EVENTS - SEASONS - SUMMER ===
    ("Temporal Events/Seasons/Summer/Early Summer", Some("Temporal Events/Seasons/Summer"), None, "Early summer sounds"),
    ("Temporal Events/Seasons/Summer/High Summer", Some("Temporal Events/Seasons/Summer"), None, "High summer sounds"),
    ("Temporal Events/Seasons/Summer/Late Summer", Some("Temporal Events/Seasons/Summer"), None, "Late summer sounds"),
    ("Temporal Events/Seasons/Summer/Summer Storm", Some("Temporal Events/Seasons/Summer"), None, "Summer storm sounds"),
    
    // === TEMPORAL EVENTS - SEASONS - WINTER ===
    ("Temporal Events/Seasons/Winter/Deep Winter", Some("Temporal Events/Seasons/Winter"), None, "Deep winter sounds"),
    ("Temporal Events/Seasons/Winter/Early Winter", Some("Temporal Events/Seasons/Winter"), None, "Early winter sounds"),
    ("Temporal Events/Seasons/Winter/Late Winter", Some("Temporal Events/Seasons/Winter"), None, "Late winter sounds"),
    ("Temporal Events/Seasons/Winter/Winter Solstice", Some("Temporal Events/Seasons/Winter"), None, "Winter solstice sounds"),
    
    // === TEMPORAL EVENTS - TIME OF DAY - AFTERNOON ===
    ("Temporal Events/Time of Day/Afternoon/Early Afternoon", Some("Temporal Events/Time of Day/Afternoon"), None, "Early afternoon sounds"),
    ("Temporal Events/Time of Day/Afternoon/Evening Approach", Some("Temporal Events/Time of Day/Afternoon"), None, "Evening approach sounds"),
    ("Temporal Events/Time of Day/Afternoon/Late Afternoon", Some("Temporal Events/Time of Day/Afternoon"), None, "Late afternoon sounds"),
    ("Temporal Events/Time of Day/Afternoon/Midday", Some("Temporal Events/Time of Day/Afternoon"), None, "Midday sounds"),
    
    // === TEMPORAL EVENTS - TIME OF DAY - EVENING ===
    ("Temporal Events/Time of Day/Evening/Dusk", Some("Temporal Events/Time of Day/Evening"), None, "Dusk sounds"),
    ("Temporal Events/Time of Day/Evening/Early Evening", Some("Temporal Events/Time of Day/Evening"), None, "Early evening sounds"),
    ("Temporal Events/Time of Day/Evening/Late Evening", Some("Temporal Events/Time of Day/Evening"), None, "Late evening sounds"),
    ("Temporal Events/Time of Day/Evening/Sunset", Some("Temporal Events/Time of Day/Evening"), None, "Sunset sounds"),
    
    // === TEMPORAL EVENTS - TIME OF DAY - MORNING ===
    ("Temporal Events/Time of Day/Morning/Dawn", Some("Temporal Events/Time of Day/Morning"), None, "Dawn sounds"),
    ("Temporal Events/Time of Day/Morning/Early Morning", Some("Temporal Events/Time of Day/Morning"), None, "Early morning sounds"),
    ("Temporal Events/Time of Day/Morning/Late Morning", Some("Temporal Events/Time of Day/Morning"), None, "Late morning sounds"),
    ("Temporal Events/Time of Day/Morning/Mid Morning", Some("Temporal Events/Time of Day/Morning"), None, "Mid morning sounds"),
    
    // === TEMPORAL EVENTS - TIME OF DAY - NIGHT ===
    ("Temporal Events/Time of Day/Night/Deep Night", Some("Temporal Events/Time of Day/Night"), None, "Deep night sounds"),
    ("Temporal Events/Time of Day/Night/Midnight", Some("Temporal Events/Time of Day/Night"), None, "Midnight sounds"),
    ("Temporal Events/Time of Day/Night/Nightfall", Some("Temporal Events/Time of Day/Night"), None, "Nightfall sounds"),
    ("Temporal Events/Time of Day/Night/Pre-Dawn", Some("Temporal Events/Time of Day/Night"), None, "Pre-dawn sounds"),
]