use rusqlite::{Connection, Result, Row};
use crate::models::{VirtualFolder, VirtualFolderTree, VirtualFolderWithContents, FolderTemplate, AutoOrganizationSuggestion};

// Module declarations
pub mod crud_ops;
pub mod hierarchy_ops;
pub mod content_ops;
pub mod search_ops;
pub mod tag_suggestions;
pub mod system_init;
pub mod utils;

// Tests module
#[cfg(test)]
pub mod tests;

// Re-export main operations struct
pub use crud_ops::VirtualFolderCrud;
pub use hierarchy_ops::VirtualFolderHierarchy;
pub use content_ops::VirtualFolderContent;
pub use search_ops::VirtualFolderSearch;
pub use tag_suggestions::VirtualFolderTagSuggestions;
pub use system_init::VirtualFolderSystemInit;
pub use utils::VirtualFolderUtils;

/// Main database operations struct for virtual folders
pub struct VirtualFolderOps;

impl VirtualFolderOps {
    // CRUD Operations
    pub fn create_virtual_folder(conn: &Connection, folder: &VirtualFolder) -> Result<i64> {
        VirtualFolderCrud::create_virtual_folder(conn, folder)
    }

    pub fn get_virtual_folder_by_id(conn: &Connection, id: i64) -> Result<VirtualFolder> {
        VirtualFolderCrud::get_virtual_folder_by_id(conn, id)
    }

    pub fn update_virtual_folder(conn: &Connection, folder: &VirtualFolder) -> Result<()> {
        VirtualFolderCrud::update_virtual_folder(conn, folder)
    }

    pub fn delete_virtual_folder(conn: &Connection, id: i64) -> Result<()> {
        VirtualFolderCrud::delete_virtual_folder(conn, id)
    }

    // Hierarchy Operations
    pub fn get_folder_children(conn: &Connection, parent_id: Option<i64>) -> Result<Vec<VirtualFolder>> {
        VirtualFolderHierarchy::get_folder_children(conn, parent_id)
    }

    pub fn get_all_virtual_folders(conn: &Connection) -> Result<Vec<VirtualFolder>> {
        VirtualFolderHierarchy::get_all_virtual_folders(conn)
    }

    pub fn get_folder_tree(conn: &Connection) -> Result<Vec<VirtualFolderTree>> {
        VirtualFolderHierarchy::get_folder_tree(conn)
    }

    pub fn get_folder_path(conn: &Connection, folder_id: i64) -> Result<Vec<VirtualFolder>> {
        VirtualFolderHierarchy::get_folder_path(conn, folder_id)
    }

    pub fn move_folder(conn: &Connection, folder_id: i64, new_parent_id: Option<i64>) -> Result<()> {
        VirtualFolderHierarchy::move_folder(conn, folder_id, new_parent_id)
    }

    // Content Management
    pub fn add_file_to_folder(conn: &Connection, folder_id: i64, audio_file_id: i64) -> Result<()> {
        VirtualFolderContent::add_file_to_folder(conn, folder_id, audio_file_id)
    }

    pub fn remove_file_from_folder(conn: &Connection, folder_id: i64, audio_file_id: i64) -> Result<()> {
        VirtualFolderContent::remove_file_from_folder(conn, folder_id, audio_file_id)
    }

    pub fn get_folder_contents(conn: &Connection, folder_id: i64) -> Result<VirtualFolderWithContents> {
        VirtualFolderContent::get_folder_contents(conn, folder_id)
    }

    pub fn get_file_folders(conn: &Connection, audio_file_id: i64) -> Result<Vec<VirtualFolder>> {
        VirtualFolderContent::get_file_folders(conn, audio_file_id)
    }

    // Search and Discovery
    pub fn search_folders(conn: &Connection, query: &str) -> Result<Vec<VirtualFolder>> {
        VirtualFolderSearch::search_folders(conn, query)
    }

    pub fn get_folders_containing_files(conn: &Connection, file_ids: &[i64]) -> Result<Vec<VirtualFolder>> {
        VirtualFolderSearch::get_folders_containing_files(conn, file_ids)
    }

    // Templates
    pub fn create_folder_template(conn: &Connection, template: &FolderTemplate) -> Result<i64> {
        VirtualFolderCrud::create_folder_template(conn, template)
    }

    pub fn get_folder_templates(conn: &Connection, category: Option<&str>) -> Result<Vec<FolderTemplate>> {
        VirtualFolderCrud::get_folder_templates(conn, category)
    }

    // System Initialization
    pub fn initialize_default_virtual_folders(conn: &Connection) -> Result<()> {
        VirtualFolderSystemInit::initialize_default_virtual_folders(conn)
    }

    pub fn get_unorganized_tagged_files(conn: &Connection) -> Result<Vec<i64>> {
        VirtualFolderSystemInit::get_unorganized_tagged_files(conn)
    }

    // Tag-based Suggestions
    pub fn suggest_folders_for_file(conn: &Connection, audio_file_id: i64, limit: Option<usize>) -> Result<Vec<(VirtualFolder, f64)>> {
        VirtualFolderTagSuggestions::suggest_folders_for_file(conn, audio_file_id, limit)
    }

    pub fn get_auto_organization_suggestions(conn: &Connection, threshold: f64) -> Result<Vec<AutoOrganizationSuggestion>> {
        VirtualFolderTagSuggestions::get_auto_organization_suggestions(conn, threshold)
    }

    pub fn get_matching_tags(conn: &Connection, audio_file_id: i64, folder_id: i64) -> Result<Vec<String>> {
        VirtualFolderTagSuggestions::get_matching_tags(conn, audio_file_id, folder_id)
    }

    // Utility functions
    pub fn row_to_virtual_folder(row: &Row) -> Result<VirtualFolder> {
        VirtualFolderUtils::row_to_virtual_folder(row)
    }
}