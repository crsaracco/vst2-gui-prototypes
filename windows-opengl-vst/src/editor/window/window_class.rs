use std::ptr::null_mut;

use super::util;

pub struct WindowClass {
    name: String,
    hinstance: *mut winapi::HINSTANCE__,
}

impl WindowClass {
    pub fn new(name: &str, window_process: winapi::WNDPROC) -> Self {
        let win32_name = util::win32_string(name);

        unsafe {
            let hinstance = kernel32::GetModuleHandleW(null_mut());
            let window_class = winapi::WNDCLASSW {
                style: winapi::CS_OWNDC | winapi::CS_HREDRAW | winapi::CS_VREDRAW, // TODO: explain these flags
                lpfnWndProc: window_process, // Custom window process for handling events
                hInstance: hinstance,
                lpszClassName: win32_name.as_ptr(), // "name of the class" TODO: (???)
                cbClsExtra: 0,                      // TODO: what is?
                cbWndExtra: 0,                      // TODO: what is?
                hIcon: null_mut(), // TODO: what is? (I assume the icon of the window)
                hCursor: null_mut(), // TODO: what is? (I guess you can change the cursor)
                hbrBackground: null_mut(), // TODO: what is?
                lpszMenuName: null_mut(), // TODO: what is?
            };

            user32::RegisterClassW(&window_class);

            Self{
                name: name.to_string(),
                hinstance,
            }
        }
    }

    pub fn name_win32_ptr(&self) -> *const u16 {
        let win32_name = util::win32_string(&self.name);
        win32_name.as_ptr()
    }

    pub fn hinstance(&self) -> *mut winapi::HINSTANCE__ {
        self.hinstance
    }
}
