use std::vec::Vec;

use crate::clipboard::{ClipboardHistory, ClipboardManager, ClipboardItem};
use crate::paste::PasteService;

#[tauri::command]
pub fn list_clipboard_items(history: tauri::State<'_, ClipboardHistory>) -> Vec<ClipboardItem> {
    match history.list() {
        Ok(items) => items,
        Err(e) => {
            println!("Failed to list clipboard history: {e}");
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn clear_clipboard_items(history: tauri::State<'_, ClipboardHistory>) {
    match history.clear() {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to clear clipboard history: {e}");
        }
    }
}

#[tauri::command]
pub fn paste_from_selection(app: tauri::AppHandle, text: String) {
    PasteService::new(app).paste_from_selection(text);
}
