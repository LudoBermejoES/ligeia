#!/usr/bin/env node

/**
 * Script to find tags that appear in audio files but are not in the tag_vocabulary
 * Usage: node find-missing-tags.js
 */

const fs = require('fs');
const path = require('path');

// Configuration
const JSON_FILE = path.join(__dirname, 'ligeia-library-2025-08-31.json');

console.log('🔍 Loading JSON file...');
const data = JSON.parse(fs.readFileSync(JSON_FILE, 'utf8'));

console.log(`📊 Found ${data.files.length} audio files`);

// Extract all tags from the vocabulary
const vocabularyTags = new Set();

// Add genres
if (data.tag_vocabulary.genres) {
    data.tag_vocabulary.genres.forEach(tag => vocabularyTags.add(tag));
}

// Add moods
if (data.tag_vocabulary.moods) {
    data.tag_vocabulary.moods.forEach(tag => vocabularyTags.add(tag));
}

// Add occasions
if (data.tag_vocabulary.occasions) {
    data.tag_vocabulary.occasions.forEach(tag => vocabularyTags.add(tag));
}

// Add keywords
if (data.tag_vocabulary.keywords) {
    data.tag_vocabulary.keywords.forEach(tag => vocabularyTags.add(tag));
}

console.log(`📚 Found ${vocabularyTags.size} tags in vocabulary`);

// Extract all tags from audio files
const fileTags = new Set();
const tagSources = new Map(); // Track which files use which tags

// Process each audio file
data.files.forEach((file, index) => {
    // Process genre field (can be comma-separated)
    if (file.genre) {
        const genres = file.genre.split(',').map(g => g.trim());
        genres.forEach(genre => {
            if (genre) {
                fileTags.add(genre);
                if (!tagSources.has(genre)) {
                    tagSources.set(genre, []);
                }
                tagSources.get(genre).push({
                    fileId: file.id,
                    title: file.title,
                    field: 'genre'
                });
            }
        });
    }

    // Process mood field
    if (file.mood) {
        const moods = file.mood.split(',').map(m => m.trim());
        moods.forEach(mood => {
            if (mood) {
                fileTags.add(mood);
                if (!tagSources.has(mood)) {
                    tagSources.set(mood, []);
                }
                tagSources.get(mood).push({
                    fileId: file.id,
                    title: file.title,
                    field: 'mood'
                });
            }
        });
    }

    // Process rpg_occasion array
    if (file.rpg_occasion && Array.isArray(file.rpg_occasion)) {
        file.rpg_occasion.forEach(occasion => {
            fileTags.add(occasion);
            if (!tagSources.has(occasion)) {
                tagSources.set(occasion, []);
            }
            tagSources.get(occasion).push({
                fileId: file.id,
                title: file.title,
                field: 'rpg_occasion'
            });
        });
    }

    // Process rpg_keywords array
    if (file.rpg_keywords && Array.isArray(file.rpg_keywords)) {
        file.rpg_keywords.forEach(keyword => {
            fileTags.add(keyword);
            if (!tagSources.has(keyword)) {
                tagSources.set(keyword, []);
            }
            tagSources.get(keyword).push({
                fileId: file.id,
                title: file.title,
                field: 'rpg_keywords'
            });
        });
    }

    // Process rpg_genre array (if it exists)
    if (file.rpg_genre && Array.isArray(file.rpg_genre)) {
        file.rpg_genre.forEach(genre => {
            fileTags.add(genre);
            if (!tagSources.has(genre)) {
                tagSources.set(genre, []);
            }
            tagSources.get(genre).push({
                fileId: file.id,
                title: file.title,
                field: 'rpg_genre'
            });
        });
    }

    // Process rpg_mood array (if it exists)
    if (file.rpg_mood && Array.isArray(file.rpg_mood)) {
        file.rpg_mood.forEach(mood => {
            fileTags.add(mood);
            if (!tagSources.has(mood)) {
                tagSources.set(mood, []);
            }
            tagSources.get(mood).push({
                fileId: file.id,
                title: file.title,
                field: 'rpg_mood'
            });
        });
    }

    if ((index + 1) % 5000 === 0) {
        console.log(`  Processed ${index + 1} files...`);
    }
});

console.log(`🏷️  Found ${fileTags.size} unique tags in audio files`);

// Find missing tags (in files but not in vocabulary)
const missingTags = new Set();
fileTags.forEach(tag => {
    if (!vocabularyTags.has(tag)) {
        missingTags.add(tag);
    }
});

console.log(`\n❌ Found ${missingTags.size} tags in files that are NOT in vocabulary:\n`);

// Group missing tags by category based on their format
const categorizedMissing = {
    genres: [],
    moods: [],
    occasions: [],
    keywords: [],
    unknown: []
};

// Sort missing tags and categorize them
const sortedMissing = Array.from(missingTags).sort();

sortedMissing.forEach(tag => {
    const sources = tagSources.get(tag);
    const exampleFiles = sources.slice(0, 3).map(s => `${s.title} (${s.field})`);
    const tagInfo = {
        tag,
        count: sources.length,
        examples: exampleFiles
    };

    // Categorize based on field or tag format
    const primaryField = sources[0].field;
    if (primaryField === 'genre' || primaryField === 'rpg_genre' || tag.includes(':')) {
        if (tag.startsWith('biome:') || tag.startsWith('creature:') || 
            tag.startsWith('loc:') || tag.startsWith('vehicle:') || 
            tag.startsWith('sfx:') || tag.startsWith('effect:')) {
            categorizedMissing.keywords.push(tagInfo);
        } else {
            categorizedMissing.genres.push(tagInfo);
        }
    } else if (primaryField === 'mood' || primaryField === 'rpg_mood') {
        categorizedMissing.moods.push(tagInfo);
    } else if (primaryField === 'rpg_occasion') {
        categorizedMissing.occasions.push(tagInfo);
    } else if (primaryField === 'rpg_keywords') {
        categorizedMissing.keywords.push(tagInfo);
    } else {
        categorizedMissing.unknown.push(tagInfo);
    }
});

// Display results by category
console.log('📂 GENRES:');
if (categorizedMissing.genres.length > 0) {
    categorizedMissing.genres.forEach(item => {
        console.log(`  - "${item.tag}" (used ${item.count} times)`);
        console.log(`    Examples: ${item.examples.join(', ')}`);
    });
} else {
    console.log('  None');
}

console.log('\n💭 MOODS:');
if (categorizedMissing.moods.length > 0) {
    categorizedMissing.moods.forEach(item => {
        console.log(`  - "${item.tag}" (used ${item.count} times)`);
        console.log(`    Examples: ${item.examples.join(', ')}`);
    });
} else {
    console.log('  None');
}

console.log('\n🎭 OCCASIONS:');
if (categorizedMissing.occasions.length > 0) {
    categorizedMissing.occasions.forEach(item => {
        console.log(`  - "${item.tag}" (used ${item.count} times)`);
        console.log(`    Examples: ${item.examples.join(', ')}`);
    });
} else {
    console.log('  None');
}

console.log('\n🔑 KEYWORDS:');
if (categorizedMissing.keywords.length > 0) {
    categorizedMissing.keywords.forEach(item => {
        console.log(`  - "${item.tag}" (used ${item.count} times)`);
        console.log(`    Examples: ${item.examples.join(', ')}`);
    });
} else {
    console.log('  None');
}

if (categorizedMissing.unknown.length > 0) {
    console.log('\n❓ UNKNOWN CATEGORY:');
    categorizedMissing.unknown.forEach(item => {
        console.log(`  - "${item.tag}" (used ${item.count} times)`);
        console.log(`    Examples: ${item.examples.join(', ')}`);
    });
}

// Write results to file
const outputFile = 'missing-tags-report.json';
const report = {
    summary: {
        totalVocabularyTags: vocabularyTags.size,
        totalFileTags: fileTags.size,
        totalMissingTags: missingTags.size,
        generatedAt: new Date().toISOString()
    },
    missingTags: categorizedMissing,
    allMissingTagsList: sortedMissing
};

fs.writeFileSync(outputFile, JSON.stringify(report, null, 2));
console.log(`\n📄 Detailed report saved to: ${outputFile}`);

// Statistics
console.log('\n📊 STATISTICS:');
console.log(`  Total vocabulary tags: ${vocabularyTags.size}`);
console.log(`  Total unique tags in files: ${fileTags.size}`);
console.log(`  Missing tags: ${missingTags.size}`);
console.log(`  Coverage: ${((vocabularyTags.size / fileTags.size) * 100).toFixed(1)}%`);