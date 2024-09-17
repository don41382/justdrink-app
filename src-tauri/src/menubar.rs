#[cfg(target_os = "macos")]
use cocoa::appkit::{NSApplication, NSApplicationPresentationOptions};
#[cfg(target_os = "macos")]
use cocoa::base::id;
#[cfg(target_os = "macos")]
use objc::class;
#[cfg(target_os = "macos")]
use objc::{msg_send, sel, sel_impl};

#[cfg(target_os = "macos")]
pub fn set_persistent_presentation_mode(hide: bool) {
    unsafe {
        let ns_app: id = msg_send![class!(NSApplication), sharedApplication];
        let option = if hide {
            NSApplicationPresentationOptions::NSApplicationPresentationHideDock
                | NSApplicationPresentationOptions::NSApplicationPresentationHideMenuBar
                | NSApplicationPresentationOptions::NSApplicationPresentationDisableProcessSwitching
                | NSApplicationPresentationOptions::NSApplicationPresentationDisableHideApplication
        } else {
            NSApplicationPresentationOptions::NSApplicationPresentationDefault
        };
        ns_app.setPresentationOptions_(option);
    }
}

#[cfg(not(target_os = "macos"))]
pub fn set_persistent_presentation_mode(hide: bool) {
    // No-op for non-macOS platforms
}
