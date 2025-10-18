// Code Examples: Demonstrating Patterns from Our Rust egui Script Manager
// This file shows practical examples of key concepts used in the project

use crossbeam::channel::{self, Receiver, Sender};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

// Example 1: Shared State with Arc<RwLock<T>> (like our folder state)
#[derive(Debug, Clone)]
struct Folder {
    id: i32,
    name: String,
}

struct AppState {
    folders: Arc<RwLock<Vec<Folder>>>,
    selected_folder: Arc<RwLock<Option<i32>>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            folders: Arc::new(RwLock::new(Vec::new())),
            selected_folder: Arc::new(RwLock::new(None)),
        }
    }

    // Reading shared state
    fn get_folders(&self) -> Vec<Folder> {
        self.folders.read().unwrap().clone()
    }

    // Writing to shared state
    fn add_folder(&self, folder: Folder) {
        self.folders.write().unwrap().push(folder);
    }

    // Updating selected folder
    fn select_folder(&self, id: i32) {
        *self.selected_folder.write().unwrap() = Some(id);
    }

    fn get_selected_folder(&self) -> Option<i32> {
        *self.selected_folder.read().unwrap()
    }
}

// Example 2: Async Database Operations (like our Prisma calls)
async fn load_folders_from_db() -> Result<Vec<Folder>, Box<dyn std::error::Error>> {
    // Simulate async DB call (in real code, this would be Prisma)
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    Ok(vec![
        Folder {
            id: 1,
            name: "Scripts".to_string(),
        },
        Folder {
            id: 2,
            name: "Tools".to_string(),
        },
    ])
}

async fn save_folder_to_db(folder: &Folder) -> Result<(), Box<dyn std::error::Error>> {
    // Simulate async DB save
    println!("Saving folder: {:?}", folder);
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    Ok(())
}

// Example 3: Event/Command System with Crossbeam Channels
#[derive(Debug)]
enum AppCommand {
    AddFolder(Folder),
    SelectFolder(i32),
}

#[derive(Debug)]
enum AppEvent {
    FolderAdded(Folder),
    FolderSelected(i32),
}

struct EventSystem {
    command_sender: Sender<AppCommand>,
    command_receiver: Receiver<AppCommand>,
    event_sender: Sender<AppEvent>,
    event_receiver: Receiver<AppEvent>,
}

impl EventSystem {
    fn new() -> Self {
        let (cmd_tx, cmd_rx) = channel::unbounded();
        let (evt_tx, evt_rx) = channel::unbounded();

        Self {
            command_sender: cmd_tx,
            command_receiver: cmd_rx,
            event_sender: evt_tx,
            event_receiver: evt_rx,
        }
    }

    fn dispatch_command(&self, command: AppCommand) {
        println!("Dispatching command: {:?}", command);
        let _ = self.command_sender.send(command);
    }

    fn dispatch_event(&self, event: AppEvent) {
        println!("Dispatching event: {:?}", event);
        let _ = self.event_sender.send(event);
    }
}

// Example 4: Tokio Tasks for Background Operations
async fn command_handler(
    mut command_rx: Receiver<AppCommand>,
    event_tx: Sender<AppEvent>,
    state: Arc<AppState>,
) {
    while let Ok(command) = command_rx.recv() {
        match command {
            AppCommand::AddFolder(folder) => {
                // Simulate async DB operation
                if let Err(e) = save_folder_to_db(&folder).await {
                    eprintln!("Failed to save folder: {:?}", e);
                    continue;
                }

                // Update shared state
                state.add_folder(folder.clone());

                // Dispatch event
                let _ = event_tx.send(AppEvent::FolderAdded(folder));
            }
            AppCommand::SelectFolder(id) => {
                state.select_folder(id);
                let _ = event_tx.send(AppEvent::FolderSelected(id));
            }
        }
    }
}

// Example 5: UI Update Handling (simplified egui pattern)
struct UiState {
    state: Arc<AppState>,
    event_system: Arc<EventSystem>,
}

impl UiState {
    fn new(state: Arc<AppState>, event_system: Arc<EventSystem>) -> Self {
        Self {
            state,
            event_system,
        }
    }

    // Simulate UI rendering and event handling
    fn render(&self) {
        // Read current state
        let folders = self.state.get_folders();
        let selected = self.state.get_selected_folder();

        println!(
            "Rendering {} folders, selected: {:?}",
            folders.len(),
            selected
        );

        // Simulate button clicks
        if folders.is_empty() {
            // Add a folder
            let new_folder = Folder {
                id: 1,
                name: "New Folder".to_string(),
            };
            self.event_system
                .dispatch_command(AppCommand::AddFolder(new_folder));
        } else if selected.is_none() {
            // Select first folder
            if let Some(folder) = folders.first() {
                self.event_system
                    .dispatch_command(AppCommand::SelectFolder(folder.id));
            }
        }
    }
}

// Example 6: Running Terminal Commands (like our script execution)
async fn run_terminal_command(command: String) -> Result<String, Box<dyn std::error::Error>> {
    let output = tokio::process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    println!("Executed: {}", command);
    if !stdout.is_empty() {
        println!("Output: {}", stdout);
    }
    if !stderr.is_empty() {
        eprintln!("Error: {}", stderr);
    }

    Ok(stdout)
}

// Example 7: Main Application Loop
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize shared state
    let app_state = Arc::new(AppState::new());

    // Initialize event system
    let event_system = Arc::new(EventSystem::new());

    // Start command handler in background
    let command_handler_state = Arc::clone(&app_state);
    let event_tx = event_system.event_sender.clone();
    let command_rx = event_system.command_receiver.clone();

    tokio::spawn(async move {
        command_handler(command_rx, event_tx, command_handler_state).await;
    });

    // Initialize UI state
    let ui_state = UiState::new(Arc::clone(&app_state), Arc::clone(&event_system));

    // Simulate application loop
    for _ in 0..5 {
        // Process events (in real app, this would be in UI event loop)
        while let Ok(event) = event_system.event_receiver.try_recv() {
            println!("Received event: {:?}", event);
        }

        // Render UI
        ui_state.render();

        // Simulate frame delay
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    // Example terminal command
    let _ = run_terminal_command("echo 'Hello from terminal!'".to_string()).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_state() {
        let state = AppState::new();

        // Test writing
        let folder = Folder {
            id: 1,
            name: "Test".to_string(),
        };
        state.add_folder(folder.clone());

        // Test reading
        let folders = state.get_folders();
        assert_eq!(folders.len(), 1);
        assert_eq!(folders[0].name, "Test");
    }

    #[test]
    fn test_event_system() {
        let event_system = EventSystem::new();

        // Test command dispatch
        let command = AppCommand::SelectFolder(42);
        event_system.dispatch_command(command);

        // Test event dispatch
        let event = AppEvent::FolderSelected(42);
        event_system.dispatch_event(event);

        // Verify messages were sent
        assert!(event_system.command_receiver.try_recv().is_ok());
        assert!(event_system.event_receiver.try_recv().is_ok());
    }
}
