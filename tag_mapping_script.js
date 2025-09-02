const fs = require('fs');
const path = require('path');
const NodeID3 = require('node-id3');

// Master vocabulary from Ligeia vocabulary files
const VOCABULARY = {
    genre: [
        'post-metal', 'blues', 'lounge', 'dieselpunk', 'steampunk', 'atompunk', 'solarpunk', 'post-apocalyptic', 'western', 'mystery-noir', 'modern-urban', 'superhero',
        'ambient:dark-ambient', 'ambient:space-ambient', 'ambient:nature-ambient', 'ambient:ritual', 'ambient:drone', 'ambient:textural', 'ambient:new-age', 'ambient:lofi-ambient',
        'diegetic:tavern-band', 'diegetic:radio', 'diegetic:gramophone', 'diegetic:street-musician',
        'electronic:cyberpunk', 'electronic:idm', 'electronic:glitch', 'electronic:industrial', 'electronic:ebm', 'electronic:techno', 'electronic:trance', 'electronic:dnb', 'electronic:downtempo', 'electronic:shoegaze-electronic',
        'fantasy:high-fantasy', 'fantasy:grimdark', 'fantasy:fairy',
        'folk:celtic', 'folk:nordic', 'folk:middle-eastern', 'folk:mediterranean', 'folk:asian-east', 'folk:asian-south', 'folk:african', 'folk:andino', 'folk:balkan', 'folk:sea-shanty', 'folk:wild-west-folk',
        'historical:baroque', 'historical:renaissance', 'historical:medieval', 'historical:romantic',
        'horror:atonal', 'horror:dissonant-strings', 'horror:sound-design', 'horror:psychological', 'horror:jump-scare', 'horror:ritual', 'horror:cosmic', 'horror:gothic',
        'jazz:noir', 'jazz:swing', 'jazz:cool', 'jazz:latin', 'jazz:bebop',
        'metal:power', 'metal:symphonic', 'metal:black', 'metal:doom', 'metal:folk-metal', 'metal:industrial-metal',
        'mythic:norse', 'mythic:greco-roman', 'mythic:egyptian', 'mythic:celtic', 'mythic:japanese', 'mythic:mesoamerican',
        'orchestral:cinematic', 'orchestral:hybrid', 'orchestral:heroic', 'orchestral:dark', 'orchestral:minimal', 'orchestral:romantic', 'orchestral:baroque', 'orchestral:renaissance', 'orchestral:medieval', 'orchestral:percussive',
        'rock:post-rock', 'rock:gothic-rock', 'rock:progressive',
        'sci-fi:space-opera', 'sci-fi:hard-sci-fi', 'sci-fi:cyberpunk', 'sci-fi:biopunk', 'sci-fi:post-human',
        'sound-design:risers', 'sound-design:impacts', 'sound-design:whooshes', 'sound-design:stingers', 'sound-design:booms', 'sound-design:weapons', 'sound-design:movement', 'sound-design:objects', 'sound-design:voice', 'sound-design:magic'
    ],
    
    mood: [
        'heroic', 'triumphant', 'noble', 'uplifting', 'hopeful', 'inspiring', 'adventurous', 'confident', 'victorious',
        'festive', 'playful', 'whimsical', 'merry', 'lighthearted',
        'tender', 'romantic', 'serene', 'pastoral', 'warm', 'comforting', 'nostalgic', 'bittersweet',
        'mysterious', 'enigmatic', 'curious', 'contemplative', 'dreamlike', 'ethereal', 'mythic', 'arcane', 'otherworldly',
        'solemn', 'ceremonial', 'sacred', 'ritualistic', 'austere', 'stoic',
        'ominous', 'foreboding', 'tense', 'suspenseful', 'uneasy', 'eerie', 'creepy', 'unsettling', 'menacing', 'sinister', 'gothic',
        'dread', 'grim', 'bleak', 'oppressive', 'claustrophobic', 'tragic', 'melancholic', 'sorrowful', 'desolate', 'lonely', 'fatalistic', 'nihilistic',
        'driving', 'relentless', 'frenetic', 'furious', 'aggressive', 'percussive', 'charged', 'urgent', 'high-stakes', 'chaotic', 'volatile', 'explosive',
        'brooding-intensity', 'building', 'rising-tension', 'calm-before-storm',
        'airy', 'hazy', 'glacial', 'glitchy', 'noisy', 'grainy', 'organic', 'mechanical', 'rusted', 'industrial', 'neon', 'digital-cold', 'bio-organic', 'wet', 'dry',
        'eldritch', 'uncanny', 'body-horror', 'liminal', 'dissonant', 'abhorrent', 'ritual-fear', 'cosmic-dread'
    ],
    
    occasion: [
        'session-start', 'recap', 'table-chatter', 'break', 'session-end', 'credits', 'level-up', 'quest-complete', 'achievement', 'loot-found', 'character-death', 'epilogue', 'flashback', 'montage',
        'overworld-travel', 'wilderness-exploration', 'urban-exploration', 'dungeon-crawl', 'ruin-delving', 'underdark-journey', 'sewers', 'cave-exploration', 'mountain-pass', 'desert-crossing', 'jungle-trek', 'swamp-march', 'arctic-trek', 'sea-voyage', 'river-journey', 'airship-voyage', 'space-cruise', 'hyperspace-transit', 'derelict-ship-exploration', 'space-station-walk',
        'tavern', 'inn', 'market', 'black-market', 'noble-court', 'audience-with-ruler', 'council-debate', 'negotiation', 'interrogation', 'trial', 'festival', 'wedding', 'funeral', 'ceremony', 'religious-service', 'gambling-den', 'speakeasy', 'noir-club', 'tea-house', 'conversation',
        'crowd-celebration', 'crowd-angry', 'crowd-market', 'crowd-religious', 'crowd-funeral', 'crowd-panic',
        'entertainment', 'bard-performance', 'theater', 'gambling', 'sports', 'street-performance',
        'guild-meeting', 'council-session', 'court-proceeding', 'academic-discourse', 'military-order', 'trade-negotiation',
        'crime-scene', 'library-research', 'occult-research', 'stakeout', 'tailing', 'surveillance', 'infiltration', 'lockpicking', 'safecracking', 'hacking', 'netrun', 'vault-breach', 'disguise', 'escape', 'extraction', 'clean-getaway',
        'riddle-solving', 'mechanism-puzzle', 'arcane-puzzle', 'trap-primed', 'trap-triggered',
        'chase-timer', 'bomb-timer', 'reactor-meltdown', 'airlock-timer',
        'combat-ambush', 'combat-skirmish', 'combat-duel', 'combat-horde', 'combat-siege', 'combat-naval', 'combat-aerial', 'combat-vehicular', 'combat-mecha', 'combat-space-battle',
        'boss-intro', 'boss-loop', 'boss-final-phase', 'victory-fanfare', 'defeat-lament',
        'chase', 'car-chase', 'foot-chase', 'dogfight', 'boarding-action',
        'battle-ambience', 'pre-battle', 'climax-combat', 'victory', 'defeat', 'aftermath',
        'armor-defense', 'plate-armor', 'chain-mail', 'leather-armor', 'shields', 'magical-protection', 'breaking-armor',
        'monster-combat', 'dragon-fight', 'undead-combat', 'beast-battle', 'demon-fight', 'giant-combat', 'swarm-attack',
        'haunting', 'possession', 'ritual', 'summoning', 'banishment', 'eldritch-reveal', 'sanity-slip', 'ghost-encounter', 'vampire-lair', 'werewolf-hunt', 'zombie-siege', 'cult-gathering', 'sacrificial-altar',
        'spellcasting-prep', 'battle-magic', 'divination', 'telepathy', 'dream-walk', 'astral-travel', 'time-warp', 'portal-crossing', 'teleportation',
        'lab-experiment', 'biotech-lab', 'nanotech-swarm', 'ai-core', 'cyberdeck-dive', 'cyber-combat', 'drone-control', 'mech-dock', 'warp-jump', 'tractor-beam', 'ship-docking', 'eva-walk',
        'elemental-magic', 'fire-magic', 'ice-magic', 'lightning-magic', 'earth-magic', 'water-magic', 'air-magic', 'healing-magic', 'necromancy', 'illusion-magic', 'enchantment', 'transmutation',
        'magical-creatures', 'dragons', 'fae', 'demons', 'angels', 'spirits', 'elementals',
        'magical-environments', 'ley-lines', 'portals', 'magical-laboratories', 'sacred-groves', 'cursed-lands', 'magical-storms',
        'artifacts', 'enchanted-weapons', 'spell-components', 'magical-books', 'crystals', 'potions', 'talismans',
        'rituals', 'binding', 'transformation', 'communication-magic', 'protection-magic',
        'campfire', 'short-rest', 'long-rest', 'foraging', 'hunting', 'tracking', 'blacksmithing', 'fletching', 'alchemy', 'enchanting', 'cooking', 'base-building', 'training', 'shopping', 'bargain', 'healing', 'hospital-ward',
        'sunrise', 'sunset', 'night-watch', 'storm', 'rain', 'blizzard', 'sandstorm', 'earthquake', 'flood', 'eclipse', 'meteor-shower', 'radiation-storm', 'anomaly-event', 'void-rift', 'volcanic-eruption',
        'natural-landscapes', 'forests', 'mountains', 'deserts', 'oceans', 'rivers', 'caves',
        'weather', 'storms', 'snow', 'wind', 'fog', 'heat',
        'settlements', 'taverns', 'markets', 'cities', 'villages', 'temples', 'castles',
        'dungeons-ruins', 'ancient-ruins', 'tombs', 'mines', 'crypts', 'forgotten-places',
        'magical-realms', 'fairy-realms', 'elemental-planes', 'astral-plane', 'shadow-realm', 'divine-realms', 'void',
        'time-seasons', 'dawn', 'day', 'dusk', 'night', 'spring', 'summer', 'autumn', 'winter',
        'scene-transition', 'reveal-stinger', 'jump-scare', 'mystery-sting', 'map-open', 'map-close', 'quest-accepted', 'quest-failed', 'dice-roll', 'success-cue', 'failure-cue'
    ],
    
    keywords: [
        // Biomes & Terrain
        'biome:forest', 'biome:ancient-forest', 'biome:rainforest', 'biome:swamp', 'biome:bog', 'biome:marsh', 'biome:desert', 'biome:dunes', 'biome:oasis', 'biome:arctic', 'biome:tundra', 'biome:glacier', 'biome:mountain', 'biome:canyon', 'biome:steppe', 'biome:plains', 'biome:grassland', 'biome:jungle', 'biome:savanna', 'biome:volcanic', 'biome:underdark', 'biome:cave', 'biome:sewers', 'biome:coast', 'biome:open-sea', 'biome:river', 'biome:lake', 'biome:sky', 'biome:astral', 'biome:void', 'biome:otherworld',
        
        // Locations & Structures  
        'loc:castle', 'loc:keep', 'loc:fortress', 'loc:watchtower', 'loc:dungeon', 'loc:catacombs', 'loc:crypt', 'loc:temple', 'loc:shrine', 'loc:monastery', 'loc:library', 'loc:academy', 'loc:mage-tower', 'loc:throne-room', 'loc:market', 'loc:slums', 'loc:harbor', 'loc:mine', 'loc:smithy', 'loc:inn', 'loc:tavern', 'loc:prison', 'loc:arena', 'loc:laboratory', 'loc:biolab', 'loc:reactor', 'loc:spaceport', 'loc:hangar', 'loc:bridge-deck', 'loc:engineering', 'loc:cryosleep-bay', 'loc:cargo-bay', 'loc:derelict', 'loc:ruins', 'loc:ancient-city', 'loc:fairy-glen',
        
        // Cultures, Eras & Styles
        'style:medieval-european', 'style:renaissance', 'style:baroque', 'style:romantic-era', 'style:ancient-greek', 'style:ancient-roman', 'style:egyptian', 'style:norse', 'style:celtic', 'style:arabesque', 'style:persian', 'style:ottoman', 'style:indian-classical', 'style:japanese-traditional', 'style:chinese-traditional', 'style:korean-traditional', 'style:balinese-gamelan', 'style:andino', 'style:west-african', 'style:native-north-american', 'style:mexican-folk', 'style:iberian-folk', 'style:byzantine-chant', 'style:gothic', 'style:western-frontier', 'style:1920s', 'style:noir-1940s', 'style:cold-war', 'style:cyberpunk-neon', 'style:dieselpunk', 'style:steampunk', 'style:post-apocalyptic',
        
        // Creatures & Foes
        'creature:dragon', 'creature:drake', 'creature:wyvern', 'creature:giant', 'creature:troll', 'creature:ogre', 'creature:goblin', 'creature:orc', 'creature:kobold', 'creature:gnoll', 'creature:undead', 'creature:zombie', 'creature:ghoul', 'creature:skeleton', 'creature:lich', 'creature:vampire', 'creature:werewolf', 'creature:ghost', 'creature:demon', 'creature:devil', 'creature:angel', 'creature:fae', 'creature:dryad', 'creature:elemental', 'creature:construct', 'creature:golem', 'creature:slime', 'creature:beast', 'creature:wolf', 'creature:bear', 'creature:spider', 'creature:kraken', 'creature:siren', 'creature:aberration', 'creature:eldritch-entity', 'creature:alien', 'creature:parasite', 'creature:machine-swarm',
        
        // Factions & NPC Types
        'faction:empire', 'faction:rebels', 'faction:mercenary-band', 'faction:thieves-guild', 'faction:mages-guild', 'faction:knightly-order', 'faction:church', 'faction:cult', 'faction:cartel', 'faction:megacorp', 'faction:syndicate', 'faction:raiders', 'faction:mutants', 'faction:androids',
        'npc:noble', 'npc:merchant', 'npc:smuggler', 'npc:guard', 'npc:assassin', 'npc:priest', 'npc:witch', 'npc:warlock', 'npc:necromancer', 'npc:ranger', 'npc:alchemist', 'npc:scientist', 'npc:hacker', 'npc:pilot', 'npc:ai',
        
        // Magic, Powers & Elements
        'magic:abjuration', 'magic:conjuration', 'magic:divination', 'magic:enchantment', 'magic:evocation', 'magic:illusion', 'magic:necromancy', 'magic:transmutation',
        'element:fire', 'element:ice', 'element:lightning', 'element:wind', 'element:earth', 'element:water', 'element:poison', 'element:acid', 'element:shadow', 'element:light', 'element:void', 'element:metal', 'element:wood',
        'ritual:blood-magic', 'ritual:summoning', 'ritual:sacrifice', 'ritual:binding', 'ritual:banishment',
        
        // Technology & Vehicles
        'tech:medieval', 'tech:clockwork', 'tech:steam', 'tech:diesel', 'tech:nuclear', 'tech:fusion', 'tech:antimatter', 'tech:biotech', 'tech:nanotech', 'tech:ai', 'tech:cybernetics', 'tech:synthetic',
        'vehicle:horse', 'vehicle:carriage', 'vehicle:war-wagon', 'vehicle:train', 'vehicle:subway', 'vehicle:motorbike', 'vehicle:armored-vehicle', 'vehicle:airship', 'vehicle:zeppelin', 'vehicle:airplane', 'vehicle:helicopter', 'vehicle:mech', 'vehicle:tank', 'vehicle:starfighter', 'vehicle:shuttle', 'vehicle:frigate', 'vehicle:battleship', 'vehicle:freighter',
        
        // Weather & Natural Phenomena
        'weather:clear', 'weather:wind', 'weather:rain', 'weather:thunderstorm', 'weather:snow', 'weather:blizzard', 'weather:heatwave', 'weather:sandstorm', 'weather:fog', 'weather:mist', 'weather:hail', 'weather:aurora', 'weather:eclipse', 'weather:meteor-shower', 'weather:solar-storm', 'weather:radiation-storm', 'weather:anomaly',
        
        // Instruments & Timbres
        'timbre:strings-warm', 'timbre:strings-dissonant', 'timbre:low-brass', 'timbre:braams', 'timbre:solo-violin', 'timbre:solo-cello', 'timbre:harp', 'timbre:flute', 'timbre:whistle', 'timbre:bagpipes', 'timbre:nyckelharpa', 'timbre:hurdy-gurdy', 'timbre:oud', 'timbre:sitar', 'timbre:shakuhachi', 'timbre:erhu', 'timbre:guzheng', 'timbre:koto', 'timbre:gamelan', 'timbre:frame-drum', 'timbre:taiko', 'timbre:bodhran', 'timbre:dulcimer', 'timbre:lute', 'timbre:organ', 'timbre:church-choir', 'timbre:male-chant', 'timbre:female-vocalise', 'timbre:child-choir', 'timbre:synthetic-pad', 'timbre:analog-synth', 'timbre:fm-synth', 'timbre:granular', 'timbre:noise-texture', 'timbre:clockwork', 'timbre:metallic-hits', 'timbre:sub-boom',
        
        // SFX & Foley
        'sfx:sword-clash', 'sfx:bow-release', 'sfx:gunshot', 'sfx:reload', 'sfx:ricochet', 'sfx:melee-impact', 'sfx:weapon-draw', 'sfx:weapon-sheath', 'sfx:footsteps', 'sfx:armor-clank', 'sfx:cloth-rustle', 'sfx:creature-movement', 'sfx:vehicle-movement', 'sfx:door-creak', 'sfx:gate-bang', 'sfx:chains', 'sfx:coins', 'sfx:pages-turning', 'sfx:quill-scratch', 'sfx:container-open', 'sfx:tool-use', 'sfx:machinery', 'sfx:chant', 'sfx:chant-latin', 'sfx:crowd-voices', 'sfx:emotional-vocals', 'sfx:spell-casting-voice', 'sfx:breathing', 'sfx:whispers', 'sfx:explosion', 'sfx:metal-impact', 'sfx:wood-impact', 'sfx:stone-impact', 'sfx:glass-impact', 'sfx:body-impact', 'sfx:campfire', 'sfx:wood-crackle', 'sfx:water-drip', 'sfx:river', 'sfx:ocean-surf', 'sfx:thunder', 'sfx:wind', 'sfx:rain', 'sfx:market-crowd', 'sfx:tavern-murmur', 'sfx:church-bells', 'sfx:magic-whoosh', 'sfx:spell-impact', 'sfx:shield', 'sfx:portal-open', 'sfx:portal-close', 'sfx:teleport', 'sfx:transformation', 'sfx:energy-blast', 'sfx:healing-magic', 'sfx:curse', 'sfx:monster-roar', 'sfx:dragon-breath', 'sfx:zombie-moans', 'sfx:ghost-wail', 'sfx:beast-sounds', 'sfx:space-engine-hum', 'sfx:alarm', 'sfx:scanner-beeps', 'sfx:keyboard', 'sfx:hacking-glitches', 'sfx:warp',
        
        // Gamey / UI-ish  
        'ui:ping', 'ui:notify', 'ui:confirm', 'ui:error', 'ui:upgrade', 'ui:inventory', 'ui:map-open', 'ui:map-close', 'ui:crafting-complete', 'ui:purchase', 'ui:sell', 'ui:quest-update', 'ui:skill-point', 'ui:dice-roll',
        
        // Audio Structure & Utility
        'util:loopable', 'util:stinger', 'util:intro', 'util:outro', 'util:transition', 'util:bed', 'util:drone', 'util:motif', 'util:theme', 'util:alt-mix', 'util:instrumental', 'util:with-vocals', 'util:diegetic', 'util:non-diegetic', 'util:stem-percussion', 'util:stem-ambient', 'util:stem-melody'
    ]
};

// Dictionary tags from Dictionary.js (key-value pairs)
const DICTIONARY_TAGS = {
    "477": ["ocean", "sea", "water", "storm", "sirens", "shipwreck", "ship", "boat", "tension", "survivor"],
    "476": ["dogs", "wolf", "wolves", "omen", "spirits", "hounds", "ghosts", "field", "fjell"],
    "475": ["facility", "outpost", "base", "station", "ship", "tension", "suspense", "mysterious"],
    "474": ["peacful", "dragons", "city", "forest", "animals", "exploration", "rain"],
    // ... (add more as needed - this is a sample for the mapping logic)
};

// Function to create intelligent mappings from dictionary tags to vocabulary tags
function mapDictionaryToVocabulary(dictionaryTags) {
    const mappings = {};
    
    // Comprehensive mapping rules
    const MAPPING_RULES = {
        // Weather and elements
        'storm': ['weather:thunderstorm', 'mood:ominous'],
        'rain': ['weather:rain', 'mood:melancholic'], 
        'snow': ['weather:snow', 'biome:arctic'],
        'ice': ['weather:snow', 'element:ice', 'biome:glacier'],
        'wind': ['weather:wind', 'mood:airy'],
        'thunder': ['weather:thunderstorm', 'mood:ominous'],
        'blizzard': ['weather:blizzard', 'mood:harsh'],
        'fog': ['weather:fog', 'mood:mysterious'],
        
        // Water environments  
        'ocean': ['biome:open-sea', 'occasion:sea-voyage'],
        'sea': ['biome:open-sea', 'occasion:sea-voyage'],
        'water': ['biome:river', 'element:water'],
        'underwater': ['biome:open-sea', 'occasion:sea-voyage'],
        'river': ['biome:river', 'occasion:river-journey'], 
        'swamp': ['biome:swamp', 'occasion:swamp-march'],
        
        // Land environments
        'forest': ['biome:forest', 'occasion:wilderness-exploration'],
        'woods': ['biome:forest', 'occasion:wilderness-exploration'],
        'jungle': ['biome:jungle', 'occasion:jungle-trek'],
        'desert': ['biome:desert', 'occasion:desert-crossing'],
        'mountain': ['biome:mountain', 'occasion:mountain-pass'],
        'cave': ['biome:cave', 'occasion:cave-exploration'],
        'arctic': ['biome:arctic', 'occasion:arctic-trek'],
        'tundra': ['biome:tundra', 'occasion:arctic-trek'],
        
        // Locations
        'castle': ['loc:castle', 'occasion:noble-court'],
        'tavern': ['loc:tavern', 'occasion:tavern'],
        'inn': ['loc:inn', 'occasion:inn'],
        'temple': ['loc:temple', 'occasion:religious-service'],
        'church': ['loc:temple', 'occasion:religious-service'],
        'dungeon': ['loc:dungeon', 'occasion:dungeon-crawl'],
        'city': ['occasion:urban-exploration', 'mood:urban'],
        'town': ['occasion:urban-exploration', 'biome:plains'],
        'village': ['occasion:urban-exploration', 'mood:pastoral'],
        'market': ['loc:market', 'occasion:market'],
        'laboratory': ['loc:laboratory', 'occasion:lab-experiment'],
        'prison': ['loc:prison', 'mood:oppressive'],
        'jail': ['loc:prison', 'mood:oppressive'],
        'crypt': ['loc:crypt', 'mood:eerie'],
        'tomb': ['loc:crypt', 'mood:eerie'],
        'mine': ['loc:mine', 'occasion:mining'],
        
        // Creatures
        'dragon': ['creature:dragon', 'occasion:dragon-fight'],
        'dragons': ['creature:dragon', 'occasion:dragon-fight'],
        'wolf': ['creature:wolf', 'occasion:beast-battle'],
        'wolves': ['creature:wolf', 'occasion:beast-battle'],
        'undead': ['creature:undead', 'occasion:undead-combat'],
        'vampire': ['creature:vampire', 'mood:gothic'],
        'ghost': ['creature:ghost', 'mood:eerie'],
        'ghosts': ['creature:ghost', 'mood:eerie'],
        'spirits': ['creature:ghost', 'mood:ethereal'],
        'demon': ['creature:demon', 'occasion:demon-fight'],
        'demons': ['creature:demon', 'occasion:demon-fight'],
        'monster': ['occasion:monster-combat', 'mood:menacing'],
        'monsters': ['occasion:monster-combat', 'mood:menacing'],
        'alien': ['creature:alien', 'sci-fi:hard-sci-fi'],
        'aliens': ['creature:alien', 'sci-fi:hard-sci-fi'],
        'spider': ['creature:spider', 'mood:creepy'],
        'spiders': ['creature:spider', 'mood:creepy'],
        'giant': ['creature:giant', 'occasion:giant-combat'],
        'troll': ['creature:troll', 'occasion:giant-combat'],
        'skeleton': ['creature:skeleton', 'occasion:undead-combat'],
        'zombie': ['creature:zombie', 'occasion:zombie-siege'],
        'zombies': ['creature:zombie', 'occasion:zombie-siege'],
        
        // Combat and action
        'battle': ['occasion:climax-combat', 'mood:aggressive'],
        'combat': ['occasion:climax-combat', 'mood:aggressive'],
        'fight': ['occasion:climax-combat', 'mood:aggressive'],
        'war': ['occasion:climax-combat', 'mood:aggressive'],
        'chase': ['occasion:chase', 'mood:urgent'],
        'action': ['mood:driving', 'mood:urgent'],
        'tension': ['mood:tense', 'mood:suspenseful'],
        'suspense': ['mood:suspenseful', 'mood:uneasy'],
        'boss': ['occasion:boss-loop', 'mood:menacing'],
        'victory': ['occasion:victory', 'mood:triumphant'],
        'defeat': ['occasion:defeat', 'mood:tragic'],
        
        // Magic and fantasy
        'magic': ['occasion:battle-magic', 'fantasy:high-fantasy'],
        'magical': ['occasion:battle-magic', 'fantasy:high-fantasy'],
        'spell': ['occasion:spellcasting-prep', 'fantasy:high-fantasy'],
        'spells': ['occasion:spellcasting-prep', 'fantasy:high-fantasy'],
        'wizard': ['npc:mage', 'fantasy:high-fantasy'],
        'wizards': ['npc:mage', 'fantasy:high-fantasy'],
        'ritual': ['occasion:ritual', 'mood:ritualistic'],
        'summoning': ['occasion:summoning', 'ritual:summoning'],
        'necromancy': ['occasion:necromancy', 'magic:necromancy'],
        'fire': ['element:fire', 'occasion:fire-magic'],
        'lava': ['element:fire', 'biome:volcanic'],
        'lightning': ['element:lightning', 'occasion:lightning-magic'],
        'shadow': ['element:shadow', 'mood:dark'],
        'dark': ['mood:grim', 'mood:ominous'],
        'light': ['element:light', 'mood:hopeful'],
        
        // Technology and sci-fi
        'space': ['sci-fi:space-opera', 'occasion:space-cruise'],
        'spaceship': ['sci-fi:space-opera', 'occasion:space-cruise'],
        'spacestation': ['sci-fi:space-opera', 'occasion:space-station-walk'],
        'cyberpunk': ['sci-fi:cyberpunk', 'electronic:cyberpunk'],
        'robot': ['tech:ai', 'creature:construct'],
        'robots': ['tech:ai', 'creature:construct'],
        'android': ['tech:ai', 'creature:construct'],
        'mechanical': ['tech:clockwork', 'mood:mechanical'],
        'steampunk': ['steampunk', 'tech:steam'],
        'future': ['sci-fi:hard-sci-fi', 'mood:neon'],
        'alien': ['creature:alien', 'sci-fi:hard-sci-fi'],
        'mech': ['vehicle:mech', 'tech:ai'],
        'mechs': ['vehicle:mech', 'tech:ai'],
        'technology': ['tech:ai', 'sci-fi:hard-sci-fi'],
        
        // Social and cultural
        'medieval': ['historical:medieval', 'style:medieval-european'],
        'victorian': ['historical:romantic', 'style:romantic-era'],
        'asian': ['style:japanese-traditional', 'folk:asian-east'],
        'japanese': ['style:japanese-traditional', 'mythic:japanese'],
        'chinese': ['style:chinese-traditional', 'folk:asian-east'],
        'celtic': ['folk:celtic', 'mythic:celtic'],
        'norse': ['folk:nordic', 'mythic:norse'],
        'nordic': ['folk:nordic', 'mythic:norse'],
        'egyptian': ['style:egyptian', 'mythic:egyptian'],
        'greek': ['style:ancient-greek', 'mythic:greco-roman'],
        'roman': ['style:ancient-roman', 'mythic:greco-roman'],
        'western': ['western', 'folk:wild-west-folk'],
        'arabian': ['style:arabesque', 'folk:middle-eastern'],
        'tribal': ['folk:african', 'style:west-african'],
        '1920s': ['style:1920s', 'jazz:swing'],
        '1930s': ['style:1920s', 'jazz:swing'],
        '1940s': ['style:noir-1940s', 'jazz:noir'],
        
        // Music and instruments  
        'jazz': ['jazz:swing', 'mood:lighthearted'],
        'drums': ['timbre:frame-drum', 'mood:percussive'],
        'chant': ['timbre:male-chant', 'mood:ceremonial'],
        'choir': ['timbre:church-choir', 'mood:sacred'],
        'orchestra': ['orchestral:cinematic', 'mood:noble'],
        'harp': ['timbre:harp', 'mood:ethereal'],
        'organ': ['timbre:organ', 'mood:sacred'],
        'synth': ['electronic:idm', 'timbre:analog-synth'],
        'synthesizer': ['electronic:idm', 'timbre:analog-synth'],
        'bagpipes': ['timbre:bagpipes', 'folk:celtic'],
        
        // Moods and atmospheres
        'peaceful': ['mood:serene', 'mood:calm'],
        'calm': ['mood:serene', 'mood:contemplative'],
        'quiet': ['mood:contemplative', 'mood:serene'],
        'mysterious': ['mood:mysterious', 'mood:enigmatic'],
        'scary': ['mood:menacing', 'horror:psychological'],
        'creepy': ['mood:creepy', 'horror:psychological'],
        'spooky': ['mood:eerie', 'horror:psychological'],
        'haunted': ['mood:eerie', 'occasion:haunting'],
        'eerie': ['mood:eerie', 'horror:psychological'],
        'ominous': ['mood:ominous', 'mood:foreboding'],
        'tense': ['mood:tense', 'mood:suspenseful'],
        'suspense': ['mood:suspenseful', 'mood:uneasy'],
        'ethereal': ['mood:ethereal', 'mood:otherworldly'],
        'heroic': ['mood:heroic', 'mood:noble'],
        'epic': ['mood:heroic', 'orchestral:heroic'],
        'dramatic': ['mood:building', 'orchestral:cinematic'],
        'tragedy': ['mood:tragic', 'mood:sorrowful'],
        'sad': ['mood:melancholic', 'mood:sorrowful'],
        'happy': ['mood:playful', 'mood:uplifting'],
        'festive': ['mood:festive', 'occasion:festival'],
        'celebration': ['mood:festive', 'occasion:crowd-celebration'],
        'party': ['mood:festive', 'occasion:festival'],
        'uplifting': ['mood:uplifting', 'mood:hopeful'],
        'romantic': ['mood:romantic', 'mood:tender'],
        'nostalgic': ['mood:nostalgic', 'mood:bittersweet'],
        'noble': ['mood:noble', 'mood:heroic'],
        'triumphant': ['mood:triumphant', 'mood:victorious'],
        'grim': ['mood:grim', 'mood:bleak'],
        'desolate': ['mood:desolate', 'mood:lonely'],
        'industrial': ['mood:industrial', 'electronic:industrial'],
        'cyberpunk': ['sci-fi:cyberpunk', 'mood:neon'],
        'gothic': ['mood:gothic', 'horror:gothic'],
        
        // Vehicles and transportation
        'ship': ['occasion:sea-voyage', 'vehicle:ship'],
        'ships': ['occasion:sea-voyage', 'vehicle:ship'],  
        'boat': ['occasion:sea-voyage', 'vehicle:ship'],
        'sailing': ['occasion:sea-voyage', 'folk:sea-shanty'],
        'train': ['vehicle:train', 'tech:steam'],
        'trains': ['vehicle:train', 'tech:steam'],
        'airship': ['vehicle:airship', 'steampunk'],
        'horse': ['vehicle:horse', 'occasion:overworld-travel'],
        'horses': ['vehicle:horse', 'occasion:overworld-travel'],
        'carriage': ['vehicle:carriage', 'occasion:overworld-travel'],
        
        // Activities and occasions
        'exploration': ['occasion:wilderness-exploration', 'mood:adventurous'],
        'travel': ['occasion:overworld-travel', 'mood:adventurous'], 
        'journey': ['occasion:overworld-travel', 'mood:adventurous'],
        'rest': ['occasion:short-rest', 'mood:comforting'],
        'campfire': ['occasion:campfire', 'sfx:campfire'],
        'cooking': ['occasion:cooking', 'mood:comforting'],
        'shopping': ['occasion:shopping', 'loc:market'],
        'research': ['occasion:library-research', 'mood:contemplative'],
        'study': ['occasion:library-research', 'mood:contemplative'],
        'prayer': ['occasion:religious-service', 'mood:sacred'],
        'meditation': ['occasion:religious-service', 'mood:contemplative'],
        'ceremony': ['occasion:ceremony', 'mood:ceremonial'],
        'festival': ['occasion:festival', 'mood:festive'],
        'wedding': ['occasion:wedding', 'mood:romantic'],
        'funeral': ['occasion:funeral', 'mood:solemn'],
        'gambling': ['occasion:gambling', 'mood:tense'],
        'stealth': ['occasion:infiltration', 'mood:tense'],
        'sneak': ['occasion:infiltration', 'mood:tense'],
        'heist': ['occasion:infiltration', 'mood:tense'],
        'escape': ['occasion:escape', 'mood:urgent'],
        'chase': ['occasion:chase', 'mood:urgent']
    };
    
    // Apply mapping rules to each dictionary entry
    for (const [trackNumber, tags] of Object.entries(dictionaryTags)) {
        const vocabularyTags = new Set();
        
        // Map each dictionary tag to vocabulary tags
        for (const tag of tags) {
            const lowerTag = tag.toLowerCase().trim();
            
            // Direct mapping from rules
            if (MAPPING_RULES[lowerTag]) {
                MAPPING_RULES[lowerTag].forEach(vocabTag => vocabularyTags.add(vocabTag));
            }
            
            // Fuzzy matching for partial word matches
            for (const [ruleKey, ruleValues] of Object.entries(MAPPING_RULES)) {
                if (lowerTag.includes(ruleKey) || ruleKey.includes(lowerTag)) {
                    ruleValues.forEach(vocabTag => vocabularyTags.add(vocabTag));
                }
            }
        }
        
        // Ensure we always have some tags - add default contextual tags
        if (vocabularyTags.size === 0) {
            vocabularyTags.add('util:loopable');
            vocabularyTags.add('ambient:textural');
        }
        
        mappings[trackNumber] = Array.from(vocabularyTags).slice(0, 8); // Limit to 8 tags per track
    }
    
    return mappings;
}

// Load Dictionary.js data and create mappings
function generateMappedJSON() {
    // This would normally load from Dictionary.js - for now using sample data
    const sampleDictionary = DICTIONARY_TAGS;
    
    // Generate mappings
    const mappings = mapDictionaryToVocabulary(sampleDictionary);
    
    // Write result to JSON file
    const outputPath = 'mapped_vocabulary_tags.json';
    fs.writeFileSync(outputPath, JSON.stringify(mappings, null, 2));
    
    console.log(`Generated ${outputPath} with vocabulary mappings for ${Object.keys(mappings).length} tracks`);
    console.log('Sample mapping:', JSON.stringify(mappings["477"], null, 2));
}

// Export for use
module.exports = { mapDictionaryToVocabulary, VOCABULARY, generateMappedJSON };

// Run if called directly
if (require.main === module) {
    generateMappedJSON();
}