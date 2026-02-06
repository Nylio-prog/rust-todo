// Rust CLI Todo Application Library
// This library provides the core functionality for a command-line todo application
// with support for multiple time horizons and project contexts.
//
// The library is organized into several modules:
// - error: Custom error types for the application
// - task: Task data structure and operations
// - context: Context management for organizing tasks by project
// - store: Data persistence using JSON files
// - display: Formatting and displaying tasks
// - cli: Command-line interface definitions
//
// This structure demonstrates Rust's module system and separation of concerns.

// Public module declarations - these modules are accessible to external code
pub mod cli;
pub mod context;
pub mod display;
pub mod error;
pub mod store;
pub mod task;

// Re-export commonly used types for convenience
// This allows users to write `use rust_todo::Task` instead of `use rust_todo::task::Task`
pub use error::{AppError, Result};
pub use task::{Priority, Task, TimeHorizon};
pub use context::Context;
// TODO: Uncomment when ContextManager is implemented
// pub use context::ContextManager;
// TODO: Uncomment when Store is implemented
// pub use store::Store;
