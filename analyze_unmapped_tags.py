#!/usr/bin/env python3
"""
Script to analyze vocabulary files and find tags that don't have mappings yet.
Generates a report of all unmapped tags in new_tags_to_add.md
"""

import os
import re
from collections import defaultdict
from pathlib import Path

def extract_tags_from_vocabulary_file(file_path):
    """Extract all tags from a vocabulary file."""
    tags = set()
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        # Find all tag definitions in format: ("category", "tag", Some("description"), parent)
        pattern = r'\("([^"]+)",\s*"([^"]+)",\s*Some\("([^"]+)"\),\s*[^)]+\)'
        matches = re.findall(pattern, content)
        
        for category, tag, description in matches:
            # The tag field already contains the full tag format, just use it directly
            tags.add(tag)
                
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
    
    return tags

def extract_tags_from_folder_structure(file_path):
    """Extract folder paths from folder_structure.rs."""
    folders = set()
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        # Find folder definitions - looking for string literals that look like paths
        lines = content.split('\n')
        for line in lines:
            line = line.strip()
            if '"' in line and ('/' in line or line.count('"') >= 2):
                # Extract strings that look like folder paths
                matches = re.findall(r'"([^"]+)"', line)
                for match in matches:
                    if '/' in match and not match.startswith('//'):
                        folders.add(match)
                        
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
    
    return folders

def extract_mapped_tags_from_mapping_files(mapping_dir):
    """Extract all tags that have mappings in the tag_mappings directory."""
    mapped_tags = set()
    mapped_folders = set()
    
    try:
        for file_path in Path(mapping_dir).glob("*.rs"):
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                
            # Find all mappings in format: ("tag", "folder/path", confidence, "description")
            pattern = r'\("([^"]+)",\s*"([^"]+)",\s*\d+,\s*"[^"]*"\)'
            matches = re.findall(pattern, content)
            
            for tag, folder_path in matches:
                mapped_tags.add(tag)
                mapped_folders.add(folder_path)
                
    except Exception as e:
        print(f"Error reading mapping files: {e}")
    
    return mapped_tags, mapped_folders

def main():
    # Base directory
    base_dir = Path("C:/code/ligeia/src-tauri/src/data")
    
    print("Analyzing vocabulary files...")
    
    # Extract all tags from vocabulary files
    all_vocabulary_tags = set()
    
    # Process each vocabulary file
    vocabulary_files = [
        "genre_vocabulary.rs",
        "mood_vocabulary.rs", 
        "occasion_vocabulary.rs",
        "keyword_vocabulary.rs"
    ]
    
    vocabulary_tags_by_file = {}
    
    for vocab_file in vocabulary_files:
        file_path = base_dir / vocab_file
        if file_path.exists():
            tags = extract_tags_from_vocabulary_file(file_path)
            vocabulary_tags_by_file[vocab_file] = tags
            all_vocabulary_tags.update(tags)
            print(f"Found {len(tags)} tags in {vocab_file}")
        else:
            print(f"Warning: {vocab_file} not found")
    
    # Extract folder structure
    folder_structure_file = base_dir / "folder_structure.rs"
    folder_paths = set()
    if folder_structure_file.exists():
        folder_paths = extract_tags_from_folder_structure(folder_structure_file)
        print(f"Found {len(folder_paths)} folder paths in folder_structure.rs")
    
    # Extract mapped tags
    mapping_dir = base_dir / "tag_mappings"
    mapped_tags, mapped_folders = extract_mapped_tags_from_mapping_files(mapping_dir)
    print(f"Found {len(mapped_tags)} mapped tags and {len(mapped_folders)} mapped folders")
    
    # Find unmapped tags
    unmapped_tags = all_vocabulary_tags - mapped_tags
    unmapped_folders = folder_paths - mapped_folders
    
    print(f"\nAnalysis complete:")
    print(f"Total vocabulary tags: {len(all_vocabulary_tags)}")
    print(f"Mapped tags: {len(mapped_tags)}")
    print(f"Unmapped tags: {len(unmapped_tags)}")
    print(f"Unmapped folders: {len(unmapped_folders)}")
    
    # Generate markdown report
    report_path = Path("C:/code/ligeia/new_tags_to_add.md")
    
    with open(report_path, 'w', encoding='utf-8') as f:
        f.write("# Unmapped Tags Analysis Report\n\n")
        f.write(f"Generated from vocabulary files analysis\n\n")
        
        f.write("## Summary\n\n")
        f.write(f"- **Total vocabulary tags**: {len(all_vocabulary_tags)}\n")
        f.write(f"- **Currently mapped tags**: {len(mapped_tags)}\n")
        f.write(f"- **Unmapped tags**: {len(unmapped_tags)}\n")
        f.write(f"- **Coverage**: {(len(mapped_tags) / len(all_vocabulary_tags) * 100):.1f}%\n\n")
        
        f.write(f"- **Total folder paths**: {len(folder_paths)}\n")
        f.write(f"- **Used folder paths**: {len(mapped_folders)}\n")
        f.write(f"- **Unused folder paths**: {len(unmapped_folders)}\n\n")
        
        # Unmapped tags by category
        f.write("## Unmapped Tags by Category\n\n")
        
        categories = defaultdict(list)
        for tag in unmapped_tags:
            if ':' in tag:
                category = tag.split(':', 1)[0]
                categories[category].append(tag)
            else:
                categories['other'].append(tag)
        
        for category in sorted(categories.keys()):
            f.write(f"### {category.title()} Tags ({len(categories[category])})\n\n")
            for tag in sorted(categories[category]):
                f.write(f"- `{tag}`\n")
            f.write("\n")
        
        # Unmapped tags by source file
        f.write("## Unmapped Tags by Source File\n\n")
        
        for vocab_file, file_tags in vocabulary_tags_by_file.items():
            unmapped_from_file = file_tags - mapped_tags
            if unmapped_from_file:
                f.write(f"### {vocab_file} ({len(unmapped_from_file)} unmapped)\n\n")
                for tag in sorted(unmapped_from_file):
                    f.write(f"- `{tag}`\n")
                f.write("\n")
        
        # Unused folder paths
        if unmapped_folders:
            f.write("## Unused Folder Paths\n\n")
            f.write("These folder paths exist in folder_structure.rs but are not used in any mappings:\n\n")
            for folder in sorted(unmapped_folders):
                f.write(f"- `{folder}`\n")
            f.write("\n")
        
        # Recommendations
        f.write("## Recommendations\n\n")
        f.write("### High Priority Tags to Map\n\n")
        
        high_priority_patterns = [
            'occasion:', 'keyword:sfx:', 'keyword:loc:', 'keyword:creature:',
            'keyword:biome:', 'mood:', 'genre:'
        ]
        
        high_priority_unmapped = []
        for tag in unmapped_tags:
            for pattern in high_priority_patterns:
                if tag.startswith(pattern):
                    high_priority_unmapped.append(tag)
                    break
        
        if high_priority_unmapped:
            f.write("These unmapped tags are likely to be frequently used and should be prioritized:\n\n")
            for tag in sorted(high_priority_unmapped)[:50]:  # Limit to top 50
                f.write(f"- `{tag}`\n")
        else:
            f.write("Most high-priority tags appear to be mapped already. Great job!\n")
        
        f.write("\n### Mapping Strategy\n\n")
        f.write("1. **Occasion tags** - Map to Session Structure folders\n")
        f.write("2. **SFX keywords** - Map to SFX & Foley folders\n")
        f.write("3. **Location keywords** - Map to Environment folders\n")
        f.write("4. **Creature keywords** - Map to SFX & Foley/Creature Sounds\n")
        f.write("5. **Mood tags** - Map to Moods & Atmosphere folders\n")
        f.write("6. **Genre tags** - Map to appropriate Cultural Styles folders\n")
    
    print(f"\nReport generated: {report_path}")
    print("Analysis complete!")

if __name__ == "__main__":
    main()