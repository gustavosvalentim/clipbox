use std::sync::{Arc, Mutex};

use enigo::{Enigo, Mouse};
use tauri::{Manager, LogicalPosition, Position, WebviewUrl, WebviewWindow, WebviewWindowBuilder, Window, WindowEvent};
use tauri::window::{Effect, EffectState, EffectsBuilder};

pub struct Settings {
    pub width: f64,
    pub height: f64,
    pub transparent: bool,
    pub decorations: bool,
    pub radius: f64,
}

#[derive(Debug)]
pub enum WindowError {
    TauriError(tauri::Error),
    EnigoError,
}

impl std::fmt::Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowError::TauriError(e) => write!(f, "Tauri error: {e}"),
            WindowError::EnigoError => write!(f, "Failed to get cursor position"),
        }
    }
}

pub trait ClipboxAppHandle {
    fn new_window(&self, settings: Settings) -> Result<WebviewWindow, WindowError>;
}

struct ClipboxApp {
    app: tauri::AppHandle,
}

impl ClipboxAppHandle for ClipboxApp {
    fn new_window(&self, settings: Settings) -> Result<WebviewWindow, WindowError> {
        let window = WebviewWindowBuilder::new(&self.app, "main", WebviewUrl::default())
            .inner_size(settings.width, settings.height)
            .decorations(settings.decorations)
            .transparent(settings.transparent)
            .always_on_top(true)
            .visible(false)
            .visible_on_all_workspaces(true)
            .shadow(true)
            .effects(
                EffectsBuilder::new()
                    .effect(Effect::Menu)
                    .state(EffectState::Active)
                    .radius(settings.radius)
                    .build(),
            )
            .build();

        let window = match window {
            Ok(window) => window,
            Err(e) => return Err(WindowError::TauriError(e)),
        };

        Ok(window)
    }
}

pub trait ClipboxWindowHandle {
    fn show_on_cursor(&self, app: &tauri::AppHandle) -> Result<(), WindowError>;
}

pub struct ClipboxWindow {
    window: WebviewWindow,
}

impl ClipboxWindowHandle for ClipboxWindow {
    fn show_on_cursor(&self, app: &tauri::AppHandle) -> Result<(), WindowError> {
        let enigo = app.state::<Arc<Mutex<Enigo>>>();
        let enigo = match enigo.lock() {
            Ok(enigo) => enigo,
            Err(_) => return Err(WindowError::EnigoError),
        };

        let (mouse_x, mouse_y) = match enigo.location() {
            Ok(location) => location,
            Err(_) => return Err(WindowError::EnigoError),
        };

        // Physical position causes the position to be off on HiDPI screens
        // TODO: clamp the position to the screen size
        let new_pos = LogicalPosition {
            x: f64::from(mouse_x),
            y: f64::from(mouse_y),
        };

        match self.window.set_position(Position::Logical(new_pos)) {
            Ok(_) => {}
            Err(e) => return Err(WindowError::TauriError(e)),
        }

        let window = self.window.clone();
        // this is a hack to make the window appear on the right
        // position without flickering.
        // Because tauri window methods are async, show() may run before
        // set_position() finishes, causing the window to briefly appear
        // on the old position before moving to the new one.
        // Since we don't want to block the main thread, we spawn another
        // one to show the window.
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(25));

            match window.show() {
                Ok(_) => {}
                Err(e) => println!("Failed to show window: {e}"),
            }

            match window.set_focus() {
                Ok(_) => {}
                Err(e) => println!("Failed to focus window: {e}"),
            }
        });

        Ok(())
    }
}

pub trait ClipboxAppExt {
    fn clipbox(&mut self) -> impl ClipboxAppHandle;
}

impl ClipboxAppExt for tauri::AppHandle {
    fn clipbox(&mut self) -> impl ClipboxAppHandle {
        ClipboxApp {
            app: self.clone(),
        }
    }
}

pub trait ClipboxWindowExt {
    fn clipbox(&self) -> impl ClipboxWindowHandle;
}

impl ClipboxWindowExt for WebviewWindow {
    fn clipbox(&self) -> impl ClipboxWindowHandle {
        ClipboxWindow{
            window: self.clone(),
        }
    }
}

pub fn window_events_handler(window: &Window, event: &WindowEvent) {
    if let WindowEvent::Focused(focused) = event {
        if !focused {
            let _ = window.hide();
        }
    }
}
