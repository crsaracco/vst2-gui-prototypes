use std::ptr::null_mut;
use log::*;

use super::util;
use super::window_class::WindowClass;

pub struct Win32Window {
    handle: winapi::HWND,
}

impl Win32Window {
    pub fn new(parent: *mut std::ffi::c_void, window_class: WindowClass, title: &str, dummy: bool) -> Self {
        let title = util::win32_string(title);

        // TODO: what are these flags?
        let mut dw_style = 0;
        if !dummy {
            dw_style = winapi::WS_CHILDWINDOW;
        }

        unsafe {
            // Create a window using your window class. Store the instance in hinstance(?)
            let window_handle = user32::CreateWindowExW(
                0,                             // TODO: what is this?
                window_class.name_win32_ptr(), // Name of the class
                title.as_ptr(),                // Title of the window
                dw_style,
                winapi::CW_USEDEFAULT,         // Default X coordinate
                winapi::CW_USEDEFAULT,         // Default Y coordinate
                1024,                          // Default width
                768,                           // Default height
                parent as *mut winapi::HWND__, // Parent window
                null_mut(),                    // No menus
                window_class.hinstance(),      // The instance to the handle...?
                null_mut(),                    // TODO: what is this?
            );

            if window_handle.is_null() {
                info!("Failed to create the OpenGL window.");
                panic!("Failed to create the OpenGL window.");
            }

            Self {
                handle: window_handle,
            }
        }
    }

    pub fn handle(&self) -> winapi::HWND {
        self.handle
    }
}
