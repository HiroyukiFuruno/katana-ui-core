use minifb::{CursorStyle, Window};

use super::window_interaction::StorybookCursorStyle;

pub(super) fn apply_cursor_style(window: &mut Window, cursor: StorybookCursorStyle) {
    window.set_cursor_style(cursor.fallback_minifb_cursor());
    if cursor == StorybookCursorStyle::PointingHand {
        platform_pointing_hand_cursor();
    }
}

#[cfg(target_os = "macos")]
fn platform_pointing_hand_cursor() {
    macos::set_pointing_hand_cursor();
}

#[cfg(not(target_os = "macos"))]
fn platform_pointing_hand_cursor() {}

impl StorybookCursorStyle {
    fn fallback_minifb_cursor(self) -> CursorStyle {
        match self {
            Self::Arrow => CursorStyle::Arrow,
            Self::Ibeam => CursorStyle::Ibeam,
            Self::ResizeAll => CursorStyle::ResizeAll,
            Self::PointingHand => CursorStyle::OpenHand,
        }
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use std::ffi::{c_char, c_void};

    type Id = *mut c_void;
    type Sel = *mut c_void;

    const NS_CURSOR_CLASS: &[u8] = b"NSCursor\0";
    const POINTING_HAND_CURSOR_SELECTOR: &[u8] = b"pointingHandCursor\0";
    const SET_SELECTOR: &[u8] = b"set\0";

    #[link(name = "AppKit", kind = "framework")]
    unsafe extern "C" {}

    #[link(name = "objc")]
    unsafe extern "C" {
        fn objc_getClass(name: *const c_char) -> Id;
        fn sel_registerName(name: *const c_char) -> Sel;
        #[link_name = "objc_msgSend"]
        fn objc_msg_send_id(receiver: Id, selector: Sel) -> Id;
    }

    pub(super) fn set_pointing_hand_cursor() {
        unsafe {
            let class = objc_getClass(NS_CURSOR_CLASS.as_ptr().cast::<c_char>());
            let selector =
                sel_registerName(POINTING_HAND_CURSOR_SELECTOR.as_ptr().cast::<c_char>());
            let cursor = objc_msg_send_id(class, selector);
            let set_selector = sel_registerName(SET_SELECTOR.as_ptr().cast::<c_char>());
            let _ = objc_msg_send_id(cursor, set_selector);
        }
    }
}
