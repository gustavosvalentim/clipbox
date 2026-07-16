use std::vec::Vec;

use tauri::Manager;
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::clipboard::{ClipboardEventsEmitter, ClipboardItem, ClipboardStore};
use crate::paste::{paste_from_selection, PasteState};
use crate::window::get_main_window;

#[tauri::command]
pub fn fetch_clipboard(history: tauri::State<'_, ClipboardStore>) -> Vec<ClipboardItem> {
    println!("Fetch clipboard");
    history.list().unwrap_or_default()
}

#[tauri::command]
pub fn clear(app: tauri::AppHandle, history: tauri::State<'_, ClipboardStore>) {
    if let Err(e) = history.clear() {
        println!("Failed to clear clipboard history: {e}");
    }

    if let Err(e) = app.emit_clipboard_changed() {
        println!("Failed to emit clipboard changed event: {e}");
    }
}

#[tauri::command]
pub fn paste(app: tauri::AppHandle, text: &str) {
    if let Err(e) = paste_from_selection(&app, text) {
        println!("Failed to paste from selection: {e}");
    }
}

#[tauri::command]
pub fn quit(app: tauri::AppHandle) {
    let Some(window) = get_main_window(&app) else {
        println!("Failed to get main window");
        return;
    };

    let _ = window.close();
}

#[tauri::command]
pub fn close(app: tauri::AppHandle) {
    let Some(window) = get_main_window(&app) else {
        println!("Failed to get main window");
        return;
    };

    let _ = window.hide();

    let paste_target = app.state::<PasteState>();
    let _ = paste_target.restore_focus();
}

#[tauri::command]
pub fn delete_item(app: tauri::AppHandle, history: tauri::State<'_, ClipboardStore>, text: String) {
    if text.is_empty() {
        return;
    }

    let Ok(item_idx) = history.delete(&text) else {
        println!("Failed to delete item from clipboard history");
        return;
    };

    if let Err(e) = app.emit_clipboard_changed() {
        println!("Failed to emit clipboard changed event: {e}");
    }

    if item_idx == 0 {
        let Some(item) = history.first() else {
            return;
        };

        if let Err(e) = app.clipboard().write_text(item.text) {
            println!("Failed to write text to clipboard: {e}");
        }
    }
}
