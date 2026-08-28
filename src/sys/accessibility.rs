use std::ffi::c_void;

use objc2::rc::autoreleasepool;
use objc2::runtime::AnyObject;
use objc2::{class, msg_send};
use tracing::warn;

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    fn AXIsProcessTrustedWithOptions(options: *const c_void) -> bool;

    static kAXTrustedCheckOptionPrompt: *const c_void;
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    static kCFBooleanFalse: *const c_void;
    static kCFBooleanTrue: *const c_void;
}

#[inline]
fn ax_is_trusted() -> bool {
    unsafe {
        autoreleasepool(|_| {
            let keys: [*mut AnyObject; 1] = [kAXTrustedCheckOptionPrompt as *mut AnyObject];
            let vals: [*mut AnyObject; 1] = [kCFBooleanFalse as *mut AnyObject];
            let dict: *mut AnyObject = msg_send![
                class!(NSDictionary),
                dictionaryWithObjects: vals.as_ptr(),
                forKeys:              keys.as_ptr(),
                count:                1usize
            ];

            AXIsProcessTrustedWithOptions(dict.cast())
        })
    }
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn prompt_ax_trust_dialog() {
    autoreleasepool(|_| {
        let keys: [*mut AnyObject; 1] = [kAXTrustedCheckOptionPrompt as *mut AnyObject];
        let vals: [*mut AnyObject; 1] = [kCFBooleanTrue as *mut AnyObject];
        let dict: *mut AnyObject = msg_send![
            class!(NSDictionary),
            dictionaryWithObjects: vals.as_ptr(),
            forKeys:              keys.as_ptr(),
            count:                1usize
        ];
        let _ = AXIsProcessTrustedWithOptions(dict.cast());
    });
}

/// Request the macOS Accessibility prompt for an explicit user action such as
/// `rift service start`. The long-running daemon does not prompt itself, which
/// prevents launchd from repeatedly reopening System Settings when permission
/// is missing.
pub fn request_accessibility_permission() -> bool {
    if ax_is_trusted() {
        return true;
    }

    unsafe { prompt_ax_trust_dialog() };
    false
}

pub fn ensure_accessibility_permission() {
    if ax_is_trusted() {
        return;
    }

    warn!("Accessibility permission is not granted; automatic prompting is disabled");
    eprintln!(
        "Rift does not have Accessibility permission. Enable the existing Rift entry in System Settings > Privacy & Security > Accessibility, then restart Rift. Rift will not open System Settings automatically."
    );

    std::process::exit(1);
}
