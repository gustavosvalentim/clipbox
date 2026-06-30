use tauri::window::{Effect, EffectState, EffectsBuilder};
use tauri::{WebviewUrl, WebviewWindow, WebviewWindowBuilder, Window, WindowEvent};

pub struct Settings {
    pub width: f64,
    pub height: f64,
    pub transparent: bool,
    pub decorations: bool,
    pub radius: f64,
}

pub fn create(app: &tauri::AppHandle, settings: Settings) -> Result<WebviewWindow, tauri::Error> {
    let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
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
        .build()?;

    Ok(window)
}

pub fn window_events_handler(window: &Window, event: &WindowEvent) {
    if let WindowEvent::Focused(focused) = event {
        if !focused {
            let _ = window.hide();
        }
    }
}
