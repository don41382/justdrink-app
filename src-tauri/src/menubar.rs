use cocoa::appkit::{NSApplication, NSApplicationPresentationOptions};
#[macro_use]
use cocoa::base::id;
use objc::class;
use objc::{msg_send, sel, sel_impl};

#[cfg(target_os = "macos")]
pub fn set_persistent_presentation_mode(hide: bool) {
    unsafe {
        let ns_app: id = msg_send![class!(NSApplication), sharedApplication];
        let option = if hide {
            (NSApplicationPresentationOptions::NSApplicationPresentationHideDock
                | NSApplicationPresentationOptions::NSApplicationPresentationHideMenuBar
                | NSApplicationPresentationOptions::NSApplicationPresentationDisableProcessSwitching
                | NSApplicationPresentationOptions::NSApplicationPresentationDisableHideApplication)
        } else {
            NSApplicationPresentationOptions::NSApplicationPresentationDefault
        };
        ns_app.setPresentationOptions_(option);
    }
}

#[cfg(not(target_os = "macos"))]
pub fn set_hide_menu_bar(_hide: bool) {
    // No-op for non-macOS platforms
}
