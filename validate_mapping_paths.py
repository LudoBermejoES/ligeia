#!/usr/bin/env python3
"""
Script to validate that all folder paths in mapping files exist in folder_structure.rs
"""

import os
import re
from pathlib import Path

def extract_folder_paths_from_structure(file_path):
    """Extract all valid folder paths from folder_structure.rs."""
    paths = set()
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        # Find all folder definitions in format: ("path", Some("parent") or None, icon, description)
        pattern = r'\("([^"]+)",\s*(?:Some\("([^"]*)"\)|None),\s*[^,]*,\s*"[^"]*"\)'
        matches = re.findall(pattern, content)
        
        for path, parent in matches:
            paths.add(path)
                
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
    
    return paths

def extract_mapped_paths_from_mapping_files(mapping_dir):
    """Extract all folder paths used in mapping files."""
    mapped_paths = set()
    
    try:
        for file_path in Path(mapping_dir).glob("*.rs"):
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                
            # Find all mappings in format: ("tag", "folder/path", confidence, "description")
            pattern = r'\("([^"]+)",\s*"([^"]+)",\s*\d+,\s*"[^"]*"\)'
            matches = re.findall(pattern, content)
            
            for tag, folder_path in matches:
                mapped_paths.add((folder_path, file_path.name))
                
    except Exception as e:
        print(f"Error reading mapping files: {e}")
    
    return mapped_paths

def main():
    # Base directory
    base_dir = Path("C:/code/ligeia/src-tauri/src/data")
    
    print("Validating folder paths in mapping files...")
    
    # Extract valid paths from folder_structure.rs
    structure_file = base_dir / "folder_structure.rs"
    if not structure_file.exists():
        print(f"Error: {structure_file} not found")
        return
        
    valid_paths = extract_folder_paths_from_structure(structure_file)
    print(f"Found {len(valid_paths)} valid folder paths in folder_structure.rs")
    
    # Extract paths used in mapping files
    mapping_dir = base_dir / "tag_mappings"
    mapped_paths = extract_mapped_paths_from_mapping_files(mapping_dir)
    print(f"Found {len(mapped_paths)} mapped folder paths in mapping files")
    
    # Find invalid paths
    invalid_paths = []
    for mapped_path, source_file in mapped_paths:
        if mapped_path not in valid_paths:
            invalid_paths.append((mapped_path, source_file))
    
    # Generate report
    report_path = Path("C:/code/ligeia/invalid_folder_paths.md")
    
    with open(report_path, 'w', encoding='utf-8') as f:
        f.write("# Invalid Folder Paths Report\n\n")
        f.write(f"Generated from mapping files validation\n\n")
        
        f.write("## Summary\n\n")
        f.write(f"- **Valid folder paths in structure**: {len(valid_paths)}\n")
        f.write(f"- **Mapped folder paths used**: {len(set(path for path, _ in mapped_paths))}\n")
        f.write(f"- **Invalid folder paths**: {len(invalid_paths)}\n\n")
        
        if invalid_paths:
            f.write("## Invalid Folder Paths\n\n")
            f.write("These paths are used in mapping files but don't exist in folder_structure.rs:\n\n")
            
            # Group by source file
            by_file = {}
            for path, source in invalid_paths:
                if source not in by_file:
                    by_file[source] = []
                by_file[source].append(path)
            
            for source_file in sorted(by_file.keys()):
                f.write(f"### {source_file}\n\n")
                for path in sorted(by_file[source_file]):
                    f.write(f"- `{path}`\n")
                f.write("\n")
        else:
            f.write("## ✅ All Paths Valid!\n\n")
            f.write("All folder paths used in mapping files exist in folder_structure.rs\n")
        
        f.write("## Sample Valid Paths for Reference\n\n")
        f.write("Here are some valid folder paths from folder_structure.rs:\n\n")
        
        # Show some examples of valid paths
        sample_paths = sorted(list(valid_paths))[:50]
        for path in sample_paths:
            f.write(f"- `{path}`\n")
    
    print(f"\nValidation complete!")
    print(f"Invalid paths found: {len(invalid_paths)}")
    print(f"Report generated: {report_path}")
    
    if invalid_paths:
        print("\nSome examples of invalid paths:")
        for path, source in invalid_paths[:10]:
            print(f"  {path} (from {source})")

if __name__ == "__main__":
    main()