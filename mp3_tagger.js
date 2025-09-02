const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const NodeID3 = require('node-id3');

// Configuration
const DICTIONARY_PATH = "i:\\Musica\\Ligeia\\Ambient Music & Effects\\Long loopable music 10 minutes\\Dictionary.js";
const MP3_BASE_PATH = "i:\\Musica\\Ligeia\\Ambient Music & Effects\\Long loopable music 10 minutes";

// Step 1: Load Dictionary.js
function loadDictionary() {
    console.log("Loading Dictionary.js...");
    try {
        const content = fs.readFileSync(DICTIONARY_PATH, 'utf8');
        console.log("File loaded, parsing...");
        
        // Use require to load the file properly
        const tempPath = path.join(__dirname, 'temp_dict.js');
        fs.writeFileSync(tempPath, content);
        delete require.cache[require.resolve('./temp_dict.js')];
        const { data } = require('./temp_dict.js');
        fs.unlinkSync(tempPath);
        
        console.log(`Loaded ${Object.keys(data).length} dictionary entries`);
        return data;
    } catch (error) {
        console.error("Error loading dictionary:", error.message);
        process.exit(1);
    }
}

// Step 2: Find MP3 file for a given number
function findMP3File(trackNumber) {
    try {
        const result = execSync(`find "${MP3_BASE_PATH}" -name "${trackNumber}-*.mp3" -o -name "${trackNumber}_*.mp3"`, { encoding: 'utf8' });
        const files = result.trim().split('\n').filter(f => f.length > 0);
        return files[0] || null;
    } catch (error) {
        return null;
    }
}

// Step 3: Map dictionary tags to vocabulary tags
function mapToVocabulary(dictionaryTags) {
    const vocabularyTags = {
        genre: [],
        mood: [],
        occasion: [],
        keywords: []
    };
    
    // Comprehensive mapping rules (from tag_mapping_script.js)
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
        'city': ['occasion:urban-exploration'],
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
    
    // Apply mapping rules (with fuzzy matching like tag_mapping_script.js)
    const vocabularyTagsSet = new Set();
    
    dictionaryTags.forEach(tag => {
        const lowerTag = tag.toLowerCase().trim();
        
        // Direct mapping from rules
        if (MAPPING_RULES[lowerTag]) {
            MAPPING_RULES[lowerTag].forEach(vocabTag => vocabularyTagsSet.add(vocabTag));
        }
        
        // Fuzzy matching for partial word matches
        for (const [ruleKey, ruleValues] of Object.entries(MAPPING_RULES)) {
            if (lowerTag.includes(ruleKey) || ruleKey.includes(lowerTag)) {
                ruleValues.forEach(vocabTag => vocabularyTagsSet.add(vocabTag));
            }
        }
    });
    
    // Categorize vocabulary tags into proper categories
    Array.from(vocabularyTagsSet).forEach(vocabTag => {
        // Keywords (prefixed categories)
        if (vocabTag.includes(':') && ['biome', 'loc', 'creature', 'element', 'tech', 'vehicle', 'weather', 'timbre', 'sfx', 'ui', 'util', 'faction', 'npc', 'magic', 'ritual', 'style'].some(prefix => vocabTag.startsWith(prefix + ':'))) {
            vocabularyTags.keywords.push(vocabTag);
        }
        // Moods (direct mood keywords or mood: prefixed)
        else if (vocabTag.startsWith('mood:')) {
            vocabularyTags.mood.push(vocabTag.replace('mood:', ''));
        }
        else if (['serene', 'calm', 'contemplative', 'mysterious', 'enigmatic', 'menacing', 'creepy', 'eerie', 'ominous', 'foreboding', 'tense', 'suspenseful', 'uneasy', 'ethereal', 'otherworldly', 'heroic', 'noble', 'triumphant', 'victorious', 'tragic', 'sorrowful', 'melancholic', 'playful', 'uplifting', 'festive', 'romantic', 'tender', 'nostalgic', 'bittersweet', 'grim', 'bleak', 'desolate', 'lonely', 'driving', 'urgent', 'aggressive', 'building', 'lighthearted', 'percussive', 'ceremonial', 'sacred', 'pastoral', 'adventurous', 'comforting', 'contemplative', 'ritualistic', 'hopeful', 'mechanical', 'neon', 'gothic', 'industrial'].includes(vocabTag)) {
            vocabularyTags.mood.push(vocabTag);
        }
        // Occasions (direct occasion keywords or occasion: prefixed)
        else if (vocabTag.startsWith('occasion:')) {
            vocabularyTags.occasion.push(vocabTag.replace('occasion:', ''));
        }
        else if (vocabTag.includes('-voyage') || vocabTag.includes('-exploration') || vocabTag.includes('-combat') || vocabTag.includes('-fight') || vocabTag.includes('-siege') || vocabTag.includes('-trek') || vocabTag.includes('-journey') || vocabTag.includes('-march') || vocabTag.includes('-crossing') || vocabTag.includes('-pass') || ['tavern', 'inn', 'market', 'noble-court', 'religious-service', 'dungeon-crawl', 'urban-exploration', 'lab-experiment', 'mining', 'sea-voyage', 'river-journey', 'wilderness-exploration', 'cave-exploration', 'desert-crossing', 'mountain-pass', 'jungle-trek', 'swamp-march', 'arctic-trek', 'space-cruise', 'space-station-walk', 'dragon-fight', 'beast-battle', 'undead-combat', 'demon-fight', 'giant-combat', 'zombie-siege', 'climax-combat', 'chase', 'boss-loop', 'victory', 'defeat', 'battle-magic', 'spellcasting-prep', 'summoning', 'necromancy', 'fire-magic', 'lightning-magic', 'ritual', 'haunting', 'monster-combat', 'infiltration', 'escape', 'short-rest', 'campfire', 'cooking', 'shopping', 'library-research', 'ceremony', 'festival', 'wedding', 'funeral', 'gambling', 'overworld-travel'].includes(vocabTag)) {
            vocabularyTags.occasion.push(vocabTag);
        }
        // Genres (everything else, including genre: prefixed and standalone genres)
        else {
            vocabularyTags.genre.push(vocabTag);
        }
    });
    
    // Add default tags if empty
    if (vocabularyTags.genre.length === 0) vocabularyTags.genre.push('ambient:textural');
    if (vocabularyTags.mood.length === 0) vocabularyTags.mood.push('mysterious');
    if (vocabularyTags.occasion.length === 0) vocabularyTags.occasion.push('wilderness-exploration');
    if (vocabularyTags.keywords.length === 0) vocabularyTags.keywords.push('util:loopable');
    
    return vocabularyTags;
}

// Step 4: Write tags to MP3 file
function writeTagsToMP3(mp3FilePath, vocabularyTags) {
    try {
        // First, read existing tags to preserve other metadata
        let existingTags = {};
        try {
            existingTags = NodeID3.read(mp3FilePath) || {};
        } catch (error) {
            console.log(`Warning: Could not read existing tags from ${path.basename(mp3FilePath)}`);
        }
        
        // Clear existing userDefinedText entries for our specific tags
        let filteredUserDefinedText = [];
        if (existingTags.userDefinedText && Array.isArray(existingTags.userDefinedText)) {
            filteredUserDefinedText = existingTags.userDefinedText.filter(tag => 
                !['Occasion', 'Keywords', 'Quality'].includes(tag.description)
            );
        }
        
        // According to TAGS.md, use these ID3v2.4 frames:
        // Genre → TCON + internal taxonomy  
        // Mood → TMOO or TXXX:Mood
        // Occasion → TXXX:Occasion
        // Keywords → TXXX:Keywords
        
        const tags = {
            // Clear and set genre (TCON frame)
            genre: vocabularyTags.genre.join('; '),
            
            // Set mood using TMOO frame (according to TAGS.md)
            mood: vocabularyTags.mood.join('; '),
            
            // Preserve existing user-defined text but add/replace our specific ones
            userDefinedText: [
                ...filteredUserDefinedText,  // Keep other existing user-defined tags
                {
                    description: 'Occasion', 
                    value: vocabularyTags.occasion.join('; ')
                },
                {
                    description: 'Keywords',
                    value: vocabularyTags.keywords.join('; ')
                },
                {
                    description: 'Quality',
                    value: 'High'
                }
            ]
        };
        
        const success = NodeID3.update(tags, mp3FilePath);
        if (success) {
            console.log(`✓ Tags written to ${path.basename(mp3FilePath)} (cleared existing genre/mood/occasion/keywords/quality)`);
            return true;
        } else {
            console.log(`✗ Failed to write tags to ${path.basename(mp3FilePath)}`);
            return false;
        }
    } catch (error) {
        console.error(`Error writing tags to ${path.basename(mp3FilePath)}:`, error.message);
        return false;
    }
}

// Test all four steps with a single file
function testSteps() {
    console.log("=== Testing Steps 1, 2, 3 & 4 ===");
    
    // Test Step 1
    const dictionary = loadDictionary();
    const firstKey = Object.keys(dictionary)[0];
    console.log(`First entry: ${firstKey} -> ${JSON.stringify(dictionary[firstKey])}`);
    
    // Test Step 2
    console.log(`\nTesting MP3 file finding for track ${firstKey}:`);
    const mp3File = findMP3File(firstKey);
    console.log(`Found: ${mp3File || 'NOT FOUND'}`);
    
    if (!mp3File) {
        console.log("Cannot proceed with tagging test - no MP3 file found");
        return;
    }
    
    // Test Step 3
    console.log(`\nTesting tag mapping for track ${firstKey}:`);
    const mappedTags = mapToVocabulary(dictionary[firstKey]);
    console.log(`Mapped tags:`, JSON.stringify(mappedTags, null, 2));
    
    // Test Step 4
    console.log(`\nTesting tag writing for track ${firstKey}:`);
    const writeSuccess = writeTagsToMP3(mp3File, mappedTags);
    
    if (writeSuccess) {
        // Verify the tags were written
        console.log(`\nVerifying written tags:`);
        try {
            const readTags = NodeID3.read(mp3File);
            console.log(`Genre: ${readTags.genre || 'Not found'}`);
            
            const userTags = readTags.userDefinedText || [];
            userTags.forEach(tag => {
                console.log(`${tag.description}: ${tag.value}`);
            });
        } catch (error) {
            console.log(`Error reading back tags: ${error.message}`);
        }
    }
}

// Step 5: Process all files
function processAllFiles() {
    console.log("=== Processing ALL MP3 Files ===");
    
    const dictionary = loadDictionary();
    const totalTracks = Object.keys(dictionary).length;
    let processedCount = 0;
    let successCount = 0;
    let skippedCount = 0;
    
    console.log(`\nProcessing ${totalTracks} tracks...`);
    
    for (const [trackNumber, dictionaryTags] of Object.entries(dictionary)) {
        processedCount++;
        
        // Progress indicator
        if (processedCount % 50 === 0 || processedCount <= 10) {
            console.log(`Progress: ${processedCount}/${totalTracks} (${Math.round(processedCount/totalTracks*100)}%)`);
        }
        
        // Find MP3 file
        const mp3File = findMP3File(trackNumber);
        if (!mp3File) {
            console.log(`Track ${trackNumber}: MP3 file not found - SKIPPED`);
            skippedCount++;
            continue;
        }
        
        // Map to vocabulary tags
        const vocabularyTags = mapToVocabulary(dictionaryTags);
        
        // Write tags to MP3
        const success = writeTagsToMP3(mp3File, vocabularyTags);
        if (success) {
            successCount++;
        }
    }
    
    console.log(`\n=== Processing Complete ===`);
    console.log(`Total tracks: ${totalTracks}`);
    console.log(`Successfully tagged: ${successCount}`);
    console.log(`Skipped (no MP3 file): ${skippedCount}`);
    console.log(`Failed: ${totalTracks - successCount - skippedCount}`);
}

// Main execution
if (require.main === module) {
    const args = process.argv.slice(2);
    
    if (args.includes('--test')) {
        testSteps();
    } else if (args.includes('--all')) {
        processAllFiles();
    } else {
        console.log("Usage:");
        console.log("  node mp3_tagger.js --test     # Test with one file");
        console.log("  node mp3_tagger.js --all      # Process all files");
    }
}

module.exports = { loadDictionary, findMP3File, mapToVocabulary, writeTagsToMP3, processAllFiles };