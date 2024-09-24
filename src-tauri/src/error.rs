use tauri::AppHandle;

const WINDOW_LABEL: &str = "error";

pub fn alert(app: &AppHandle) -> Result<(), anyhow::Error> {
    let window = tauri::WebviewWindowBuilder::new(
        app,
        WINDOW_LABEL,
        tauri::WebviewUrl::App("/error".into()),
    )
        .title("Error")
        .center()
        .inner_size(400.0, 700.0)
        .transparent(false)
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
