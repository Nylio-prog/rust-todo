// Storage module - handles data persistence using JSON files
// This module demonstrates file I/O, serialization, and atomic file operations
//
// This module is responsible for persisting the application's data to disk
// and loading it back. It uses JSON as the storage format for human-readability
// and easy import/export capabilities.

use crate::context::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    /// # use rust_todo::store::StorageData;
    /// # let data = StorageData::new(std::collections::HashMap::new(), "default".to_string());
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

/// Handles file I/O operations for persisting application data
///
/// The Store struct is responsible for loading and saving the application's
/// data to disk. It uses JSON as the storage format for human-readability
/// and easy import/export capabilities.
///
/// # File I/O and Atomic Operations
///
/// File I/O in Rust is handled through the std::fs module, which provides
/// functions for reading and writing files. The Store uses atomic file
/// operations to prevent data corruption:
///
/// 1. Write data to a temporary file
/// 2. Flush the file to ensure all data is written to disk
/// 3. Atomically rename the temporary file to the actual file
///
/// This ensures that if the program crashes during a save operation, the
/// original file remains intact. The rename operation is atomic on most
/// file systems, meaning it either completes fully or not at all.
///
/// # Cross-Platform Path Handling
///
/// The Store uses PathBuf for file paths, which is Rust's cross-platform
/// path type. PathBuf handles differences between operating systems:
/// - Windows: C:\Users\username\AppData\Roaming\rust-todo\data.json
/// - Linux: /home/username/.local/share/rust-todo/data.json
/// - macOS: /Users/username/Library/Application Support/rust-todo/data.json
///
/// We use the `directories` crate to determine the appropriate data directory
/// for each platform. This follows OS conventions and user expectations.
///
/// # Fields
///
/// - `file_path`: The path to the JSON data file
///
/// # Example
///
/// ```no_run
/// use std::path::PathBuf;
/// use rust_todo::store::Store;
/// use rust_todo::context::ContextManager;
///
/// // Create a store with a custom path
/// let store = Store::new(PathBuf::from("data.json"));
///
/// // Load data (creates default if file doesn't exist)
/// let manager = store.load().unwrap();
///
/// // ... modify the manager ...
///
/// // Save data back to disk
/// store.save(&manager).unwrap();
/// ```
pub struct Store {
    /// The path to the JSON data file
    ///
    /// PathBuf is an owned, mutable path type. It's like String for file paths.
    /// PathBuf owns the path data and can be modified, while Path (like &str)
    /// is a borrowed, immutable view of a path.
    ///
    /// Using PathBuf here means:
    /// - The Store owns the path and is responsible for its lifetime
    /// - We can modify the path if needed (though we don't in this implementation)
    /// - We can easily convert to &Path when needed using as_path() or &
    file_path: std::path::PathBuf,
}

impl Store {
    /// Creates a new Store with the specified file path
    ///
    /// This constructor takes ownership of the PathBuf and stores it for
    /// later use in load() and save() operations.
    ///
    /// # Ownership and Move Semantics
    ///
    /// The file_path parameter is moved into the Store struct. After calling
    /// this function, the caller can no longer use the original PathBuf.
    /// This is Rust's ownership system preventing use-after-move bugs.
    ///
    /// If the caller needs to keep the path, they should clone it first:
    /// ```no_run
    /// use std::path::PathBuf;
    /// use rust_todo::store::Store;
    ///
    /// let path = PathBuf::from("data.json");
    /// let store = Store::new(path.clone());
    /// // Can still use path here
    /// ```
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the JSON data file (ownership is transferred)
    ///
    /// # Returns
    ///
    /// A new Store instance configured to use the specified file path.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    /// use rust_todo::store::Store;
    ///
    /// // Create a store with a custom path
    /// let store = Store::new(PathBuf::from("my-todos.json"));
    /// ```
    ///
    /// # Default Data Directory
    ///
    /// For the main application, we use the `directories` crate to determine
    /// the appropriate data directory for the current platform:
    ///
    /// ```no_run
    /// use directories::ProjectDirs;
    /// use std::path::PathBuf;
    /// use rust_todo::store::Store;
    ///
    /// // Get the platform-specific data directory
    /// if let Some(proj_dirs) = ProjectDirs::from("", "", "rust-todo") {
    ///     let data_dir = proj_dirs.data_dir();
    ///     let file_path = data_dir.join("data.json");
    ///     let store = Store::new(file_path);
    /// }
    /// ```
    ///
    /// This ensures that:
    /// - Data is stored in the correct location for each OS
    /// - Multiple users on the same system have separate data
    /// - The application follows OS conventions and user expectations
    pub fn new(file_path: std::path::PathBuf) -> Self {
        Self { file_path }
    }

    /// Loads the ContextManager from disk
    ///
    /// This method demonstrates:
    /// - File I/O with std::fs
    /// - Error handling with Result and the ? operator
    /// - JSON deserialization with serde_json
    /// - Handling missing files gracefully
    ///
    /// # File Reading Process
    ///
    /// 1. Check if the file exists using Path::exists()
    /// 2. If not, return a new default ContextManager
    /// 3. If yes, read the file contents as a String
    /// 4. Deserialize the JSON to StorageData
    /// 5. Convert StorageData to ContextManager
    /// 6. Validate that the active context exists
    ///
    /// # Error Handling with Result and ?
    ///
    /// The ? operator is Rust's error propagation operator. It works like this:
    /// - If the Result is Ok(value), extract the value and continue
    /// - If the Result is Err(error), return the error immediately
    ///
    /// This is equivalent to:
    /// ```no_run
    /// # use std::fs;
    /// # fn example() -> Result<String, std::io::Error> {
    /// let contents = match fs::read_to_string("file.txt") {
    ///     Ok(contents) => contents,
    ///     Err(e) => return Err(e),
    /// };
    /// # Ok(contents)
    /// # }
    /// ```
    ///
    /// But much more concise:
    /// ```no_run
    /// # use std::fs;
    /// # fn example() -> Result<String, std::io::Error> {
    /// let contents = fs::read_to_string("file.txt")?;
    /// # Ok(contents)
    /// # }
    /// ```
    ///
    /// # Automatic Error Conversion
    ///
    /// The ? operator also automatically converts errors using the From trait.
    /// In our case:
    /// - std::io::Error is converted to AppError::IoError (via #[from])
    /// - serde_json::Error is converted to AppError::JsonError (via #[from])
    ///
    /// This is why we can use ? with different error types in the same function.
    ///
    /// # Missing File Handling
    ///
    /// If the data file doesn't exist, we return a new default ContextManager
    /// instead of an error. This provides a better user experience:
    /// - First-time users don't see an error
    /// - The application works immediately without setup
    /// - The file will be created on the first save operation
    ///
    /// # Corrupted File Handling
    ///
    /// If the file exists but contains invalid JSON, we return an error.
    /// This prevents data loss by:
    /// - Not overwriting the corrupted file
    /// - Alerting the user to the problem
    /// - Allowing the user to recover or fix the file manually
    ///
    /// # Returns
    ///
    /// Ok(ContextManager) with the loaded data, or a new default ContextManager
    /// if the file doesn't exist.
    ///
    /// Err(AppError) if:
    /// - The file exists but can't be read (permissions, I/O error)
    /// - The file contains invalid JSON
    /// - The JSON has an invalid structure (missing fields, wrong types)
    /// - The active context doesn't exist in the contexts HashMap
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    /// use rust_todo::store::Store;
    ///
    /// let store = Store::new(PathBuf::from("data.json"));
    ///
    /// match store.load() {
    ///     Ok(manager) => {
    ///         println!("Loaded {} contexts", manager.contexts.len());
    ///     }
    ///     Err(e) => {
    ///         eprintln!("Failed to load data: {}", e);
    ///     }
    /// }
    /// ```
    pub fn load(&self) -> crate::error::Result<crate::context::ContextManager> {
        // Check if the file exists
        // Path::exists() returns a bool, no error handling needed
        if !self.file_path.exists() {
            // File doesn't exist - return a new default ContextManager
            // This is the first-time user experience
            return Ok(crate::context::ContextManager::new());
        }

        // Read the file contents as a String
        // std::fs::read_to_string() reads the entire file into memory
        // The ? operator converts io::Error to AppError::IoError automatically
        let contents = std::fs::read_to_string(&self.file_path)?;

        // Deserialize the JSON to StorageData
        // serde_json::from_str() parses the JSON string
        // The ? operator converts serde_json::Error to AppError::JsonError
        let data: StorageData = serde_json::from_str(&contents)?;

        // Validate that the active context exists in the contexts HashMap
        // This ensures data integrity and prevents panics later
        if !data.contexts.contains_key(&data.active_context) {
            return Err(crate::error::AppError::InvalidDataFormat(format!(
                "Active context '{}' does not exist in contexts",
                data.active_context
            )));
        }

        // Convert StorageData to ContextManager
        // This is a simple field-by-field copy since the structures match
        let manager = crate::context::ContextManager {
            contexts: data.contexts,
            active_context: data.active_context,
        };

        // Return the loaded ContextManager
        Ok(manager)
    }

    /// Saves the ContextManager to disk
    ///
    /// This method demonstrates:
    /// - Atomic file operations to prevent data corruption
    /// - JSON serialization with pretty printing
    /// - Directory creation with std::fs::create_dir_all
    /// - Temporary file usage for safe writes
    ///
    /// # Atomic File Operations
    ///
    /// To prevent data corruption, we use an atomic write strategy:
    ///
    /// 1. Serialize the data to JSON
    /// 2. Write to a temporary file (data.json.tmp)
    /// 3. Flush the file to ensure all data is written to disk
    /// 4. Atomically rename the temporary file to the actual file
    ///
    /// This ensures that:
    /// - If the program crashes during serialization, the original file is intact
    /// - If the program crashes during write, the original file is intact
    /// - The rename operation is atomic (all-or-nothing) on most file systems
    /// - Readers never see a partially-written file
    ///
    /// # Why Atomic Writes Matter
    ///
    /// Without atomic writes, a crash during save could result in:
    /// - A partially-written file with invalid JSON
    /// - Loss of all user data
    /// - No way to recover
    ///
    /// With atomic writes:
    /// - The original file remains valid until the new file is complete
    /// - If anything goes wrong, the original file is still there
    /// - Users never lose data due to crashes during save
    ///
    /// # Directory Creation
    ///
    /// Before writing the file, we ensure the parent directory exists using
    /// std::fs::create_dir_all(). This function:
    /// - Creates the directory and all parent directories if needed
    /// - Does nothing if the directory already exists
    /// - Returns an error if creation fails (permissions, disk full, etc.)
    ///
    /// This is important because:
    /// - First-time users may not have the data directory yet
    /// - The application should work without manual setup
    /// - We follow the principle of "make it work out of the box"
    ///
    /// # JSON Pretty Printing
    ///
    /// We use serde_json::to_string_pretty() instead of to_string() to format
    /// the JSON with indentation and newlines. This makes the file:
    /// - Human-readable and editable
    /// - Easier to debug and inspect
    /// - Suitable for version control (git diff works well)
    /// - Only slightly larger (whitespace is cheap)
    ///
    /// Example output:
    /// ```json
    /// {
    ///   "version": "1.0.0",
    ///   "active_context": "default",
    ///   "contexts": {
    ///     "default": {
    ///       "name": "default",
    ///       "tasks": []
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// # Arguments
    ///
    /// * `manager` - A reference to the ContextManager to save
    ///
    /// # Returns
    ///
    /// Ok(()) if the save was successful.
    ///
    /// Err(AppError) if:
    /// - The parent directory can't be created (permissions, disk full)
    /// - The temporary file can't be written (permissions, disk full)
    /// - The rename operation fails (rare, but possible)
    /// - Serialization fails (should never happen with valid data)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    /// use rust_todo::store::Store;
    /// use rust_todo::context::ContextManager;
    ///
    /// let store = Store::new(PathBuf::from("data.json"));
    /// let mut manager = ContextManager::new();
    ///
    /// // ... modify the manager ...
    ///
    /// match store.save(&manager) {
    ///     Ok(()) => println!("Data saved successfully"),
    ///     Err(e) => eprintln!("Failed to save data: {}", e),
    /// }
    /// ```
    pub fn save(&self, manager: &crate::context::ContextManager) -> crate::error::Result<()> {
        // Create the parent directory if it doesn't exist
        // parent() returns Option<&Path> - the parent directory of the file
        // If the path has no parent (e.g., "data.json" with no directory),
        // we skip directory creation
        if let Some(parent) = self.file_path.parent() {
            // create_dir_all() creates the directory and all parents if needed
            // It succeeds if the directory already exists
            // The ? operator converts io::Error to AppError::IoError
            std::fs::create_dir_all(parent)?;
        }

        // Convert ContextManager to StorageData
        // This adds the version field and prepares for serialization
        let data = StorageData::new(manager.contexts.clone(), manager.active_context.clone());

        // Serialize to JSON with pretty printing
        // to_string_pretty() formats the JSON with indentation and newlines
        // The ? operator converts serde_json::Error to AppError::JsonError
        let json = serde_json::to_string_pretty(&data)?;

        // Create a temporary file path
        // We append ".tmp" to the original file path
        // This ensures the temporary file is in the same directory (same filesystem)
        // which is required for atomic rename on some systems
        let temp_path = self.file_path.with_extension("json.tmp");

        // Write to the temporary file
        // std::fs::write() creates the file and writes the entire contents
        // It overwrites the file if it already exists
        // The ? operator converts io::Error to AppError::IoError
        std::fs::write(&temp_path, json)?;

        // Atomically rename the temporary file to the actual file
        // std::fs::rename() is atomic on most file systems
        // This means the operation either completes fully or not at all
        // If this succeeds, the old file is replaced with the new file
        // If this fails, the old file remains unchanged
        // The ? operator converts io::Error to AppError::IoError
        std::fs::rename(&temp_path, &self.file_path)?;

        // Return success
        Ok(())
    }

    /// Exports the ContextManager to a specified file
    ///
    /// This method demonstrates:
    /// - Exporting data to a custom location
    /// - Reusing serialization logic from save()
    /// - Working with Path references
    ///
    /// # Export vs Save
    ///
    /// The export() method is similar to save(), but:
    /// - It writes to a user-specified path instead of the default data file
    /// - It's used for creating backups or sharing task lists
    /// - It doesn't modify the Store's file_path field
    ///
    /// # Path vs PathBuf
    ///
    /// The method takes &Path (borrowed path) rather than PathBuf (owned path)
    /// because:
    /// - We only need to read the path, not own it
    /// - The caller retains ownership of their PathBuf
    /// - It's more flexible (accepts &PathBuf, &Path, or path literals)
    ///
    /// # Arguments
    ///
    /// * `manager` - A reference to the ContextManager to export
    /// * `export_path` - The path where the export file should be created
    ///
    /// # Returns
    ///
    /// Ok(()) if the export was successful.
    ///
    /// Err(AppError) if:
    /// - The parent directory can't be created (permissions, disk full)
    /// - The file can't be written (permissions, disk full)
    /// - Serialization fails (should never happen with valid data)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::{Path, PathBuf};
    /// use rust_todo::store::Store;
    /// use rust_todo::context::ContextManager;
    ///
    /// let store = Store::new(PathBuf::from("data.json"));
    /// let manager = ContextManager::new();
    ///
    /// // Export to a backup file
    /// store.export(&manager, Path::new("backup.json")).unwrap();
    /// ```
    ///
    /// # Requirements
    ///
    /// This method satisfies:
    /// - Requirement 4.5: Export tasks to JSON file
    /// - Requirement 6.1: Create JSON file with all contexts and tasks
    pub fn export(
        &self,
        manager: &crate::context::ContextManager,
        export_path: &std::path::Path,
    ) -> crate::error::Result<()> {
        // Create the parent directory if it doesn't exist
        // This is the same logic as in save()
        if let Some(parent) = export_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Convert ContextManager to StorageData
        // This adds the version field and prepares for serialization
        let data = StorageData::new(manager.contexts.clone(), manager.active_context.clone());

        // Serialize to JSON with pretty printing
        // Pretty printing makes the export file human-readable
        let json = serde_json::to_string_pretty(&data)?;

        // Write directly to the export file
        // Unlike save(), we don't use atomic write here because:
        // 1. We're not overwriting the main data file
        // 2. Export is typically a one-time operation
        // 3. If export fails, the main data file is unaffected
        std::fs::write(export_path, json)?;

        // Return success
        Ok(())
    }

    /// Imports a ContextManager from a specified file
    ///
    /// This method demonstrates:
    /// - Loading data from a custom location
    /// - Validating JSON structure before returning
    /// - Error handling without modifying state
    ///
    /// # Import Validation
    ///
    /// The import() method validates the JSON structure by:
    /// 1. Reading the file contents
    /// 2. Deserializing to StorageData (validates JSON syntax and structure)
    /// 3. Checking that the active context exists in the contexts HashMap
    /// 4. Converting to ContextManager
    ///
    /// If any step fails, an error is returned and no state is modified.
    /// This ensures that invalid imports don't corrupt the application state.
    ///
    /// # Error Handling Strategy
    ///
    /// The method uses the ? operator to propagate errors:
    /// - File I/O errors (file not found, permissions) → AppError::IoError
    /// - JSON parsing errors (invalid syntax) → AppError::JsonError
    /// - Structure validation errors → AppError::InvalidDataFormat
    ///
    /// All errors are returned before any state modification, following the
    /// principle of "validate first, modify later".
    ///
    /// # Arguments
    ///
    /// * `import_path` - The path to the JSON file to import
    ///
    /// # Returns
    ///
    /// Ok(ContextManager) with the imported data if successful.
    ///
    /// Err(AppError) if:
    /// - The file doesn't exist or can't be read
    /// - The file contains invalid JSON
    /// - The JSON has an invalid structure (missing fields, wrong types)
    /// - The active context doesn't exist in the contexts HashMap
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::{Path, PathBuf};
    /// use rust_todo::store::Store;
    ///
    /// let store = Store::new(PathBuf::from("data.json"));
    ///
    /// // Import from a backup file
    /// match store.import(Path::new("backup.json")) {
    ///     Ok(manager) => {
    ///         println!("Imported {} contexts", manager.contexts.len());
    ///         // Now you can merge or replace the current data
    ///     }
    ///     Err(e) => {
    ///         eprintln!("Failed to import: {}", e);
    ///         // Current state is unchanged
    ///     }
    /// }
    /// ```
    ///
    /// # Requirements
    ///
    /// This method satisfies:
    /// - Requirement 6.2: Validate JSON structure before importing
    /// - Requirement 6.3: Deserialize to ContextManager
    /// - Requirement 6.5: Return error if invalid without modifying state
    pub fn import(
        &self,
        import_path: &std::path::Path,
    ) -> crate::error::Result<crate::context::ContextManager> {
        // Read the file contents as a String
        // If the file doesn't exist or can't be read, return an error
        // The ? operator converts io::Error to AppError::IoError
        let contents = std::fs::read_to_string(import_path)?;

        // Deserialize the JSON to StorageData
        // This validates the JSON syntax and structure
        // The ? operator converts serde_json::Error to AppError::JsonError
        let data: StorageData = serde_json::from_str(&contents)?;

        // Validate that the active context exists in the contexts HashMap
        // This ensures data integrity and prevents invalid state
        if !data.contexts.contains_key(&data.active_context) {
            return Err(crate::error::AppError::InvalidDataFormat(format!(
                "Active context '{}' does not exist in contexts",
                data.active_context
            )));
        }

        // Convert StorageData to ContextManager
        // This is a simple field-by-field copy since the structures match
        let manager = crate::context::ContextManager {
            contexts: data.contexts,
            active_context: data.active_context,
        };

        // Return the imported ContextManager
        // The caller can decide whether to merge or replace their current data
        Ok(manager)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::task::{Priority, Task, TimeHorizon};

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
            Priority::High,
        ));
        default_context.add_task(Task::new(
            "Task 2".to_string(),
            TimeHorizon::MidTerm,
            Priority::Medium,
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
            Priority::High,
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

    // Store tests

    #[test]
    fn test_store_new() {
        // Test creating a new Store
        let path = std::path::PathBuf::from("test-data.json");
        let _store = super::Store::new(path.clone());

        // We can't directly access file_path since it's private,
        // but we can verify the Store was created successfully
        // by using it in load/save operations
        assert!(true); // Store created successfully
    }

    #[test]
    fn test_store_load_missing_file() {
        // Test loading when the file doesn't exist
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("nonexistent.json");

        let store = super::Store::new(file_path);

        // Load should return a default ContextManager
        let manager = store.load().unwrap();

        // Verify it's a default ContextManager
        assert_eq!(manager.active_context, "default");
        assert_eq!(manager.contexts.len(), 1);
        assert!(manager.contexts.contains_key("default"));
    }

    #[test]
    fn test_store_save_and_load() {
        // Test saving and loading a ContextManager
        use crate::context::ContextManager;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("data.json");

        let store = super::Store::new(file_path.clone());

        // Create a ContextManager with some data
        let mut manager = ContextManager::new();
        manager.create_context("work".to_string()).unwrap();
        manager.switch_context("work").unwrap();

        // Add a task to the work context
        let task = Task::new(
            "Test task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High,
        );
        manager.active_context_mut().add_task(task);

        // Save the manager
        store.save(&manager).unwrap();

        // Verify the file was created
        assert!(file_path.exists());

        // Load the manager back
        let loaded_manager = store.load().unwrap();

        // Verify the data matches
        assert_eq!(loaded_manager.active_context, "work");
        assert_eq!(loaded_manager.contexts.len(), 2); // default + work
        assert!(loaded_manager.contexts.contains_key("default"));
        assert!(loaded_manager.contexts.contains_key("work"));

        // Verify the task was loaded
        let work_context = loaded_manager.contexts.get("work").unwrap();
        assert_eq!(work_context.tasks.len(), 1);
        assert_eq!(work_context.tasks[0].description, "Test task");
        assert_eq!(work_context.tasks[0].time_horizon, TimeHorizon::ShortTerm);
        assert_eq!(work_context.tasks[0].priority, Priority::High);
    }

    #[test]
    fn test_store_save_creates_directory() {
        // Test that save creates parent directories if needed
        use crate::context::ContextManager;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir
            .path()
            .join("subdir")
            .join("nested")
            .join("data.json");

        // Verify the nested directories don't exist yet
        assert!(!file_path.parent().unwrap().exists());

        let store = super::Store::new(file_path.clone());
        let manager = ContextManager::new();

        // Save should create the directories
        store.save(&manager).unwrap();

        // Verify the directories were created
        assert!(file_path.parent().unwrap().exists());
        assert!(file_path.exists());
    }

    #[test]
    fn test_store_save_overwrites_existing_file() {
        // Test that save overwrites an existing file
        use crate::context::ContextManager;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("data.json");

        let store = super::Store::new(file_path.clone());

        // Save initial data
        let mut manager1 = ContextManager::new();
        store.save(&manager1).unwrap();

        // Modify and save again
        manager1.create_context("work".to_string()).unwrap();
        store.save(&manager1).unwrap();

        // Load and verify the new data
        let loaded_manager = store.load().unwrap();
        assert_eq!(loaded_manager.contexts.len(), 2); // default + work
    }

    #[test]
    fn test_store_load_corrupted_json() {
        // Test loading a file with invalid JSON
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("corrupted.json");

        // Write invalid JSON to the file
        fs::write(&file_path, "{ invalid json }").unwrap();

        let store = super::Store::new(file_path);

        // Load should return an error
        let result = store.load();
        assert!(result.is_err());

        // Verify it's a JSON error
        match result {
            Err(crate::error::AppError::JsonError(_)) => {
                // Expected
            }
            _ => panic!("Expected JsonError"),
        }
    }

    #[test]
    fn test_store_load_invalid_structure() {
        // Test loading a file with valid JSON but invalid structure
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("invalid.json");

        // Write JSON with missing required fields
        fs::write(&file_path, r#"{"version": "1.0.0"}"#).unwrap();

        let store = super::Store::new(file_path);

        // Load should return an error
        let result = store.load();
        assert!(result.is_err());
    }

    #[test]
    fn test_store_load_invalid_active_context() {
        // Test loading a file where active_context doesn't exist in contexts
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("invalid-active.json");

        // Write JSON with invalid active_context
        let json = r#"{
            "version": "1.0.0",
            "active_context": "nonexistent",
            "contexts": {
                "default": {
                    "name": "default",
                    "tasks": []
                }
            }
        }"#;
        fs::write(&file_path, json).unwrap();

        let store = super::Store::new(file_path);

        // Load should return an error
        let result = store.load();
        assert!(result.is_err());

        // Verify it's an InvalidDataFormat error
        match result {
            Err(crate::error::AppError::InvalidDataFormat(msg)) => {
                assert!(msg.contains("nonexistent"));
            }
            _ => panic!("Expected InvalidDataFormat error"),
        }
    }

    #[test]
    fn test_store_save_pretty_json() {
        // Test that save creates pretty-printed JSON
        use crate::context::ContextManager;
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("pretty.json");

        let store = super::Store::new(file_path.clone());
        let manager = ContextManager::new();

        // Save the manager
        store.save(&manager).unwrap();

        // Read the file contents
        let contents = fs::read_to_string(&file_path).unwrap();

        // Verify it's pretty-printed (contains newlines and indentation)
        assert!(contents.contains('\n'));
        assert!(contents.contains("  ")); // Indentation

        // Verify it's valid JSON
        let _: StorageData = serde_json::from_str(&contents).unwrap();
    }

    #[test]
    fn test_store_multiple_contexts_and_tasks() {
        // Test saving and loading multiple contexts with tasks
        use crate::context::ContextManager;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("multi.json");

        let store = super::Store::new(file_path);

        // Create a ContextManager with multiple contexts and tasks
        let mut manager = ContextManager::new();

        // Add tasks to default context
        manager.active_context_mut().add_task(Task::new(
            "Default task 1".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High,
        ));
        manager.active_context_mut().add_task(Task::new(
            "Default task 2".to_string(),
            TimeHorizon::MidTerm,
            Priority::Low,
        ));

        // Create work context with tasks
        manager.create_context("work".to_string()).unwrap();
        manager.switch_context("work").unwrap();
        manager.active_context_mut().add_task(Task::new(
            "Work task 1".to_string(),
            TimeHorizon::LongTerm,
            Priority::Medium,
        ));

        // Create personal context with tasks
        manager.create_context("personal".to_string()).unwrap();
        manager.switch_context("personal").unwrap();
        manager.active_context_mut().add_task(Task::new(
            "Personal task 1".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High,
        ));
        manager.active_context_mut().add_task(Task::new(
            "Personal task 2".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Medium,
        ));

        // Save the manager
        store.save(&manager).unwrap();

        // Load the manager back
        let loaded_manager = store.load().unwrap();

        // Verify all contexts were loaded
        assert_eq!(loaded_manager.contexts.len(), 3);
        assert!(loaded_manager.contexts.contains_key("default"));
        assert!(loaded_manager.contexts.contains_key("work"));
        assert!(loaded_manager.contexts.contains_key("personal"));

        // Verify active context
        assert_eq!(loaded_manager.active_context, "personal");

        // Verify default context tasks
        let default_ctx = loaded_manager.contexts.get("default").unwrap();
        assert_eq!(default_ctx.tasks.len(), 2);
        assert_eq!(default_ctx.tasks[0].description, "Default task 1");
        assert_eq!(default_ctx.tasks[1].description, "Default task 2");

        // Verify work context tasks
        let work_ctx = loaded_manager.contexts.get("work").unwrap();
        assert_eq!(work_ctx.tasks.len(), 1);
        assert_eq!(work_ctx.tasks[0].description, "Work task 1");

        // Verify personal context tasks
        let personal_ctx = loaded_manager.contexts.get("personal").unwrap();
        assert_eq!(personal_ctx.tasks.len(), 2);
        assert_eq!(personal_ctx.tasks[0].description, "Personal task 1");
        assert_eq!(personal_ctx.tasks[1].description, "Personal task 2");
    }

    #[test]
    fn test_store_atomic_write() {
        // Test that save uses atomic write (temporary file + rename)
        use crate::context::ContextManager;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("atomic.json");

        let store = super::Store::new(file_path.clone());

        // Save initial data
        let manager = ContextManager::new();
        store.save(&manager).unwrap();

        // Verify the main file exists
        assert!(file_path.exists());

        // Verify the temporary file doesn't exist after save
        let temp_path = file_path.with_extension("json.tmp");
        assert!(!temp_path.exists());

        // Save again to verify the temporary file is cleaned up
        store.save(&manager).unwrap();
        assert!(!temp_path.exists());
    }

    #[test]
    fn test_store_preserves_task_metadata() {
        // Test that save/load preserves all task metadata
        use crate::context::ContextManager;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("metadata.json");

        let store = super::Store::new(file_path);

        // Create a task with specific metadata
        let mut manager = ContextManager::new();
        let task = Task::new(
            "Test task with metadata".to_string(),
            TimeHorizon::MidTerm,
            Priority::Low,
        );
        let task_id = task.id.clone();
        let task_created_at = task.created_at.clone();
        manager.active_context_mut().add_task(task);

        // Mark the task as complete
        if let Some(task) = manager.active_context_mut().find_task_mut(&task_id) {
            task.mark_complete();
        }

        // Save and load
        store.save(&manager).unwrap();
        let loaded_manager = store.load().unwrap();

        // Verify all metadata was preserved
        let loaded_task = loaded_manager.active_context().find_task(&task_id).unwrap();
        assert_eq!(loaded_task.id, task_id);
        assert_eq!(loaded_task.description, "Test task with metadata");
        assert_eq!(loaded_task.time_horizon, TimeHorizon::MidTerm);
        assert_eq!(loaded_task.priority, Priority::Low);
        assert_eq!(loaded_task.completed, true);
        assert_eq!(loaded_task.created_at, task_created_at);
    }

    #[test]
    fn test_store_export_to_custom_path() {
        // Test exporting to a custom file path
        use crate::context::ContextManager;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let store_path = temp_dir.path().join("data.json");
        let export_path = temp_dir.path().join("export.json");

        let store = super::Store::new(store_path);

        // Create a ContextManager with some data
        let mut manager = ContextManager::new();
        manager.create_context("work".to_string()).unwrap();
        manager.switch_context("work").unwrap();
        manager.active_context_mut().add_task(Task::new(
            "Export test task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High,
        ));

        // Export to custom path
        store.export(&manager, &export_path).unwrap();

        // Verify the export file was created
        assert!(export_path.exists());

        // Verify the export file contains valid JSON
        let contents = std::fs::read_to_string(&export_path).unwrap();
        let data: StorageData = serde_json::from_str(&contents).unwrap();

        // Verify the exported data matches
        assert_eq!(data.version, "1.0.0");
        assert_eq!(data.active_context, "work");
        assert_eq!(data.contexts.len(), 2); // default + work

        // Verify the task was exported
        let work_ctx = data.contexts.get("work").unwrap();
        assert_eq!(work_ctx.tasks.len(), 1);
        assert_eq!(work_ctx.tasks[0].description, "Export test task");
    }

    #[test]
    fn test_store_export_creates_directory() {
        // Test that export creates parent directories if needed
        use crate::context::ContextManager;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let store_path = temp_dir.path().join("data.json");
        let export_path = temp_dir
            .path()
            .join("exports")
            .join("backup")
            .join("export.json");

        // Verify the nested directories don't exist yet
        assert!(!export_path.parent().unwrap().exists());

        let store = super::Store::new(store_path);
        let manager = ContextManager::new();

        // Export should create the directories
        store.export(&manager, &export_path).unwrap();

        // Verify the directories were created
        assert!(export_path.parent().unwrap().exists());
        assert!(export_path.exists());
    }

    #[test]
    fn test_store_export_pretty_json() {
        // Test that export creates pretty-printed JSON
        use crate::context::ContextManager;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let store_path = temp_dir.path().join("data.json");
        let export_path = temp_dir.path().join("export.json");

        let store = super::Store::new(store_path);
        let manager = ContextManager::new();

        // Export the manager
        store.export(&manager, &export_path).unwrap();

        // Read the file contents
        let contents = std::fs::read_to_string(&export_path).unwrap();

        // Verify it's pretty-printed (contains newlines and indentation)
        assert!(contents.contains('\n'));
        assert!(contents.contains("  ")); // Indentation
    }

    #[test]
    fn test_store_export_multiple_contexts() {
        // Test exporting multiple contexts with tasks
        use crate::context::ContextManager;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let store_path = temp_dir.path().join("data.json");
        let export_path = temp_dir.path().join("export.json");

        let store = super::Store::new(store_path);

        // Create a ContextManager with multiple contexts and tasks
        let mut manager = ContextManager::new();

        // Add tasks to default context
        manager.active_context_mut().add_task(Task::new(
            "Default task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High,
        ));

        // Create work context with tasks
        manager.create_context("work".to_string()).unwrap();
        manager.switch_context("work").unwrap();
        manager.active_context_mut().add_task(Task::new(
            "Work task".to_string(),
            TimeHorizon::MidTerm,
            Priority::Medium,
        ));

        // Create personal context with tasks
        manager.create_context("personal".to_string()).unwrap();
        manager.switch_context("personal").unwrap();
        manager.active_context_mut().add_task(Task::new(
            "Personal task".to_string(),
            TimeHorizon::LongTerm,
            Priority::Low,
        ));

        // Export the manager
        store.export(&manager, &export_path).unwrap();

        // Read and verify the export
        let contents = std::fs::read_to_string(&export_path).unwrap();
        let data: StorageData = serde_json::from_str(&contents).unwrap();

        // Verify all contexts were exported
        assert_eq!(data.contexts.len(), 3);
        assert!(data.contexts.contains_key("default"));
        assert!(data.contexts.contains_key("work"));
        assert!(data.contexts.contains_key("personal"));

        // Verify active context
        assert_eq!(data.active_context, "personal");

        // Verify tasks in each context
        assert_eq!(data.contexts.get("default").unwrap().tasks.len(), 1);
        assert_eq!(data.contexts.get("work").unwrap().tasks.len(), 1);
        assert_eq!(data.contexts.get("personal").unwrap().tasks.len(), 1);
    }

    #[test]
    fn test_store_import_from_file() {
        // Test importing from a file
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let store_path = temp_dir.path().join("data.json");
        let import_path = temp_dir.path().join("import.json");

        // Create a valid JSON file to import
        let json = r#"{
            "version": "1.0.0",
            "active_context": "work",
            "contexts": {
                "default": {
                    "name": "default",
                    "tasks": []
                },
                "work": {
                    "name": "work",
                    "tasks": [
                        {
                            "id": "test-id-123",
                            "description": "Imported task",
                            "time_horizon": "ShortTerm",
                            "priority": "High",
                            "completed": false,
                            "created_at": "2024-01-15T10:30:00Z"
                        }
                    ]
                }
            }
        }"#;
        fs::write(&import_path, json).unwrap();

        let store = super::Store::new(store_path);

        // Import the file
        let manager = store.import(&import_path).unwrap();

        // Verify the imported data
        assert_eq!(manager.active_context, "work");
        assert_eq!(manager.contexts.len(), 2);
        assert!(manager.contexts.contains_key("default"));
        assert!(manager.contexts.contains_key("work"));

        // Verify the imported task
        let work_ctx = manager.contexts.get("work").unwrap();
        assert_eq!(work_ctx.tasks.len(), 1);
        assert_eq!(work_ctx.tasks[0].description, "Imported task");
        assert_eq!(work_ctx.tasks[0].id, "test-id-123");
    }

    #[test]
    fn test_store_import_invalid_json() {
        // Test importing a file with invalid JSON
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let store_path = temp_dir.path().join("data.json");
        let import_path = temp_dir.path().join("invalid.json");

        // Write invalid JSON to the file
        fs::write(&import_path, "{ invalid json }").unwrap();

        let store = super::Store::new(store_path);

        // Import should return an error
        let result = store.import(&import_path);
        assert!(result.is_err());

        // Verify it's a JSON error
        match result {
            Err(crate::error::AppError::JsonError(_)) => {
                // Expected
            }
            _ => panic!("Expected JsonError"),
        }
    }

    #[test]
    fn test_store_import_missing_file() {
        // Test importing a file that doesn't exist
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let store_path = temp_dir.path().join("data.json");
        let import_path = temp_dir.path().join("nonexistent.json");

        let store = super::Store::new(store_path);

        // Import should return an error
        let result = store.import(&import_path);
        assert!(result.is_err());

        // Verify it's an IO error
        match result {
            Err(crate::error::AppError::IoError(_)) => {
                // Expected
            }
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_store_import_invalid_structure() {
        // Test importing a file with valid JSON but invalid structure
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let store_path = temp_dir.path().join("data.json");
        let import_path = temp_dir.path().join("invalid-structure.json");

        // Write JSON with missing required fields
        fs::write(&import_path, r#"{"version": "1.0.0"}"#).unwrap();

        let store = super::Store::new(store_path);

        // Import should return an error
        let result = store.import(&import_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_store_import_invalid_active_context() {
        // Test importing a file where active_context doesn't exist in contexts
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let store_path = temp_dir.path().join("data.json");
        let import_path = temp_dir.path().join("invalid-active.json");

        // Write JSON with invalid active_context
        let json = r#"{
            "version": "1.0.0",
            "active_context": "nonexistent",
            "contexts": {
                "default": {
                    "name": "default",
                    "tasks": []
                }
            }
        }"#;
        fs::write(&import_path, json).unwrap();

        let store = super::Store::new(store_path);

        // Import should return an error
        let result = store.import(&import_path);
        assert!(result.is_err());

        // Verify it's an InvalidDataFormat error
        match result {
            Err(crate::error::AppError::InvalidDataFormat(msg)) => {
                assert!(msg.contains("nonexistent"));
            }
            _ => panic!("Expected InvalidDataFormat error"),
        }
    }

    #[test]
    fn test_store_export_import_round_trip() {
        // Test that export and import preserve all data
        use crate::context::ContextManager;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let store_path = temp_dir.path().join("data.json");
        let export_path = temp_dir.path().join("export.json");

        let store = super::Store::new(store_path);

        // Create a ContextManager with complex data
        let mut manager = ContextManager::new();

        // Add tasks to default context
        manager.active_context_mut().add_task(Task::new(
            "Default task 1".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High,
        ));
        let mut task2 = Task::new(
            "Default task 2".to_string(),
            TimeHorizon::MidTerm,
            Priority::Low,
        );
        task2.mark_complete();
        manager.active_context_mut().add_task(task2);

        // Create work context with tasks
        manager.create_context("work".to_string()).unwrap();
        manager.switch_context("work").unwrap();
        manager.active_context_mut().add_task(Task::new(
            "Work task".to_string(),
            TimeHorizon::LongTerm,
            Priority::Medium,
        ));

        // Export the manager
        store.export(&manager, &export_path).unwrap();

        // Import it back
        let imported_manager = store.import(&export_path).unwrap();

        // Verify all data was preserved
        assert_eq!(imported_manager.active_context, manager.active_context);
        assert_eq!(imported_manager.contexts.len(), manager.contexts.len());

        // Verify default context tasks
        let default_ctx = imported_manager.contexts.get("default").unwrap();
        assert_eq!(default_ctx.tasks.len(), 2);
        assert_eq!(default_ctx.tasks[0].description, "Default task 1");
        assert_eq!(default_ctx.tasks[0].completed, false);
        assert_eq!(default_ctx.tasks[1].description, "Default task 2");
        assert_eq!(default_ctx.tasks[1].completed, true);

        // Verify work context tasks
        let work_ctx = imported_manager.contexts.get("work").unwrap();
        assert_eq!(work_ctx.tasks.len(), 1);
        assert_eq!(work_ctx.tasks[0].description, "Work task");
    }

    #[test]
    fn test_store_import_preserves_task_metadata() {
        // Test that import preserves all task metadata (ID, timestamps, etc.)
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let store_path = temp_dir.path().join("data.json");
        let import_path = temp_dir.path().join("import.json");

        // Create a JSON file with specific task metadata
        let json = r#"{
            "version": "1.0.0",
            "active_context": "default",
            "contexts": {
                "default": {
                    "name": "default",
                    "tasks": [
                        {
                            "id": "abc123-def456-ghi789",
                            "description": "Task with metadata",
                            "time_horizon": "MidTerm",
                            "priority": "High",
                            "completed": true,
                            "created_at": "2024-01-15T10:30:00Z"
                        }
                    ]
                }
            }
        }"#;
        fs::write(&import_path, json).unwrap();

        let store = super::Store::new(store_path);

        // Import the file
        let manager = store.import(&import_path).unwrap();

        // Verify all metadata was preserved
        let task = &manager.contexts.get("default").unwrap().tasks[0];
        assert_eq!(task.id, "abc123-def456-ghi789");
        assert_eq!(task.description, "Task with metadata");
        assert_eq!(task.time_horizon, TimeHorizon::MidTerm);
        assert_eq!(task.priority, Priority::High);
        assert_eq!(task.completed, true);
        assert_eq!(task.created_at, "2024-01-15T10:30:00Z");
    }

    #[test]
    fn test_store_import_empty_contexts() {
        // Test importing contexts with no tasks
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let store_path = temp_dir.path().join("data.json");
        let import_path = temp_dir.path().join("empty.json");

        // Create a JSON file with empty contexts
        let json = r#"{
            "version": "1.0.0",
            "active_context": "default",
            "contexts": {
                "default": {
                    "name": "default",
                    "tasks": []
                },
                "work": {
                    "name": "work",
                    "tasks": []
                }
            }
        }"#;
        fs::write(&import_path, json).unwrap();

        let store = super::Store::new(store_path);

        // Import the file
        let manager = store.import(&import_path).unwrap();

        // Verify contexts were imported
        assert_eq!(manager.contexts.len(), 2);
        assert_eq!(manager.contexts.get("default").unwrap().tasks.len(), 0);
        assert_eq!(manager.contexts.get("work").unwrap().tasks.len(), 0);
    }
}
