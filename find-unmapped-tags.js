#!/usr/bin/env node

/**
 * Script to find tags that are in the database but not covered in tag_mappings files
 * Usage: node find-unmapped-tags.js
 */

const fs = require('fs');
const path = require('path');

// Configuration
const JSON_FILE = path.join(__dirname, 'ligeia-library-2025-08-31.json');
const MAPPINGS_DIR = path.join(__dirname, 'src-tauri', 'src', 'data', 'tag_mappings');

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

// Read all tag mapping files and extract tags
console.log('📂 Reading tag mapping files...');
const mappedTags = new Set();
const mappingFiles = fs.readdirSync(MAPPINGS_DIR).filter(file => file.endsWith('.rs'));

console.log(`📁 Found ${mappingFiles.length} mapping files:`);
mappingFiles.forEach(file => console.log(`  - ${file}`));

mappingFiles.forEach(file => {
    const filePath = path.join(MAPPINGS_DIR, file);
    const content = fs.readFileSync(filePath, 'utf8');
    
    // Parse Rust-style tuples: ("tag", "path", priority, "description")
    // Use regex to extract the tag part (first element of each tuple)
    const tupleRegex = /\(\s*"([^"]+)"\s*,\s*"[^"]+"\s*,\s*\d+\s*,\s*"[^"]+"\s*\)/g;
    let match;
    
    while ((match = tupleRegex.exec(content)) !== null) {
        const tag = match[1];
        mappedTags.add(tag);
    }
});

console.log(`🗂️  Found ${mappedTags.size} tags in mapping files`);

// Find tags in vocabulary that are NOT in mappings
const unmappedTags = new Set();
vocabularyTags.forEach(tag => {
    if (!mappedTags.has(tag)) {
        unmappedTags.add(tag);
    }
});

console.log(`\n❌ Found ${unmappedTags.size} tags in vocabulary that are NOT in mapping files:\n`);

// Categorize unmapped tags
const categorizedUnmapped = {
    genres: [],
    moods: [],
    occasions: [],
    keywords: [],
    unknown: []
};

// Sort unmapped tags
const sortedUnmapped = Array.from(unmappedTags).sort();

sortedUnmapped.forEach(tag => {
    // Categorize based on tag format and vocabulary sections
    if (data.tag_vocabulary.genres && data.tag_vocabulary.genres.includes(tag)) {
        categorizedUnmapped.genres.push(tag);
    } else if (data.tag_vocabulary.moods && data.tag_vocabulary.moods.includes(tag)) {
        categorizedUnmapped.moods.push(tag);
    } else if (data.tag_vocabulary.occasions && data.tag_vocabulary.occasions.includes(tag)) {
        categorizedUnmapped.occasions.push(tag);
    } else if (data.tag_vocabulary.keywords && data.tag_vocabulary.keywords.includes(tag)) {
        categorizedUnmapped.keywords.push(tag);
    } else {
        categorizedUnmapped.unknown.push(tag);
    }
});

// Display results by category
console.log('📂 GENRES NOT IN MAPPINGS:');
if (categorizedUnmapped.genres.length > 0) {
    categorizedUnmapped.genres.forEach(tag => {
        console.log(`  - "${tag}"`);
    });
    console.log(`  Total: ${categorizedUnmapped.genres.length}`);
} else {
    console.log('  None');
}

console.log('\n💭 MOODS NOT IN MAPPINGS:');
if (categorizedUnmapped.moods.length > 0) {
    categorizedUnmapped.moods.forEach(tag => {
        console.log(`  - "${tag}"`);
    });
    console.log(`  Total: ${categorizedUnmapped.moods.length}`);
} else {
    console.log('  None');
}

console.log('\n🎭 OCCASIONS NOT IN MAPPINGS:');
if (categorizedUnmapped.occasions.length > 0) {
    categorizedUnmapped.occasions.forEach(tag => {
        console.log(`  - "${tag}"`);
    });
    console.log(`  Total: ${categorizedUnmapped.occasions.length}`);
} else {
    console.log('  None');
}

console.log('\n🔑 KEYWORDS NOT IN MAPPINGS:');
if (categorizedUnmapped.keywords.length > 0) {
    categorizedUnmapped.keywords.forEach(tag => {
        console.log(`  - "${tag}"`);
    });
    console.log(`  Total: ${categorizedUnmapped.keywords.length}`);
} else {
    console.log('  None');
}

if (categorizedUnmapped.unknown.length > 0) {
    console.log('\n❓ UNKNOWN CATEGORY:');
    categorizedUnmapped.unknown.forEach(tag => {
        console.log(`  - "${tag}"`);
    });
    console.log(`  Total: ${categorizedUnmapped.unknown.length}`);
}

// Also find tags that are in mappings but NOT in vocabulary (potential issues)
const extraMappedTags = new Set();
mappedTags.forEach(tag => {
    if (!vocabularyTags.has(tag)) {
        extraMappedTags.add(tag);
    }
});

if (extraMappedTags.size > 0) {
    console.log(`\n⚠️  Found ${extraMappedTags.size} tags in mapping files that are NOT in vocabulary:`);
    Array.from(extraMappedTags).sort().forEach(tag => {
        console.log(`  - "${tag}"`);
    });
}

// Write results to file
const outputFile = 'unmapped-tags-report.json';
const report = {
    summary: {
        totalVocabularyTags: vocabularyTags.size,
        totalMappedTags: mappedTags.size,
        totalUnmappedTags: unmappedTags.size,
        totalExtraMappedTags: extraMappedTags.size,
        coverage: ((mappedTags.size / vocabularyTags.size) * 100).toFixed(1) + '%',
        generatedAt: new Date().toISOString()
    },
    unmappedTags: categorizedUnmapped,
    extraMappedTags: Array.from(extraMappedTags).sort(),
    allUnmappedTagsList: sortedUnmapped,
    mappingFiles: mappingFiles
};

fs.writeFileSync(outputFile, JSON.stringify(report, null, 2));
console.log(`\n📄 Detailed report saved to: ${outputFile}`);

// Statistics
console.log('\n📊 STATISTICS:');
console.log(`  Total vocabulary tags: ${vocabularyTags.size}`);
console.log(`  Total mapped tags: ${mappedTags.size}`);
console.log(`  Unmapped tags: ${unmappedTags.size}`);
console.log(`  Extra mapped tags: ${extraMappedTags.size}`);
console.log(`  Mapping coverage: ${((mappedTags.size / vocabularyTags.size) * 100).toFixed(1)}%`);

// Coverage by category
console.log('\n📈 COVERAGE BY CATEGORY:');
const genreCoverage = data.tag_vocabulary.genres ? 
    (((data.tag_vocabulary.genres.length - categorizedUnmapped.genres.length) / data.tag_vocabulary.genres.length) * 100).toFixed(1) : 'N/A';
const moodCoverage = data.tag_vocabulary.moods ? 
    (((data.tag_vocabulary.moods.length - categorizedUnmapped.moods.length) / data.tag_vocabulary.moods.length) * 100).toFixed(1) : 'N/A';
const occasionCoverage = data.tag_vocabulary.occasions ? 
    (((data.tag_vocabulary.occasions.length - categorizedUnmapped.occasions.length) / data.tag_vocabulary.occasions.length) * 100).toFixed(1) : 'N/A';
const keywordCoverage = data.tag_vocabulary.keywords ? 
    (((data.tag_vocabulary.keywords.length - categorizedUnmapped.keywords.length) / data.tag_vocabulary.keywords.length) * 100).toFixed(1) : 'N/A';

console.log(`  Genres: ${genreCoverage}%`);
console.log(`  Moods: ${moodCoverage}%`);
console.log(`  Occasions: ${occasionCoverage}%`);
console.log(`  Keywords: ${keywordCoverage}%`);