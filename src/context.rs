// Context module - manages project contexts and their associated tasks
// This module demonstrates Rust's HashMap usage, borrowing patterns, and error handling

use crate::task::Task;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a project context containing tasks
///
/// A Context is similar to a git branch - it's a named workspace that contains
/// its own set of tasks. This allows users to organize tasks by project or
/// area of responsibility.
///
/// # Ownership and Collections
///
/// The Context struct owns a Vec<Task>, which means:
/// 1. The Context owns all its tasks
/// 2. When a Context is dropped, all its tasks are automatically cleaned up
/// 3. We can freely add, remove, and modify tasks within the context
///
/// Vec<T> is Rust's growable array type. It's heap-allocated and can grow
/// or shrink as needed. This is perfect for storing a dynamic list of tasks.
///
/// # Serialization
///
/// The #[derive(Serialize, Deserialize)] attributes enable automatic JSON
/// conversion, allowing contexts to be saved to and loaded from disk.
///
/// # Fields
///
/// - `name`: The context's name (e.g., "work", "personal", "learning")
/// - `tasks`: A vector of tasks belonging to this context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    /// The name of this context
    pub name: String,

    /// The tasks belonging to this context
    pub tasks: Vec<Task>,
}

impl Context {
    /// Creates a new context with the given name and an empty task list
    ///
    /// This constructor demonstrates:
    /// - Taking ownership of the name String
    /// - Creating an empty Vec with Vec::new()
    /// - Returning an owned value (Self)
    ///
    /// # Arguments
    ///
    /// * `name` - The name for this context (ownership is transferred)
    ///
    /// # Returns
    ///
    /// A new Context with the specified name and an empty task list.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::context::Context;
    ///
    /// let context = Context::new("work".to_string());
    /// assert_eq!(context.name, "work");
    /// assert_eq!(context.tasks.len(), 0);
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            tasks: Vec::new(), // Create an empty vector for tasks
        }
    }

    /// Adds a task to the context
    ///
    /// This method demonstrates Vec operations and ownership transfer.
    ///
    /// # Ownership and Vec::push
    ///
    /// The `push` method takes ownership of the task and adds it to the Vec.
    /// After calling this method, the task is owned by the context's Vec,
    /// and the caller can no longer use the original task variable.
    ///
    /// Vec::push is an O(1) operation (amortized) because Vec pre-allocates
    /// extra capacity. When the Vec is full, it allocates a new, larger buffer
    /// and moves all elements to it.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to add (ownership is transferred to the context)
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::context::Context;
    /// use rust_todo::task::{Task, TimeHorizon, Priority};
    ///
    /// let mut context = Context::new("work".to_string());
    /// let task = Task::new("Write code".to_string(), TimeHorizon::ShortTerm, Priority::High);
    /// context.add_task(task);
    /// assert_eq!(context.tasks.len(), 1);
    /// ```
    pub fn add_task(&mut self, task: Task) {
        // Push the task onto the end of the Vec
        // The task is moved into the Vec, transferring ownership
        self.tasks.push(task);
    }

    /// Finds a task by ID and returns an immutable reference
    ///
    /// This method demonstrates:
    /// - Borrowing with immutable references (&)
    /// - Iterator methods (iter, find)
    /// - Option type for representing "found" or "not found"
    /// - Closures (the |task| ... syntax)
    ///
    /// # Borrowing and Lifetimes
    ///
    /// The return type Option<&Task> means:
    /// - If found, we return Some(&task) - a reference to the task
    /// - If not found, we return None
    ///
    /// The reference is tied to the lifetime of &self, meaning the returned
    /// reference is valid as long as the Context exists and isn't mutated.
    ///
    /// # Iterator Pattern
    ///
    /// The `iter()` method creates an iterator over immutable references to tasks.
    /// The `find()` method searches for the first element matching the predicate.
    /// This is more idiomatic than a manual for loop in Rust.
    ///
    /// # Arguments
    ///
    /// * `id` - The task ID to search for
    ///
    /// # Returns
    ///
    /// Some(&Task) if a task with the given ID exists, None otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::context::Context;
    /// use rust_todo::task::{Task, TimeHorizon, Priority};
    ///
    /// let mut context = Context::new("work".to_string());
    /// let task = Task::new("Test".to_string(), TimeHorizon::ShortTerm, Priority::Medium);
    /// let task_id = task.id.clone();
    /// context.add_task(task);
    ///
    /// let found = context.find_task(&task_id);
    /// assert!(found.is_some());
    /// assert_eq!(found.unwrap().description, "Test");
    /// ```
    pub fn find_task(&self, id: &str) -> Option<&Task> {
        // Use the iterator pattern to search for the task
        // iter() creates an iterator over &Task references
        // find() returns the first element where the closure returns true
        // The closure |task| ... is an anonymous function that takes a task reference
        self.tasks.iter().find(|task| task.id == id)
    }

    /// Finds a task by ID and returns a mutable reference
    ///
    /// This method is similar to find_task(), but returns a mutable reference
    /// that allows modifying the task.
    ///
    /// # Mutable Borrowing
    ///
    /// The return type Option<&mut Task> means:
    /// - If found, we return Some(&mut task) - a mutable reference to the task
    /// - If not found, we return None
    ///
    /// Rust's borrow checker ensures that:
    /// - Only one mutable reference exists at a time
    /// - No immutable references exist while a mutable reference is active
    /// - This prevents data races and ensures memory safety
    ///
    /// # Arguments
    ///
    /// * `id` - The task ID to search for
    ///
    /// # Returns
    ///
    /// Some(&mut Task) if a task with the given ID exists, None otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::context::Context;
    /// use rust_todo::task::{Task, TimeHorizon, Priority};
    ///
    /// let mut context = Context::new("work".to_string());
    /// let task = Task::new("Test".to_string(), TimeHorizon::ShortTerm, Priority::Medium);
    /// let task_id = task.id.clone();
    /// context.add_task(task);
    ///
    /// if let Some(task) = context.find_task_mut(&task_id) {
    ///     task.mark_complete();
    /// }
    ///
    /// let found = context.find_task(&task_id);
    /// assert_eq!(found.unwrap().completed, true);
    /// ```
    pub fn find_task_mut(&mut self, id: &str) -> Option<&mut Task> {
        // Use iter_mut() instead of iter() to get mutable references
        // This allows the caller to modify the found task
        self.tasks.iter_mut().find(|task| task.id == id)
    }

    /// Removes a task by ID and returns it
    ///
    /// This method demonstrates:
    /// - Error handling with Result type
    /// - Vec::remove() for removing elements by index
    /// - Iterator::position() for finding an element's index
    /// - Ownership transfer (the removed task is returned to the caller)
    ///
    /// # Error Handling
    ///
    /// If the task is not found, this method returns an AppError::TaskNotFound error.
    /// The caller can handle this error using match, if let, or the ? operator.
    ///
    /// # Vec::remove() Performance
    ///
    /// Vec::remove(index) is O(n) because it shifts all elements after the removed
    /// element to fill the gap. For better performance with frequent removals,
    /// consider using Vec::swap_remove(), which is O(1) but doesn't preserve order.
    ///
    /// However, for a personal todo app with hundreds of tasks, this performance
    /// difference is negligible.
    ///
    /// # Arguments
    ///
    /// * `id` - The task ID to remove
    ///
    /// # Returns
    ///
    /// Ok(Task) with the removed task if found, or Err(AppError::TaskNotFound) if not found.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::context::Context;
    /// use rust_todo::task::{Task, TimeHorizon, Priority};
    ///
    /// let mut context = Context::new("work".to_string());
    /// let task = Task::new("Test".to_string(), TimeHorizon::ShortTerm, Priority::Medium);
    /// let task_id = task.id.clone();
    /// context.add_task(task);
    ///
    /// let removed = context.remove_task(&task_id).unwrap();
    /// assert_eq!(removed.description, "Test");
    /// assert_eq!(context.tasks.len(), 0);
    /// ```
    pub fn remove_task(&mut self, id: &str) -> crate::error::Result<Task> {
        // Find the index of the task with the given ID
        // position() returns Some(index) if found, None otherwise
        let position = self.tasks.iter().position(|task| task.id == id);

        // Use match to handle the Option
        match position {
            Some(index) => {
                // Task found - remove it and return it
                // remove() takes ownership of the element and returns it
                Ok(self.tasks.remove(index))
            }
            None => {
                // Task not found - return an error
                // We need to import AppError from the error module
                Err(crate::error::AppError::TaskNotFound(id.to_string()))
            }
        }
    }

    /// Gets tasks filtered by time horizon
    ///
    /// This method demonstrates:
    /// - Iterator methods (iter, filter, collect)
    /// - Closures with pattern matching
    /// - Collecting iterator results into a Vec
    ///
    /// # Iterator Pattern
    ///
    /// Rust's iterators are lazy - they don't do any work until you consume them
    /// with methods like collect(), for_each(), or fold(). This allows the compiler
    /// to optimize iterator chains very efficiently.
    ///
    /// The filter() method creates a new iterator that only yields elements
    /// matching the predicate. The collect() method consumes the iterator and
    /// builds a Vec from the results.
    ///
    /// # Arguments
    ///
    /// * `horizon` - The time horizon to filter by
    ///
    /// # Returns
    ///
    /// A Vec of immutable references to tasks matching the specified time horizon.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::context::Context;
    /// use rust_todo::task::{Task, TimeHorizon, Priority};
    ///
    /// let mut context = Context::new("work".to_string());
    /// context.add_task(Task::new("Short 1".to_string(), TimeHorizon::ShortTerm, Priority::High));
    /// context.add_task(Task::new("Long 1".to_string(), TimeHorizon::LongTerm, Priority::Medium));
    /// context.add_task(Task::new("Short 2".to_string(), TimeHorizon::ShortTerm, Priority::Low));
    ///
    /// let short_tasks = context.tasks_by_horizon(TimeHorizon::ShortTerm);
    /// assert_eq!(short_tasks.len(), 2);
    /// ```
    pub fn tasks_by_horizon(&self, horizon: crate::task::TimeHorizon) -> Vec<&Task> {
        // Use iterator methods to filter tasks
        // iter() creates an iterator over &Task references
        // filter() keeps only tasks matching the time horizon
        // collect() builds a Vec from the filtered results
        self.tasks
            .iter()
            .filter(|task| task.time_horizon == horizon)
            .collect()
    }

    /// Gets all tasks sorted by priority within their time horizon
    ///
    /// This method demonstrates:
    /// - Sorting with custom comparison logic
    /// - Tuple comparison for multi-level sorting
    /// - Cloning references for sorting
    ///
    /// # Sorting Strategy
    ///
    /// Tasks are sorted by:
    /// 1. Time horizon (ShortTerm < MidTerm < LongTerm)
    /// 2. Priority within each horizon (High > Medium > Low, so we reverse)
    ///
    /// We use tuple comparison: (horizon, -priority) where we reverse the priority
    /// order by using Reverse wrapper or by comparing in reverse order.
    ///
    /// # Performance Note
    ///
    /// This method creates a new Vec and sorts it, which is O(n log n).
    /// For a personal todo app with hundreds of tasks, this is fast enough.
    /// The original tasks Vec remains unchanged.
    ///
    /// # Returns
    ///
    /// A Vec of immutable references to tasks, sorted by time horizon and then
    /// by priority (high to low) within each horizon.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::context::Context;
    /// use rust_todo::task::{Task, TimeHorizon, Priority};
    ///
    /// let mut context = Context::new("work".to_string());
    /// context.add_task(Task::new("Short Low".to_string(), TimeHorizon::ShortTerm, Priority::Low));
    /// context.add_task(Task::new("Long High".to_string(), TimeHorizon::LongTerm, Priority::High));
    /// context.add_task(Task::new("Short High".to_string(), TimeHorizon::ShortTerm, Priority::High));
    ///
    /// let sorted = context.sorted_tasks();
    /// // Should be: Short High, Short Low, Long High
    /// assert_eq!(sorted[0].description, "Short High");
    /// assert_eq!(sorted[1].description, "Short Low");
    /// assert_eq!(sorted[2].description, "Long High");
    /// ```
    pub fn sorted_tasks(&self) -> Vec<&Task> {
        // Create a Vec of references to all tasks
        let mut tasks: Vec<&Task> = self.tasks.iter().collect();

        // Sort by time horizon first, then by priority (high to low) within each horizon
        // We use sort_by_key with a tuple for multi-level sorting
        // For priority, we want high to low, so we reverse the comparison
        tasks.sort_by_key(|task| {
            // Create a tuple for comparison
            // TimeHorizon variants are ordered: ShortTerm < MidTerm < LongTerm
            // Priority is ordered: Low < Medium < High
            // We want high priority first, so we use Reverse
            (task.time_horizon, std::cmp::Reverse(task.priority))
        });

        tasks
    }
}

/// Manages all contexts and tracks the active one
///
/// The ContextManager is the top-level data structure that holds all project
/// contexts and keeps track of which one is currently active. This is similar
/// to how git manages branches - you have multiple branches but only one is
/// checked out at a time.
///
/// # HashMap Usage
///
/// We use a HashMap<String, Context> to store contexts by name. HashMap provides:
/// - O(1) average-case lookup, insertion, and deletion
/// - Efficient storage for key-value pairs
/// - Automatic handling of hash collisions
///
/// HashMap is part of Rust's standard library and uses a cryptographically
/// secure hash function by default. For our use case (context names), this
/// provides good performance and prevents hash collision attacks.
///
/// # Serialization
///
/// The #[derive(Serialize, Deserialize)] attributes enable automatic JSON
/// conversion. When serialized, the HashMap becomes a JSON object with
/// context names as keys.
///
/// # Fields
///
/// - `contexts`: A HashMap mapping context names to Context objects
/// - `active_context`: The name of the currently active context
///
/// # Invariants
///
/// The ContextManager maintains these invariants:
/// 1. There is always at least one context (the default context)
/// 2. The active_context always refers to an existing context in the HashMap
/// 3. Context names are unique (enforced by HashMap)
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextManager {
    /// All contexts, indexed by name
    pub contexts: HashMap<String, Context>,

    /// The name of the currently active context
    pub active_context: String,
}

impl Default for ContextManager {
    /// Creates a default ContextManager with a "default" context
    ///
    /// This implements the Default trait, which is a standard Rust trait
    /// for types that have a sensible default value. This allows:
    /// - Using ContextManager::default() instead of ContextManager::new()
    /// - Automatic initialization in some contexts (e.g., #[derive(Default)])
    /// - Integration with Rust's ecosystem (many libraries expect Default)
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::context::ContextManager;
    ///
    /// let manager = ContextManager::default();
    /// assert_eq!(manager.active_context, "default");
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl ContextManager {
    /// Creates a new ContextManager with a default context
    ///
    /// This constructor demonstrates:
    /// - Creating and populating a HashMap
    /// - Using the insert() method to add key-value pairs
    /// - Establishing initial state with invariants satisfied
    ///
    /// # Default Context
    ///
    /// The default context is named "default" and starts with an empty task list.
    /// This ensures that:
    /// 1. Users always have a context to work with
    /// 2. The active_context field always points to a valid context
    /// 3. The application can start immediately without setup
    ///
    /// # HashMap::new() and insert()
    ///
    /// HashMap::new() creates an empty HashMap with no allocated capacity.
    /// The HashMap will allocate memory as needed when items are inserted.
    ///
    /// The insert() method:
    /// - Takes ownership of both the key and value
    /// - Returns None if the key didn't exist
    /// - Returns Some(old_value) if the key existed (replacing the old value)
    ///
    /// # Returns
    ///
    /// A new ContextManager with a single "default" context that is active.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::context::ContextManager;
    ///
    /// let manager = ContextManager::new();
    /// assert_eq!(manager.active_context, "default");
    /// assert_eq!(manager.contexts.len(), 1);
    /// assert!(manager.contexts.contains_key("default"));
    /// ```
    pub fn new() -> Self {
        // Create an empty HashMap to store contexts
        let mut contexts = HashMap::new();

        // Create the default context
        let default_context = Context::new("default".to_string());

        // Insert the default context into the HashMap
        // The key is the context name, the value is the Context object
        contexts.insert("default".to_string(), default_context);

        // Return the ContextManager with the default context active
        Self {
            contexts,
            active_context: "default".to_string(),
        }
    }
}

impl ContextManager {
    /// Creates a new context with the given name
    ///
    /// This method demonstrates:
    /// - HashMap insertion with duplicate checking
    /// - Error handling with Result type
    /// - Ownership transfer of the context name
    ///
    /// # Duplicate Name Checking
    ///
    /// Before creating a new context, we check if a context with the same name
    /// already exists using HashMap::contains_key(). This is an O(1) operation
    /// thanks to HashMap's hash-based lookup.
    ///
    /// If a duplicate is found, we return an error without modifying the HashMap.
    /// This ensures that context names remain unique.
    ///
    /// # HashMap::insert()
    ///
    /// The insert() method takes ownership of both the key (String) and value (Context).
    /// If the key already exists, it would replace the old value and return Some(old_value).
    /// However, we check for duplicates first, so insert() will always return None here.
    ///
    /// # Arguments
    ///
    /// * `name` - The name for the new context (ownership is transferred)
    ///
    /// # Returns
    ///
    /// Ok(()) if the context was created successfully, or
    /// Err(AppError::ContextAlreadyExists) if a context with that name already exists.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::context::ContextManager;
    ///
    /// let mut manager = ContextManager::new();
    ///
    /// // Create a new context
    /// let result = manager.create_context("work".to_string());
    /// assert!(result.is_ok());
    /// assert_eq!(manager.contexts.len(), 2); // default + work
    ///
    /// // Try to create a duplicate
    /// let result = manager.create_context("work".to_string());
    /// assert!(result.is_err());
    /// ```
    pub fn create_context(&mut self, name: String) -> crate::error::Result<()> {
        // Check if a context with this name already exists
        // contains_key() borrows the key, so we pass a reference
        if self.contexts.contains_key(&name) {
            // Context already exists - return an error
            return Err(crate::error::AppError::ContextAlreadyExists(name));
        }

        // Create a new context with the given name
        let context = Context::new(name.clone());

        // Insert the context into the HashMap
        // insert() takes ownership of both the key and value
        self.contexts.insert(name, context);

        // Return success
        Ok(())
    }

    /// Switches to a different context
    ///
    /// This method demonstrates:
    /// - HashMap lookup with contains_key()
    /// - Mutable state modification
    /// - Error handling for invalid context names
    ///
    /// # Context Validation
    ///
    /// Before switching, we verify that the target context exists using
    /// HashMap::contains_key(). This prevents setting active_context to
    /// an invalid value, which would violate our invariant that active_context
    /// always refers to an existing context.
    ///
    /// # String Ownership
    ///
    /// The method takes a &str parameter (borrowed string slice) rather than
    /// String (owned string). This is more flexible because:
    /// - Callers can pass &String, &str, or string literals
    /// - We don't need to take ownership of the caller's string
    /// - We create our own String when updating active_context
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the context to switch to (borrowed)
    ///
    /// # Returns
    ///
    /// Ok(()) if the context was switched successfully, or
    /// Err(AppError::ContextNotFound) if no context with that name exists.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::context::ContextManager;
    ///
    /// let mut manager = ContextManager::new();
    /// manager.create_context("work".to_string()).unwrap();
    ///
    /// // Switch to the work context
    /// let result = manager.switch_context("work");
    /// assert!(result.is_ok());
    /// assert_eq!(manager.active_context, "work");
    ///
    /// // Try to switch to a non-existent context
    /// let result = manager.switch_context("nonexistent");
    /// assert!(result.is_err());
    /// ```
    pub fn switch_context(&mut self, name: &str) -> crate::error::Result<()> {
        // Check if the context exists
        if !self.contexts.contains_key(name) {
            // Context not found - return an error
            return Err(crate::error::AppError::ContextNotFound(name.to_string()));
        }

        // Update the active context
        // We create a new String from the &str
        self.active_context = name.to_string();

        // Return success
        Ok(())
    }

    /// Deletes a context and all its tasks
    ///
    /// This method demonstrates:
    /// - HashMap removal with remove()
    /// - Business logic constraints (can't delete last context)
    /// - Multiple error conditions
    ///
    /// # Last Context Protection
    ///
    /// We enforce a business rule that at least one context must always exist.
    /// This ensures:
    /// - Users always have a context to work with
    /// - The active_context field always points to a valid context
    /// - The application never enters an invalid state
    ///
    /// We check the HashMap size before deletion. If there's only one context,
    /// we return an error without modifying the HashMap.
    ///
    /// # Active Context Handling
    ///
    /// If the user tries to delete the currently active context, we return an
    /// error. The user must switch to a different context first. This prevents
    /// the active_context field from pointing to a non-existent context.
    ///
    /// # HashMap::remove()
    ///
    /// The remove() method:
    /// - Takes ownership of the key (or borrows it)
    /// - Removes the key-value pair from the HashMap
    /// - Returns Some(value) if the key existed, None otherwise
    /// - Is an O(1) operation on average
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the context to delete (borrowed)
    ///
    /// # Returns
    ///
    /// Ok(()) if the context was deleted successfully,
    /// Err(AppError::ContextNotFound) if no context with that name exists,
    /// Err(AppError::CannotDeleteLastContext) if this is the only context, or
    /// Err(AppError::ContextNotFound) if trying to delete the active context.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::context::ContextManager;
    ///
    /// let mut manager = ContextManager::new();
    /// manager.create_context("work".to_string()).unwrap();
    /// manager.create_context("personal".to_string()).unwrap();
    ///
    /// // Delete a context (not active)
    /// let result = manager.delete_context("work");
    /// assert!(result.is_ok());
    /// assert_eq!(manager.contexts.len(), 2); // default + personal
    ///
    /// // Try to delete the last context
    /// manager.delete_context("personal").unwrap();
    /// let result = manager.delete_context("default");
    /// assert!(result.is_err()); // Can't delete the last context
    /// ```
    pub fn delete_context(&mut self, name: &str) -> crate::error::Result<()> {
        // Check if the context exists first
        if !self.contexts.contains_key(name) {
            return Err(crate::error::AppError::ContextNotFound(name.to_string()));
        }

        // Check if this is the last context
        if self.contexts.len() <= 1 {
            return Err(crate::error::AppError::CannotDeleteLastContext);
        }

        // Check if this is the active context
        if self.active_context == name {
            // User must switch to a different context first
            // We could auto-switch, but explicit is better than implicit
            return Err(crate::error::AppError::ContextNotFound(format!(
                "Cannot delete active context '{}'. Switch to another context first.",
                name
            )));
        }

        // Remove the context from the HashMap
        // remove() returns Some(context) if found, None otherwise
        // We already checked that it exists, so we can safely unwrap
        self.contexts.remove(name);

        // Return success
        Ok(())
    }

    /// Gets an immutable reference to the active context
    ///
    /// This method demonstrates:
    /// - HashMap lookup with get()
    /// - Unwrapping with confidence (we maintain the invariant)
    /// - Returning borrowed references
    ///
    /// # Invariant Assumption
    ///
    /// This method uses unwrap() because we maintain the invariant that
    /// active_context always refers to an existing context. This invariant
    /// is established in new() and maintained by all mutation methods.
    ///
    /// Using unwrap() here is safe because:
    /// 1. new() creates a default context and sets it as active
    /// 2. switch_context() validates the context exists before switching
    /// 3. delete_context() prevents deleting the active context
    /// 4. create_context() doesn't modify active_context
    ///
    /// If this unwrap() ever panics, it indicates a bug in our code that
    /// violated the invariant.
    ///
    /// # HashMap::get()
    ///
    /// The get() method:
    /// - Takes a borrowed key (&K)
    /// - Returns Option<&V> - a reference to the value if found
    /// - Is an O(1) operation on average
    /// - Does not remove the value from the HashMap
    ///
    /// # Returns
    ///
    /// An immutable reference to the active Context.
    ///
    /// # Panics
    ///
    /// Panics if the active_context doesn't exist in the HashMap. This should
    /// never happen if our invariants are maintained correctly.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::context::ContextManager;
    ///
    /// let manager = ContextManager::new();
    /// let context = manager.active_context();
    /// assert_eq!(context.name, "default");
    /// ```
    pub fn active_context(&self) -> &Context {
        // Get the active context from the HashMap
        // We use unwrap() because we maintain the invariant that active_context
        // always refers to an existing context
        self.contexts
            .get(&self.active_context)
            .expect("Active context must exist in contexts HashMap")
    }

    /// Gets a mutable reference to the active context
    ///
    /// This method is similar to active_context(), but returns a mutable reference
    /// that allows modifying the context (e.g., adding or removing tasks).
    ///
    /// # Mutable Borrowing
    ///
    /// The return type &mut Context means:
    /// - The caller can modify the context
    /// - Only one mutable reference can exist at a time (enforced by borrow checker)
    /// - No immutable references can exist while a mutable reference is active
    ///
    /// This is Rust's way of preventing data races at compile time.
    ///
    /// # HashMap::get_mut()
    ///
    /// The get_mut() method is like get(), but returns Option<&mut V> instead
    /// of Option<&V>. This allows the caller to modify the value in place.
    ///
    /// # Returns
    ///
    /// A mutable reference to the active Context.
    ///
    /// # Panics
    ///
    /// Panics if the active_context doesn't exist in the HashMap. This should
    /// never happen if our invariants are maintained correctly.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::context::ContextManager;
    /// use rust_todo::task::{Task, TimeHorizon, Priority};
    ///
    /// let mut manager = ContextManager::new();
    ///
    /// // Get mutable reference and add a task
    /// let context = manager.active_context_mut();
    /// let task = Task::new("Test".to_string(), TimeHorizon::ShortTerm, Priority::High);
    /// context.add_task(task);
    ///
    /// assert_eq!(manager.active_context().tasks.len(), 1);
    /// ```
    pub fn active_context_mut(&mut self) -> &mut Context {
        // Get a mutable reference to the active context
        // We use unwrap() for the same reason as active_context()
        self.contexts
            .get_mut(&self.active_context)
            .expect("Active context must exist in contexts HashMap")
    }

    /// Lists all context names
    ///
    /// This method demonstrates:
    /// - HashMap iteration with keys()
    /// - Iterator methods (map, collect)
    /// - Converting owned Strings to borrowed &str
    ///
    /// # HashMap::keys()
    ///
    /// The keys() method returns an iterator over references to the keys.
    /// For HashMap<String, Context>, this gives us an iterator over &String.
    ///
    /// # String to &str Conversion
    ///
    /// We use map(|s| s.as_str()) to convert &String to &str. This is a
    /// zero-cost conversion that creates a string slice pointing to the
    /// String's data.
    ///
    /// Why return Vec<&str> instead of Vec<String>?
    /// - More efficient: no cloning of strings
    /// - More flexible: callers can use the references without taking ownership
    /// - Idiomatic: Rust prefers borrowing over cloning when possible
    ///
    /// # Lifetime Elision
    ///
    /// The return type Vec<&str> has an implicit lifetime tied to &self.
    /// The full signature would be: fn list_contexts<'a>(&'a self) -> Vec<&'a str>
    ///
    /// This means the returned references are valid as long as the ContextManager
    /// exists and isn't mutated.
    ///
    /// # Returns
    ///
    /// A Vec of string slices containing all context names.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_todo::context::ContextManager;
    ///
    /// let mut manager = ContextManager::new();
    /// manager.create_context("work".to_string()).unwrap();
    /// manager.create_context("personal".to_string()).unwrap();
    ///
    /// let names = manager.list_contexts();
    /// assert_eq!(names.len(), 3); // default + work + personal
    /// assert!(names.contains(&"default"));
    /// assert!(names.contains(&"work"));
    /// assert!(names.contains(&"personal"));
    /// ```
    pub fn list_contexts(&self) -> Vec<&str> {
        // Get an iterator over the keys (context names)
        // Convert each &String to &str using as_str()
        // Collect the results into a Vec
        self.contexts.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::{Priority, TimeHorizon};

    #[test]
    fn test_context_new() {
        // Test creating a new context
        let context = Context::new("work".to_string());

        // Verify the name is set correctly
        assert_eq!(context.name, "work");

        // Verify the task list is empty
        assert_eq!(context.tasks.len(), 0);
        assert!(context.tasks.is_empty());
    }

    #[test]
    fn test_context_with_different_names() {
        // Test creating contexts with various names
        let work = Context::new("work".to_string());
        let personal = Context::new("personal".to_string());
        let learning = Context::new("learning".to_string());

        assert_eq!(work.name, "work");
        assert_eq!(personal.name, "personal");
        assert_eq!(learning.name, "learning");
    }

    #[test]
    fn test_context_serialization() {
        // Test that Context can be serialized to JSON
        let context = Context::new("test".to_string());

        let json = serde_json::to_string(&context).unwrap();

        // Verify JSON contains expected fields
        assert!(json.contains("\"name\""));
        assert!(json.contains("\"tasks\""));
        assert!(json.contains("test"));
    }

    #[test]
    fn test_context_deserialization() {
        // Test that Context can be deserialized from JSON
        let json = r#"{
            "name": "test-context",
            "tasks": []
        }"#;

        let context: Context = serde_json::from_str(json).unwrap();

        assert_eq!(context.name, "test-context");
        assert_eq!(context.tasks.len(), 0);
    }

    #[test]
    fn test_context_with_tasks_serialization() {
        // Test serialization of a context with tasks
        let mut context = Context::new("work".to_string());

        // Add a task to the context
        let task = Task::new(
            "Test task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High,
        );
        context.tasks.push(task);

        let json = serde_json::to_string(&context).unwrap();

        // Verify JSON contains task data
        assert!(json.contains("Test task"));
        assert!(json.contains("ShortTerm"));
        assert!(json.contains("High"));
    }

    #[test]
    fn test_context_with_tasks_deserialization() {
        // Test deserialization of a context with tasks
        let json = r#"{
            "name": "work",
            "tasks": [
                {
                    "id": "123e4567-e89b-12d3-a456-426614174000",
                    "description": "Test task",
                    "time_horizon": "MidTerm",
                    "priority": "Medium",
                    "completed": false,
                    "created_at": "2024-01-15T10:30:00Z"
                }
            ]
        }"#;

        let context: Context = serde_json::from_str(json).unwrap();

        assert_eq!(context.name, "work");
        assert_eq!(context.tasks.len(), 1);
        assert_eq!(context.tasks[0].description, "Test task");
        assert_eq!(context.tasks[0].time_horizon, TimeHorizon::MidTerm);
        assert_eq!(context.tasks[0].priority, Priority::Medium);
    }

    #[test]
    fn test_context_clone() {
        // Test that Context can be cloned
        let mut context1 = Context::new("original".to_string());

        let task = Task::new("Task 1".to_string(), TimeHorizon::ShortTerm, Priority::Low);
        context1.tasks.push(task);

        // Clone the context
        let context2 = context1.clone();

        // Both contexts should have the same data
        assert_eq!(context1.name, context2.name);
        assert_eq!(context1.tasks.len(), context2.tasks.len());
        assert_eq!(context1.tasks[0].description, context2.tasks[0].description);
    }

    #[test]
    fn test_add_task() {
        // Test adding tasks to a context
        let mut context = Context::new("work".to_string());

        let task1 = Task::new("Task 1".to_string(), TimeHorizon::ShortTerm, Priority::High);
        let task2 = Task::new("Task 2".to_string(), TimeHorizon::MidTerm, Priority::Low);

        context.add_task(task1);
        assert_eq!(context.tasks.len(), 1);

        context.add_task(task2);
        assert_eq!(context.tasks.len(), 2);

        assert_eq!(context.tasks[0].description, "Task 1");
        assert_eq!(context.tasks[1].description, "Task 2");
    }

    #[test]
    fn test_find_task() {
        // Test finding a task by ID
        let mut context = Context::new("work".to_string());

        let task = Task::new(
            "Test task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Medium,
        );
        let task_id = task.id.clone();
        context.add_task(task);

        // Find the task
        let found = context.find_task(&task_id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().description, "Test task");

        // Try to find a non-existent task
        let not_found = context.find_task("non-existent-id");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_find_task_mut() {
        // Test finding and modifying a task
        let mut context = Context::new("work".to_string());

        let task = Task::new(
            "Test task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Medium,
        );
        let task_id = task.id.clone();
        context.add_task(task);

        // Find and modify the task
        if let Some(task) = context.find_task_mut(&task_id) {
            task.mark_complete();
        }

        // Verify the task was modified
        let found = context.find_task(&task_id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().completed, true);
    }

    #[test]
    fn test_remove_task() {
        // Test removing a task
        let mut context = Context::new("work".to_string());

        let task1 = Task::new("Task 1".to_string(), TimeHorizon::ShortTerm, Priority::High);
        let task2 = Task::new("Task 2".to_string(), TimeHorizon::MidTerm, Priority::Low);
        let task1_id = task1.id.clone();
        let task2_id = task2.id.clone();

        context.add_task(task1);
        context.add_task(task2);
        assert_eq!(context.tasks.len(), 2);

        // Remove the first task
        let removed = context.remove_task(&task1_id);
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().description, "Task 1");
        assert_eq!(context.tasks.len(), 1);

        // Verify the remaining task is task2
        assert_eq!(context.tasks[0].id, task2_id);
    }

    #[test]
    fn test_remove_task_not_found() {
        // Test removing a non-existent task
        let mut context = Context::new("work".to_string());

        let task = Task::new("Task 1".to_string(), TimeHorizon::ShortTerm, Priority::High);
        context.add_task(task);

        // Try to remove a non-existent task
        let result = context.remove_task("non-existent-id");
        assert!(result.is_err());

        // Verify the error type
        match result {
            Err(crate::error::AppError::TaskNotFound(id)) => {
                assert_eq!(id, "non-existent-id");
            }
            _ => panic!("Expected TaskNotFound error"),
        }

        // Verify the original task is still there
        assert_eq!(context.tasks.len(), 1);
    }

    #[test]
    fn test_tasks_by_horizon() {
        // Test filtering tasks by time horizon
        let mut context = Context::new("work".to_string());

        context.add_task(Task::new(
            "Short 1".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High,
        ));
        context.add_task(Task::new(
            "Mid 1".to_string(),
            TimeHorizon::MidTerm,
            Priority::Medium,
        ));
        context.add_task(Task::new(
            "Short 2".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Low,
        ));
        context.add_task(Task::new(
            "Long 1".to_string(),
            TimeHorizon::LongTerm,
            Priority::High,
        ));
        context.add_task(Task::new(
            "Mid 2".to_string(),
            TimeHorizon::MidTerm,
            Priority::Low,
        ));

        // Filter by ShortTerm
        let short_tasks = context.tasks_by_horizon(TimeHorizon::ShortTerm);
        assert_eq!(short_tasks.len(), 2);
        assert!(short_tasks
            .iter()
            .all(|t| t.time_horizon == TimeHorizon::ShortTerm));

        // Filter by MidTerm
        let mid_tasks = context.tasks_by_horizon(TimeHorizon::MidTerm);
        assert_eq!(mid_tasks.len(), 2);
        assert!(mid_tasks
            .iter()
            .all(|t| t.time_horizon == TimeHorizon::MidTerm));

        // Filter by LongTerm
        let long_tasks = context.tasks_by_horizon(TimeHorizon::LongTerm);
        assert_eq!(long_tasks.len(), 1);
        assert_eq!(long_tasks[0].description, "Long 1");
    }

    #[test]
    fn test_tasks_by_horizon_empty() {
        // Test filtering when no tasks match
        let mut context = Context::new("work".to_string());

        context.add_task(Task::new(
            "Short 1".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High,
        ));

        // Filter by LongTerm (no matches)
        let long_tasks = context.tasks_by_horizon(TimeHorizon::LongTerm);
        assert_eq!(long_tasks.len(), 0);
    }

    #[test]
    fn test_sorted_tasks() {
        // Test sorting tasks by time horizon and priority
        let mut context = Context::new("work".to_string());

        // Add tasks in random order
        context.add_task(Task::new(
            "Long Low".to_string(),
            TimeHorizon::LongTerm,
            Priority::Low,
        ));
        context.add_task(Task::new(
            "Short Med".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Medium,
        ));
        context.add_task(Task::new(
            "Mid High".to_string(),
            TimeHorizon::MidTerm,
            Priority::High,
        ));
        context.add_task(Task::new(
            "Short High".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High,
        ));
        context.add_task(Task::new(
            "Short Low".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Low,
        ));
        context.add_task(Task::new(
            "Long High".to_string(),
            TimeHorizon::LongTerm,
            Priority::High,
        ));

        let sorted = context.sorted_tasks();

        // Verify the order: ShortTerm (High, Med, Low), MidTerm (High), LongTerm (High, Low)
        assert_eq!(sorted.len(), 6);

        // ShortTerm tasks should come first
        assert_eq!(sorted[0].description, "Short High");
        assert_eq!(sorted[0].time_horizon, TimeHorizon::ShortTerm);
        assert_eq!(sorted[0].priority, Priority::High);

        assert_eq!(sorted[1].description, "Short Med");
        assert_eq!(sorted[1].time_horizon, TimeHorizon::ShortTerm);
        assert_eq!(sorted[1].priority, Priority::Medium);

        assert_eq!(sorted[2].description, "Short Low");
        assert_eq!(sorted[2].time_horizon, TimeHorizon::ShortTerm);
        assert_eq!(sorted[2].priority, Priority::Low);

        // MidTerm tasks should come next
        assert_eq!(sorted[3].description, "Mid High");
        assert_eq!(sorted[3].time_horizon, TimeHorizon::MidTerm);

        // LongTerm tasks should come last
        assert_eq!(sorted[4].description, "Long High");
        assert_eq!(sorted[4].time_horizon, TimeHorizon::LongTerm);
        assert_eq!(sorted[4].priority, Priority::High);

        assert_eq!(sorted[5].description, "Long Low");
        assert_eq!(sorted[5].time_horizon, TimeHorizon::LongTerm);
        assert_eq!(sorted[5].priority, Priority::Low);
    }

    #[test]
    fn test_sorted_tasks_empty() {
        // Test sorting an empty context
        let context = Context::new("work".to_string());

        let sorted = context.sorted_tasks();
        assert_eq!(sorted.len(), 0);
    }

    #[test]
    fn test_sorted_tasks_single() {
        // Test sorting with a single task
        let mut context = Context::new("work".to_string());

        context.add_task(Task::new(
            "Only task".to_string(),
            TimeHorizon::MidTerm,
            Priority::Medium,
        ));

        let sorted = context.sorted_tasks();
        assert_eq!(sorted.len(), 1);
        assert_eq!(sorted[0].description, "Only task");
    }

    // ContextManager tests

    #[test]
    fn test_context_manager_new() {
        // Test creating a new ContextManager
        let manager = ContextManager::new();

        // Verify the default context exists
        assert_eq!(manager.contexts.len(), 1);
        assert!(manager.contexts.contains_key("default"));

        // Verify the default context is active
        assert_eq!(manager.active_context, "default");

        // Verify the default context has no tasks
        let default_context = manager.contexts.get("default").unwrap();
        assert_eq!(default_context.name, "default");
        assert_eq!(default_context.tasks.len(), 0);
    }

    #[test]
    fn test_context_manager_serialization() {
        // Test that ContextManager can be serialized to JSON
        let manager = ContextManager::new();

        let json = serde_json::to_string(&manager).unwrap();

        // Verify JSON contains expected fields
        assert!(json.contains("\"contexts\""));
        assert!(json.contains("\"active_context\""));
        assert!(json.contains("\"default\""));
    }

    #[test]
    fn test_context_manager_deserialization() {
        // Test that ContextManager can be deserialized from JSON
        let json = r#"{
            "contexts": {
                "default": {
                    "name": "default",
                    "tasks": []
                }
            },
            "active_context": "default"
        }"#;

        let manager: ContextManager = serde_json::from_str(json).unwrap();

        assert_eq!(manager.active_context, "default");
        assert_eq!(manager.contexts.len(), 1);
        assert!(manager.contexts.contains_key("default"));
    }

    #[test]
    fn test_context_manager_with_multiple_contexts() {
        // Test deserialization with multiple contexts
        let json = r#"{
            "contexts": {
                "default": {
                    "name": "default",
                    "tasks": []
                },
                "work": {
                    "name": "work",
                    "tasks": []
                },
                "personal": {
                    "name": "personal",
                    "tasks": []
                }
            },
            "active_context": "work"
        }"#;

        let manager: ContextManager = serde_json::from_str(json).unwrap();

        assert_eq!(manager.active_context, "work");
        assert_eq!(manager.contexts.len(), 3);
        assert!(manager.contexts.contains_key("default"));
        assert!(manager.contexts.contains_key("work"));
        assert!(manager.contexts.contains_key("personal"));
    }

    #[test]
    fn test_context_manager_round_trip() {
        // Test serialization and deserialization round-trip
        let manager1 = ContextManager::new();

        // Serialize to JSON
        let json = serde_json::to_string(&manager1).unwrap();

        // Deserialize back
        let manager2: ContextManager = serde_json::from_str(&json).unwrap();

        // Verify they match
        assert_eq!(manager1.active_context, manager2.active_context);
        assert_eq!(manager1.contexts.len(), manager2.contexts.len());

        // Verify the default context exists in both
        assert!(manager2.contexts.contains_key("default"));
    }

    // ContextManager operations tests

    #[test]
    fn test_create_context() {
        // Test creating a new context
        let mut manager = ContextManager::new();

        // Create a new context
        let result = manager.create_context("work".to_string());
        assert!(result.is_ok());

        // Verify the context was created
        assert_eq!(manager.contexts.len(), 2); // default + work
        assert!(manager.contexts.contains_key("work"));

        // Verify the context has the correct name and is empty
        let work_context = manager.contexts.get("work").unwrap();
        assert_eq!(work_context.name, "work");
        assert_eq!(work_context.tasks.len(), 0);
    }

    #[test]
    fn test_create_context_duplicate() {
        // Test creating a context with a duplicate name
        let mut manager = ContextManager::new();

        // Create a context
        manager.create_context("work".to_string()).unwrap();

        // Try to create a duplicate
        let result = manager.create_context("work".to_string());
        assert!(result.is_err());

        // Verify the error type
        match result {
            Err(crate::error::AppError::ContextAlreadyExists(name)) => {
                assert_eq!(name, "work");
            }
            _ => panic!("Expected ContextAlreadyExists error"),
        }

        // Verify only one "work" context exists
        assert_eq!(manager.contexts.len(), 2); // default + work
    }

    #[test]
    fn test_create_multiple_contexts() {
        // Test creating multiple contexts
        let mut manager = ContextManager::new();

        manager.create_context("work".to_string()).unwrap();
        manager.create_context("personal".to_string()).unwrap();
        manager.create_context("learning".to_string()).unwrap();

        assert_eq!(manager.contexts.len(), 4); // default + 3 new
        assert!(manager.contexts.contains_key("work"));
        assert!(manager.contexts.contains_key("personal"));
        assert!(manager.contexts.contains_key("learning"));
    }

    #[test]
    fn test_switch_context() {
        // Test switching to a different context
        let mut manager = ContextManager::new();
        manager.create_context("work".to_string()).unwrap();

        // Initially on default
        assert_eq!(manager.active_context, "default");

        // Switch to work
        let result = manager.switch_context("work");
        assert!(result.is_ok());
        assert_eq!(manager.active_context, "work");

        // Switch back to default
        let result = manager.switch_context("default");
        assert!(result.is_ok());
        assert_eq!(manager.active_context, "default");
    }

    #[test]
    fn test_switch_context_not_found() {
        // Test switching to a non-existent context
        let mut manager = ContextManager::new();

        let result = manager.switch_context("nonexistent");
        assert!(result.is_err());

        // Verify the error type
        match result {
            Err(crate::error::AppError::ContextNotFound(name)) => {
                assert_eq!(name, "nonexistent");
            }
            _ => panic!("Expected ContextNotFound error"),
        }

        // Verify the active context didn't change
        assert_eq!(manager.active_context, "default");
    }

    #[test]
    fn test_delete_context() {
        // Test deleting a context
        let mut manager = ContextManager::new();
        manager.create_context("work".to_string()).unwrap();
        manager.create_context("personal".to_string()).unwrap();

        // Delete the work context
        let result = manager.delete_context("work");
        assert!(result.is_ok());

        // Verify the context was deleted
        assert_eq!(manager.contexts.len(), 2); // default + personal
        assert!(!manager.contexts.contains_key("work"));
        assert!(manager.contexts.contains_key("default"));
        assert!(manager.contexts.contains_key("personal"));
    }

    #[test]
    fn test_delete_context_not_found() {
        // Test deleting a non-existent context
        let mut manager = ContextManager::new();

        let result = manager.delete_context("nonexistent");
        assert!(result.is_err());

        // Verify the error type
        match result {
            Err(crate::error::AppError::ContextNotFound(_)) => {
                // Expected
            }
            _ => panic!("Expected ContextNotFound error"),
        }
    }

    #[test]
    fn test_delete_last_context() {
        // Test that we can't delete the last context
        let mut manager = ContextManager::new();

        // Try to delete the only context
        let result = manager.delete_context("default");
        assert!(result.is_err());

        // Verify the error type
        match result {
            Err(crate::error::AppError::CannotDeleteLastContext) => {
                // Expected
            }
            _ => panic!("Expected CannotDeleteLastContext error"),
        }

        // Verify the context still exists
        assert_eq!(manager.contexts.len(), 1);
        assert!(manager.contexts.contains_key("default"));
    }

    #[test]
    fn test_delete_active_context() {
        // Test that we can't delete the active context
        let mut manager = ContextManager::new();
        manager.create_context("work".to_string()).unwrap();

        // Switch to work
        manager.switch_context("work").unwrap();

        // Try to delete the active context
        let result = manager.delete_context("work");
        assert!(result.is_err());

        // Verify the context still exists
        assert_eq!(manager.contexts.len(), 2);
        assert!(manager.contexts.contains_key("work"));
        assert_eq!(manager.active_context, "work");
    }

    #[test]
    fn test_active_context() {
        // Test getting the active context
        let manager = ContextManager::new();

        let context = manager.active_context();
        assert_eq!(context.name, "default");
        assert_eq!(context.tasks.len(), 0);
    }

    #[test]
    fn test_active_context_after_switch() {
        // Test that active_context returns the correct context after switching
        let mut manager = ContextManager::new();
        manager.create_context("work".to_string()).unwrap();

        // Add a task to work context
        let work_context = manager.contexts.get_mut("work").unwrap();
        work_context.add_task(Task::new(
            "Work task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High,
        ));

        // Switch to work
        manager.switch_context("work").unwrap();

        // Get the active context
        let context = manager.active_context();
        assert_eq!(context.name, "work");
        assert_eq!(context.tasks.len(), 1);
        assert_eq!(context.tasks[0].description, "Work task");
    }

    #[test]
    fn test_active_context_mut() {
        // Test getting a mutable reference to the active context
        let mut manager = ContextManager::new();

        // Add a task through the mutable reference
        let context = manager.active_context_mut();
        context.add_task(Task::new(
            "Test task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Medium,
        ));

        // Verify the task was added
        assert_eq!(manager.active_context().tasks.len(), 1);
        assert_eq!(manager.active_context().tasks[0].description, "Test task");
    }

    #[test]
    fn test_active_context_mut_modify_task() {
        // Test modifying a task through active_context_mut
        let mut manager = ContextManager::new();

        // Add a task
        let task = Task::new(
            "Test task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::Medium,
        );
        let task_id = task.id.clone();
        manager.active_context_mut().add_task(task);

        // Modify the task
        let context = manager.active_context_mut();
        if let Some(task) = context.find_task_mut(&task_id) {
            task.mark_complete();
        }

        // Verify the task was modified
        let found = manager.active_context().find_task(&task_id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().completed, true);
    }

    #[test]
    fn test_list_contexts() {
        // Test listing all context names
        let mut manager = ContextManager::new();

        // Initially only default
        let names = manager.list_contexts();
        assert_eq!(names.len(), 1);
        assert!(names.contains(&"default"));

        // Add more contexts
        manager.create_context("work".to_string()).unwrap();
        manager.create_context("personal".to_string()).unwrap();
        manager.create_context("learning".to_string()).unwrap();

        // List all contexts
        let names = manager.list_contexts();
        assert_eq!(names.len(), 4);
        assert!(names.contains(&"default"));
        assert!(names.contains(&"work"));
        assert!(names.contains(&"personal"));
        assert!(names.contains(&"learning"));
    }

    #[test]
    fn test_list_contexts_after_deletion() {
        // Test that list_contexts reflects deletions
        let mut manager = ContextManager::new();
        manager.create_context("work".to_string()).unwrap();
        manager.create_context("personal".to_string()).unwrap();

        // Delete one context
        manager.delete_context("work").unwrap();

        // List contexts
        let names = manager.list_contexts();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"default"));
        assert!(names.contains(&"personal"));
        assert!(!names.contains(&"work"));
    }

    #[test]
    fn test_context_workflow() {
        // Test a complete workflow: create, switch, add tasks, switch back
        let mut manager = ContextManager::new();

        // Create work context
        manager.create_context("work".to_string()).unwrap();

        // Add task to default context
        manager.active_context_mut().add_task(Task::new(
            "Default task".to_string(),
            TimeHorizon::ShortTerm,
            Priority::High,
        ));

        // Switch to work context
        manager.switch_context("work").unwrap();

        // Add task to work context
        manager.active_context_mut().add_task(Task::new(
            "Work task".to_string(),
            TimeHorizon::MidTerm,
            Priority::Medium,
        ));

        // Verify work context has 1 task
        assert_eq!(manager.active_context().tasks.len(), 1);
        assert_eq!(manager.active_context().tasks[0].description, "Work task");

        // Switch back to default
        manager.switch_context("default").unwrap();

        // Verify default context still has its task
        assert_eq!(manager.active_context().tasks.len(), 1);
        assert_eq!(
            manager.active_context().tasks[0].description,
            "Default task"
        );

        // Verify work context still has its task
        let work_context = manager.contexts.get("work").unwrap();
        assert_eq!(work_context.tasks.len(), 1);
        assert_eq!(work_context.tasks[0].description, "Work task");
    }
}
