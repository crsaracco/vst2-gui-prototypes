use std::ffi::c_void;
use std::sync::{Arc, Mutex};

use log::*;
use vst::editor::Editor as VstEditor;
use vst::plugin::HostCallback;

use crate::parameters::Parameters;

mod window;

const DEFAULT_WIDTH: i32 = 1024;
const DEFAULT_HEIGHT: i32 = 1024;

pub struct Editor {
    params: Arc<Parameters>,
    window: Option<window::Window>,
    host_callback: Arc<Mutex<HostCallback>>,
}

impl Editor {
    pub fn new(host_callback: Arc<Mutex<HostCallback>>, params: Arc<Parameters>) -> Self {
        Self {
            params,
            window: None,
            host_callback,
        }
    }
}

impl VstEditor for Editor {
    fn size(&self) -> (i32, i32) {
        info!("Editor::size()");

        if self.window.is_some() {
            return (
                self.window.as_ref().unwrap().get_width() as i32,
                self.window.as_ref().unwrap().get_height() as i32,
            );
        }

        (DEFAULT_WIDTH, DEFAULT_HEIGHT)
    }

    // Typically ignored by DAWs. Just return (0, 0).
    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn close(&mut self) {
        info!("Editor::close()");
        self.window = None
    }

    fn open(&mut self, parent: *mut c_void) -> bool {
        info!("Editor::open()");

        self.window = Some(window::Window::new(self.host_callback.clone(), self.params.clone(), parent));

        // success
        true
    }

    fn is_open(&mut self) -> bool {
        info!("Editor::is_open()");
        if self.window.is_some() {
            return true;
        }
        false
    }
}
