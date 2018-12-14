extern crate vst;
extern crate log;
extern crate simplelog;
extern crate xcb;

use std::{
    ffi::{c_void},
    os::raw::{c_ulong},
    fs::File,
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
        info!("Editor::open()");

        info!("parent_handle: {}", parent as u32);

        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let foreground = conn.generate_id();
        xcb::create_gc(&conn, foreground, screen.root(), &[
            (xcb::GC_FOREGROUND, screen.black_pixel()),
            (xcb::GC_GRAPHICS_EXPOSURES, 0),
        ]);
        let win = conn.generate_id();
        xcb::create_window(
            &conn,
            xcb::COPY_FROM_PARENT as u8,
            win,
            parent as u32,
            0, 0,
            1000, 1000,
            10,
            xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
            screen.root_visual(), &[
                (xcb::CW_BACK_PIXEL, screen.white_pixel()),
                (xcb::CW_EVENT_MASK,
                 xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
            ]
        );
        xcb::map_window(&conn, win);
        conn.flush();
    }

    fn is_open(&mut self) -> bool {
        info!("Editor::is_open()");
        self.is_open
    }
}

plugin_main!(GuiVst);