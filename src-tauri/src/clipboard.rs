use md5::{Digest, Md5};
use serde::Serialize;
use std::io;
use std::sync::{Arc, Mutex, PoisonError};
use std::vec::Vec;

use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use tauri::Emitter;
use tauri_plugin_clipboard_manager::ClipboardExt;

const MAX_ITEMS: usize = 120;

pub type ClipboardHistory = Arc<Mutex<Vec<ClipboardItem>>>;

#[derive(Debug)]
pub enum ClipboardError {
    PoisonError,
}

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClipboardError::PoisonError => write!(f, "Clipboard poisoned"),
        }
    }
}

pub trait ClipboardManager {
    fn new_manager() -> Self;
    fn add_text(&self, text: String);
    fn clear(&self) -> Result<(), ClipboardError>;
    fn list(&self) -> Result<Vec<ClipboardItem>, ClipboardError>;
}

impl ClipboardManager for ClipboardHistory {
    fn new_manager() -> Self {
        Arc::new(Mutex::new(Vec::new()))
    }

    fn add_text(&self, text: String) {
        if text.is_empty() {
            return;
        }

        let new_item = ClipboardItem::new(text);
        let mut existing_item_idx: Option<usize> = None;
        let mut history_lock = self.lock().expect("Failed to lock clipboard history");

        for (idx, item) in history_lock.iter().enumerate() {
            if item.hash == new_item.hash {
                existing_item_idx = Some(idx);
                break;
            }
        }

        let history_len = match existing_item_idx {
            Some(_) => history_lock.len(),
            None => history_lock.len() + 1,
        };

        if history_len > MAX_ITEMS {
            history_lock.pop();
        }

        if let Some(existing_item_idx) = existing_item_idx {
            history_lock.remove(existing_item_idx);
        }

        history_lock.insert(0, new_item);
    }

    fn clear(&self) -> Result<(), ClipboardError> {
        match self.lock() {
            Ok(mut history_lock) => {
                history_lock.clear();
                Ok(())
            }
            Err(PoisonError { .. }) => Err(ClipboardError::PoisonError),
        }
    }

    fn list(&self) -> Result<Vec<ClipboardItem>, ClipboardError> {
        match self.lock() {
            Ok(history_lock) => Ok(history_lock.clone()),
            Err(PoisonError { .. }) => Err(ClipboardError::PoisonError),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ClipboardItem {
    text: String,
    hash: String,
}

impl ClipboardItem {
    fn new(text: String) -> Self {
        let text_digest = Md5::digest(text.as_bytes());
        Self {
            text,
            hash: format!("{:?}", text_digest),
        }
    }
}

pub struct ClipboardEventsListener {
    history: ClipboardHistory,
    handler: Arc<tauri::AppHandle>,
}

impl ClipboardEventsListener {
    pub fn new(
        app_handler: tauri::AppHandle,
        history: ClipboardHistory,
    ) -> ClipboardEventsListener {
        Self {
            history,
            handler: Arc::new(app_handler),
        }
    }
}

impl ClipboardHandler for ClipboardEventsListener {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let text = self.handler.clipboard().read_text();

        // TODO: this is probably an image and we should get it using
        // `AppHandle.clipboard().read_image()`.
        // Need to figure out how to handle this in the UI and backend
        if text.is_err() {
            return CallbackResult::Next;
        }

        // I know this sucks, but it's just until I add image support
        let text = text.unwrap();

        println!("Clipboard changed: {text}");

        self.history.add_text(text.clone());
        self.handler.emit("clipboard-changed", text).unwrap();

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        println!("Clipboard error: {error}");
        CallbackResult::Next
    }
}

pub fn change_listener(
    app_handler: tauri::AppHandle,
    history: ClipboardHistory,
) -> Master<ClipboardEventsListener> {
    Master::new(ClipboardEventsListener::new(app_handler, history))
        .expect("Failed to create clipboard listener")
}
