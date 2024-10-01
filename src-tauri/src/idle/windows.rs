extern crate winapi;

use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;
use std::ptr::null_mut;
use winapi::shared::minwindef::DWORD;
use winapi::shared::windef::{HWND, RECT};
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::GetModuleFileNameExW;
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use winapi::um::winuser::{
    GetForegroundWindow, GetMonitorInfoW, GetWindowRect, GetWindowThreadProcessId, IsWindowVisible,
    MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};
use winapi::um::winver::{GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW}; // Correct import from winver

fn get_friendly_app_name(exe_path: &PathBuf) -> Option<String> {
    unsafe {
        // Get the version info size
        let exe_path_wide: Vec<u16> = exe_path.as_os_str().encode_wide().chain(Some(0)).collect();
        let version_info_size = GetFileVersionInfoSizeW(exe_path_wide.as_ptr(), null_mut());

        if version_info_size > 0 {
            let mut version_info: Vec<u8> = vec![0; version_info_size as usize];

            // Get version info
            if GetFileVersionInfoW(
                exe_path_wide.as_ptr(),
                0,
                version_info_size,
                version_info.as_mut_ptr() as *mut _,
            ) != 0
            {
                let mut block = null_mut();
                let mut size = 0;

                // Query for the FileDescription field
                if VerQueryValueW(
                    version_info.as_ptr() as *const _,
                    OsStr::new("\\StringFileInfo\\040904B0\\FileDescription")
                        .encode_wide()
                        .chain(Some(0))
                        .collect::<Vec<u16>>()
                        .as_ptr(),
                    &mut block,
                    &mut size,
                ) != 0
                {
                    let description_wide =
                        std::slice::from_raw_parts(block as *const u16, size as usize - 1);
                    return Some(
                        OsString::from_wide(description_wide)
                            .to_string_lossy()
                            .into_owned(),
                    );
                }
            }
        }
    }
    None
}

pub fn sleep_is_prevented_by_apps() -> Result<Option<String>, anyhow::Error> {
    unsafe {
        // Get the foreground (active) window
        let hwnd: HWND = GetForegroundWindow();

        // Check if the window is visible
        if !hwnd.is_null() && IsWindowVisible(hwnd) != 0 {
            // Get the dimensions of the window
            let mut window_rect: RECT = std::mem::zeroed();
            if GetWindowRect(hwnd, &mut window_rect) != 0 {
                // Get the monitor for this window
                let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
                let mut monitor_info: MONITORINFO = std::mem::zeroed();
                monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;

                if GetMonitorInfoW(monitor, &mut monitor_info) != 0 {
                    // Compare window dimensions with monitor dimensions
                    if window_rect.left == monitor_info.rcMonitor.left
                        && window_rect.top == monitor_info.rcMonitor.top
                        && window_rect.right == monitor_info.rcMonitor.right
                        && window_rect.bottom == monitor_info.rcMonitor.bottom
                    {
                        // The window is in fullscreen mode, now get the application name

                        // Get the process ID of the window
                        let mut process_id: DWORD = 0;
                        GetWindowThreadProcessId(hwnd, &mut process_id);

                        // Open the process with read permissions
                        let process_handle =
                            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
                        if !process_handle.is_null() {
                            // Buffer to store the file path of the executable
                            let mut exe_path: [u16; 260] = [0; 260];

                            // Get the full file path of the process
                            if GetModuleFileNameExW(
                                process_handle,
                                null_mut(),
                                exe_path.as_mut_ptr(),
                                exe_path.len() as u32,
                            ) > 0
                            {
                                let exe_path =
                                    OsString::from_wide(&exe_path).to_string_lossy().to_string();
                                let exe_path = PathBuf::from(exe_path);

                                // Get the friendly application name
                                if let Some(friendly_name) = get_friendly_app_name(&exe_path) {
                                    CloseHandle(process_handle);
                                    return Ok(Some(friendly_name));
                                }

                                // If no friendly name found, return executable name
                                let app_name = exe_path
                                    .file_name()
                                    .map(|s| s.to_string_lossy().into_owned())
                                    .unwrap_or_else(|| "Unknown".to_string());

                                CloseHandle(process_handle);
                                return Ok(Some(app_name));
                            }

                            // Close the handle if unable to get the file path
                            CloseHandle(process_handle);
                        }
                        return Ok(Some("Unknown".to_string()));
                    }
                }
            }
        }
    }
    Ok(None) // No fullscreen application found
}
