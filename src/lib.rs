#![warn(clippy::all, rust_2018_idioms)]

use std::sync::OnceLock;

pub static RT_HANDLE: OnceLock<tokio::runtime::Handle> = OnceLock::new();

pub fn spawn_task<F>(future: F)
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    RT_HANDLE.get().unwrap().spawn(future);
}

pub fn run_terminal_command(command: String) {
    spawn_task(async move {
        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .await;
        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("Command executed: {}", command);
                if !stdout.is_empty() {
                    println!("Output: {}", stdout);
                }
                if !stderr.is_empty() {
                    eprintln!("Error: {}", stderr);
                }
            }
            Err(e) => eprintln!("Failed to execute command: {:?}", e),
        }
    });
}

pub fn send_event(message: AppMessage) {
    let _ = EVENT_SENDER.get().unwrap().send(message);
}

pub fn dispatch_folder_event(event: FolderEvent) {
    println!("Dispatching folder event: {:?}", event);
    send_event(AppMessage::Event(AppEvent::Folder(event)));
}

pub fn dispatch_folder_command(command: FolderCommand) {
    println!("Dispatching folder command: {:?}", command);
    let wrapped = WrappedFolderCommand {
        command,
        callback: None,
    };
    send_event(AppMessage::Command(AppCommand::Folder(wrapped)));
}

pub fn dispatch_folder_command_with_callback<F>(command: FolderCommand, callback: Option<F>)
where
    F: Fn() + Send + 'static,
{
    let cb_box = callback.map(|f| Box::new(f) as Box<dyn Fn() + Send + 'static>);
    let wrapped = WrappedFolderCommand {
        command,
        callback: cb_box,
    };
    send_event(AppMessage::Command(AppCommand::Folder(wrapped)));
}

pub fn with_folder_state<F, R>(f: F) -> R
where
    F: FnOnce(&crate::state::folder_state::FoldersState) -> R,
{
    f(&crate::state::folder_state::FOLDER_STATE)
}

pub fn with_folder_state_reducer<F, R>(f: F) -> R
where
    F: FnOnce(&crate::state::folder_state::FolderReducer<'static>) -> R,
{
    // FOLDER_STATE is a 'static LazyLock, so we can create a FolderReducer<'static> safely
    let reducer = crate::state::folder_state::FolderReducer {
        state: &crate::state::folder_state::FOLDER_STATE,
    };
    f(&reducer)
}
pub mod app;
pub mod component;
pub mod db;
pub mod domain;
pub mod ext;
pub mod prisma;
pub mod state;
pub static PRISMA_CLIENT: OnceLock<prisma::PrismaClient> = OnceLock::new();
pub use app::App;

// Event system
use crossbeam::channel::{Receiver, Sender};

use crate::domain::folder::{
    folder_command_handler::FolderCommand, folder_event_handler::FolderEvent,
};

// Wrapper for commands that can optionally carry a callback to be executed after handling.
// The callback is optional and boxed; we use `Fn()` for object safety and simplicity.
pub struct WrappedFolderCommand {
    pub command: FolderCommand,
    pub callback: Option<Box<dyn Fn() + Send + 'static>>,
}

impl std::fmt::Debug for WrappedFolderCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WrappedFolderCommand")
            .field("command", &self.command)
            .field("has_callback", &self.callback.is_some())
            .finish()
    }
}

// Domain-specific message types
#[derive(Debug)]
pub enum AppCommand {
    Folder(WrappedFolderCommand),
}

#[derive(Debug)]
pub enum AppEvent {
    Folder(FolderEvent),
}

pub enum AppMessage {
    Command(AppCommand),
    Event(AppEvent),
    Callback(Box<dyn Fn() + Send + 'static>),
}

impl std::fmt::Debug for AppMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppMessage::Command(cmd) => f.debug_tuple("Command").field(cmd).finish(),
            AppMessage::Event(evt) => f.debug_tuple("Event").field(evt).finish(),
            AppMessage::Callback(_) => f.debug_tuple("Callback").field(&"<callback>").finish(),
        }
    }
}

pub static EVENT_SENDER: OnceLock<Sender<AppMessage>> = OnceLock::new();
pub static EVENT_RECEIVER: OnceLock<Receiver<AppMessage>> = OnceLock::new();
