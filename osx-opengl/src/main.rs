use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps, NSApplicationActivationPolicyRegular, NSBackingStoreBuffered, NSRunningApplication, NSWindow, NSWindowStyleMask, NSOpenGLPixelFormat, NSOpenGLView, NSView};
use cocoa::base::{id, nil, NO, YES};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize};
use cocoa::appkit::NSOpenGLPFAOpenGLProfiles::NSOpenGLProfileVersion3_2Core;
use cocoa::appkit::NSOpenGLPixelFormatAttribute::{NSOpenGLPFAColorSize, NSOpenGLPFAOpenGLProfile, NSOpenGLPFAAlphaSize, NSOpenGLPFADoubleBuffer, NSOpenGLPFAAccelerated};
use core_foundation::string::CFString;
use core_foundation::bundle::{CFBundleGetBundleWithIdentifier, CFBundleGetFunctionPointerForName};
use core_foundation::base::TCFType;
use cocoa::appkit::NSOpenGLContext;
#[macro_use] extern crate objc;

mod view;

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        let rect = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(640.0, 480.0));

        // Create window
        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(
                rect,
                NSWindowStyleMask::NSTitledWindowMask,
                NSBackingStoreBuffered,
                NO,
            )
            .autorelease();
        window.cascadeTopLeftFromPoint_(NSPoint::new(20.0, 20.0));
        window.makeKeyAndOrderFront_(nil);
        window.setAcceptsMouseMovedEvents_(YES);

        // OpenGL view
        let pixel_format_attributes = [
            NSOpenGLPFAOpenGLProfile as u32, NSOpenGLProfileVersion3_2Core as u32,
            NSOpenGLPFAColorSize as u32, 24,
            NSOpenGLPFAAlphaSize as u32, 8,
            NSOpenGLPFADoubleBuffer as u32,
            NSOpenGLPFAAccelerated as u32,
            0,
        ];

        let pixel_format = NSOpenGLPixelFormat::alloc(nil)
            .initWithAttributes_(&pixel_format_attributes)
            .autorelease();

        let opengl_view = NSOpenGLView::alloc(nil)
            .initWithFrame_pixelFormat_(rect, pixel_format)
            .autorelease();

        window.setContentView_(opengl_view);
        NSOpenGLView::display_(opengl_view);

        let context: id = msg_send![opengl_view, openGLContext];

        gl::load_with(|addr| {
            let symbol_name: CFString = std::str::FromStr::from_str(addr).unwrap();
            let framework_name: CFString = std::str::FromStr::from_str("com.apple.opengl").unwrap();
            let framework = CFBundleGetBundleWithIdentifier(framework_name.as_concrete_TypeRef());
            let symbol = CFBundleGetFunctionPointerForName(framework, symbol_name.as_concrete_TypeRef());
            symbol as *const _
        });

        // Set a delegate for the window to handle events
        let event_delegate: id = msg_send![view::view_class(), new];
        window.setDelegate_(event_delegate);
        window.makeFirstResponder_(event_delegate);
        NSView::initWithFrame_(event_delegate, NSView::frame(opengl_view as id));
        opengl_view.addSubview_(event_delegate);

        // Draw
        context.makeCurrentContext();
        gl::ClearColor(0.5, 0.5, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        context.flushBuffer();
        msg_send![opengl_view, setNeedsDisplay: YES];

        // Run the "app"
        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        app.run();
    }
}
