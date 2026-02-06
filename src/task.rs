// Task module - defines the core Task data structure and related types
// This module demonstrates Rust's enum types, struct definitions, and trait implementations

use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Represents the time horizon for task completion
///
/// Time horizons help organize tasks by their expected completion timeframe:
/// - ShortTerm: Day-to-day tasks (within days)
/// - MidTerm: Tasks to complete within a month
/// - LongTerm: Tasks to complete within a year
///
/// Rust enums are powerful types that can represent a fixed set of variants.
/// The #[derive] attributes automatically implement common traits:
/// - Debug: enables printing with {:?} for debugging
/// - Clone: allows creating copies of the value
/// - Copy: allows implicit copying (since all variants have no data)
/// - PartialEq, Eq: enables equality comparisons (==, !=)
/// - PartialOrd, Ord: enables ordering comparisons (<, >, <=, >=)
/// - Serialize, Deserialize: enables JSON conversion via serde
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TimeHorizon {
    ShortTerm,
    MidTerm,
    LongTerm,
}

/// Implements string parsing for TimeHorizon
///
/// The FromStr trait allows converting strings to our enum type.
/// This is used when parsing command-line arguments.
///
/// Example:
/// ```
/// use rust_todo::task::TimeHorizon;
/// use std::str::FromStr;
///
/// let horizon = TimeHorizon::from_str("short")?;
/// # Ok::<(), rust_todo::error::AppError>(())
/// ```
impl FromStr for TimeHorizon {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Pattern matching is a core Rust feature for handling different cases
        // We match on the lowercase version to make parsing case-insensitive
        match s.to_lowercase().as_str() {
            "short" | "shortterm" | "short-term" => Ok(TimeHorizon::ShortTerm),
            "mid" | "midterm" | "mid-term" => Ok(TimeHorizon::MidTerm),
            "long" | "longterm" | "long-term" => Ok(TimeHorizon::LongTerm),
            // If no pattern matches, return an error
            // The to_string() method creates an owned String from the &str
            _ => Err(AppError::InvalidTimeHorizon(s.to_string())),
        }
    }
}

/// Represents task priority levels
///
/// Priority determines the importance of a task. Higher priority tasks
/// should be completed before lower priority tasks.
///
/// The explicit discriminant values (Low = 0, Medium = 1, High = 2) allow
/// us to use numeric comparisons. This is useful for sorting tasks.
///
/// The Ord and PartialOrd traits enable comparison operations (<, >, <=, >=).
/// With these traits, we can sort tasks by priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Low = 0,
    Medium = 1,
    High = 2,
}

/// Implements string parsing for Priority
///
/// Similar to TimeHorizon, this allows converting command-line arguments
/// to Priority enum values.
impl FromStr for Priority {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" | "l" => Ok(Priority::Low),
            "medium" | "med" | "m" => Ok(Priority::Medium),
            "high" | "hi" | "h" => Ok(Priority::High),
            _ => Err(AppError::InvalidPriority(s.to_string())),
        }
    }
}

/// Represents a single todo task
///
/// The Task struct is the core data structure of our application.
/// It contains all the information needed to track a todo item.
///
/// # Ownership and Struct Design
///
/// In Rust, structs own their data by default. Each Task owns its String fields
/// (id, description, created_at), which means when a Task is dropped, its strings
/// are automatically cleaned up. This is Rust's ownership system in action.
///
/// The struct uses owned types (String) rather than references (&str) because:
/// 1. Tasks need to live independently and be stored in collections
/// 2. We don't want lifetime annotations complicating the API
/// 3. The data needs to persist beyond the scope where it was created
///
/// # Serialization
///
/// The #[derive(Serialize, Deserialize)] attributes from serde enable automatic
/// JSON conversion. This allows us to save tasks to disk and load them back
/// without writing manual serialization code.
///
/// # Fields
///
/// - `id`: Unique identifier (UUID v4) for the task
/// - `description`: The task's text description
/// - `time_horizon`: When the task should be completed (short/mid/long term)
/// - `priority`: How important the task is (low/medium/high)
/// - `completed`: Whether the task has been finished
/// - `created_at`: ISO 8601 timestamp of when the task was created
///
/// # Future Extensibility
///
/// Additional fields can be added later (due_date, tags, notes) without breaking
/// existing JSON files by using #[serde(default)] on new fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier for the task (UUID v4 format)
    pub id: String,

    /// The task's description text
    pub description: String,

    /// Time horizon for completion
    pub time_horizon: TimeHorizon,

    /// Priority level
    pub priority: Priority,

    /// Whether the task is completed
    pub completed: bool,

    /// ISO 8601 timestamp of task creation
    pub created_at: String,
}

impl Task {
    /// Creates a new task with generated UUID and timestamp
    ///
    /// This constructor demonstrates several Rust concepts:
    /// - Taking ownership of the description String (not borrowing)
    /// - Generating unique IDs using the uuid crate
    /// - Working with timestamps using the chrono crate
    /// - Returning an owned value (Self)
    ///
    /// # Arguments
    ///
    /// * `description` - The task description (ownership is transferred to the Task)
    /// * `time_horizon` - When the task should be completed
    /// * `priority` - How important the task is
    ///
    /// # Returns
    ///
    /// A new Task with a unique ID, the provided fields, completed set to false,
    /// and created_at set to the current UTC time.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::task::{Task, TimeHorizon, Priority};
    ///
    /// let task = Task::new(
    ///     "Write documentation".to_string(),
    ///     TimeHorizon::ShortTerm,
    ///     Priority::High
    /// );
    /// assert_eq!(task.description, "Write documentation");
    /// assert_eq!(task.completed, false);
    /// ```
    pub fn new(description: String, time_horizon: TimeHorizon, priority: Priority) -> Self {
        // Generate a new UUID v4 (random UUID)
        // The uuid crate provides this functionality
        // We convert it to a String for easy storage and display
        let id = uuid::Uuid::new_v4().to_string();

        // Get the current UTC time and format it as ISO 8601
        // ISO 8601 is a standard format: "2024-01-15T10:30:00Z"
        // Using UTC ensures consistency across time zones
        let created_at = chrono::Utc::now().to_rfc3339();

        // Construct and return the Task
        // Note: We use 'Self' as shorthand for 'Task'
        // All fields must be initialized (Rust doesn't allow uninitialized fields)
        Self {
            id,
            description,
            time_horizon,
            priority,
            completed: false, // New tasks start as incomplete
            created_at,
        }
    }

    /// Marks the task as complete
    ///
    /// This method demonstrates Rust's borrowing system with mutable references.
    ///
    /// # Borrowing and Mutability
    ///
    /// The `&mut self` parameter is a *mutable reference* to the Task.
    /// This means:
    /// 1. We can modify the task's fields (because it's mutable)
    /// 2. We're borrowing the task, not taking ownership (because of &)
    /// 3. The caller retains ownership and can continue using the task after this call
    ///
    /// Rust's borrow checker ensures that:
    /// - Only one mutable reference exists at a time (no data races)
    /// - No immutable references exist while we have a mutable reference
    /// - The reference is valid for the entire method call
    ///
    /// This is safer than languages like C++ where you might accidentally
    /// modify data through multiple pointers simultaneously.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::task::{Task, TimeHorizon, Priority};
    ///
    /// let mut task = Task::new(
    ///     "Complete documentation".to_string(),
    ///     TimeHorizon::ShortTerm,
    ///     Priority::High
    /// );
    ///
    /// assert_eq!(task.completed, false);
    /// task.mark_complete();
    /// assert_eq!(task.completed, true);
    /// ```
    pub fn mark_complete(&mut self) {
        // Simply set the completed field to true
        // The &mut self reference allows us to modify the task's state
        self.completed = true;
    }

    /// Updates task properties
    ///
    /// This method allows modifying the task's description, time horizon, and priority.
    /// Any field can be updated independently by passing Some(value), or left unchanged
    /// by passing None.
    ///
    /// # Borrowing and Option Types
    ///
    /// Like mark_complete(), this method takes `&mut self` to modify the task.
    ///
    /// The parameters use Option<T> to represent optional updates:
    /// - Some(value): Update the field to this new value
    /// - None: Leave the field unchanged
    ///
    /// This pattern is common in Rust for optional parameters. It's more explicit
    /// and type-safe than using null or default values in other languages.
    ///
    /// # Arguments
    ///
    /// * `description` - Optional new description (None = no change)
    /// * `time_horizon` - Optional new time horizon (None = no change)
    /// * `priority` - Optional new priority (None = no change)
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::task::{Task, TimeHorizon, Priority};
    ///
    /// let mut task = Task::new(
    ///     "Original description".to_string(),
    ///     TimeHorizon::ShortTerm,
    ///     Priority::Low
    /// );
    ///
    /// // Update only the priority
    /// task.update(None, None, Some(Priority::High));
    /// assert_eq!(task.priority, Priority::High);
    /// assert_eq!(task.description, "Original description");
    ///
    /// // Update description and time horizon
    /// task.update(
    ///     Some("New description".to_string()),
    ///     Some(TimeHorizon::LongTerm),
    ///     None
    /// );
    /// assert_eq!(task.description, "New description");
    /// assert_eq!(task.time_horizon, TimeHorizon::LongTerm);
    /// assert_eq!(task.priority, Priority::High); // Unchanged
    /// ```
    pub fn update(
        &mut self,
        description: Option<String>,
        time_horizon: Option<TimeHorizon>,
        priority: Option<Priority>,
    ) {
        // Use if let to check if a new value was provided
        // if let Some(value) = option { ... } is Rust's way of handling Option types
        // It's more concise than match when we only care about the Some case

        // Update description if provided
        if let Some(new_description) = description {
            // Take ownership of the new String and assign it to self.description
            // The old description is automatically dropped (memory freed)
            self.description = new_description;
        }

        // Update time horizon if provided
        if let Some(new_horizon) = time_horizon {
            // TimeHorizon is Copy, so this is a simple value copy
            self.time_horizon = new_horizon;
        }

        // Update priority if provided
        if let Some(new_priority) = priority {
            // Priority is also Copy, so this is a simple value copy
            self.priority = new_priority;
        }

        // Note: Fields not provided (None) remain unchanged
        // This is the power of Option<T> - explicit optional parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_horizon_from_str() {
        // Test valid inputs
        assert_eq!(
            TimeHorizon::from_str("short").unwrap(),
            TimeHorizon::ShortTerm
        );
        assert_eq!(
            TimeHorizon::from_str("ShortTerm").unwrap(),
            TimeHorizon::ShortTerm
        );
        assert_eq!(
            TimeHorizon::from_str("short-term").unwrap(),
            TimeHorizon::ShortTerm
        );

        assert_eq!(TimeHorizon::from_str("mid").unwrap(), TimeHorizon::MidTerm);
        assert_eq!(
            TimeHorizon::from_str("MidTerm").unwrap(),
            TimeHorizon::MidTerm
        );
        assert_eq!(
            TimeHorizon::from_str("mid-term").unwrap(),
            TimeHorizon::MidTerm
        );

        assert_eq!(
            TimeHorizon::from_str("long").unwrap(),
            TimeHorizon::LongTerm
        );
        assert_eq!(
            TimeHorizon::from_str("LongTerm").unwrap(),
            TimeHorizon::LongTerm
        );
        assert_eq!(
            TimeHorizon::from_str("long-term").unwrap(),
            TimeHorizon::LongTerm
        );

        // Test invalid input
        assert!(TimeHorizon::from_str("invalid").is_err());
        assert!(TimeHorizon::from_str("").is_err());
    }

    #[test]
    fn test_priority_from_str() {
        // Test valid inputs
        assert_eq!(Priority::from_str("low").unwrap(), Priority::Low);
        assert_eq!(Priority::from_str("Low").unwrap(), Priority::Low);
        assert_eq!(Priority::from_str("l").unwrap(), Priority::Low);

        assert_eq!(Priority::from_str("medium").unwrap(), Priority::Medium);
        assert_eq!(Priority::from_str("Medium").unwrap(), Priority::Medium);
        assert_eq!(Priority::from_str("med").unwrap(), Priority::Medium);
        assert_eq!(Priority::from_str("m").unwrap(), Priority::Medium);

        assert_eq!(Priority::from_str("high").unwrap(), Priority::High);
        assert_eq!(Priority::from_str("High").unwrap(), Priority::High);
        assert_eq!(Priority::from_str("hi").unwrap(), Priority::High);
        assert_eq!(Priority::from_str("h").unwrap(), Priority::High);

        // Test invalid input
        assert!(Priority::from_str("invalid").is_err());
        assert!(Priority::from_str("").is_err());
    }

    #[test]
    fn test_priority_ordering() {
        // Test that priorities can be compared and ordered correctly
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
        assert!(Priority::High > Priority::Low);

        // Test sorting
        let mut priorities = vec![Priority::Low, Priority::High, Priority::Medium];
        priorities.sort();
        assert_eq!(
            priorities,
            vec![Priority::Low, Priority::Medium, Priority::High]
        );
    }

    #[test]
    fn test_serialization() {
        // Test that enums can be serialized to JSON
        let horizon = TimeHorizon::ShortTerm;
        let json = serde_json::to_string(&horizon).unwrap();
        assert_eq!(json, "\"ShortTerm\"");

        let priority = Priority::High;
        let json = serde_json::to_string(&priority).unwrap();
        assert_eq!(json, "\"High\"");
    }

    #[test]
    fn test_deserialization() {
        // Test that enums can be deserialized from JSON
        let horizon: TimeHorizon = serde_json::from_str("\"ShortTerm\"").unwrap();
        assert_eq!(horizon, TimeHorizon::ShortTerm);

        let priority: Priority = serde_json::from_str("\"High\"").unwrap();
        assert_eq!(priority, Priority::High);
    }

    #[test]
    fn test_task_creation() {
        // Test creating a new task
        let task = Task::new(
            "Write tests".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High,
        );

        // Verify all fields are set correctly
        assert_eq!(task.description, "Write tests");
        assert_eq!(task.time_horizon, TimeHorizon::ShortTerm);
        assert_eq!(task.priority, Priority::High);
        assert_eq!(task.completed, false);

        // Verify ID is a valid UUID (36 characters with hyphens)
        assert_eq!(task.id.len(), 36);
        assert!(task.id.contains('-'));

        // Verify created_at is not empty and looks like ISO 8601
        assert!(!task.created_at.is_empty());
        assert!(task.created_at.contains('T'));
        assert!(task.created_at.contains('Z') || task.created_at.contains('+'));
    }

    #[test]
    fn test_task_unique_ids() {
        // Test that each task gets a unique ID
        let task1 = Task::new(
            "Task 1".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Medium,
        );
        let task2 = Task::new(
            "Task 2".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Medium,
        );

        // IDs should be different
        assert_ne!(task1.id, task2.id);
    }

    #[test]
    fn test_task_serialization() {
        // Test that Task can be serialized to JSON
        let task = Task::new("Test task".to_string(), TimeHorizon::MidTerm, Priority::Low);

        let json = serde_json::to_string(&task).unwrap();

        // Verify JSON contains expected fields
        assert!(json.contains("\"id\""));
        assert!(json.contains("\"description\""));
        assert!(json.contains("\"time_horizon\""));
        assert!(json.contains("\"priority\""));
        assert!(json.contains("\"completed\""));
        assert!(json.contains("\"created_at\""));
        assert!(json.contains("Test task"));
        assert!(json.contains("MidTerm"));
        assert!(json.contains("Low"));
    }

    #[test]
    fn test_task_deserialization() {
        // Test that Task can be deserialized from JSON
        let json = r#"{
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "description": "Test task",
            "time_horizon": "LongTerm",
            "priority": "High",
            "completed": true,
            "created_at": "2024-01-15T10:30:00Z"
        }"#;

        let task: Task = serde_json::from_str(json).unwrap();

        assert_eq!(task.id, "123e4567-e89b-12d3-a456-426614174000");
        assert_eq!(task.description, "Test task");
        assert_eq!(task.time_horizon, TimeHorizon::LongTerm);
        assert_eq!(task.priority, Priority::High);
        assert_eq!(task.completed, true);
        assert_eq!(task.created_at, "2024-01-15T10:30:00Z");
    }

    #[test]
    fn test_task_with_different_priorities() {
        // Test creating tasks with all priority levels
        let low = Task::new(
            "Low priority".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Low,
        );
        let med = Task::new(
            "Med priority".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Medium,
        );
        let high = Task::new(
            "High priority".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High,
        );

        assert_eq!(low.priority, Priority::Low);
        assert_eq!(med.priority, Priority::Medium);
        assert_eq!(high.priority, Priority::High);
    }

    #[test]
    fn test_task_with_different_horizons() {
        // Test creating tasks with all time horizons
        let short = Task::new(
            "Short term".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Medium,
        );
        let mid = Task::new(
            "Mid term".to_string(),
            TimeHorizon::MidTerm,
            Priority::Medium,
        );
        let long = Task::new(
            "Long term".to_string(),
            TimeHorizon::LongTerm,
            Priority::Medium,
        );

        assert_eq!(short.time_horizon, TimeHorizon::ShortTerm);
        assert_eq!(mid.time_horizon, TimeHorizon::MidTerm);
        assert_eq!(long.time_horizon, TimeHorizon::LongTerm);
    }

    #[test]
    fn test_mark_complete() {
        // Test marking a task as complete
        let mut task = Task::new(
            "Complete this task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Medium,
        );

        // Initially, task should not be completed
        assert_eq!(task.completed, false);

        // Mark the task as complete
        task.mark_complete();

        // Now it should be completed
        assert_eq!(task.completed, true);

        // Marking complete again should have no effect (idempotent)
        task.mark_complete();
        assert_eq!(task.completed, true);
    }

    #[test]
    fn test_update_description() {
        // Test updating only the description
        let mut task = Task::new(
            "Original description".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Low,
        );

        let original_horizon = task.time_horizon;
        let original_priority = task.priority;

        // Update only the description
        task.update(Some("New description".to_string()), None, None);

        // Description should be updated
        assert_eq!(task.description, "New description");

        // Other fields should remain unchanged
        assert_eq!(task.time_horizon, original_horizon);
        assert_eq!(task.priority, original_priority);
    }

    #[test]
    fn test_update_time_horizon() {
        // Test updating only the time horizon
        let mut task = Task::new(
            "Test task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Medium,
        );

        let original_description = task.description.clone();
        let original_priority = task.priority;

        // Update only the time horizon
        task.update(None, Some(TimeHorizon::LongTerm), None);

        // Time horizon should be updated
        assert_eq!(task.time_horizon, TimeHorizon::LongTerm);

        // Other fields should remain unchanged
        assert_eq!(task.description, original_description);
        assert_eq!(task.priority, original_priority);
    }

    #[test]
    fn test_update_priority() {
        // Test updating only the priority
        let mut task = Task::new("Test task".to_string(), TimeHorizon::MidTerm, Priority::Low);

        let original_description = task.description.clone();
        let original_horizon = task.time_horizon;

        // Update only the priority
        task.update(None, None, Some(Priority::High));

        // Priority should be updated
        assert_eq!(task.priority, Priority::High);

        // Other fields should remain unchanged
        assert_eq!(task.description, original_description);
        assert_eq!(task.time_horizon, original_horizon);
    }

    #[test]
    fn test_update_multiple_fields() {
        // Test updating multiple fields at once
        let mut task = Task::new(
            "Original".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Low,
        );

        // Update all three fields
        task.update(
            Some("Updated description".to_string()),
            Some(TimeHorizon::LongTerm),
            Some(Priority::High),
        );

        // All fields should be updated
        assert_eq!(task.description, "Updated description");
        assert_eq!(task.time_horizon, TimeHorizon::LongTerm);
        assert_eq!(task.priority, Priority::High);
    }

    #[test]
    fn test_update_with_no_changes() {
        // Test calling update with all None values (no changes)
        let mut task = Task::new(
            "Test task".to_string(),
            TimeHorizon::MidTerm,
            Priority::Medium,
        );

        let original_description = task.description.clone();
        let original_horizon = task.time_horizon;
        let original_priority = task.priority;

        // Call update with no changes
        task.update(None, None, None);

        // All fields should remain unchanged
        assert_eq!(task.description, original_description);
        assert_eq!(task.time_horizon, original_horizon);
        assert_eq!(task.priority, original_priority);
    }

    #[test]
    fn test_update_preserves_other_fields() {
        // Test that update doesn't affect id, completed, or created_at
        let mut task = Task::new(
            "Test task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Low,
        );

        let original_id = task.id.clone();
        let original_created_at = task.created_at.clone();
        let original_completed = task.completed;

        // Update some fields
        task.update(
            Some("New description".to_string()),
            Some(TimeHorizon::LongTerm),
            Some(Priority::High),
        );

        // ID, created_at, and completed should remain unchanged
        assert_eq!(task.id, original_id);
        assert_eq!(task.created_at, original_created_at);
        assert_eq!(task.completed, original_completed);
    }
}
