use anyhow::{anyhow, Result};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use std::time::Duration;
use log::{info, warn, error};

/// Database connection pool for managing SQLite connections
/// Provides concurrent access to database without blocking
pub struct DatabasePool {
    pool: Pool<SqliteConnectionManager>,
}

/// Type alias for pooled connection to simplify usage
pub type DbPooledConnection = PooledConnection<SqliteConnectionManager>;

impl DatabasePool {
    /// Create a new database connection pool
    pub fn new(database_path: &str, max_connections: u32) -> Result<Self> {
        info!("Creating database connection pool with max {} connections", max_connections);
        
        let manager = SqliteConnectionManager::file(database_path)
            .with_init(|conn| {
                // Enable foreign key constraints
                conn.execute("PRAGMA foreign_keys = ON", [])?;
                // Set WAL mode for better concurrent access
                conn.execute("PRAGMA journal_mode = WAL", [])?;
                // Optimize for concurrent reads
                conn.execute("PRAGMA synchronous = NORMAL", [])?;
                // Set reasonable timeout
                conn.execute("PRAGMA busy_timeout = 30000", [])?;
                Ok(())
            });

        let pool = Pool::builder()
            .max_size(max_connections)
            .min_idle(Some(2)) // Keep 2 connections ready
            .max_lifetime(Some(Duration::from_secs(3600))) // 1 hour max lifetime
            .idle_timeout(Some(Duration::from_secs(600))) // 10 minute idle timeout
            .connection_timeout(Duration::from_secs(30)) // 30 second connection timeout
            .build(manager)
            .map_err(|e| anyhow!("Failed to create connection pool: {}", e))?;

        info!("Database connection pool created successfully");
        Ok(Self { pool })
    }

    /// Get a connection from the pool
    pub fn get_connection(&self) -> Result<DbPooledConnection> {
        self.pool.get()
            .map_err(|e| {
                error!("Failed to get connection from pool: {}", e);
                anyhow!("Connection pool error: {}", e)
            })
    }

    /// Get pool statistics for monitoring
    pub fn get_stats(&self) -> PoolStats {
        let state = self.pool.state();
        PoolStats {
            connections: state.connections,
            idle_connections: state.idle_connections,
            max_size: self.pool.max_size(),
        }
    }

    /// Initialize database schema and data using a pooled connection
    pub fn initialize_database(&self) -> Result<()> {
        info!("Initializing database schema and data");
        let conn = self.get_connection()?;
        
        // Initialize schema - create all tables
        crate::database::SchemaManager::create_tables_static(&conn)?;
        
        // Initialize tag vocabulary
        crate::database::VocabularyRepository::initialize_tag_vocabulary_static(&conn)?;
        
        // Initialize audio files table
        crate::database::AudioFileOps::create_table(&conn)?;
        
        // Initialize atmosphere tables  
        crate::database::AtmosphereOps::create_tables(&conn)?;
        
        // Initialize virtual folders
        crate::database::VirtualFolderOps::initialize_default_virtual_folders(&conn)?;
        
        info!("Database initialization completed");
        Ok(())
    }

    /// Test connection pool health
    pub fn health_check(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        // Simple query to verify database is accessible
        let result: i32 = conn.query_row("SELECT 1", [], |row| row.get(0))
            .map_err(|e| anyhow!("Database health check failed: {}", e))?;
            
        if result != 1 {
            return Err(anyhow!("Database health check returned unexpected value: {}", result));
        }
        
        Ok(())
    }
}

/// Pool statistics for monitoring
#[derive(Debug)]
pub struct PoolStats {
    pub connections: u32,
    pub idle_connections: u32, 
    pub max_size: u32,
}

impl std::fmt::Display for PoolStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pool Stats: {}/{} connections ({} idle)", 
               self.connections, self.max_size, self.idle_connections)
    }
}

// Extension trait to add transaction support to pooled connections
pub trait PooledConnectionExt {
    fn begin_transaction(&mut self) -> Result<rusqlite::Transaction>;
}

impl PooledConnectionExt for DbPooledConnection {
    fn begin_transaction(&mut self) -> Result<rusqlite::Transaction> {
        (**self).transaction()
            .map_err(|e| anyhow!("Failed to begin transaction: {}", e))
    }
}