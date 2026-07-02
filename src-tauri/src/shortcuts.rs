use tauri::Manager;

use crate::window::{ClipboxWindowExt, ClipboxWindowHandle};

fn show_window_shortcut_handler(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        match window.clipbox().show_on_cursor(app) {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to show window on cursor: {:?}", e);
            }
        }
    }
}

pub fn register_shortcuts(app: &tauri::AppHandle) -> Result<(), tauri::Error> {
    #[cfg(desktop)]
    {
        use tauri_plugin_global_shortcut::{
            Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
        };

        let show_window_shortcut =
            Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyV);

        let global_shortcut_handler = tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app, shortcut, event| {
                if shortcut == &show_window_shortcut {
                    match event.state() {
                        ShortcutState::Pressed => show_window_shortcut_handler(app),
                        ShortcutState::Released => {}
                    }
                }
            })
            .build();

        app.plugin(global_shortcut_handler)?;

        match app.global_shortcut().register(show_window_shortcut) {
            Ok(_) => println!("Registered shortcut"),
            Err(e) => println!("Failed to register shortcut: {e}"),
        };
    }

    Ok(())
}
