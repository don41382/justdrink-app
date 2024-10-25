pub fn enforce_full_screen(hide: bool) {
    #[cfg(target_os = "macos")]
    return mac::enforce_full_screen(hide);

    #[cfg(target_os = "windows")]
    return;
}

#[cfg(target_os = "macos")]
mod mac;
