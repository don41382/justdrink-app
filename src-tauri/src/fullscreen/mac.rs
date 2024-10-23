use std::panic;
use cocoa::appkit::{NSApplication, NSApplicationPresentationOptions};
use cocoa::base::id;
use log::error;
use objc::class;
use objc::{msg_send, sel, sel_impl};

pub fn enforce_full_screen(hide: bool) {
    let result = panic::catch_unwind(|| {
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
    });

    if let Err(err) = result {
        error!("Caught a panic while setting presentation options: {:?}", err);
    }
}