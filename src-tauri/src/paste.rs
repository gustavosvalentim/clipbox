use tauri::Manager;
use tauri_plugin_clipboard_manager::ClipboardExt;

pub struct PasteService {
    app: tauri::AppHandle,
}

// TODO: handling the clipboard should be moved to a separate service
// because this depends on the tauri::AppHandle, which is hard to mock
impl PasteService {
    pub fn new(app: tauri::AppHandle) -> Self {
        Self { app }
    }

    // TODO: paste from selection should get the hash instead of selected text
    pub fn paste_from_selection(&self, text: String) {
        // This can possibly belong to the command that calls this service
        // since it's a framework-specific thing
        self.app.clipboard().write_text(text).unwrap();

        if let Some(window) = self.app.get_webview_window("main") {
            let _ = window.hide();
        }
    }
}
