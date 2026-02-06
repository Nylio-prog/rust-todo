// Example demonstrating the display module functionality
// Run with: cargo run --example display_demo

use rust_todo::display::{display_contexts, display_task_detail, display_tasks, format_task_line};
use rust_todo::task::{Priority, Task, TimeHorizon};

fn main() {
    println!("=== Display Module Demo ===\n");

    // Create some sample tasks
    let mut task1 = Task::new(
        "Write documentation for display module".to_string(),
        TimeHorizon::ShortTerm,
        Priority::High,
    );

    let task2 = Task::new(
        "Refactor context manager".to_string(),
        TimeHorizon::MidTerm,
        Priority::Medium,
    );

    let mut task3 = Task::new(
        "Learn advanced Rust patterns".to_string(),
        TimeHorizon::LongTerm,
        Priority::Low,
    );

    let task4 = Task::new(
        "Fix bug in task sorting".to_string(),
        TimeHorizon::ShortTerm,
        Priority::High,
    );

    let task5 = Task::new(
        "Implement property-based tests".to_string(),
        TimeHorizon::MidTerm,
        Priority::High,
    );

    // Mark some tasks as complete
    task1.mark_complete();
    task3.mark_complete();

    // Demo 1: Format individual task lines
    println!("--- Individual Task Formatting ---\n");
    println!("{}", format_task_line(&task1));
    println!("{}", format_task_line(&task2));
    println!("{}", format_task_line(&task3));
    println!();

    // Demo 2: Display all tasks grouped by horizon
    println!("--- All Tasks (including completed) ---\n");
    let all_tasks = vec![&task1, &task2, &task3, &task4, &task5];
    display_tasks(&all_tasks, true);
    println!();

    // Demo 3: Display only incomplete tasks
    println!("--- Incomplete Tasks Only ---\n");
    display_tasks(&all_tasks, false);
    println!();

    // Demo 4: Display detailed task information
    println!("--- Task Detail View ---\n");
    display_task_detail(&task2);
    println!();

    // Demo 5: Display contexts
    println!("--- Context List ---\n");
    let contexts = vec!["default", "work", "personal", "learning"];
    display_contexts(&contexts, "work");
    println!();

    println!("=== Demo Complete ===");
}
