extern crate vst;
extern crate log;
extern crate simplelog;
extern crate x11;

use std::{
    ffi::{c_void},
    os::raw::{c_ulong, c_uint},
    ptr,
    fs::File,
    mem,
};
use vst::{
    plugin_main,
    plugin::{
        Plugin,
        Info,
        Category,
    },
    editor::Editor
};
use log::*;
use x11::xlib;






struct GuiVst {
    editor: GuiVstEditor,
}

impl Default for GuiVst {
    fn default() -> Self {
        // Set up a logger so we can see what's going on in the VST
        let mut logger_config = simplelog::Config::default();
        logger_config.time_format = Some("%H:%M:%S%.6f");

        simplelog::CombinedLogger::init(
            vec![
                simplelog::WriteLogger::new(
                    simplelog::LevelFilter::max(),
                    logger_config,
                    File::create("/tmp/plugin.log").unwrap()
                ),
            ]
        ).unwrap();

        Self {
            editor: GuiVstEditor::new(),
        }
    }
}

impl Plugin for GuiVst {
    fn get_info(&self) -> Info {
        Info {
            name: "gui-vst".to_string(),
            vendor: "crsaracco".to_string(),
            unique_id: 1147000001, // Make sure this is a unique number across all of your VSTs!
            category: Category::Synth,
            inputs: 0,
            outputs: 2,
            parameters: 0,
            initial_delay: 0,
            ..Info::default()
        }
    }

    fn init(&mut self) {
        info!("init()");
    }

    fn get_editor(&mut self) -> Option<&mut Editor> {
        info!("get_editor()");
        Some(&mut self.editor)
    }
}





struct GuiVstEditor {
    window_handle: c_ulong,
    is_open: bool,
}

impl GuiVstEditor {
    fn new() -> Self {
        Self {
            window_handle: 0,
            is_open: false,
        }
    }
}

impl Editor for GuiVstEditor {
    fn size(&self) -> (i32, i32) {
        info!("Editor::size()");
        (1000, 1000)
    }

    fn position(&self) -> (i32, i32) {
        info!("Editor::position()");
        (0, 0)
    }

    fn close(&mut self) {
        info!("Editor::close()");
        self.is_open = false;
    }

    fn open(&mut self, parent: *mut c_void) {
        unsafe {
            info!("Editor::open()");

            /*
            NOTE: This commented-out part isn't working.
            Main issue for right now is I don't really know what I'm doing with this `*mut c_void`

            // TODO: Can I get the display from the parent somehow?
            // (just trying to get default stuff for now, just to see *anything* working)
            let display = xlib::XOpenDisplay(ptr::null());
            let screen = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen);
            let mut attributes: xlib::XSetWindowAttributes = mem::uninitialized();
            // TODO: set attributes

            let window = xlib::XCreateWindow( // Name          Rust type                  C type
               display,                       // display       *mut Display               Display*
               root,                          // parent        c_ulong                    Window
               0,                             // x             c_int                      int
               0,                             // y             c_int                      int
               1000,                          // width         c_uint                     unsigned int
               1000,                          // height        c_uint                     unsigned int
               0,                             // border_width  c_uint                     unsigned int
               0,                             // depth         c_int                      int
               xlib::InputOutput as c_uint,   // class         c_uint                     unsigned int
               ptr::null_mut(),               // visual        *mut Visual                visual*
               0,                             // valuemask     c_ulong                    unsigned long
               &mut attributes,               // attributes    *mut XSetWindowAttributes  XSetWindowAttributes*
            );

            // TODO: call XMapWindow to show the window after creating it
            self.window_handle = window;
            self.is_open = true;
            */
        }
    }

    fn is_open(&mut self) -> bool {
        info!("Editor::is_open()");
        self.is_open
    }
}

plugin_main!(GuiVst);