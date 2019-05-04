use std::ffi::{c_void, CString, OsStr};
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

pub fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

pub struct ProcAddressLoader {
    opengl32_dll: *mut winapi::HINSTANCE__,
}

impl ProcAddressLoader {
    pub fn new() -> Self {
        // Some functions load from opengl32.dll, and the others load from the fancy dummy context
        // that we have to spin up.
        // TODO: spin up fancy dummy context within this struct? Or maybe make the dummy context its own struct?
        let opengl32_dll = unsafe {
            kernel32::LoadLibraryA(b"opengl32.dll\0".as_ptr() as *const _)
        };
        if opengl32_dll.is_null() {
            panic!("WGL: opengl32.dll not found!");
        } else {
            println!("WGL: opengl32.dll loaded!");
        }

        Self {
            opengl32_dll,
        }
    }

    pub fn get_proc_address(&self, name: &str) -> *const c_void {
        unsafe {
            let mut ptr = opengl32::wglGetProcAddress(
                CString::new(name).unwrap().as_ptr() as *const _
            );

            if ptr.is_null() {
                ptr = kernel32::GetProcAddress(
                    self.opengl32_dll,
                    CString::new(name).unwrap().as_ptr() as *const _
                );
            }
            else {
                //println!("Loaded with rendering context: {}", name);
            }

            if ptr.is_null() {
                //println!("Couldn't load function: {}", name);
            }
            else {
                //println!("Loaded with opengl32.dll: {}", name);
            }

            ptr
        }
    }
}