#[cfg(test)]
mod tests {
    use crate::database::VirtualFolderOps;
    use crate::database::schema::SchemaManager;
    use rusqlite::Connection;
    use crate::models::VirtualFolder;
    
    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute("PRAGMA foreign_keys = ON", []).expect("Failed to enable foreign keys");
        
        let schema = SchemaManager::new(&conn);
        schema.create_tables(&conn).expect("Failed to create tables");
        
        conn
    }
    
    #[test]
    fn test_create_and_get_virtual_folder() {
        let conn = setup_test_db();
        
        let folder = VirtualFolder {
            id: None,
            name: "Test Folder".to_string(),
            description: Some("A test folder".to_string()),
            parent_folder_id: None,
            color: Some("#ff0000".to_string()),
            icon: Some("📁".to_string()),
            created_at: "".to_string(), // Will be set by create
            updated_at: "".to_string(),
            created_by: Some("test_user".to_string()),
            folder_order: 1,
            is_system_folder: false,
            metadata: None,
        };
        
        let folder_id = VirtualFolderOps::create_virtual_folder(&conn, &folder).unwrap();
        assert!(folder_id > 0);
        
        let retrieved_folder = VirtualFolderOps::get_virtual_folder_by_id(&conn, folder_id).unwrap();
        assert_eq!(retrieved_folder.name, "Test Folder");
        assert_eq!(retrieved_folder.description, Some("A test folder".to_string()));
        assert_eq!(retrieved_folder.color, Some("#ff0000".to_string()));
    }
    
    #[test]
    fn test_folder_hierarchy() {
        let conn = setup_test_db();
        
        // Create parent folder
        let parent_folder = VirtualFolder {
            name: "Parent Folder".to_string(),
            ..Default::default()
        };
        let parent_id = VirtualFolderOps::create_virtual_folder(&conn, &parent_folder).unwrap();
        
        // Create child folder
        let child_folder = VirtualFolder {
            name: "Child Folder".to_string(),
            parent_folder_id: Some(parent_id),
            ..Default::default()
        };
        let child_id = VirtualFolderOps::create_virtual_folder(&conn, &child_folder).unwrap();
        
        // Test getting children
        let children = VirtualFolderOps::get_folder_children(&conn, Some(parent_id)).unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name, "Child Folder");
        
        // Test getting path
        let path = VirtualFolderOps::get_folder_path(&conn, child_id).unwrap();
        assert_eq!(path.len(), 2);
        assert_eq!(path[0].name, "Parent Folder");
        assert_eq!(path[1].name, "Child Folder");
    }
    
    #[test]
    fn test_prevent_circular_reference() {
        let conn = setup_test_db();
        
        // Create two folders
        let folder1 = VirtualFolder {
            name: "Folder 1".to_string(),
            ..Default::default()
        };
        let id1 = VirtualFolderOps::create_virtual_folder(&conn, &folder1).unwrap();
        
        let folder2 = VirtualFolder {
            name: "Folder 2".to_string(),
            parent_folder_id: Some(id1),
            ..Default::default()
        };
        let id2 = VirtualFolderOps::create_virtual_folder(&conn, &folder2).unwrap();
        
        // Try to make folder1 a child of folder2 (circular reference)
        let result = VirtualFolderOps::move_folder(&conn, id1, Some(id2));
        assert!(result.is_err());
    }
    
    #[test]
    fn test_search_folders() {
        let conn = setup_test_db();
        
        // Create test folders
        let folder1 = VirtualFolder {
            name: "Combat Sounds".to_string(),
            description: Some("Battle and combat audio".to_string()),
            ..Default::default()
        };
        VirtualFolderOps::create_virtual_folder(&conn, &folder1).unwrap();
        
        let folder2 = VirtualFolder {
            name: "Peaceful Ambience".to_string(),
            description: Some("Calm and serene sounds".to_string()),
            ..Default::default()
        };
        VirtualFolderOps::create_virtual_folder(&conn, &folder2).unwrap();
        
        // Search by name
        let results = VirtualFolderOps::search_folders(&conn, "Combat").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Combat Sounds");
        
        // Search by description
        let results = VirtualFolderOps::search_folders(&conn, "calm").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Peaceful Ambience");
    }
}