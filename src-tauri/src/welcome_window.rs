use tauri::AppHandle;

const WINDOW_LABEL: &str = "welcome";

pub fn show(app: &AppHandle) -> Result<(), anyhow::Error> {
    let window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App("/welcome".into()),
    )
    .title("Welcome to Motion Minute")
    .center()
    .inner_size(550.0, 750.0)
    .transparent(true)
    .always_on_top(true)
    .focused(true)
    .decorations(false)
    .skip_taskbar(true)
    .resizable(true)
    .shadow(false)
    .build()?;

    window.show()?;

    Ok(())
}
