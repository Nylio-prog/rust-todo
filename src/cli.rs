// CLI module - defines command-line interface using clap
// This module demonstrates derive macros, enums for subcommands, and argument parsing
//
// This module uses the clap crate to define the command-line interface for the todo application.
// Clap provides a declarative way to define CLI arguments using derive macros, which automatically
// generates argument parsing code, help messages, and validation.
//
// # Key Rust Concepts Demonstrated
//
// - **Derive Macros**: Using #[derive(Parser)] to automatically implement CLI parsing
// - **Enums for Subcommands**: Using enums to represent different CLI commands
// - **Attributes**: Using #[command(...)] and #[arg(...)] to configure behavior
// - **Documentation as Help Text**: Doc comments become help text in the CLI

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Rust CLI Todo Application
///
/// A command-line todo application with multiple time horizons and project contexts.
/// This application helps you organize tasks by time horizon (short-term, mid-term, long-term)
/// and manage multiple project contexts.
///
/// # Clap Parser Derive
///
/// The #[derive(Parser)] attribute automatically implements argument parsing for this struct.
/// Clap will:
/// - Parse command-line arguments
/// - Generate help messages from doc comments
/// - Validate arguments and provide error messages
/// - Handle --help and --version flags automatically
///
/// # Command Attribute
///
/// The #[command(...)] attributes configure the CLI behavior:
/// - name: The program name shown in help messages
/// - about: A brief description shown in help messages
/// - version: Automatically uses the version from Cargo.toml
/// - author: Automatically uses the author from Cargo.toml
///
/// # Example Usage
///
/// ```bash
/// # Add a new task
/// todo add "Write documentation" -t short -p high
///
/// # List all tasks
/// todo list --all
///
/// # Complete a task
/// todo complete abc123
///
/// # Switch context
/// todo context switch work
/// ```
#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A CLI todo application with multiple time horizons and contexts", long_about = None)]
#[command(version)]
#[command(author)]
pub struct Cli {
    /// The subcommand to execute
    ///
    /// This field uses the Subcommand derive to parse subcommands.
    /// Each variant of the Commands enum becomes a subcommand in the CLI.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI commands
///
/// This enum defines all the subcommands available in the application.
/// Each variant represents a different operation the user can perform.
///
/// # Subcommand Derive
///
/// The #[derive(Subcommand)] attribute tells clap that this enum represents
/// subcommands. Each variant becomes a subcommand with its own arguments.
///
/// # Variant Documentation
///
/// The doc comments on each variant become the help text for that subcommand.
/// Users will see these when they run `todo <subcommand> --help`.
///
/// # Argument Attributes
///
/// The #[arg(...)] attributes configure individual arguments:
/// - short: Single-letter flag (e.g., -t)
/// - long: Long flag (e.g., --horizon)
/// - default_value: Default value if not provided
/// - help: Help text for the argument
///
/// # Example
///
/// ```bash
/// # Add command with optional flags
/// todo add "Task description" -t mid -p high
///
/// # List command with flags
/// todo list --all
/// todo list -t short
/// ```
#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task to the active context
    ///
    /// Creates a new task with the specified description, time horizon, and priority.
    /// If time horizon or priority are not specified, defaults are used (short-term, medium).
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Add with defaults (short-term, medium priority)
    /// todo add "Write tests"
    ///
    /// # Add with specific time horizon and priority
    /// todo add "Learn Rust" -t long -p high
    ///
    /// # Using full flag names
    /// todo add "Fix bug" --horizon mid --priority high
    /// ```
    Add {
        /// Task description
        ///
        /// The text describing what needs to be done. This is a required positional argument.
        description: String,

        /// Time horizon: short, mid, or long
        ///
        /// Specifies when the task should be completed:
        /// - short: Day-to-day tasks (within days)
        /// - mid: Tasks to complete within a month
        /// - long: Tasks to complete within a year
        ///
        /// Default: short
        #[arg(short = 't', long = "horizon", default_value = "short")]
        horizon: String,

        /// Priority: low, medium, or high
        ///
        /// Specifies how important the task is. Higher priority tasks should be
        /// completed before lower priority tasks.
        ///
        /// Default: medium
        #[arg(short = 'p', long = "priority", default_value = "medium")]
        priority: String,
    },

    /// List tasks in the active context
    ///
    /// Displays tasks grouped by time horizon and sorted by priority within each horizon.
    /// By default, shows all incomplete tasks. Use flags to customize the view.
    ///
    /// # Examples
    ///
    /// ```bash
    /// # List all incomplete tasks (default)
    /// todo list
    ///
    /// # List all tasks including completed ones
    /// todo list --all
    ///
    /// # List only short-term tasks
    /// todo list -t short
    ///
    /// # List mid-term tasks including completed ones
    /// todo list -t mid --all
    /// ```
    List {
        /// Show all tasks including completed ones
        ///
        /// By default, completed tasks are hidden. Use this flag to show them.
        #[arg(short = 'a', long = "all")]
        all: bool,

        /// Filter by time horizon: short, mid, or long
        ///
        /// If specified, only shows tasks from the specified time horizon.
        /// If not specified, shows tasks from all time horizons.
        #[arg(short = 't', long = "horizon")]
        horizon: Option<String>,
    },

    /// Mark a task as complete
    ///
    /// Marks the specified task as completed. The task ID can be a partial match
    /// (e.g., the first 6 characters shown in the list view).
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Complete a task using full ID
    /// todo complete 123e4567-e89b-12d3-a456-426614174000
    ///
    /// # Complete a task using partial ID
    /// todo complete 123e45
    /// ```
    Complete {
        /// Task ID (can be partial, will match prefix)
        ///
        /// The unique identifier of the task to complete. You can use the full UUID
        /// or just the first few characters (as shown in the list view).
        id: String,
    },

    /// Edit a task's properties
    ///
    /// Modifies one or more properties of an existing task. You can change the
    /// description, time horizon, and/or priority. Only specified properties are changed.
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Change only the description
    /// todo edit abc123 -d "Updated description"
    ///
    /// # Change only the priority
    /// todo edit abc123 -p high
    ///
    /// # Change multiple properties
    /// todo edit abc123 -d "New description" -t long -p low
    /// ```
    Edit {
        /// Task ID to edit
        ///
        /// The unique identifier of the task to modify. Can be a partial ID.
        id: String,

        /// New description
        ///
        /// If specified, updates the task's description to this value.
        #[arg(short = 'd', long = "description")]
        description: Option<String>,

        /// New time horizon: short, mid, or long
        ///
        /// If specified, moves the task to this time horizon.
        #[arg(short = 't', long = "horizon")]
        horizon: Option<String>,

        /// New priority: low, medium, or high
        ///
        /// If specified, changes the task's priority to this value.
        #[arg(short = 'p', long = "priority")]
        priority: Option<String>,
    },

    /// Delete a task
    ///
    /// Permanently removes the specified task from the active context.
    /// This action cannot be undone.
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Delete a task using full ID
    /// todo delete 123e4567-e89b-12d3-a456-426614174000
    ///
    /// # Delete a task using partial ID
    /// todo delete 123e45
    /// ```
    Delete {
        /// Task ID to delete
        ///
        /// The unique identifier of the task to remove. Can be a partial ID.
        id: String,
    },

    /// Manage project contexts
    ///
    /// Contexts allow you to organize tasks by project or area of responsibility.
    /// Each context has its own set of tasks, similar to git branches.
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Create a new context
    /// todo context new work
    ///
    /// # Switch to a different context
    /// todo context switch work
    ///
    /// # List all contexts
    /// todo context list
    ///
    /// # Delete a context
    /// todo context delete old-project
    /// ```
    Context {
        /// Context action to perform
        ///
        /// This is a nested subcommand that specifies what context operation to perform.
        #[command(subcommand)]
        action: ContextAction,
    },

    /// Export tasks to a file
    ///
    /// Creates a JSON file containing all contexts and their tasks. This is useful
    /// for backups or sharing task lists with others.
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Export to a specific file
    /// todo export backup.json
    ///
    /// # Export to a file with path
    /// todo export ~/backups/todos-2024-01-15.json
    /// ```
    Export {
        /// Output file path
        ///
        /// The path where the export file should be created. The file will contain
        /// all contexts and tasks in JSON format.
        path: PathBuf,
    },

    /// Import tasks from a file
    ///
    /// Loads tasks from a JSON file. By default, replaces the current data.
    /// Use --merge to combine with existing data.
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Import and replace current data
    /// todo import backup.json
    ///
    /// # Import and merge with existing data
    /// todo import backup.json --merge
    /// ```
    Import {
        /// Input file path
        ///
        /// The path to the JSON file to import. The file must be in the correct format
        /// (as created by the export command).
        path: PathBuf,

        /// Merge with existing data instead of replacing
        ///
        /// If specified, imported contexts and tasks are added to the existing data.
        /// If a context name conflicts, the imported context is renamed with a suffix.
        #[arg(short = 'm', long = "merge")]
        merge: bool,
    },
}

/// Context management subcommands
///
/// This enum defines the operations available for managing contexts.
/// Each variant represents a different context operation.
///
/// # Nested Subcommands
///
/// This is an example of nested subcommands in clap. The Context command has
/// its own set of subcommands, creating a hierarchy:
/// - todo context new <name>
/// - todo context switch <name>
/// - todo context list
/// - todo context delete <name>
///
/// # Example
///
/// ```bash
/// # Create a new context
/// todo context new work
///
/// # Switch to it
/// todo context switch work
///
/// # List all contexts
/// todo context list
///
/// # Delete a context
/// todo context delete old-project
/// ```
#[derive(Subcommand)]
pub enum ContextAction {
    /// Create a new context
    ///
    /// Creates a new empty context with the specified name. Context names must be unique.
    ///
    /// # Example
    ///
    /// ```bash
    /// todo context new work
    /// todo context new personal
    /// ```
    New {
        /// Name for the new context
        ///
        /// The name must be unique. If a context with this name already exists,
        /// an error will be returned.
        name: String,
    },

    /// Switch to a different context
    ///
    /// Changes the active context to the specified one. All subsequent task operations
    /// will apply to this context until you switch again.
    ///
    /// # Example
    ///
    /// ```bash
    /// todo context switch work
    /// todo context switch default
    /// ```
    Switch {
        /// Name of the context to switch to
        ///
        /// The context must exist. Use `todo context list` to see available contexts.
        name: String,
    },

    /// List all contexts
    ///
    /// Displays all available contexts with an indicator showing which one is active.
    ///
    /// # Example
    ///
    /// ```bash
    /// todo context list
    /// ```
    List,

    /// Delete a context
    ///
    /// Permanently removes the specified context and all its tasks. This action cannot
    /// be undone. You cannot delete the active context or the last remaining context.
    ///
    /// # Example
    ///
    /// ```bash
    /// todo context delete old-project
    /// ```
    Delete {
        /// Name of the context to delete
        ///
        /// The context must exist and must not be the active context. Switch to a
        /// different context first if you want to delete the active one.
        name: String,
    },
}
