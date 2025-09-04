# Batch Gemini Tagging - Comprehensive Architecture Plan

## Executive Summary

After extensive research of the current Ligeia codebase, I have identified the critical architectural challenges and designed a comprehensive solution for reliable multi-batch Gemini tagging. The current implementation has fundamental issues with database connection management, transaction handling, and async/await patterns that cause failures when processing large batches.

## Critical Problems Identified

### 1. Database Connection Contention
- **Issue**: Every Tauri command acquires `state.db.lock().unwrap()` creating multiple concurrent locks
- **Evidence**: 169+ lock operations found across handlers with no transaction boundaries
- **Result**: Connection exhaustion and blocking when processing multiple batches

### 2. Broken Transaction Management  
- **Issue**: Only `TagMappingCache::store_mappings` uses transactions (`unchecked_transaction()`)
- **Evidence**: No other database operations use transactions despite multi-step operations
- **Result**: Partial writes, inconsistent state, and potential data corruption

### 3. Async/Sync Boundary Issues
- **Issue**: `process_files_async` mixes synchronous database calls with async Gemini API calls
- **Evidence**: Database locks held during long-running async operations
- **Result**: Deadlocks and timeouts under load

### 4. Memory Inefficient Caching
- **Issue**: Cache mappings loaded synchronously before each async batch
- **Evidence**: `load_cached_mappings()` called for every batch processing
- **Result**: Unnecessary memory usage and startup delays

## Proposed Architecture Solution

### Phase 1: Database Connection Pool (CRITICAL)

**Problem**: Single connection with mutex creates bottlenecks
**Solution**: Implement proper connection pooling

```rust
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;

pub struct DatabasePool {
    pool: Pool<SqliteConnectionManager>,
}

impl DatabasePool {
    pub fn new(database_url: &str, max_connections: u32) -> Result<Self> {
        let manager = SqliteConnectionManager::file(database_url);
        let pool = Pool::builder()
            .max_size(max_connections)
            .build(manager)?;
        Ok(Self { pool })
    }
    
    pub fn get_connection(&self) -> Result<PooledConnection<SqliteConnectionManager>> {
        self.pool.get().map_err(|e| anyhow!("Connection pool error: {}", e))
    }
}
```

**Benefits**:
- Multiple concurrent database operations
- Eliminates connection blocking between batches
- Automatic connection lifecycle management

### Phase 2: Transactional Batch Processing

**Problem**: No transaction boundaries for multi-step operations
**Solution**: Wrap each batch in a single transaction

```rust
pub async fn process_batch_transactional(
    &self,
    batch: Vec<AudioFile>,
    batch_idx: usize,
    pool: &DatabasePool
) -> Result<Vec<TaggedFile>> {
    // Get dedicated connection for this batch
    let conn = pool.get_connection()?;
    let tx = conn.begin_transaction()?;
    
    // Step 1: Process with Gemini (external, not in transaction)
    let gemini_results = self.call_gemini_for_batch(batch).await?;
    
    // Step 2: Apply all database changes in single transaction
    let mut tagged_files = Vec::new();
    for result in gemini_results {
        // Update audio_files table
        tx.execute(UPDATE_AUDIO_FILE_QUERY, params![...])?;
        
        // Remove old tags
        tx.execute(DELETE_OLD_TAGS_QUERY, params![...])?;
        
        // Insert new tags
        for tag in &result.tags {
            tx.execute(INSERT_TAG_QUERY, params![...])?;
        }
        
        tagged_files.push(result);
    }
    
    // Step 3: Commit all changes atomically
    tx.commit()?;
    
    Ok(tagged_files)
}
```

**Benefits**:
- Atomic operations - all succeed or all fail
- No partial state corruption
- Faster than individual commits

### Phase 3: Async-First Architecture

**Problem**: Sync/async mixing causes deadlocks
**Solution**: Restructure to be fully async

```rust
#[tauri::command]
pub async fn auto_tag_files_v2(
    app_handle: AppHandle,
    batch_size: Option<usize>,
    max_parallel: Option<usize>,
) -> Result<String, String> {
    // Get async database pool
    let state = app_handle.state::<AppState>();
    let pool = &state.db_pool;
    
    // Load files asynchronously
    let files = get_untagged_files_async(pool).await?;
    
    if files.is_empty() {
        return Ok("No files to process".to_string());
    }
    
    // Spawn background processor
    let pool_clone = pool.clone();
    let app_handle_clone = app_handle.clone();
    
    tokio::spawn(async move {
        match process_all_batches_async(files, pool_clone, app_handle_clone).await {
            Ok(result) => emit_completion(app_handle_clone, true, result),
            Err(e) => emit_completion(app_handle_clone, false, format!("Error: {}", e)),
        }
    });
    
    Ok("Processing started".to_string())
}

async fn process_all_batches_async(
    files: Vec<AudioFile>,
    pool: DatabasePool,
    app_handle: AppHandle,
) -> Result<String> {
    let batch_size = 50;
    let max_concurrent = 3;
    
    // Pre-load cache once
    let cache = load_tag_mapping_cache_async(&pool).await?;
    
    // Process batches with controlled concurrency
    let batches: Vec<_> = files.chunks(batch_size).collect();
    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    
    let results = futures::stream::iter(batches.into_iter().enumerate())
        .map(|(idx, batch)| {
            let sem = semaphore.clone();
            let pool = pool.clone();
            let cache = cache.clone();
            async move {
                let _permit = sem.acquire().await?;
                process_single_batch_async(batch, idx, &pool, &cache).await
            }
        })
        .buffer_unordered(max_concurrent)
        .collect::<Vec<_>>()
        .await;
    
    // Aggregate results
    let mut total_processed = 0;
    let mut total_failed = 0;
    
    for result in results {
        match result {
            Ok(count) => total_processed += count,
            Err(_) => total_failed += 1,
        }
    }
    
    Ok(format!("Processed: {}, Failed batches: {}", total_processed, total_failed))
}
```

**Benefits**:
- No blocking operations in async context
- Controlled concurrency prevents resource exhaustion
- Cleaner error handling and recovery

### Phase 4: Efficient Caching Strategy

**Problem**: Cache reloaded for every batch
**Solution**: Single cache load with smart updates

```rust
#[derive(Clone)]
pub struct TagMappingCacheManager {
    cache: Arc<RwLock<TagMappingCache>>,
    pool: DatabasePool,
}

impl TagMappingCacheManager {
    pub async fn new(pool: &DatabasePool) -> Result<Self> {
        let cache = Self::load_full_cache(pool).await?;
        Ok(Self {
            cache: Arc::new(RwLock::new(cache)),
            pool: pool.clone(),
        })
    }
    
    pub async fn get_cached_mapping(&self, invalid_tag: &str, tag_type: &str) -> Option<String> {
        let cache = self.cache.read().await;
        cache.get_mapping(invalid_tag, tag_type)
    }
    
    pub async fn store_new_mappings(&self, mappings: &TagMapping) -> Result<()> {
        // Update database
        let conn = self.pool.get_connection()?;
        TagMappingCache::store_mappings(&conn, mappings)?;
        
        // Update in-memory cache
        let mut cache = self.cache.write().await;
        cache.merge_mappings(mappings);
        
        Ok(())
    }
}
```

**Benefits**:
- Single cache load at startup
- Concurrent read access with RwLock
- Atomic updates to both database and memory

## Implementation Phases

### Phase 1: Foundation (Week 1)
1. **Database Pool Integration** 
   - Replace `Database` struct with `DatabasePool`
   - Update `AppState` to use connection pool
   - Modify all handlers to use pooled connections
   - **CRITICAL DISCOVERY**: Must use `deadpool-sqlite` instead of `r2d2_sqlite` due to version conflicts

2. **Library Compatibility Issues Discovered**
   - **Problem**: `r2d2_sqlite 0.25.0` (latest) only supports `rusqlite 0.32`
   - **Conflict**: Project uses `rusqlite 0.37` → `libsqlite3-sys` version conflict
   - **Solution**: Use `deadpool-sqlite 0.8` which supports `rusqlite 0.37`
   - **Benefit**: Better async integration with Tauri's tokio runtime

3. **Transaction Boundaries**
   - Identify all multi-step database operations
   - Wrap in proper transactions  
   - Add rollback handling for failures

### Phase 2: Async Restructuring (Week 2)
1. **Async Database Layer**
   - Convert blocking database calls to async
   - Use `tokio::task::spawn_blocking` for SQLite operations
   - Implement async connection pooling

2. **Batch Processing Pipeline**
   - Create dedicated batch processor
   - Implement semaphore-based concurrency control
   - Add comprehensive error handling and retry logic

### Phase 3: Optimization (Week 3)
1. **Smart Caching**
   - Implement `TagMappingCacheManager`
   - Pre-load cache at application startup
   - Add cache invalidation strategies

2. **Progress Tracking**
   - Real-time progress events
   - Batch-level success/failure reporting
   - Resumable processing from failure points

### Phase 4: Testing and Deployment (Week 4)
1. **Load Testing**
   - Test with 10,000+ files
   - Concurrent batch processing verification
   - Memory usage optimization

2. **Error Recovery**
   - Partial failure handling
   - Automatic retry with exponential backoff
   - State consistency verification

## Key Architectural Patterns

### 1. Connection Pool Pattern
```rust
// Instead of single connection with mutex
let db = state.db.lock().unwrap(); // BAD

// Use async connection pool (deadpool-sqlite)
let conn = state.db_pool.get_connection().await?; // GOOD
```

### 2. Transaction Boundary Pattern
```rust
pub async fn atomic_batch_operation(pool: &DatabasePool, batch: &BatchData) -> Result<()> {
    let conn = pool.get_connection()?;
    let tx = conn.begin_transaction()?;
    
    // All related operations in single transaction
    for item in batch.items {
        operation1(&tx, &item)?;
        operation2(&tx, &item)?;
        operation3(&tx, &item)?;
    }
    
    tx.commit()?; // Atomic success
    Ok(())
}
```

### 3. Async Concurrency Pattern
```rust
// Controlled concurrency for external API calls
let semaphore = Arc::new(Semaphore::new(3)); // Max 3 concurrent

let results = futures::stream::iter(batches)
    .map(|batch| {
        let sem = semaphore.clone();
        async move {
            let _permit = sem.acquire().await?;
            process_batch(batch).await
        }
    })
    .buffer_unordered(3)
    .collect()
    .await;
```

### 4. Cache-Aside Pattern
```rust
pub async fn get_with_cache<T>(
    cache: &Cache<String, T>,
    key: &str,
    loader: impl Future<Output = Result<T>>
) -> Result<T> {
    if let Some(cached) = cache.get(key).await {
        return Ok(cached);
    }
    
    let value = loader.await?;
    cache.insert(key.to_string(), value.clone()).await;
    Ok(value)
}
```

## Performance Expectations

### Before (Current Implementation)
- **Throughput**: ~10-20 files/minute (due to blocking)
- **Concurrent Batches**: 1 (database locked)
- **Memory Usage**: High (repeated cache loads)
- **Failure Recovery**: None (partial state corruption)

### After (Proposed Architecture)
- **Throughput**: ~200-500 files/minute (3 concurrent batches)
- **Concurrent Batches**: 3-5 (configurable)
- **Memory Usage**: Low (single cache load)
- **Failure Recovery**: Complete (atomic transactions)

## Risk Mitigation

### 1. Database Migration
- **Risk**: Connection pool migration breaks existing code
- **Mitigation**: Keep backward compatibility with `AppState.db: Mutex<Database>`
- **Discovery**: Must switch to `deadpool-sqlite` due to `r2d2_sqlite` version conflicts

### 2. Library Compatibility 
- **Risk**: Version conflicts between rusqlite versions
- **Issue Found**: `r2d2_sqlite` stuck on rusqlite 0.32, project needs 0.37
- **Solution**: Use `deadpool-sqlite` with full rusqlite 0.37 support
- **Additional Benefit**: Better async integration with Tauri

### 3. Transaction Deadlocks
- **Risk**: Multiple transactions on same data
- **Mitigation**: Consistent lock ordering, timeout handling

### 4. Memory Usage
- **Risk**: Connection pool increases memory usage
- **Mitigation**: Configurable pool size, connection lifecycle monitoring

### 5. API Rate Limits
- **Risk**: Gemini API rate limiting with concurrent batches
- **Mitigation**: Exponential backoff, configurable concurrency limits

## Success Metrics

### Functional Requirements
- ✅ Process 1000+ files without failure
- ✅ Handle concurrent batch processing
- ✅ Maintain data consistency under failure
- ✅ Resume processing after interruption

### Performance Requirements
- ✅ 5x improvement in throughput
- ✅ <100MB memory usage for processing
- ✅ <5 second startup time with cache
- ✅ <1% data inconsistency rate

### Reliability Requirements
- ✅ 99%+ batch success rate
- ✅ Automatic recovery from failures
- ✅ Zero data corruption incidents
- ✅ Graceful degradation under load

## Implementation Status & Discoveries

### 📋 **Phase 1 TO BE IMPLEMENTED** - Connection Pool Foundation

**Major Discovery**: Must abandon `r2d2_sqlite` due to **version compatibility hell**:

- **Root Cause**: `r2d2_sqlite 0.25.0` (latest) requires `rusqlite 0.32`
- **Conflict**: Your project uses `rusqlite 0.37` → incompatible `libsqlite3-sys` versions
- **Impact**: Rust's linker refuses multiple versions of same native library (`sqlite3`)

**Solution Required**:
- Migrate to `deadpool-sqlite 0.8` (supports rusqlite 0.37)
- Better async integration (native tokio support)
- Create `DatabasePool` struct with async API
- Update `AppState` to include both old and new systems
- Maintain backward compatibility during transition

### 📋 **Implementation Steps Required**

1. **Add dependencies** to `Cargo.toml`
2. **Create DatabasePool** struct and implementation
3. **Update AppState** to use connection pool
4. **Modify gemini_handler.rs** to use pooled connections  
5. **Implement transaction boundaries** for batch operations
6. **Performance testing** with real batch loads

## Conclusion

The current Gemini tagging implementation suffers from fundamental architectural issues that prevent reliable multi-batch processing. The proposed solution addresses these issues through:

1. **Async Connection Pool**: Eliminates database contention using `deadpool-sqlite`
2. **Transactions**: Ensures data consistency 
3. **Async Architecture**: Prevents blocking and deadlocks
4. **Smart Caching**: Reduces memory usage and improves performance

**Critical Learning**: The Rust SQLite ecosystem has version fragmentation issues. `deadpool-sqlite` provides a more modern, async-first approach that's better suited for Tauri applications than the older `r2d2` ecosystem.

This architecture will enable Ligeia to reliably process thousands of files in multiple concurrent batches while maintaining data integrity and providing real-time progress feedback to users.