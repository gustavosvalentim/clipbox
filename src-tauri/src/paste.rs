use std::sync::Mutex;

use enigo::{Direction, Enigo, Key, Keyboard};
use tauri::Manager;
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::clipboard::{ClipboardItem, ClipboardManager, InMemoryClipboardHistory};
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

pub fn paste(app: &tauri::AppHandle, text: &str) -> Result<ClipboardItem, PasteError> {
    let history = app.state::<InMemoryClipboardHistory>();
    let paste_target = app.state::<PasteState>();
    let enigo = app.state::<Mutex<Enigo>>();

    if !history.exists(text) {
        return Err(PasteError::ItemNotFound);
    }

    if app.clipboard().write_text(text).is_err() {
        return Err(PasteError::ClipboardError);
    }

    let _ = history.move_to_top(text);

    if let Some(window) = get_main_window(app) {
        if window.hide().is_err() {
            return Err(PasteError::WindowError);
        }
    }

    paste_target.activate_last_focused_window();

    if let Ok(mut enigo) = enigo.lock() {
        simulate_paste_inputs(&mut enigo);
    }

    Ok(history.first().unwrap())
}

fn simulate_paste_inputs(enigo: &mut Enigo) {
    #[cfg(target_os = "macos")]
    let mod_key = Key::Meta;

    #[cfg(not(target_os = "macos"))]
    let mod_key = Key::Control;

    let _ = enigo.key(mod_key, Direction::Press);
    let _ = enigo.key(Key::Unicode('v'), Direction::Press);
    let _ = enigo.key(mod_key, Direction::Release);
}

pub struct AppInfo {
    pub pid: i32,
}

pub struct PasteState {
    target: Mutex<AppInfo>,
}

impl PasteState {
    pub fn new() -> Self {
        Self {
            target: Mutex::new(AppInfo { pid: 0 }),
        }
    }

    pub fn load_focused_window(&self) {
        #[cfg(target_os = "macos")]
        {
            use crate::window::macos::active_window_pid;

            if let Ok(mut target) = self.target.lock() {
                target.pid = active_window_pid();
            }
        }
    }

    pub fn activate_last_focused_window(&self) {
        #[cfg(target_os = "macos")]
        {
            use crate::window::macos::set_focused_window;

            if let Ok(target) = self.target.lock() {
                set_focused_window(target.pid);
            }
        }
    }
}

