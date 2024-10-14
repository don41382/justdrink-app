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
    .transparent(true)
    .always_on_top(true)
    .focused(true)
    .decorations(false)
    .skip_taskbar(true)
    .resizable(true)
    .shadow(true)
    .build()?;

    window.show()?;

    Ok(())
}
