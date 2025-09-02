#!/usr/bin/env python3
"""
Verify that all folder paths in tag mappings exist in the actual folder structure.
"""

import re
import os
from typing import Set, List, Dict, Tuple
from pathlib import Path

def extract_folder_paths_from_structure() -> Set[str]:
    """Extract all folder paths from folder_structure.rs"""
    structure_file = Path("src-tauri/src/data/folder_structure.rs")
    
    if not structure_file.exists():
        print(f"ERROR: {structure_file} not found!")
        return set()
    
    folder_paths = set()
    
    with open(structure_file, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Find all folder path definitions: ("path", ...)
    pattern = r'\("([^"]+)",\s*(?:Some\("([^"]+)"\)|None)'
    matches = re.findall(pattern, content)
    
    for match in matches:
        folder_path = match[0]
        folder_paths.add(folder_path)
    
    print(f"Found {len(folder_paths)} folder paths in folder_structure.rs")
    return folder_paths

def extract_mappings_from_file(file_path: Path) -> List[Tuple[str, List[str]]]:
    """Extract tag->folder mappings from a mapping file"""
    if not file_path.exists():
        print(f"WARNING: {file_path} not found!")
        return []
    
    mappings = []
    
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Find all mapping entries: ("tag", &["folder1", "folder2"])
    pattern = r'\("([^"]+)",\s*&\[([^\]]+)\]\)'
    matches = re.findall(pattern, content)
    
    for match in matches:
        tag = match[0]
        folders_str = match[1]
        
        # Extract folder paths from the array
        folder_pattern = r'"([^"]+)"'
        folders = re.findall(folder_pattern, folders_str)
        mappings.append((tag, folders))
    
    return mappings

def verify_mappings():
    """Main verification function"""
    print("=" * 60)
    print("FOLDER MAPPING VERIFICATION")
    print("=" * 60)
    
    # Get all actual folder paths
    actual_folders = extract_folder_paths_from_structure()
    
    if not actual_folders:
        print("ERROR: No folder paths found in folder_structure.rs")
        return
    
    # Check each mapping file
    mapping_files = [
        "src-tauri/src/data/tag_mappings/genre_mappings.rs",
        "src-tauri/src/data/tag_mappings/mood_mappings.rs", 
        "src-tauri/src/data/tag_mappings/occasion_mappings.rs",
        "src-tauri/src/data/tag_mappings/keyword_mappings.rs"
    ]
    
    total_mappings = 0
    total_folders_referenced = 0
    invalid_folders = {}
    valid_folders = set()
    
    for mapping_file in mapping_files:
        file_path = Path(mapping_file)
        print(f"\nChecking {file_path.name}...")
        
        mappings = extract_mappings_from_file(file_path)
        if not mappings:
            print(f"  No mappings found in {file_path.name}")
            continue
            
        total_mappings += len(mappings)
        
        file_invalid_folders = []
        
        for tag, folders in mappings:
            for folder in folders:
                total_folders_referenced += 1
                if folder not in actual_folders:
                    file_invalid_folders.append((tag, folder))
                else:
                    valid_folders.add(folder)
        
        if file_invalid_folders:
            invalid_folders[file_path.name] = file_invalid_folders
            print(f"  [X] Found {len(file_invalid_folders)} invalid folder references")
        else:
            print(f"  [OK] All {len(mappings)} mappings are valid")
    
    # Summary report
    print("\n" + "=" * 60)
    print("SUMMARY REPORT")
    print("=" * 60)
    print(f"Total tag mappings: {total_mappings}")
    print(f"Total folder references: {total_folders_referenced}")
    print(f"Valid folder references: {total_folders_referenced - sum(len(v) for v in invalid_folders.values())}")
    print(f"Invalid folder references: {sum(len(v) for v in invalid_folders.values())}")
    
    if invalid_folders:
        print(f"\n[X] INVALID FOLDER REFERENCES:")
        for file_name, invalid_list in invalid_folders.items():
            print(f"\n{file_name}:")
            for tag, folder in invalid_list:
                print(f"  - Tag '{tag}' -> '{folder}'")
        
        print(f"\nMISSING FOLDERS THAT NEED TO BE CREATED:")
        missing_folders = set()
        for invalid_list in invalid_folders.values():
            for tag, folder in invalid_list:
                missing_folders.add(folder)
        
        for folder in sorted(missing_folders):
            print(f"  - {folder}")
            
    else:
        print(f"\n[OK] ALL FOLDER MAPPINGS ARE VALID!")
    
    # Show folder coverage
    unused_folders = actual_folders - valid_folders
    if unused_folders:
        print(f"\nUNUSED FOLDERS (not referenced by any mappings):")
        for folder in sorted(unused_folders):
            print(f"  - {folder}")

if __name__ == "__main__":
    verify_mappings()