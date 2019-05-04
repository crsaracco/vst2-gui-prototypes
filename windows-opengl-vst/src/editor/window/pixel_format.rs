use log::*;

/// NOTE: should only be used to create "dummy" pixel formats.
pub fn dummy_pixel_format(device_context_handler: *mut winapi::HDC__) -> (i32, winapi::PIXELFORMATDESCRIPTOR) {
    let mut pixel_format_descriptor = winapi::PIXELFORMATDESCRIPTOR {
        nSize: std::mem::size_of::<winapi::PIXELFORMATDESCRIPTOR>() as u16,
        nVersion: 1,
        dwFlags: winapi::PFD_DRAW_TO_WINDOW | winapi::PFD_SUPPORT_OPENGL | winapi::PFD_DOUBLEBUFFER,
        iPixelType: winapi::PFD_TYPE_RGBA,
        cColorBits: 32,
        cRedBits: 0,
        cRedShift: 0,
        cGreenBits: 0,
        cGreenShift: 0,
        cBlueBits: 0,
        cBlueShift: 0,
        cAlphaBits: 8,
        cAlphaShift: 0,
        cAccumBits: 0,
        cAccumRedBits: 0,
        cAccumGreenBits: 0,
        cAccumBlueBits: 0,
        cAccumAlphaBits: 0,
        cDepthBits: 24,
        cStencilBits: 8,
        cAuxBuffers: 0,
        iLayerType: winapi::PFD_MAIN_PLANE,
        bReserved: 0,
        dwLayerMask: 0,
        dwVisibleMask: 0,
        dwDamageMask: 0,
    };

    let pixel_format = unsafe {
        gdi32::ChoosePixelFormat(device_context_handler, &pixel_format_descriptor)
    };
    if pixel_format == 0 {
        info!("Failed to find a suitable pixel format.");
        panic!("Failed to find a suitable pixel format.");
    }

    (pixel_format, pixel_format_descriptor)
}