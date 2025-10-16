#![warn(clippy::all, rust_2018_idioms)]

use std::sync::OnceLock;

pub static RT_HANDLE: OnceLock<tokio::runtime::Handle> = OnceLock::new();

pub fn spawn_task<F>(future: F)
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    RT_HANDLE.get().unwrap().spawn(future);
}

pub fn send_event(message: AppMessage) {
    let _ = EVENT_SENDER.get().unwrap().send(message);
}

pub fn send_folder_event(event: FolderEvent) {
    send_event(AppMessage::Event(AppEvent::Folder(event)));
}

pub fn with_folder_state<F, R>(f: F) -> R
where
    F: FnOnce(&crate::state::folder_state::FoldersState) -> R,
{
    let state = FOLDER_STATE.lock();
    f(&state)
}

pub fn with_folder_state_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut crate::state::folder_state::FoldersState) -> R,
{
    let mut state = FOLDER_STATE.lock();
    f(&mut state)
}

pub mod app;
pub mod component;
pub mod db;
pub mod domain;
pub mod ext;
pub mod prisma;
pub mod state;
pub static PRISMA_CLIENT: OnceLock<prisma::PrismaClient> = OnceLock::new();

// Re-export commonly used items
pub use state::folder_state::FOLDER_STATE;

pub use app::App;

// Event system
use crossbeam::channel::{Receiver, Sender};

use crate::domain::folder::{
    folder_command_handler::FolderCommand, folder_event_handler::FolderEvent,
};

// Domain-specific message types
#[derive(Debug)]
pub enum AppCommand {
    Folder(FolderCommand),
}

#[derive(Debug)]
pub enum AppEvent {
    Folder(FolderEvent),
}

#[derive(Debug)]
pub enum AppMessage {
    Command(AppCommand),
    Event(AppEvent),
}

pub static EVENT_SENDER: OnceLock<Sender<AppMessage>> = OnceLock::new();
pub static EVENT_RECEIVER: OnceLock<Receiver<AppMessage>> = OnceLock::new();
