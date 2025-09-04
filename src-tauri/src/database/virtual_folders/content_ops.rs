use rusqlite::{Connection, Result, params};
use crate::models::{VirtualFolder, VirtualFolderWithContents, VirtualFolderTree, AudioFile};
use chrono::Utc;

/// Content management operations for virtual folders
pub struct VirtualFolderContent;

impl VirtualFolderContent {
    pub fn add_file_to_folder(conn: &Connection, folder_id: i64, audio_file_id: i64) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        let mut stmt = conn.prepare(
            "INSERT INTO virtual_folder_contents (folder_id, audio_file_id, added_at, file_order)
             VALUES (?, ?, ?, (SELECT COALESCE(MAX(file_order), 0) + 1 FROM virtual_folder_contents WHERE folder_id = ?))"
        )?;
        
        stmt.execute(params![folder_id, audio_file_id, &now, folder_id])?;
        Ok(())
    }
    
    pub fn remove_file_from_folder(conn: &Connection, folder_id: i64, audio_file_id: i64) -> Result<()> {
        let mut stmt = conn.prepare(
            "DELETE FROM virtual_folder_contents WHERE folder_id = ? AND audio_file_id = ?"
        )?;
        stmt.execute(params![folder_id, audio_file_id])?;
        Ok(())
    }
    
    pub fn get_folder_contents(conn: &Connection, folder_id: i64) -> Result<VirtualFolderWithContents> {
        // Get folder info
        use crate::database::virtual_folders::crud_ops::VirtualFolderCrud;
        let folder = VirtualFolderCrud::get_virtual_folder_by_id(conn, folder_id)?;
        
        // Get breadcrumb path
        use crate::database::virtual_folders::hierarchy_ops::VirtualFolderHierarchy;
        let breadcrumb = VirtualFolderHierarchy::get_folder_path(conn, folder_id)?;
        
        // Get subfolders with file counts
        let subfolder_list = VirtualFolderHierarchy::get_folder_children(conn, Some(folder_id))?;
        let mut subfolders = Vec::new();
        
        for subfolder in subfolder_list {
            if let Some(subfolder_id) = subfolder.id {
                let file_count = Self::count_folder_files_recursive(conn, subfolder_id)?;
                let folder_tree = VirtualFolderTree {
                    folder: subfolder,
                    children: Vec::new(), // Not needed for contents view
                    file_count,
                    total_file_count: file_count, // Same as file_count for this context
                };
                subfolders.push(folder_tree);
            }
        }
        
        // Get audio files in this folder
        let mut stmt = conn.prepare(
            "SELECT af.* FROM audio_files af
             JOIN virtual_folder_contents vfc ON af.id = vfc.audio_file_id
             WHERE vfc.folder_id = ?
             ORDER BY vfc.file_order, af.title"
        )?;
        
        let audio_file_iter = stmt.query_map([folder_id], |row| {
            // Map row to AudioFile - this is a simplified version, you may need to adjust based on your AudioFile struct
            Ok(AudioFile {
                id: Some(row.get("id")?),
                file_path: row.get("file_path")?,
                title: row.get("title")?,
                artist: row.get("artist")?,
                album: row.get("album")?,
                album_artist: row.get("album_artist")?,
                genre: row.get("genre")?,
                year: row.get("year")?,
                date: row.get("date")?,
                track_number: row.get("track_number")?,
                total_tracks: row.get("total_tracks")?,
                disc_number: row.get("disc_number")?,
                total_discs: row.get("total_discs")?,
                duration: row.get("duration")?,
                composer: row.get("composer")?,
                conductor: row.get("conductor")?,
                lyricist: row.get("lyricist")?,
                original_artist: row.get("original_artist")?,
                remixer: row.get("remixer")?,
                arranger: row.get("arranger")?,
                engineer: row.get("engineer")?,
                producer: row.get("producer")?,
                dj_mixer: row.get("dj_mixer")?,
                mixer: row.get("mixer")?,
                content_group: row.get("content_group")?,
                subtitle: row.get("subtitle")?,
                initial_key: row.get("initial_key")?,
                bpm: row.get("bpm")?,
                language: row.get("language")?,
                media_type: row.get("media_type")?,
                original_filename: row.get("original_filename")?,
                original_lyricist: row.get("original_lyricist")?,
                original_release_time: row.get("original_release_time")?,
                playlist_delay: row.get("playlist_delay")?,
                recording_time: row.get("recording_time")?,
                release_time: row.get("release_time")?,
                tagging_time: row.get("tagging_time")?,
                encoding_time: row.get("encoding_time")?,
                encoding_settings: row.get("encoding_settings")?,
                encoded_by: row.get("encoded_by")?,
                copyright: row.get("copyright")?,
                file_owner: row.get("file_owner")?,
                internet_radio_station_name: row.get("internet_radio_station_name")?,
                internet_radio_station_owner: row.get("internet_radio_station_owner")?,
                isrc: row.get("isrc")?,
                publisher: row.get("publisher")?,
                mood: row.get("mood")?,
                occasion: row.get("occasion")?,
                tempo: row.get("tempo")?,
                content_type: row.get("content_type")?,
                category: row.get("category")?,
                auto_tagged: row.get("auto_tagged")?,
                auto_tag_date: row.get("auto_tag_date")?,
                auto_tag_version: row.get("auto_tag_version")?,
            })
        })?;
        
        let mut audio_files = Vec::new();
        for audio_file in audio_file_iter {
            audio_files.push(audio_file?);
        }
        
        Ok(VirtualFolderWithContents {
            folder,
            audio_files,
            subfolders,
            breadcrumb,
        })
    }
    
    pub fn get_file_folders(conn: &Connection, audio_file_id: i64) -> Result<Vec<VirtualFolder>> {
        let mut stmt = conn.prepare(
            "SELECT vf.* FROM virtual_folders vf
             JOIN virtual_folder_contents vfc ON vf.id = vfc.folder_id
             WHERE vfc.audio_file_id = ?
             ORDER BY vf.name"
        )?;
        
        use crate::database::virtual_folders::utils::VirtualFolderUtils;
        let folder_iter = stmt.query_map([audio_file_id], |row| VirtualFolderUtils::row_to_virtual_folder(row))?;
        
        let mut folders = Vec::new();
        for folder in folder_iter {
            folders.push(folder?);
        }
        
        Ok(folders)
    }
    
    /// Recursively count all files in a folder and its subfolders
    pub fn count_folder_files_recursive(conn: &Connection, folder_id: i64) -> Result<i64> {
        // Count direct files in this folder
        let mut stmt = conn.prepare(
            "SELECT COUNT(*) FROM virtual_folder_contents WHERE folder_id = ?"
        )?;
        let direct_files: i64 = stmt.query_row([folder_id], |row| row.get(0))?;
        
        // Count files in subfolders recursively
        use crate::database::virtual_folders::hierarchy_ops::VirtualFolderHierarchy;
        let subfolders = VirtualFolderHierarchy::get_folder_children(conn, Some(folder_id))?;
        
        let mut total_files = direct_files;
        for subfolder in subfolders {
            if let Some(subfolder_id) = subfolder.id {
                total_files += Self::count_folder_files_recursive(conn, subfolder_id)?;
            }
        }
        
        Ok(total_files)
    }
}