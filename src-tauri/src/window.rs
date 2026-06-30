use tauri::window::{Color, Effect, EffectState, EffectsBuilder};
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
        .background_color(Color(0, 0, 0, 0))
        .always_on_top(true)
        .visible_on_all_workspaces(true)
        .effects(
            EffectsBuilder::new()
                .effect(Effect::HudWindow)
                .state(EffectState::Active)
                .radius(settings.radius)
                .build(),
        )
        .build()?;

    let _ = window.hide();

    Ok(window)
}

pub fn window_events_handler(window: &Window, event: &WindowEvent) {
    if let WindowEvent::Focused(focused) = event {
        if !focused {
            let _ = window.hide();
        }
    }
}
