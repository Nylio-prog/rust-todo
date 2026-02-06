// Storage module - handles data persistence using JSON files
// This module demonstrates file I/O, serialization, and atomic file operations
//
// This module is responsible for persisting the application's data to disk
// and loading it back. It uses JSON as the storage format for human-readability
// and easy import/export capabilities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::context::Context;

/// Represents the on-disk data format
///
/// StorageData is the top-level structure that gets serialized to JSON
/// and saved to disk. It contains all the application's data: contexts,
/// tasks, and the active context.
///
/// # Serialization and Versioning
///
/// The #[derive(Serialize, Deserialize)] attributes from serde enable automatic
/// JSON conversion. This allows us to save the entire application state with
/// a single serde_json::to_string() call.
///
/// The `version` field enables schema migrations in the future. If we need to
/// change the data format (e.g., add new fields, restructure data), we can:
/// 1. Check the version when loading
/// 2. Apply migration logic to convert old formats to new formats
/// 3. Increment the version when saving
///
/// # Semantic Versioning
///
/// The version follows semantic versioning (MAJOR.MINOR.PATCH):
/// - MAJOR: Breaking changes that require migration logic
/// - MINOR: Backward-compatible additions (new optional fields)
/// - PATCH: Bug fixes that don't affect the schema
///
/// Current version: "1.0.0" - Initial release format
///
/// # Fields
///
/// - `version`: Schema version string (e.g., "1.0.0")
/// - `contexts`: HashMap mapping context names to Context objects
/// - `active_context`: The name of the currently active context
///
/// # JSON Format Example
///
/// ```json
/// {
///   "version": "1.0.0",
///   "active_context": "default",
///   "contexts": {
///     "default": {
///       "name": "default",
///       "tasks": [...]
///     }
///   }
/// }
/// ```
///
/// # Design Rationale
///
/// We use a separate StorageData struct instead of directly serializing
/// ContextManager because:
/// 1. It decouples the in-memory representation from the on-disk format
/// 2. It allows us to add storage-specific metadata (like version)
/// 3. It makes schema migrations easier (we can transform between formats)
/// 4. It follows the principle of separation of concerns
///
/// # Extensibility
///
/// Future versions might add:
/// - `metadata`: Application settings, preferences, statistics
/// - `schema_migrations`: History of applied migrations
/// - `backup_info`: Information about backups and sync state
///
/// These can be added as optional fields with #[serde(default)] to maintain
/// backward compatibility with version 1.0.0 files.
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageData {
    /// Schema version for handling future migrations
    /// 
    /// This field is always set to "1.0.0" for the current implementation.
    /// When loading data, we can check this version and apply migration
    /// logic if needed.
    ///
    /// Example migration logic (future):
    /// ```no_run
    /// match data.version.as_str() {
    ///     "1.0.0" => { /* no migration needed */ }
    ///     "0.9.0" => { /* migrate from old format */ }
    ///     _ => { /* unknown version, return error */ }
    /// }
    /// ```
    pub version: String,
    
    /// All contexts, indexed by name
    ///
    /// This HashMap contains all the user's contexts and their tasks.
    /// The keys are context names (e.g., "default", "work", "personal"),
    /// and the values are Context objects containing task lists.
    ///
    /// HashMap is used because:
    /// - O(1) lookup by context name
    /// - Efficient insertion and deletion
    /// - Natural JSON representation as an object
    ///
    /// When serialized to JSON, this becomes:
    /// ```json
    /// "contexts": {
    ///   "default": { "name": "default", "tasks": [...] },
    ///   "work": { "name": "work", "tasks": [...] }
    /// }
    /// ```
    pub contexts: HashMap<String, Context>,
    
    /// The name of the currently active context
    ///
    /// This field stores which context is currently active. All task
    /// operations (add, edit, delete, list) apply to this context.
    ///
    /// Invariant: This must always refer to a key that exists in the
    /// `contexts` HashMap. This invariant is maintained by the
    /// ContextManager and validated when loading from disk.
    ///
    /// Example: If active_context is "work", then contexts.get("work")
    /// must return Some(context).
    pub active_context: String,
}

impl StorageData {
    /// Creates a new StorageData with the current version
    ///
    /// This constructor is used when converting from ContextManager to
    /// StorageData for saving to disk.
    ///
    /// # Arguments
    ///
    /// * `contexts` - HashMap of all contexts
    /// * `active_context` - Name of the active context
    ///
    /// # Returns
    ///
    /// A new StorageData with version set to "1.0.0"
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::collections::HashMap;
    /// use rust_todo::store::StorageData;
    /// use rust_todo::context::Context;
    ///
    /// let mut contexts = HashMap::new();
    /// contexts.insert("default".to_string(), Context::new("default".to_string()));
    ///
    /// let data = StorageData::new(contexts, "default".to_string());
    /// assert_eq!(data.version, "1.0.0");
    /// ```
    pub fn new(contexts: HashMap<String, Context>, active_context: String) -> Self {
        Self {
            version: "1.0.0".to_string(),
            contexts,
            active_context,
        }
    }
}

// TODO: Implement Store struct with load/save methods


#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::task::{Task, TimeHorizon, Priority};

    #[test]
    fn test_storage_data_new() {
        // Test creating a new StorageData
        let mut contexts = HashMap::new();
        contexts.insert("default".to_string(), Context::new("default".to_string()));
        
        let data = StorageData::new(contexts, "default".to_string());
        
        // Verify version is set correctly
        assert_eq!(data.version, "1.0.0");
        
        // Verify contexts are stored
        assert_eq!(data.contexts.len(), 1);
        assert!(data.contexts.contains_key("default"));
        
        // Verify active context is set
        assert_eq!(data.active_context, "default");
    }

    #[test]
    fn test_storage_data_serialization() {
        // Test that StorageData can be serialized to JSON
        let mut contexts = HashMap::new();
        contexts.insert("default".to_string(), Context::new("default".to_string()));
        
        let data = StorageData::new(contexts, "default".to_string());
        
        let json = serde_json::to_string(&data).unwrap();
        
        // Verify JSON contains expected fields
        assert!(json.contains("\"version\""));
        assert!(json.contains("\"contexts\""));
        assert!(json.contains("\"active_context\""));
        assert!(json.contains("\"1.0.0\""));
        assert!(json.contains("\"default\""));
    }

    #[test]
    fn test_storage_data_deserialization() {
        // Test that StorageData can be deserialized from JSON
        let json = r#"{
            "version": "1.0.0",
            "active_context": "default",
            "contexts": {
                "default": {
                    "name": "default",
                    "tasks": []
                }
            }
        }"#;
        
        let data: StorageData = serde_json::from_str(json).unwrap();
        
        assert_eq!(data.version, "1.0.0");
        assert_eq!(data.active_context, "default");
        assert_eq!(data.contexts.len(), 1);
        assert!(data.contexts.contains_key("default"));
    }

    #[test]
    fn test_storage_data_with_multiple_contexts() {
        // Test StorageData with multiple contexts
        let mut contexts = HashMap::new();
        contexts.insert("default".to_string(), Context::new("default".to_string()));
        contexts.insert("work".to_string(), Context::new("work".to_string()));
        contexts.insert("personal".to_string(), Context::new("personal".to_string()));
        
        let data = StorageData::new(contexts, "work".to_string());
        
        assert_eq!(data.version, "1.0.0");
        assert_eq!(data.active_context, "work");
        assert_eq!(data.contexts.len(), 3);
        assert!(data.contexts.contains_key("default"));
        assert!(data.contexts.contains_key("work"));
        assert!(data.contexts.contains_key("personal"));
    }

    #[test]
    fn test_storage_data_with_tasks() {
        // Test StorageData with contexts containing tasks
        let mut contexts = HashMap::new();
        
        let mut default_context = Context::new("default".to_string());
        default_context.add_task(Task::new(
            "Task 1".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High
        ));
        default_context.add_task(Task::new(
            "Task 2".to_string(),
            TimeHorizon::MidTerm,
            Priority::Medium
        ));
        
        contexts.insert("default".to_string(), default_context);
        
        let data = StorageData::new(contexts, "default".to_string());
        
        // Verify tasks are included
        let default_ctx = data.contexts.get("default").unwrap();
        assert_eq!(default_ctx.tasks.len(), 2);
        assert_eq!(default_ctx.tasks[0].description, "Task 1");
        assert_eq!(default_ctx.tasks[1].description, "Task 2");
    }

    #[test]
    fn test_storage_data_round_trip() {
        // Test serialization and deserialization round-trip
        let mut contexts = HashMap::new();
        
        let mut work_context = Context::new("work".to_string());
        work_context.add_task(Task::new(
            "Work task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High
        ));
        
        contexts.insert("default".to_string(), Context::new("default".to_string()));
        contexts.insert("work".to_string(), work_context);
        
        let data1 = StorageData::new(contexts, "work".to_string());
        
        // Serialize to JSON
        let json = serde_json::to_string(&data1).unwrap();
        
        // Deserialize back
        let data2: StorageData = serde_json::from_str(&json).unwrap();
        
        // Verify they match
        assert_eq!(data1.version, data2.version);
        assert_eq!(data1.active_context, data2.active_context);
        assert_eq!(data1.contexts.len(), data2.contexts.len());
        
        // Verify the work context and its task
        let work_ctx = data2.contexts.get("work").unwrap();
        assert_eq!(work_ctx.name, "work");
        assert_eq!(work_ctx.tasks.len(), 1);
        assert_eq!(work_ctx.tasks[0].description, "Work task");
    }

    #[test]
    fn test_storage_data_pretty_json() {
        // Test that StorageData can be serialized to pretty JSON
        let mut contexts = HashMap::new();
        contexts.insert("default".to_string(), Context::new("default".to_string()));
        
        let data = StorageData::new(contexts, "default".to_string());
        
        let json = serde_json::to_string_pretty(&data).unwrap();
        
        // Verify JSON is formatted with newlines and indentation
        assert!(json.contains('\n'));
        assert!(json.contains("  ")); // Indentation
        
        // Verify it can be deserialized back
        let data2: StorageData = serde_json::from_str(&json).unwrap();
        assert_eq!(data.version, data2.version);
    }

    #[test]
    fn test_storage_data_version_field() {
        // Test that version field is always "1.0.0"
        let mut contexts = HashMap::new();
        contexts.insert("test".to_string(), Context::new("test".to_string()));
        
        let data = StorageData::new(contexts, "test".to_string());
        
        assert_eq!(data.version, "1.0.0");
        
        // Verify version is included in JSON
        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("\"version\":\"1.0.0\"") || json.contains("\"version\": \"1.0.0\""));
    }
}
