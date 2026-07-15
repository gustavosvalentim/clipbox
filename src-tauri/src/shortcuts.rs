use std::sync::Mutex;

use enigo::{Enigo, Mouse};
use tauri::{LogicalPosition, LogicalSize, Manager, Position, WebviewWindow};
use tauri_plugin_global_shortcut::{
    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutEvent, ShortcutState,
};

use crate::paste::PasteState;
use crate::window::get_main_window;

fn show_on_cursor_handler(app: &tauri::AppHandle) {
    app.state::<PasteState>().load_focused_window();

    let Some(window) = get_main_window(app) else {
        println!("Failed to get main window");
        return;
    };

    let enigo = app.state::<Mutex<Enigo>>();
    let Ok(enigo) = enigo.lock() else {
        println!("Failed to get cursor position");
        return;
    };

    let Ok((mouse_x, mouse_y)) = enigo.location() else {
        println!("Failed to get cursor position");
        return;
    };

    // TODO: handle multi monitor setups
    // Physical position causes the position to be off on HiDPI screens
    // Enigo uses logical coordinates, meaning DPI affects the position.
    // We need to clamp the position to the screen size, so we convert
    // all sizes to logical coordinates.
    // under the hood this is just size / scale_factor (DPI).
    let window_size = get_window_logical_size(&window);
    let monitor_size = get_screen_logical_size(&window);

    let x = f64::from(mouse_x).clamp(0.0, monitor_size.width - window_size.width);
    let y = f64::from(mouse_y).clamp(0.0, monitor_size.height - window_size.height);
    let window_position = LogicalPosition { x, y };

    if let Err(e) = window.set_position(Position::Logical(window_position)) {
        println!("Failed to position window: {:?}", e);
        return;
    }

    let window = window.clone();
    // this is a hack to make the window appear on the correct
    // position without flickering.
    // Because tauri window methods are async, show() may run before
    // set_position() finishes, causing the window to briefly appear
    // on the old position before moving to the new one.
    // Since we don't want to block the main thread, we spawn another
    // one to wait and then show the window; otherwise the flickering
    // will be worse, since we block the main thread for a short time.
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(25)).await;

        if let Err(e) = window.show() {
            println!("Failed to show window: {e}")
        }

        if let Err(e) = window.set_focus() {
            println!("Failed to focus window: {e}")
        }
    });
}

pub fn register_shortcuts(app: &tauri::AppHandle) -> Result<(), tauri::Error> {
    #[cfg(desktop)]
    {
        let global_shortcut_handler = tauri_plugin_global_shortcut::Builder::new()
            .with_handler(global_shortcut_handler)
            .build();

        app.plugin(global_shortcut_handler)?;

        let show_window_shortcut = show_window_shortcut();
        if let Err(e) = app.global_shortcut().register(show_window_shortcut) {
            println!("Failed to register global shortcut: {e}");
        }
    }

    Ok(())
}

fn global_shortcut_handler(app: &tauri::AppHandle, shortcut: &Shortcut, event: ShortcutEvent) {
    let show_window_shortcut = show_window_shortcut();

    if shortcut == &show_window_shortcut {
        match event.state() {
            ShortcutState::Pressed => show_on_cursor_handler(app),
            ShortcutState::Released => {}
        }
    }
}

fn show_window_shortcut() -> Shortcut {
    #[cfg(target_os = "macos")]
    let mod_key = Modifiers::META;

    #[cfg(not(target_os = "macos"))]
    let mod_key = Modifiers::ALT;

    Shortcut::new(Some(mod_key | Modifiers::SHIFT), Code::KeyV)
}

fn get_window_logical_size(window: &WebviewWindow) -> LogicalSize<f64> {
    let Ok(window_size) = window.inner_size() else {
        return LogicalSize {
            width: 0.0,
            height: 0.0,
        };
    };

    window_size.to_logical(window.scale_factor().unwrap())
}

fn get_screen_logical_size(window: &WebviewWindow) -> LogicalSize<f64> {
    let Ok(Some(monitor)) = window.current_monitor() else {
        return LogicalSize {
            width: 0.0,
            height: 0.0,
        };
    };

    monitor.size().to_logical(window.scale_factor().unwrap())
}
