use md5::{Digest, Md5};
use serde::Serialize;
use std::io;
use std::sync::{Arc, Mutex, PoisonError};
use std::vec::Vec;

use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use tauri::Emitter;
use tauri_plugin_clipboard_manager::ClipboardExt;

const MAX_ITEMS: usize = 120;

#[derive(Debug)]
pub enum ClipboardError {
    PoisonError,
    ItemNotFound,
}

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClipboardError::PoisonError => write!(f, "Clipboard poisoned"),
            ClipboardError::ItemNotFound => write!(f, "Item not found"),
        }
    }
}

pub trait ClipboardManager {
    fn new_manager() -> Self;
    fn add_text(&self, text: String) -> Option<ClipboardItem>;
    fn clear(&self) -> Result<(), ClipboardError>;
    fn first(&self) -> Option<ClipboardItem>;
    fn list(&self) -> Result<Vec<ClipboardItem>, ClipboardError>;
    fn delete(&self, text: &str) -> Result<usize, ClipboardError>;
    fn exists(&self, hash: &str) -> bool;
    fn move_to_top(&self, hash: &str) -> Result<(), ClipboardError>;
}

#[derive(Debug, Clone, Serialize)]
pub struct InMemoryClipboardHistory {
    items: Arc<Mutex<Vec<ClipboardItem>>>,
}

impl InMemoryClipboardHistory {
    fn add_item(&self, item: ClipboardItem) {
        let mut history = self.items.lock().expect("Failed to lock clipboard history");

        if history.len() + 1 > MAX_ITEMS {
            history.pop();
        }

        history.insert(0, item);
    }

    fn hash(&self, text: &str) -> String {
        let text_digest = Md5::digest(text.as_bytes());
        format!("{:?}", text_digest)
    }
}

impl ClipboardManager for InMemoryClipboardHistory {
    fn new_manager() -> Self {
        Self {
            items: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn add_text(&self, text: String) -> Option<ClipboardItem> {
        if text.is_empty() {
            return None;
        }

        let hash = self.hash(&text);
        let item = ClipboardItem { text, hash };

        self.add_item(item);

        self.first()
    }

    fn clear(&self) -> Result<(), ClipboardError> {
        match self.items.lock() {
            Ok(mut history_lock) => {
                history_lock.clear();
                Ok(())
            }
            Err(PoisonError { .. }) => Err(ClipboardError::PoisonError),
        }
    }

    fn first(&self) -> Option<ClipboardItem> {
        if let Ok(items) = self.items.lock() {
            if items.is_empty() {
                return None;
            }

            Some(items[0].clone())
        } else {
            None
        }
    }

    fn list(&self) -> Result<Vec<ClipboardItem>, ClipboardError> {
        match self.items.lock() {
            Ok(history_lock) => Ok(history_lock.clone()),
            Err(PoisonError { .. }) => Err(ClipboardError::PoisonError),
        }
    }

    fn exists(&self, text: &str) -> bool {
        let hash = self.hash(text);
        match self.items.lock() {
            Ok(history_lock) => history_lock.iter().any(|item| item.hash == hash),
            Err(PoisonError { .. }) => false,
        }
    }

    fn delete(&self, text: &str) -> Result<usize, ClipboardError> {
        let hash = self.hash(text);
        let mut history = match self.items.lock() {
            Ok(history) => history,
            Err(_) => return Err(ClipboardError::PoisonError),
        };

        let Some(idx) = history.iter().position(|item| item.hash == hash) else {
            return Err(ClipboardError::ItemNotFound);
        };

        history.remove(idx);

        Ok(idx)
    }

    fn move_to_top(&self, text: &str) -> Result<(), ClipboardError> {
        let hash = self.hash(text);
        match self.items.lock() {
            Ok(mut history) => {
                let item_idx = history
                    .iter()
                    .position(|item| item.hash == hash)
                    .ok_or(ClipboardError::ItemNotFound)?;

                let item = history.remove(item_idx);
                history.insert(0, item);

                Ok(())
            }
            Err(PoisonError { .. }) => Err(ClipboardError::PoisonError),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ClipboardItem {
    pub text: String,
    pub hash: String,
}

pub struct ClipboardEventsListener<T>
where
    T: ClipboardManager,
{
    history: T,
    handler: Arc<tauri::AppHandle>,
}

impl<T: ClipboardManager> ClipboardEventsListener<T> {
    pub fn new(app_handler: tauri::AppHandle, history: T) -> ClipboardEventsListener<T> {
        Self {
            history,
            handler: Arc::new(app_handler),
        }
    }
}

impl<T: ClipboardManager> ClipboardHandler for ClipboardEventsListener<T> {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        println!("Clipboard changed");

        let clipbox_pid = std::process::id();

        #[cfg(target_os = "macos")]
        {
            use crate::window::macos::active_window_pid;

            if active_window_pid() as u32 == clipbox_pid {
                return CallbackResult::Next;
            }
        }

        let Ok(text) = self.handler.clipboard().read_text() else {
            // TODO: add image support
            // this is probably an image and we should get it using
            // `AppHandle.clipboard().read_image()`.
            // Need to figure out how to handle this in the UI and backend
            return CallbackResult::Next;
        };

        if self.history.exists(&text) {
            if let Err(e) = self.history.move_to_top(&text) {
                println!("Failed to move item to top: {e}");
                return CallbackResult::Next;
            }

            return CallbackResult::Next;
        }

        if self.history.add_text(text).is_some() {
            let _ = self.handler.emit_clipboard_changed();
        }

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        println!("Clipboard error: {error}");
        CallbackResult::Next
    }
}

pub fn clipboard_events_listener<T: ClipboardManager>(
    app_handler: tauri::AppHandle,
    history: T,
) -> Master<ClipboardEventsListener<T>> {
    Master::new(ClipboardEventsListener::new(app_handler, history))
        .expect("Failed to create clipboard listener")
}

const CLIPBOARD_CHANGED_EVENT: &str = "clipboard-changed";

pub trait ClipboardEventsEmitter {
    fn emit_clipboard_changed(&self) -> Result<(), tauri::Error>;
}

impl ClipboardEventsEmitter for tauri::AppHandle {
    fn emit_clipboard_changed(&self) -> Result<(), tauri::Error> {
        self.emit(CLIPBOARD_CHANGED_EVENT, "")
    }
}
