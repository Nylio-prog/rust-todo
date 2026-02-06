// Error handling module for the todo application
// This module defines custom error types using the thiserror crate
//
// Rust's error handling is based on the Result type, which can be either:
// - Ok(value) for successful operations
// - Err(error) for failures
//
// The thiserror crate provides a derive macro that automatically implements
// the std::error::Error trait, making it easy to create custom error types
// with formatted error messages.

use thiserror::Error;

/// Custom error type for the todo application
///
/// This enum represents all possible errors that can occur in the application.
/// Each variant includes context about what went wrong, making it easier for
/// users to understand and fix problems.
///
/// The #[error("...")] attribute defines the error message format for each variant.
/// The {0}, {1} syntax refers to the fields in tuple variants.
#[derive(Error, Debug)]
pub enum AppError {
    /// Error when a task with the specified ID cannot be found
    #[error("Task not found: {0}")]
    TaskNotFound(String),

    /// Error when a context with the specified name cannot be found
    #[error("Context not found: {0}")]
    ContextNotFound(String),

    /// Error when attempting to create a context that already exists
    #[error("Context already exists: {0}")]
    ContextAlreadyExists(String),

    /// Error when an invalid time horizon value is provided
    /// Valid values are: short, mid, long
    #[error("Invalid time horizon: {0}")]
    InvalidTimeHorizon(String),

    /// Error when an invalid priority value is provided
    /// Valid values are: low, medium, high
    #[error("Invalid priority: {0}")]
    InvalidPriority(String),

    /// Error when attempting to delete the last remaining context
    /// At least one context must always exist
    #[error("Cannot delete the last context")]
    CannotDeleteLastContext,

    /// Error from file system operations (reading, writing, permissions, etc.)
    /// The #[from] attribute automatically implements From<std::io::Error> for AppError,
    /// allowing us to use the ? operator to convert io::Error to AppError
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Error from JSON serialization/deserialization
    /// The #[from] attribute automatically implements From<serde_json::Error> for AppError
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Error when the data file has an invalid format or structure
    #[error("Invalid data format: {0}")]
    InvalidDataFormat(String),
}

/// Type alias for Result with our custom error type
///
/// This is a common Rust pattern that makes function signatures more concise.
/// Instead of writing Result<T, AppError>, we can just write Result<T>.
///
/// Example usage:
/// ```no_run
/// use rust_todo::error::Result;
/// use rust_todo::task::Task;
///
/// fn load_tasks() -> Result<Vec<Task>> {
///     // ... implementation
///     Ok(vec![])
/// }
/// ```
pub type Result<T> = std::result::Result<T, AppError>;
