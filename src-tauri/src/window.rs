use tauri::{
    window::{Effect, EffectState, EffectsBuilder},
    Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, Window, WindowEvent,
};

const MAIN_WINDOW_LABEL: &str = "main";

pub struct Settings {
    pub width: f64,
    pub height: f64,
    pub transparent: bool,
    pub decorations: bool,
}

#[derive(Debug)]
pub enum WindowError {
    TauriError(tauri::Error),
}

impl std::fmt::Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowError::TauriError(e) => write!(f, "Tauri error: {e}"),
        }
    }
}

pub fn create_klipo_window(
    app: &tauri::AppHandle,
    settings: Settings,
) -> Result<WebviewWindow, WindowError> {
    let window = WebviewWindowBuilder::new(app, MAIN_WINDOW_LABEL, WebviewUrl::default())
        .inner_size(settings.width, settings.height)
        .decorations(settings.decorations)
        .transparent(settings.transparent)
        .always_on_top(true)
        .visible(false)
        .visible_on_all_workspaces(true)
        .shadow(false)
        // `Menu` matches a macOS popup menu more closely than `Popover`.
        // Its radius clips the native backdrop too, rather than leaving a
        // square vibrancy layer behind the rounded web content.
        .effects(
            EffectsBuilder::new()
                .effect(Effect::Menu)
                .state(EffectState::Active)
                .radius(11.0)
                .build(),
        )
        .build();

    let window = match window {
        Ok(window) => window,
        Err(e) => return Err(WindowError::TauriError(e)),
    };

    Ok(window)
}

pub fn get_main_window(app: &tauri::AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window(MAIN_WINDOW_LABEL)
}

pub fn window_events_handler(window: &Window, event: &WindowEvent) {
    if let WindowEvent::Focused(focused) = event {
        if !focused {
            let _ = window.hide();
        }
    }
}

pub fn get_focused_window() -> Option<i32> {
    #[cfg(target_os = "macos")]
    {
        use crate::window::macos::get_focused_window;

        get_focused_window()
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("Not implemented");
        None
    }
}

pub fn set_focused_window(pid: i32) -> bool {
    #[cfg(target_os = "macos")]
    {
        use crate::window::macos::set_focused_window;

        set_focused_window(pid)
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("Not implemented");
        false
    }
}

#[cfg(target_os = "macos")]
pub mod macos {
    use objc2_app_kit::{NSApplicationActivationOptions, NSRunningApplication, NSWorkspace};

    pub fn set_focused_window(pid: i32) -> bool {
        let Some(app) = NSRunningApplication::runningApplicationWithProcessIdentifier(pid) else {
            return false;
        };

        app.activateWithOptions(NSApplicationActivationOptions::empty())
    }

    pub fn get_focused_window() -> Option<i32> {
        let workspace = NSWorkspace::sharedWorkspace();
        let app = workspace.frontmostApplication();

        Some(app?.processIdentifier())
    }
}

#[cfg(target_os = "linux")]
pub mod linux {
    pub fn set_focused_window(_pid: i32) -> bool {
        false
    }

    pub fn get_focused_window() -> Option<i32> {
        None
    }
}

#[cfg(target_os = "windows")]
pub mod windows {
    pub fn set_focused_window(_pid: i32) -> bool {
        false
    }

    pub fn get_focused_window() -> Option<i32> {
        None
    }
}
