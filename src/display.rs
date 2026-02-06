// Display module - formats and displays tasks and contexts
// This module demonstrates string formatting, iterators, and terminal colors
//
// This module is responsible for presenting task and context information to the user
// in a clear, visually appealing format. It uses the `colored` crate to add color
// to terminal output, making it easier to distinguish between different task states
// and priorities.
//
// # Key Rust Concepts Demonstrated
//
// - **String Formatting**: Using format!() macro and string interpolation
// - **Iterators**: Using iter(), filter(), map(), and other iterator methods
// - **Pattern Matching**: Using match expressions for enum handling
// - **Borrowing**: Working with references to avoid unnecessary cloning
// - **Trait Usage**: Using Display-like patterns for formatting

use colored::*;
use crate::task::{Task, TimeHorizon, Priority};

/// Formats a single task for compact display
///
/// This function creates a one-line representation of a task, including:
/// - Completion status (checkbox: [✓] for complete, [ ] for incomplete)
/// - Task ID (first 6 characters for brevity)
/// - Priority level with color coding
/// - Task description
///
/// # String Formatting in Rust
///
/// The format!() macro is similar to println!(), but returns a String instead
/// of printing to stdout. It uses the same formatting syntax:
/// - {} for default formatting
/// - {:?} for debug formatting
/// - {:<width} for left-aligned with padding
///
/// # Color Coding
///
/// The colored crate provides methods to add ANSI color codes to strings:
/// - .red(), .yellow(), .green() for colors
/// - .bold() for bold text
/// - .dimmed() for dimmed text
///
/// These methods return a ColoredString, which implements Display and can be
/// used in format!() and println!() macros.
///
/// # Arguments
///
/// * `task` - A reference to the task to format (borrowed, not owned)
///
/// # Returns
///
/// A formatted String representing the task in compact form.
///
/// # Example
///
/// ```
/// use rust_todo::task::{Task, TimeHorizon, Priority};
/// use rust_todo::display::format_task_line;
///
/// let task = Task::new("Write tests".to_string(), TimeHorizon::ShortTerm, Priority::High);
/// let line = format_task_line(&task);
/// // Output: "[ ] abc123 [HIGH] Write tests"
/// ```
pub fn format_task_line(task: &Task) -> String {
    // Determine the checkbox symbol based on completion status
    // Pattern matching is Rust's way of handling different cases
    let checkbox = if task.completed {
        "[✓]".green()  // Green checkmark for completed tasks
    } else {
        "[ ]".normal()  // Normal color for incomplete tasks
    };
    
    // Extract the first 6 characters of the task ID for brevity
    // UUIDs are 36 characters long, but 6 is enough for user identification
    // The get() method returns Option<&str>, and unwrap_or() provides a fallback
    let short_id = task.id.get(..6).unwrap_or(&task.id);
    
    // Format the priority with color coding
    // High priority is red, medium is yellow, low is dimmed
    let priority_str = match task.priority {
        Priority::High => format!("[{}]", "HIGH".red().bold()),
        Priority::Medium => format!("[{}]", "MED ".yellow()),
        Priority::Low => format!("[{}]", "LOW ".dimmed()),
    };
    
    // Combine all parts into a single formatted line
    // The format!() macro creates a new String with the interpolated values
    format!("{} {} {} {}", checkbox, short_id.dimmed(), priority_str, task.description)
}

/// Displays tasks grouped by time horizon
///
/// This function organizes and displays tasks in a structured format:
/// - Tasks are grouped by time horizon (short-term, mid-term, long-term)
/// - Within each horizon, tasks are sorted by priority (high to low)
/// - Each group has a header with the horizon name
/// - Completed tasks can be optionally filtered out
///
/// # Iterator Methods
///
/// This function demonstrates several iterator methods:
/// - filter(): Keeps only elements matching a predicate
/// - collect(): Consumes an iterator and builds a collection
/// - is_empty(): Checks if a collection has no elements
///
/// Iterators in Rust are lazy - they don't do any work until consumed by
/// methods like collect(), for_each(), or fold(). This allows the compiler
/// to optimize iterator chains very efficiently.
///
/// # Grouping Strategy
///
/// We iterate through each time horizon and filter tasks matching that horizon.
/// This is simple and clear, though not the most efficient for large datasets.
/// For a personal todo app with hundreds of tasks, this is perfectly fine.
///
/// # Arguments
///
/// * `tasks` - A slice of task references to display
/// * `show_completed` - Whether to include completed tasks in the display
///
/// # Example
///
/// ```
/// use rust_todo::task::{Task, TimeHorizon, Priority};
/// use rust_todo::display::display_tasks;
///
/// let mut tasks = vec![
///     Task::new("Task 1".to_string(), TimeHorizon::ShortTerm, Priority::High),
///     Task::new("Task 2".to_string(), TimeHorizon::LongTerm, Priority::Low),
/// ];
///
/// let task_refs: Vec<&Task> = tasks.iter().collect();
/// display_tasks(&task_refs, true);
/// ```
pub fn display_tasks(tasks: &[&Task], show_completed: bool) {
    // Define the time horizons in the order we want to display them
    // This array demonstrates Rust's array syntax: [Type; length]
    let horizons = [
        (TimeHorizon::ShortTerm, "SHORT-TERM TASKS"),
        (TimeHorizon::MidTerm, "MID-TERM TASKS"),
        (TimeHorizon::LongTerm, "LONG-TERM TASKS"),
    ];
    
    // Track if we've displayed any tasks (for formatting)
    let mut displayed_any = false;
    
    // Iterate through each time horizon
    // The iter() method creates an iterator over references to the array elements
    for (horizon, header) in horizons.iter() {
        // Filter tasks by this time horizon
        // The filter() method creates a new iterator that only yields matching elements
        // We use a closure |task| to define the filtering logic
        let mut horizon_tasks: Vec<&Task> = tasks.iter()
            .filter(|task| task.time_horizon == *horizon)
            .filter(|task| show_completed || !task.completed)
            .copied()  // Convert &&Task to &Task
            .collect();
        
        // Skip this horizon if there are no tasks
        if horizon_tasks.is_empty() {
            continue;
        }
        
        // Sort tasks by priority (high to low) within this horizon
        // sort_by_key() takes a closure that extracts the sort key from each element
        // We use Reverse to sort in descending order (high priority first)
        horizon_tasks.sort_by_key(|task| std::cmp::Reverse(task.priority));
        
        // Add spacing between groups (except before the first group)
        if displayed_any {
            println!();  // Print a blank line
        }
        displayed_any = true;
        
        // Print the horizon header in bold cyan
        println!("{}", header.cyan().bold());
        
        // Print each task in this horizon
        // The for loop automatically calls into_iter() on the Vec
        for task in horizon_tasks {
            // Use our format_task_line() function to format each task
            // We add two spaces of indentation for visual hierarchy
            println!("  {}", format_task_line(task));
        }
    }
    
    // If no tasks were displayed, show a message
    if !displayed_any {
        println!("{}", "No tasks to display.".dimmed());
    }
}

/// Displays detailed information about a single task
///
/// This function shows all available information about a task:
/// - Task ID (full UUID)
/// - Description
/// - Time horizon
/// - Priority
/// - Completion status
/// - Creation timestamp
///
/// This is useful when the user wants to see all details about a specific task,
/// as opposed to the compact format used in task lists.
///
/// # String Formatting
///
/// This function demonstrates various string formatting techniques:
/// - Multi-line output with multiple println!() calls
/// - Conditional formatting based on task state
/// - Color coding for visual emphasis
///
/// # Arguments
///
/// * `task` - A reference to the task to display
///
/// # Example
///
/// ```
/// use rust_todo::task::{Task, TimeHorizon, Priority};
/// use rust_todo::display::display_task_detail;
///
/// let task = Task::new("Write tests".to_string(), TimeHorizon::ShortTerm, Priority::High);
/// display_task_detail(&task);
/// ```
pub fn display_task_detail(task: &Task) {
    // Print a header
    println!("{}", "Task Details:".bold().underline());
    println!();
    
    // Print each field with a label
    // We use format!() to create strings and then print them
    println!("  {}: {}", "ID".bold(), task.id.dimmed());
    println!("  {}: {}", "Description".bold(), task.description);
    
    // Format the time horizon as a readable string
    let horizon_str = match task.time_horizon {
        TimeHorizon::ShortTerm => "Short-term (day-to-day)",
        TimeHorizon::MidTerm => "Mid-term (within a month)",
        TimeHorizon::LongTerm => "Long-term (within a year)",
    };
    println!("  {}: {}", "Time Horizon".bold(), horizon_str);
    
    // Format the priority with color coding
    let priority_str = match task.priority {
        Priority::High => "High".red().bold(),
        Priority::Medium => "Medium".yellow(),
        Priority::Low => "Low".dimmed(),
    };
    println!("  {}: {}", "Priority".bold(), priority_str);
    
    // Format the completion status with color coding
    let status_str = if task.completed {
        "Completed ✓".green().bold()
    } else {
        "Incomplete".normal()
    };
    println!("  {}: {}", "Status".bold(), status_str);
    
    // Print the creation timestamp
    println!("  {}: {}", "Created".bold(), task.created_at.dimmed());
}

/// Displays a list of contexts with an indicator for the active one
///
/// This function shows all available contexts and highlights which one is
/// currently active. This helps users understand which context they're
/// working in and what other contexts are available.
///
/// # Iterator Methods
///
/// This function demonstrates:
/// - Iterating over a slice with iter()
/// - Using conditional formatting based on comparison
///
/// The iter() method creates an iterator over references to the slice elements.
/// This allows us to iterate without taking ownership of the data.
///
/// # Arguments
///
/// * `contexts` - A slice of context name references
/// * `active` - The name of the currently active context
///
/// # Example
///
/// ```
/// use rust_todo::display::display_contexts;
///
/// let contexts = vec!["default", "work", "personal"];
/// display_contexts(&contexts, "work");
/// ```
pub fn display_contexts(contexts: &[&str], active: &str) {
    // Print a header
    println!("{}", "Available Contexts:".bold().underline());
    println!();
    
    // Check if there are any contexts to display
    if contexts.is_empty() {
        println!("{}", "  No contexts available.".dimmed());
        return;
    }
    
    // Iterate through each context name
    for context_name in contexts.iter() {
        // Check if this is the active context
        // We use == to compare string slices
        if *context_name == active {
            // Display the active context with a marker and in green
            println!("  {} {}", "●".green().bold(), context_name.green().bold());
        } else {
            // Display inactive contexts in normal color
            println!("  {} {}", "○".dimmed(), context_name);
        }
    }
    
    // Print a legend
    println!();
    println!("{}", "  ● = active context".dimmed());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::{Task, TimeHorizon, Priority};

    #[test]
    fn test_format_task_line_incomplete() {
        // Test formatting an incomplete task
        let task = Task::new(
            "Test task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High
        );
        
        let line = format_task_line(&task);
        
        // Verify the line contains expected elements
        // Note: We can't easily test color codes, so we check for content
        assert!(line.contains("[ ]"));
        assert!(line.contains("Test task"));
        assert!(line.contains("HIGH"));
    }

    #[test]
    fn test_format_task_line_completed() {
        // Test formatting a completed task
        let mut task = Task::new(
            "Completed task".to_string(),
            TimeHorizon::MidTerm,
            Priority::Medium
        );
        task.mark_complete();
        
        let line = format_task_line(&task);
        
        // Verify the line contains expected elements
        assert!(line.contains("[✓]"));
        assert!(line.contains("Completed task"));
        assert!(line.contains("MED"));
    }

    #[test]
    fn test_format_task_line_low_priority() {
        // Test formatting a low priority task
        let task = Task::new(
            "Low priority task".to_string(),
            TimeHorizon::LongTerm,
            Priority::Low
        );
        
        let line = format_task_line(&task);
        
        // Verify the line contains expected elements
        assert!(line.contains("[ ]"));
        assert!(line.contains("Low priority task"));
        assert!(line.contains("LOW"));
    }

    #[test]
    fn test_format_task_line_short_id() {
        // Test that the task ID is shortened
        let task = Task::new(
            "Test".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Medium
        );
        
        let line = format_task_line(&task);
        
        // The line should contain the first 6 characters of the ID
        let short_id = &task.id[..6];
        assert!(line.contains(short_id));
        
        // The line should not contain the full ID (36 characters)
        // We check that it doesn't contain a substring that would only appear in the full ID
        let full_id_suffix = &task.id[6..];
        assert!(!line.contains(full_id_suffix));
    }

    #[test]
    fn test_display_tasks_empty() {
        // Test displaying an empty task list
        let tasks: Vec<&Task> = vec![];
        
        // This should not panic
        display_tasks(&tasks, true);
    }

    #[test]
    fn test_display_tasks_single() {
        // Test displaying a single task
        let task = Task::new(
            "Single task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High
        );
        let tasks = vec![&task];
        
        // This should not panic
        display_tasks(&tasks, true);
    }

    #[test]
    fn test_display_tasks_multiple_horizons() {
        // Test displaying tasks from multiple time horizons
        let task1 = Task::new("Short".to_string(), TimeHorizon::ShortTerm, Priority::High);
        let task2 = Task::new("Mid".to_string(), TimeHorizon::MidTerm, Priority::Medium);
        let task3 = Task::new("Long".to_string(), TimeHorizon::LongTerm, Priority::Low);
        
        let tasks = vec![&task1, &task2, &task3];
        
        // This should not panic
        display_tasks(&tasks, true);
    }

    #[test]
    fn test_display_tasks_filter_completed() {
        // Test that completed tasks are filtered when show_completed is false
        let task1 = Task::new("Incomplete".to_string(), TimeHorizon::ShortTerm, Priority::High);
        let mut task2 = Task::new("Complete".to_string(), TimeHorizon::ShortTerm, Priority::High);
        task2.mark_complete();
        
        let tasks = vec![&task1, &task2];
        
        // This should not panic and should only show incomplete tasks
        display_tasks(&tasks, false);
    }

    #[test]
    fn test_display_task_detail() {
        // Test displaying detailed task information
        let task = Task::new(
            "Detailed task".to_string(),
            TimeHorizon::MidTerm,
            Priority::High
        );
        
        // This should not panic
        display_task_detail(&task);
    }

    #[test]
    fn test_display_task_detail_completed() {
        // Test displaying details of a completed task
        let mut task = Task::new(
            "Completed task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Low
        );
        task.mark_complete();
        
        // This should not panic
        display_task_detail(&task);
    }

    #[test]
    fn test_display_contexts_empty() {
        // Test displaying an empty context list
        let contexts: Vec<&str> = vec![];
        
        // This should not panic
        display_contexts(&contexts, "default");
    }

    #[test]
    fn test_display_contexts_single() {
        // Test displaying a single context
        let contexts = vec!["default"];
        
        // This should not panic
        display_contexts(&contexts, "default");
    }

    #[test]
    fn test_display_contexts_multiple() {
        // Test displaying multiple contexts
        let contexts = vec!["default", "work", "personal"];
        
        // This should not panic
        display_contexts(&contexts, "work");
    }

    #[test]
    fn test_display_contexts_active_indicator() {
        // Test that the active context is indicated
        let contexts = vec!["default", "work", "personal"];
        
        // This should not panic and should highlight "work" as active
        display_contexts(&contexts, "work");
    }
}
