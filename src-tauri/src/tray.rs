use tauri::tray::{TrayIcon, TrayIconBuilder};

pub fn create(app: &tauri::AppHandle) -> Result<TrayIcon, tauri::Error> {
    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .icon_as_template(true)
        .tooltip("Clipbox")
        .build(app)
}
