use anyhow::Error;

pub fn sleep_is_prevented_by_apps() -> Result<Option<String>, Error> {
    #[cfg(target_os = "windows")]
    return windows::sleep_is_prevented_by_apps();

    #[cfg(target_os = "macos")]
    return mac::sleep_is_prevented_by_apps();

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    Err(anyhow::anyhow!("Unsupported platform"))
}

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod mac;
