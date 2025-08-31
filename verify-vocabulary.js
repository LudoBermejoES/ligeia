#!/usr/bin/env node

/**
 * Vocabulary Verification Script
 * 
 * This script verifies that all extraMappedTags have been successfully
 * added to the Ligeia vocabulary system by checking the vocabulary files.
 */

const fs = require('fs');
const path = require('path');

// Configuration
const INPUT_FILE = './unmapped-tags-report.json';
const VOCAB_DIR = './src-tauri/src/data';

// Vocabulary file mappings
const vocabularyFiles = {
  genre: path.join(VOCAB_DIR, 'genre_vocabulary.rs'),
  mood: path.join(VOCAB_DIR, 'mood_vocabulary.rs'),
  occasion: path.join(VOCAB_DIR, 'occasion_vocabulary.rs'),
  keyword: path.join(VOCAB_DIR, 'keyword_vocabulary.rs')
};

function extractTagsFromVocabFile(filePath) {
  if (!fs.existsSync(filePath)) {
    throw new Error(`Vocabulary file not found: ${filePath}`);
  }
  
  const content = fs.readFileSync(filePath, 'utf8');
  const tags = new Set();
  
  // Extract tag values using regex
  const tagRegex = /\("[\w-]+",\s*"([^"]+)"/g;
  let match;
  while ((match = tagRegex.exec(content)) !== null) {
    tags.add(match[1]);
  }
  
  return tags;
}

function main() {
  try {
    console.log('='.repeat(60));
    console.log('Ligeia Vocabulary Verification Report');
    console.log('='.repeat(60));
    
    // Load the original extra mapped tags
    const data = JSON.parse(fs.readFileSync(INPUT_FILE, 'utf8'));
    const extraMappedTags = new Set(data.extraMappedTags);
    
    console.log(`Original extra mapped tags: ${extraMappedTags.size}`);
    
    // Load current vocabulary from files
    const currentVocab = {
      genre: extractTagsFromVocabFile(vocabularyFiles.genre),
      mood: extractTagsFromVocabFile(vocabularyFiles.mood),
      occasion: extractTagsFromVocabFile(vocabularyFiles.occasion),
      keyword: extractTagsFromVocabFile(vocabularyFiles.keyword)
    };
    
    // Show current vocabulary counts
    console.log('\nCurrent vocabulary counts:');
    let totalVocab = 0;
    Object.keys(currentVocab).forEach(type => {
      const count = currentVocab[type].size;
      console.log(`  ${type}: ${count} tags`);
      totalVocab += count;
    });
    console.log(`  Total: ${totalVocab} tags`);
    
    // Create combined vocabulary set
    const allCurrentTags = new Set();
    Object.values(currentVocab).forEach(vocabSet => {
      vocabSet.forEach(tag => allCurrentTags.add(tag));
    });
    
    // Check how many extra mapped tags are now in vocabulary
    const foundInVocab = new Set();
    const notFoundInVocab = new Set();
    
    extraMappedTags.forEach(tag => {
      if (allCurrentTags.has(tag)) {
        foundInVocab.add(tag);
      } else {
        notFoundInVocab.add(tag);
      }
    });
    
    console.log('\nIntegration Status:');
    console.log(`  Extra mapped tags found in vocabulary: ${foundInVocab.size}`);
    console.log(`  Extra mapped tags missing from vocabulary: ${notFoundInVocab.size}`);
    console.log(`  Integration success rate: ${((foundInVocab.size / extraMappedTags.size) * 100).toFixed(1)}%`);
    
    if (notFoundInVocab.size > 0) {
      console.log('\nMissing tags:');
      Array.from(notFoundInVocab).slice(0, 10).forEach(tag => {
        console.log(`  - ${tag}`);
      });
      if (notFoundInVocab.size > 10) {
        console.log(`  ... and ${notFoundInVocab.size - 10} more`);
      }
    } else {
      console.log('\n✓ All extra mapped tags have been successfully integrated!');
    }
    
    // Show breakdown by category
    console.log('\nBreakdown by vocabulary type:');
    const categories = { genre: [], mood: [], occasion: [], keyword: [] };
    
    extraMappedTags.forEach(tag => {
      if (tag.startsWith('genre:')) {
        categories.genre.push(tag);
      } else if (tag.startsWith('mood:') || 
                 ['brooding', 'energetic', 'defeat', 'cliffhanger'].includes(tag)) {
        categories.mood.push(tag);
      } else if (tag.startsWith('occasion:')) {
        categories.occasion.push(tag);
      } else {
        categories.keyword.push(tag);
      }
    });
    
    Object.keys(categories).forEach(type => {
      const categoryTags = categories[type];
      const found = categoryTags.filter(tag => currentVocab[type].has(tag));
      const missing = categoryTags.filter(tag => !currentVocab[type].has(tag));
      
      console.log(`  ${type}: ${found.length}/${categoryTags.length} integrated${missing.length > 0 ? ` (${missing.length} missing)` : ''}`);
    });
    
    console.log('\n' + '='.repeat(60));
    console.log('Summary:');
    console.log(`The vocabulary system now contains ${totalVocab} total tags.`);
    console.log(`Integration of ${extraMappedTags.size} extra mapped tags: ${foundInVocab.size} successful, ${notFoundInVocab.size} missing`);
    
    if (notFoundInVocab.size === 0) {
      console.log('\n🎉 All extra mapped tags have been successfully added to the vocabulary!');
      console.log('The mapping system should now be 100% complete.');
    } else {
      console.log(`\n⚠️  ${notFoundInVocab.size} tags still need to be added to complete the integration.`);
    }
    
    console.log('='.repeat(60));
    
  } catch (error) {
    console.error('Error:', error.message);
    process.exit(1);
  }
}

// Run if called directly
if (require.main === module) {
  main();
}