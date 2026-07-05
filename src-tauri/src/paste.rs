use tauri::Manager;
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::clipboard::{InMemoryClipboardHistory, ClipboardManager};
use crate::window::get_main_window;

#[derive(Debug)]
pub enum PasteError {
    ClipboardError,
    ItemNotFound,
    WindowError,
}

impl std::fmt::Display for PasteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PasteError::ClipboardError => write!(f, "Clipboard error"),
            PasteError::ItemNotFound => write!(f, "Item not found"),
            PasteError::WindowError => write!(f, "Window error"),
        }
    }
}

pub fn paste(app: tauri::AppHandle, text: String) -> Result<(), PasteError> {
    let history = app.state::<InMemoryClipboardHistory>();

    if !history.exists(&text) {
        return Err(PasteError::ItemNotFound);
    }

    if app.clipboard().write_text(text).is_err() {
        return Err(PasteError::ClipboardError);
    }

    if let Some(window) = get_main_window(&app) {
        if window.hide().is_err() {
            return Err(PasteError::WindowError);
        }
    }

    Ok(())
}
