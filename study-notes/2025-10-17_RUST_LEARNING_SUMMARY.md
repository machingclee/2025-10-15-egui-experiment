# Rust Learning Journey - 17 October 2025

## Overview
This document summarizes key Rust concepts and patterns learned through building an egui application with database integration, async operations, and event-driven architecture.

## 1. Ownership & Borrowing Fundamentals

### Move vs Borrow in Closures
- **Borrow**: `|state| { ui.label("text") }` - `ui` is borrowed, remains accessible
- **Move**: `move |state| { owned_var }` - variable ownership transferred to closure
- **FnOnce**: Most flexible closure trait, allows any capture method

### When Variables Are Moved
- When using `move` keyword explicitly
- When closure needs ownership (e.g., async blocks)
- When variable is owned data (not a reference)

## 2. Global State Management

### Static Variables with LazyLock
```rust
pub static FOLDER_STATE: LazyLock<Mutex<FoldersState>> =
    LazyLock::new(|| Mutex::new(FoldersState::default()));
```
- **Static lifetime**: Lives for entire program duration
- **Lazy initialization**: Created on first access
- **Thread-safe**: Mutex provides interior mutability

### Helper Functions for State Access
```rust
pub fn with_folder_state<F, R>(f: F) -> R
where
    F: FnOnce(&FoldersState) -> R,
{
    let state = FOLDER_STATE.lock();
    f(&state)
}
```
- **Closure-based access**: Avoids lifetime issues
- **Automatic locking/unlocking**: Mutex handled transparently
- **Type-safe**: Compile-time guarantees

## 3. Component-Based UI Architecture

### Struct-Based Components
```rust
#[derive(serde::Deserialize, serde::Serialize)]
pub struct FolderPanel {
    // Component state here
}

impl FolderPanel {
    pub fn show(&mut self, ctx: &egui::Context) {
        // UI rendering logic
    }
}
```
- **Serializable**: Supports app persistence
- **Stateful**: Can maintain component-specific state
- **Reusable**: Clean separation of concerns

### Direct Global State Access (Optimized)
- **Zero-copy**: No cloning of data
- **Reference-based**: Direct access to global state
- **Performance**: Minimal overhead

## 4. Event-Driven Architecture

### Command-Event Pattern
```rust
// Commands (user intentions)
enum AppCommand {
    Folder(FolderCommand),
}

// Events (state changes)
enum AppEvent {
    Folder(FolderEvent),
}
```

### Static vs Instance Handlers
- **Static handlers**: Stateless, pure functions (commands)
- **Instance handlers**: May maintain state (events)
- **Global dispatch**: Centralized event processing

## 5. Async Programming with Tokio

### Runtime Integration
```rust
pub static RT_HANDLE: OnceLock<tokio::runtime::Handle> = OnceLock::new();

pub fn spawn_task<F>(future: F)
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    RT_HANDLE.get().unwrap().spawn(future);
}
```
- **Global runtime**: Single Tokio runtime for entire app
- **Task spawning**: Fire-and-forget async operations
- **GUI compatibility**: Non-blocking UI thread

## 6. Database Integration (Prisma)

### Async Database Operations
```rust
crate::spawn_task(async move {
    let db = get_db();
    let folders = db.scripts_folder().find_many(vec![]).exec().await;
    // Handle results...
});
```
- **Non-blocking**: Database operations don't freeze UI
- **Error handling**: Proper async error propagation
- **State updates**: Results update global state via events

## 7. Performance Optimizations

### Avoiding Unnecessary Cloning
- **Reference access**: Use `&data` instead of `data.clone()`
- **Lazy evaluation**: Only compute when needed
- **Smart caching**: Change detection before updates

### Memory Management
- **Static allocation**: Global state lives forever
- **Reference counting**: Arc/Rc where needed
- **Borrow checker**: Compile-time memory safety

## 8. Code Organization & Namespaces

### Module System
```
lib.rs              // Root module, global functions
├── app.rs          // Main application logic
├── component/      // UI components
│   └── folder_col.rs
├── domain/         // Business logic
│   └── folder/
├── state/          // Global state
└── db/             // Database layer
```

### Visibility & Access
- **Crate-level**: `pub` functions in lib.rs available as `crate::function`
- **Module-level**: `pub` items accessible within module tree
- **Static access**: Global state via `crate::STATIC_VAR`

## 9. Architecture Patterns Learned

### Domain-Driven Design Elements
- **Commands**: User intentions (CreateFolder)
- **Events**: State changes (FolderAdded)
- **Handlers**: Business logic processors
- **State**: Global application state

### Component Architecture
- **Stateless components**: Pure functions
- **Stateful components**: Structs with local state
- **Global state**: Shared across components
- **Event communication**: Loose coupling

### Async Patterns
- **Spawn tasks**: Background operations
- **Event loops**: Continuous processing
- **State synchronization**: Global state updates
- **Error handling**: Async result propagation

## 10. Key Takeaways

1. **Ownership matters**: Understanding move vs borrow is crucial for closures and async code
2. **Global state**: Static variables with Mutex provide thread-safe shared state
3. **Performance**: References over clones, lazy evaluation, smart caching
4. **Architecture**: Event-driven, component-based, domain-driven patterns work well together
5. **Async integration**: Careful runtime management prevents GUI blocking
6. **Code organization**: Clear module boundaries and namespaces improve maintainability

## 11. Common Patterns Established

- **Helper functions**: `with_folder_state()`, `spawn_task()`, `send_event()`
- **Component structure**: Serializable structs with `show()` methods
- **Event handling**: Static command handlers, instance event handlers
- **State access**: Closure-based global state access
- **Async operations**: Spawn tasks for non-blocking operations

This journey covered fundamental Rust concepts (ownership, borrowing, lifetimes) through practical application development, demonstrating how they enable scalable, performant GUI applications with complex state management and async operations.</content>
<parameter name="filePath">/Users/chingcheonglee/Repos/rust/2025-10-15-egui-experiment/RUST_LEARNING_SUMMARY_2025-10-17.md