use scan_dir::ScanDir;

pub struct FileScanner;

impl FileScanner {
    pub fn scan_directory_recursive(dir_path: &str) -> Result<Vec<String>, String> {
        println!("Scanning directory recursively: {}", dir_path);
        
        let audio_extensions = vec!["mp3", "wav", "ogg", "flac", "aac", "m4a", "wma", "m4p"];
        
        let audio_files = ScanDir::files().walk(dir_path, |iter| {
            iter.filter(|&(_, ref name)| {
                audio_extensions.iter().any(|ext| {
                    name.to_lowercase().ends_with(&format!(".{}", ext))
                })
            })
            .map(|(entry, _)| entry.path().to_string_lossy().to_string())
            .collect::<Vec<String>>()
        }).map_err(|e| format!("Failed to scan directory: {:?}", e))?;
        
        println!("Found {} audio files", audio_files.len());
        for file in &audio_files {
            println!("Audio file: {}", file);
        }
        
        Ok(audio_files)
    }

    #[allow(dead_code)]
    pub fn get_supported_extensions() -> Vec<&'static str> {
        vec!["mp3", "wav", "ogg", "flac", "aac", "m4a", "wma", "m4p"]
    }

    #[allow(dead_code)]
    pub fn is_audio_file(file_path: &str) -> bool {
        let audio_extensions = Self::get_supported_extensions();
        let file_lower = file_path.to_lowercase();
        audio_extensions.iter().any(|ext| file_lower.ends_with(&format!(".{}", ext)))
    }
}