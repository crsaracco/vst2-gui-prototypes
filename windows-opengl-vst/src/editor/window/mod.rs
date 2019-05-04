// Resources:
//  - https://stackoverflow.com/a/49034511
//  - https://www.khronos.org/opengl/wiki/Creating_an_OpenGL_Context_(WGL)
//  - https://gist.github.com/nickrolfe/1127313ed1dbf80254b614a721b3ee9c
//
// Look into these for window crashing on event handling:
//  - https://github.com/wrl/rutabaga/blob/master/src/platform/win/window.c#L373
//  - https://github.com/wrl/rutabaga/blob/master/src/platform/win/window.c#L93-L120
//  - https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-getwindowlongptra
//
// Look into this for non-deterministic window class names:
//  - https://github.com/wrl/rutabaga/blob/master/src/platform/win/window.c#L126-L151

use std::mem;
use std::ptr::null_mut;
use std::ffi::CString;
use std::ffi::c_void;
use std::sync::{Arc, Mutex};

use vst::plugin::HostCallback;
use vst::host::Host;
use user32::{DefWindowProcW, DispatchMessageW, GetMessageW, TranslateMessage};
use winapi::HWND;
use winapi::winuser::MSG;
use log::*;
use rand::Rng;
use winapi::LONG_PTR;

use crate::parameters::Parameters;

mod pixel_format;
mod util;
mod win32_window;
mod window_class;

use win32_window::Win32Window;
use window_class::WindowClass;

fn draw_window(hwnd: HWND) {
    unsafe {
        gl::ClearColor(0.5f32, 0.5f32, 1.0f32, 1.0f32);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::Flush();
        let hdc = user32::GetDC(hwnd);
        gdi32::SwapBuffers(hdc);
        user32::ReleaseDC(hwnd, hdc);
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: winapi::UINT,
    wparam: winapi::WPARAM,
    lparam: winapi::LPARAM,
) -> winapi::LRESULT {
    // TODO: This was a static in C/C++. Does it need to be? Maybe put it into the WindowLongPtr?
    let mut paint_struct = mem::zeroed();

    let mut state = unsafe { &mut *(user32::GetWindowLongPtrW(hwnd, winapi::winuser::GWLP_USERDATA) as *mut WindowState) };

    match msg {
        winapi::WM_DESTROY => {
            info!("wnd_proc: Destroyed!");
            0
        },
        winapi::WM_PAINT => {
            info!("wnd_proc: Paint");

            // TODO: This came after the draw_window() function in example code I found. Should it be?
            user32::BeginPaint(hwnd, &mut paint_struct);

            draw_window(hwnd);
            user32::EndPaint(hwnd, &mut paint_struct);

            DefWindowProcW(hwnd, msg, wparam, lparam)
        },
        winapi::WM_LBUTTONDOWN => {
            info!("wnd_proc: CLICK!");
            let new_param_val = state.rng.gen_range(0.0, 1.0);
            state.params.pulse_width.set(new_param_val);
            state.host_callback.lock().unwrap().automate(1, new_param_val);
            0
        },
        _ => {
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
    }
}



fn handle_message(hwnd: HWND) -> bool {
    unsafe {
        let mut message: MSG = mem::uninitialized();
        if GetMessageW(&mut message as *mut MSG, hwnd, 0, 0) > 0 {
            TranslateMessage(&message as *const MSG);
            DispatchMessageW(&message as *const MSG);

            true
        } else {
            false
        }
    }
}

fn init_opengl(real_hdc: winapi::HDC, proc_address_loader: &util::ProcAddressLoader) -> *mut winapi::HGLRC__ {
    use std::os::raw::c_int;
    use std::os::raw::c_uint;

    type wglCreateContextAttribsArbType = unsafe extern "system" fn(winapi::HDC, winapi::HGLRC, *const c_int) -> winapi::HGLRC;
    type wglChoosePixelFormatArbType = unsafe extern "system" fn(winapi::HDC, *const c_int, *const f32, c_uint, *mut c_int, *mut c_uint) -> c_int;

    // Dummy window context setup (get wglCreateContextAttribsArb and wglChoosePixelFormatArb)
    let (wglCreateContextAttribsArb, wglChoosePixelFormatArb) = unsafe {
        let dummy_window_class = WindowClass::new(
            "wgl_dummy_class_name_dontuse".into(),
            Some(user32::DefWindowProcA),
        );

        let dummy_window = Win32Window::new(null_mut(), dummy_window_class, "Dummy Window", true);

        // Create a dummy context handler for this dummy window
        {
            let device_context_handler = user32::GetDC(dummy_window.handle());

            let (pixel_format, mut pixel_format_descriptor) = pixel_format::dummy_pixel_format(device_context_handler);

            // Set the pixel format...?
            if gdi32::SetPixelFormat(
                device_context_handler,
                pixel_format,
                &pixel_format_descriptor,
            ) == 0
            {
                info!("Failed to set the pixel format for the dummy window.");
                panic!("Failed to set the pixel format for the dummy window.");
            }

            // Describe pixel format......?
            gdi32::DescribePixelFormat(
                device_context_handler,
                pixel_format,
                std::mem::size_of::<winapi::PIXELFORMATDESCRIPTOR>() as u32,
                &mut pixel_format_descriptor,
            );

            user32::ReleaseDC(dummy_window.handle(), device_context_handler);
        }

        // Fetch context from window
        let dummy_hdc = user32::GetDC(dummy_window.handle());
        let dummy_context = opengl32::wglCreateContext(dummy_hdc);
        if dummy_context.is_null() {
            info!("Failed to create a dummy OpenGL rendering context.");
            panic!("Failed to create a dummy OpenGL rendering context.");
        }

        if opengl32::wglMakeCurrent(dummy_hdc, dummy_context) == 0 {
            info!("Failed to activate dummy OpenGL rendering context.");
            panic!("Failed to activate dummy OpenGL rendering context.");
        }

        let wglCreateContextAttribsArb = proc_address_loader.get_proc_address("wglCreateContextAttribsARB");
        let wglChoosePixelFormatArb = proc_address_loader.get_proc_address("wglChoosePixelFormatARB");

        // Destroy context
        opengl32::wglMakeCurrent(dummy_hdc, null_mut());
        opengl32::wglDeleteContext(dummy_context);
        user32::ReleaseDC(dummy_window.handle(), dummy_hdc);

        (wglCreateContextAttribsArb, wglChoosePixelFormatArb)
    };

    let wglCreateContextAttribsArb: wglCreateContextAttribsArbType = unsafe {
        std::mem::transmute(wglCreateContextAttribsArb)
    };
    let wglChoosePixelFormatArb: wglChoosePixelFormatArbType = unsafe {
        std::mem::transmute(wglChoosePixelFormatArb)
    };

    // Now we can set up pixel formats the "correct" way with wglChoosePixelFormatArb.

    // https://www.opengl.org/registry/specs/ARB/wgl_pixel_format.txt
    let wgl_draw_to_window_arb = 0x2001;
    let wgl_acceleration_arb = 0x2003;
    let wgl_support_opengl_arb = 0x2010;
    let wgl_double_buffer_arb = 0x2011;
    let wgl_pixel_type_arb = 0x2013;
    let wgl_color_bits_arb = 0x2014;
    let wgl_depth_bits_arb = 0x2022;
    let wgl_stencil_bits_arb = 0x02023;
    let wgl_full_acceleration_arb = 0x2027;
    let wgl_type_rgba_arb = 0x202b;
    let gl_true = 1;

    let pixel_format_attribs = [
        wgl_draw_to_window_arb, gl_true,
        wgl_support_opengl_arb, gl_true,
        wgl_double_buffer_arb, gl_true,
        wgl_acceleration_arb, wgl_full_acceleration_arb,
        wgl_pixel_type_arb, wgl_type_rgba_arb,
        wgl_color_bits_arb, 32,
        wgl_depth_bits_arb, 24,
        wgl_stencil_bits_arb, 8,
        0,
    ];

    let mut pixel_format: i32 = 0;
    let mut num_formats: u32 = 0;

    unsafe {
        wglChoosePixelFormatArb(real_hdc, pixel_format_attribs.as_ptr(), 0 as *const f32, 1, &mut pixel_format, &mut num_formats);
        if num_formats == 0 {
            info!("Failed to set the OpenGL 3.3 pixel format.");
            panic!("Failed to set the OpenGL 3.3 pixel format.");
        }

        let mut pixel_format_descriptor: winapi::PIXELFORMATDESCRIPTOR = mem::uninitialized();
        gdi32::DescribePixelFormat(real_hdc, pixel_format, std::mem::size_of::<winapi::PIXELFORMATDESCRIPTOR>() as u32, &mut pixel_format_descriptor);

        if gdi32::SetPixelFormat(
            real_hdc,
            pixel_format,
            &pixel_format_descriptor,
        ) == 0
        {
            info!("Failed to set the pixel format for the OpenGL window.");
            panic!("Failed to set the pixel format for the OpenGL window.");
        }

        // Specify OpenGL 3.3 core as our context type
        let wgl_context_major_version_arb = 0x2091;
        let wgl_context_minor_version_arb = 0x2092;
        let wgl_context_profile_mask_arb = 0x9126;
        let wgl_context_core_profile_bit_arb = 0x00000001;
        let gl33_attribs = [
            wgl_context_major_version_arb, 3,
            wgl_context_minor_version_arb, 3,
            wgl_context_profile_mask_arb, wgl_context_core_profile_bit_arb,
            0
        ];

        let gl33_context = wglCreateContextAttribsArb(real_hdc, 0 as *mut winapi::HGLRC__, gl33_attribs.as_ptr());
        if gl33_context as u32 == 0 {
            info!("Failed to create the OpenGL 3.3 rendering context.");
            panic!("Failed to create the OpenGL 3.3 rendering context.");
        }
        if opengl32::wglMakeCurrent(real_hdc, gl33_context) == 0 {
            info!("Failed to activate the OpenGL 3.3 rendering context.");
            panic!("Failed to activate the OpenGL 3.3 rendering context.")
        }

        return gl33_context;
    }

}

struct WindowState {
    rng: rand::rngs::ThreadRng,
    host_callback: Arc<Mutex<HostCallback>>,
    params: Arc<Parameters>,
}

pub struct Window {
    win32_window: Win32Window,
    hdc: *mut winapi::HDC__,
    context: *mut winapi::HGLRC__,
}

impl Window {
    pub fn new(host_callback: Arc<Mutex<HostCallback>>, params: Arc<Parameters>, parent: *mut c_void) -> Self {
        let window_state = Box::new(WindowState {
            rng: rand::thread_rng(),
            host_callback,
            params
        });

        let proc_address_loader = util::ProcAddressLoader::new();
        let window_class = WindowClass::new("OpenGL Window", Some(wnd_proc));
        let mut window = Win32Window::new(parent, window_class, "OpenGL Window", false);
        let hdc = unsafe {
            user32::GetDC(window.handle())
        };
        let context = init_opengl(hdc, &proc_address_loader);

        // Load all of the OpenGL function pointers.
        gl::load_with(|s| {
            proc_address_loader.get_proc_address(s)
        });

        unsafe {
            user32::ShowWindow(window.handle(), 1);
        }

        unsafe {
            // TODO: make a const for that magic number
            let window_state_cvoid = Box::into_raw(window_state) as *mut c_void;
            user32::SetWindowLongPtrW(window.handle(), winapi::winuser::GWLP_USERDATA, window_state_cvoid as LONG_PTR);
        }

        Self {
            win32_window: window,
            hdc,
            context,
        }
    }

    pub fn get_width(&self) -> u32 {
        1024
    }

    pub fn get_height(&self) -> u32 {
        768
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            opengl32::wglMakeCurrent(self.hdc, null_mut());
            opengl32::wglDeleteContext(self.context);
            user32::ReleaseDC(self.win32_window.handle(), self.hdc);
        }
    }
}
