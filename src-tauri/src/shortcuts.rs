use std::sync::{Arc, Mutex};

use enigo::{Enigo, Mouse};
use tauri::{LogicalPosition, Manager, Position};

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
                        ShortcutState::Pressed => {
                            if let Some(window) = app.get_webview_window("main") {
                                let enigo = app.state::<Arc<Mutex<Enigo>>>();
                                let (mouse_x, mouse_y) = enigo.lock().unwrap().location().unwrap();

                                let new_pos = LogicalPosition {
                                    x: f64::from(mouse_x),
                                    y: f64::from(mouse_y),
                                };
                                let _ = window.set_position(Position::Logical(new_pos));

                                window.show().unwrap();
                                window.set_focus().unwrap();
                            }
                        }
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
