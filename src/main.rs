// Main entry point for the Rust CLI Todo Application
// This file demonstrates program structure, error handling, and command routing
//
// This is the main entry point for the todo application. It:
// 1. Parses command-line arguments using clap
// 2. Initializes the storage system
// 3. Loads the current state from disk
// 4. Routes commands to appropriate handlers
// 5. Saves the updated state back to disk
// 6. Handles errors and displays them to the user
//
// # Key Rust Concepts Demonstrated
//
// - **Main Function**: The entry point for Rust programs
// - **Error Handling**: Using Result type and the ? operator
// - **Pattern Matching**: Using match to handle different commands
// - **Ownership**: Passing references vs. owned values
// - **Module System**: Importing and using code from other modules

use clap::Parser;
use colored::*;
use std::path::PathBuf;
use std::str::FromStr;

// Import our modules
use rust_todo::cli::{Cli, Commands, ContextAction};
use rust_todo::context::ContextManager;
use rust_todo::display::{display_contexts, display_tasks};
use rust_todo::error::{AppError, Result};
use rust_todo::store::Store;
use rust_todo::task::{Priority, Task, TimeHorizon};

/// Main function - the entry point for the application
///
/// In Rust, the main function is where program execution begins. Unlike some languages,
/// Rust's main function can return a Result, which allows us to use the ? operator for
/// error handling throughout the function.
///
/// # Return Type
///
/// We return Result<()> which means:
/// - Ok(()) if the program completes successfully
/// - Err(AppError) if an error occurs
///
/// When main returns an Err, Rust will:
/// 1. Print the error message to stderr
/// 2. Exit with a non-zero status code
/// 3. This is the standard Unix convention for indicating failure
///
/// # Error Handling Strategy
///
/// We use the ? operator to propagate errors up to main, where they are handled
/// by displaying a user-friendly error message. This keeps the code clean and
/// avoids nested error handling.
///
/// # Program Flow
///
/// 1. Parse CLI arguments
/// 2. Initialize storage
/// 3. Load current state
/// 4. Execute command
/// 5. Save updated state
/// 6. Display success message
///
/// # Example
///
/// ```bash
/// # Run the program
/// cargo run -- add "Write tests" -t short -p high
/// ```
fn main() -> Result<()> {
    // Parse command-line arguments using clap
    // The parse() method is provided by the Parser derive macro
    // It will:
    // - Parse arguments from std::env::args()
    // - Validate arguments according to our CLI definition
    // - Generate help messages if --help is used
    // - Exit with an error if arguments are invalid
    let cli = Cli::parse();

    // Initialize the storage system
    // We use the directories crate to find the appropriate data directory for the OS
    let store = get_store()?;

    // Load the current state from disk
    // If the file doesn't exist, this creates a new default ContextManager
    // The ? operator propagates any errors (e.g., corrupted file, permission denied)
    let mut manager = store.load()?;

    // Route the command to the appropriate handler
    // We use pattern matching to handle each command variant
    // Each handler modifies the manager and returns a Result
    match cli.command {
        Commands::Add {
            description,
            horizon,
            priority,
        } => {
            handle_add(&mut manager, description, horizon, priority)?;
        }
        Commands::List { all, horizon } => {
            handle_list(&manager, all, horizon)?;
        }
        Commands::Complete { id } => {
            handle_complete(&mut manager, id)?;
        }
        Commands::Edit {
            id,
            description,
            horizon,
            priority,
        } => {
            handle_edit(&mut manager, id, description, horizon, priority)?;
        }
        Commands::Delete { id } => {
            handle_delete(&mut manager, id)?;
        }
        Commands::Context { action } => {
            handle_context(&mut manager, action)?;
        }
        Commands::Export { path } => {
            handle_export(&store, &manager, path)?;
        }
        Commands::Import { path, merge } => {
            handle_import(&store, &mut manager, path, merge)?;
        }
    }

    // Save the updated state back to disk
    // This is called after every command to ensure data persistence
    // The ? operator propagates any errors (e.g., disk full, permission denied)
    store.save(&manager)?;

    // Return success
    // Ok(()) indicates the program completed successfully
    Ok(())
}

/// Gets the Store instance with the default data file path
///
/// This function determines the appropriate data directory for the current OS
/// and creates a Store instance pointing to the data file.
///
/// # Cross-Platform Paths
///
/// We use the `directories` crate to find the correct data directory:
/// - Linux: ~/.local/share/rust-todo/data.json
/// - macOS: ~/Library/Application Support/rust-todo/data.json
/// - Windows: %APPDATA%\rust-todo\data.json
///
/// This follows OS conventions and ensures data is stored in the right place.
///
/// # Error Handling
///
/// If we can't determine the data directory (rare), we fall back to a local file.
///
/// # Returns
///
/// A Store instance configured with the appropriate data file path.
fn get_store() -> Result<Store> {
    // Try to get the platform-specific data directory
    // ProjectDirs::from() takes (qualifier, organization, application)
    // We use empty strings for qualifier and organization
    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", "rust-todo") {
        // Get the data directory path
        let data_dir = proj_dirs.data_dir();

        // Create the full path to the data file
        let file_path = data_dir.join("data.json");

        // Return a Store instance with this path
        Ok(Store::new(file_path))
    } else {
        // Fallback: use a local file if we can't determine the data directory
        // This is rare but can happen in some environments
        Ok(Store::new(PathBuf::from("data.json")))
    }
}

/// Handles the Add command - creates a new task
///
/// This function demonstrates:
/// - String parsing with FromStr trait
/// - Error handling with Result and ?
/// - Creating and adding tasks
/// - User feedback with colored output
///
/// # Arguments
///
/// * `manager` - Mutable reference to the ContextManager
/// * `description` - The task description
/// * `horizon` - Time horizon string (short, mid, long)
/// * `priority` - Priority string (low, medium, high)
///
/// # Returns
///
/// Ok(()) if the task was added successfully, or an error if parsing fails.
///
/// # Requirements
///
/// This function satisfies:
/// - Requirement 1.1: Accept task description, time horizon, and priority
/// - Requirement 1.2: Default to short-term if not specified
/// - Requirement 1.3: Default to medium priority if not specified
/// - Requirement 1.4: Return error for invalid time horizon
/// - Requirement 1.5: Return error for invalid priority
fn handle_add(
    manager: &mut ContextManager,
    description: String,
    horizon: String,
    priority: String,
) -> Result<()> {
    // Parse the time horizon string to a TimeHorizon enum
    // FromStr::from_str() returns Result<TimeHorizon, AppError>
    // The ? operator propagates the error if parsing fails
    let time_horizon = TimeHorizon::from_str(&horizon)?;

    // Parse the priority string to a Priority enum
    // Same error handling as above
    let priority_level = Priority::from_str(&priority)?;

    // Create a new task with the parsed values
    // Task::new() generates a UUID and timestamp automatically
    let task = Task::new(description.clone(), time_horizon, priority_level);

    // Get the task ID for display (first 6 characters)
    // Clone the ID to avoid borrowing issues
    let short_id = task.id[..6].to_string();

    // Add the task to the active context
    // active_context_mut() returns a mutable reference to the active context
    // add_task() takes ownership of the task and adds it to the context's Vec
    manager.active_context_mut().add_task(task);

    // Display success message with colored output
    // The colored crate provides methods like .green() and .bold()
    println!(
        "{} Task added with ID: {}",
        "✓".green().bold(),
        short_id.cyan()
    );
    println!("  {}", description.dimmed());

    Ok(())
}

/// Handles the List command - displays tasks
///
/// This function demonstrates:
/// - Filtering tasks by time horizon
/// - Sorting tasks by priority
/// - Using the display module for formatted output
/// - Conditional logic based on flags
///
/// # Arguments
///
/// * `manager` - Reference to the ContextManager
/// * `show_all` - Whether to show completed tasks
/// * `horizon_filter` - Optional time horizon to filter by
///
/// # Returns
///
/// Ok(()) if successful, or an error if the horizon filter is invalid.
///
/// # Requirements
///
/// This function satisfies:
/// - Requirement 3.1: Display all tasks grouped by time horizon
/// - Requirement 3.2: Show task details (description, priority, status, horizon)
/// - Requirement 3.3: Display tasks from all time horizons
/// - Requirement 3.4: Sort tasks by priority within horizon
/// - Requirement 3.5: Visually distinguish completed tasks
fn handle_list(
    manager: &ContextManager,
    show_all: bool,
    horizon_filter: Option<String>,
) -> Result<()> {
    // Get the active context
    // active_context() returns an immutable reference
    let context = manager.active_context();

    // Get the tasks to display based on the horizon filter
    let tasks: Vec<&Task> = if let Some(horizon_str) = horizon_filter {
        // Parse the horizon filter string
        let horizon = TimeHorizon::from_str(&horizon_str)?;

        // Filter tasks by the specified horizon
        context.tasks_by_horizon(horizon)
    } else {
        // No filter - get all tasks sorted by horizon and priority
        context.sorted_tasks()
    };

    // Display the tasks using the display module
    // display_tasks() handles formatting, grouping, and coloring
    display_tasks(&tasks, show_all);

    // Display context information
    println!();
    println!(
        "{} Context: {}",
        "ℹ".cyan(),
        manager.active_context.cyan().bold()
    );

    Ok(())
}

/// Handles the Complete command - marks a task as done
///
/// This function demonstrates:
/// - Finding tasks by partial ID matching
/// - Modifying task state
/// - Error handling for missing tasks
/// - User feedback
///
/// # Arguments
///
/// * `manager` - Mutable reference to the ContextManager
/// * `id` - Task ID (can be partial)
///
/// # Returns
///
/// Ok(()) if the task was marked complete, or an error if not found.
///
/// # Requirements
///
/// This function satisfies:
/// - Requirement 2.3: Mark task as complete
/// - Requirement 2.4: Return error if task not found
fn handle_complete(manager: &mut ContextManager, id: String) -> Result<()> {
    // Get the active context
    let context = manager.active_context_mut();

    // Find the task by ID (supports partial matching)
    let task = find_task_by_partial_id(context, &id)?;

    // Mark the task as complete
    task.mark_complete();

    // Display success message
    println!(
        "{} Task completed: {}",
        "✓".green().bold(),
        task.description.dimmed()
    );

    Ok(())
}

/// Handles the Edit command - modifies task properties
///
/// This function demonstrates:
/// - Optional parameter handling with Option<T>
/// - Conditional updates based on provided values
/// - String parsing for enum values
/// - User feedback showing what changed
///
/// # Arguments
///
/// * `manager` - Mutable reference to the ContextManager
/// * `id` - Task ID to edit
/// * `description` - Optional new description
/// * `horizon` - Optional new time horizon
/// * `priority` - Optional new priority
///
/// # Returns
///
/// Ok(()) if the task was edited, or an error if not found or parsing fails.
///
/// # Requirements
///
/// This function satisfies:
/// - Requirement 2.1: Allow modification of task properties
/// - Requirement 2.2: Update time horizon and persist
/// - Requirement 2.4: Return error if task not found
fn handle_edit(
    manager: &mut ContextManager,
    id: String,
    description: Option<String>,
    horizon: Option<String>,
    priority: Option<String>,
) -> Result<()> {
    // Parse optional time horizon
    let time_horizon = if let Some(h) = horizon {
        Some(TimeHorizon::from_str(&h)?)
    } else {
        None
    };

    // Parse optional priority
    let priority_level = if let Some(p) = priority {
        Some(Priority::from_str(&p)?)
    } else {
        None
    };

    // Get the active context
    let context = manager.active_context_mut();

    // Find the task by ID
    let task = find_task_by_partial_id(context, &id)?;

    // Update the task with the provided values
    task.update(description.clone(), time_horizon, priority_level);

    // Display success message
    println!(
        "{} Task updated: {}",
        "✓".green().bold(),
        task.description.dimmed()
    );

    Ok(())
}

/// Handles the Delete command - removes a task
///
/// This function demonstrates:
/// - Removing items from collections
/// - Confirmation messages
/// - Error handling for missing tasks
///
/// # Arguments
///
/// * `manager` - Mutable reference to the ContextManager
/// * `id` - Task ID to delete
///
/// # Returns
///
/// Ok(()) if the task was deleted, or an error if not found.
///
/// # Requirements
///
/// This function satisfies:
/// - Requirement 2.5: Delete task permanently
/// - Requirement 2.4: Return error if task not found
fn handle_delete(manager: &mut ContextManager, id: String) -> Result<()> {
    // Get the active context
    let context = manager.active_context_mut();

    // Find the full task ID by partial match
    let full_id = find_task_id_by_partial(context, &id)?;

    // Remove the task and get it back (for displaying confirmation)
    let removed_task = context.remove_task(&full_id)?;

    // Display success message
    println!(
        "{} Task deleted: {}",
        "✓".green().bold(),
        removed_task.description.dimmed()
    );

    Ok(())
}

/// Handles the Context command - manages contexts
///
/// This function demonstrates:
/// - Nested command handling with pattern matching
/// - Context operations (create, switch, list, delete)
/// - User feedback for each operation
///
/// # Arguments
///
/// * `manager` - Mutable reference to the ContextManager
/// * `action` - The context action to perform
///
/// # Returns
///
/// Ok(()) if the operation succeeded, or an error if it failed.
///
/// # Requirements
///
/// This function satisfies:
/// - Requirement 5.1: Create new context
/// - Requirement 5.2: Switch to different context
/// - Requirement 5.3: List all contexts
/// - Requirement 5.4: Reject duplicate context names
/// - Requirement 5.5: Delete context and tasks
fn handle_context(manager: &mut ContextManager, action: ContextAction) -> Result<()> {
    match action {
        ContextAction::New { name } => {
            // Create a new context
            manager.create_context(name.clone())?;

            println!(
                "{} Context created: {}",
                "✓".green().bold(),
                name.cyan().bold()
            );
        }
        ContextAction::Switch { name } => {
            // Switch to a different context
            manager.switch_context(&name)?;

            // Get task count for the new context
            let task_count = manager.active_context().tasks.len();

            println!(
                "{} Switched to context: {} ({} tasks)",
                "✓".green().bold(),
                name.cyan().bold(),
                task_count
            );
        }
        ContextAction::List => {
            // List all contexts
            let context_names = manager.list_contexts();
            display_contexts(&context_names, &manager.active_context);
        }
        ContextAction::Delete { name } => {
            // Delete a context
            manager.delete_context(&name)?;

            println!("{} Context deleted: {}", "✓".green().bold(), name.dimmed());
        }
    }

    Ok(())
}

/// Handles the Export command - exports data to a file
///
/// This function demonstrates:
/// - File I/O operations
/// - Using the store module for export
/// - User feedback with file path
///
/// # Arguments
///
/// * `store` - Reference to the Store
/// * `manager` - Reference to the ContextManager
/// * `path` - Path where to export the data
///
/// # Returns
///
/// Ok(()) if the export succeeded, or an error if it failed.
///
/// # Requirements
///
/// This function satisfies:
/// - Requirement 4.5: Export tasks to JSON file
/// - Requirement 6.1: Create JSON file with all contexts and tasks
fn handle_export(store: &Store, manager: &ContextManager, path: PathBuf) -> Result<()> {
    // Export the data to the specified file
    store.export(manager, &path)?;

    // Display success message with the file path
    println!(
        "{} Data exported to: {}",
        "✓".green().bold(),
        path.display().to_string().cyan()
    );

    Ok(())
}

/// Handles the Import command - imports data from a file
///
/// This function demonstrates:
/// - File I/O operations
/// - Merging data structures
/// - Handling duplicate context names
/// - User feedback with import summary
///
/// # Arguments
///
/// * `store` - Reference to the Store
/// * `manager` - Mutable reference to the ContextManager
/// * `path` - Path to the file to import
/// * `merge` - Whether to merge with existing data
///
/// # Returns
///
/// Ok(()) if the import succeeded, or an error if it failed.
///
/// # Requirements
///
/// This function satisfies:
/// - Requirement 6.2: Validate JSON structure before importing
/// - Requirement 6.3: Merge imported contexts with existing data
/// - Requirement 6.4: Handle duplicate context names
/// - Requirement 6.5: Return error if invalid without modifying state
fn handle_import(
    store: &Store,
    manager: &mut ContextManager,
    path: PathBuf,
    merge: bool,
) -> Result<()> {
    // Import the data from the specified file
    // This validates the JSON structure and returns a new ContextManager
    let imported_manager = store.import(&path)?;

    if merge {
        // Merge the imported data with existing data
        let mut added_contexts = 0;
        let mut added_tasks = 0;

        #[allow(clippy::map_entry)]
        for (context_name, context) in imported_manager.contexts {
            if manager.contexts.contains_key(&context_name) {
                // Context already exists - rename the imported one
                let mut new_name = format!("{}-imported", context_name);
                let mut counter = 1;

                // Find a unique name by adding a counter
                while manager.contexts.contains_key(&new_name) {
                    counter += 1;
                    new_name = format!("{}-imported-{}", context_name, counter);
                }

                // Create the context with the new name
                manager.create_context(new_name.clone())?;

                // Add all tasks from the imported context
                let new_context = manager.contexts.get_mut(&new_name).unwrap();
                for task in context.tasks {
                    added_tasks += 1;
                    new_context.add_task(task);
                }

                added_contexts += 1;

                println!(
                    "{} Context '{}' renamed to '{}' (name conflict)",
                    "ℹ".yellow(),
                    context_name.dimmed(),
                    new_name.cyan()
                );
            } else {
                // Context doesn't exist - add it directly
                added_tasks += context.tasks.len();
                manager.contexts.insert(context_name, context);
                added_contexts += 1;
            }
        }

        println!(
            "{} Imported {} contexts and {} tasks",
            "✓".green().bold(),
            added_contexts,
            added_tasks
        );
    } else {
        // Replace existing data with imported data
        let context_count = imported_manager.contexts.len();
        let task_count: usize = imported_manager
            .contexts
            .values()
            .map(|c| c.tasks.len())
            .sum();

        *manager = imported_manager;

        println!(
            "{} Imported {} contexts and {} tasks (replaced existing data)",
            "✓".green().bold(),
            context_count,
            task_count
        );
    }

    Ok(())
}

/// Helper function to find a task by partial ID matching
///
/// This function searches for a task whose ID starts with the provided partial ID.
/// This allows users to use shortened IDs (e.g., "abc123" instead of the full UUID).
///
/// # Arguments
///
/// * `context` - Mutable reference to the context to search
/// * `partial_id` - The partial ID to match
///
/// # Returns
///
/// A mutable reference to the matching task, or an error if not found or multiple matches.
///
/// # Error Cases
///
/// - No task found with matching ID prefix
/// - Multiple tasks found with matching ID prefix (ambiguous)
fn find_task_by_partial_id<'a>(
    context: &'a mut rust_todo::context::Context,
    partial_id: &str,
) -> Result<&'a mut Task> {
    // First, find the full ID by matching
    let full_id = find_task_id_by_partial(context, partial_id)?;

    // Now find and return the mutable reference
    context
        .find_task_mut(&full_id)
        .ok_or_else(|| AppError::TaskNotFound(partial_id.to_string()))
}

/// Helper function to find a full task ID by partial matching
///
/// Similar to find_task_by_partial_id, but returns the full ID string instead
/// of a reference to the task. This is useful when we need to remove the task.
///
/// # Arguments
///
/// * `context` - Reference to the context to search
/// * `partial_id` - The partial ID to match
///
/// # Returns
///
/// The full task ID as a String, or an error if not found or multiple matches.
fn find_task_id_by_partial(
    context: &rust_todo::context::Context,
    partial_id: &str,
) -> Result<String> {
    // Find all tasks whose ID starts with the partial ID
    let matches: Vec<String> = context
        .tasks
        .iter()
        .filter(|task| task.id.starts_with(partial_id))
        .map(|task| task.id.clone())
        .collect();

    // Check the number of matches
    match matches.len() {
        0 => {
            // No matches found
            Err(AppError::TaskNotFound(partial_id.to_string()))
        }
        1 => {
            // Exactly one match - return it
            Ok(matches[0].clone())
        }
        _ => {
            // Multiple matches - ambiguous
            Err(AppError::TaskNotFound(format!(
                "Ambiguous ID '{}' matches multiple tasks: {}",
                partial_id,
                matches.join(", ")
            )))
        }
    }
}
