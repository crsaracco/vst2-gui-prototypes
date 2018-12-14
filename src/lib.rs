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
    is_open: bool,
    polyline: Vec<xcb::Point>,
    window_handle: u32,
    draw_context: u32,
}

impl GuiVstEditor {
    fn new() -> Self {
        Self {
            is_open: false,
            polyline: vec![],
            window_handle: 0,
            draw_context: 0,
        }
    }

    fn create_window(&mut self) {
        info!("GuiVstEditor::create_window()");
        self.polyline.push(xcb::Point::new(50, 10 ));
        self.polyline.push(xcb::Point::new(5, 20 ));
        self.polyline.push(xcb::Point::new(25, -20 ));
        self.polyline.push(xcb::Point::new(10, 10 ));

        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();

        self.draw_context = conn.generate_id();

        xcb::create_gc(&conn, self.draw_context, screen.root(), &[
            (xcb::GC_FOREGROUND, screen.black_pixel()),
            (xcb::GC_GRAPHICS_EXPOSURES, 0),
        ]);

        self.window_handle = conn.generate_id();
        xcb::create_window(&conn,
                           xcb::COPY_FROM_PARENT as u8,
                           self.window_handle,
                           screen.root(),
                           0, 0,
                           150, 150,
                           10,
                           xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
                           screen.root_visual(), &[
                (xcb::CW_BACK_PIXEL, screen.white_pixel()),
                (xcb::CW_EVENT_MASK,
                 xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
            ]
        );
        xcb::map_window(&conn, self.window_handle);
        conn.flush();

        let event = conn.wait_for_event();
        match event {
            None => { }
            Some(event) => {
                let r = event.response_type() & !0x80;
                match r {
                    xcb::EXPOSE => {
                        self.draw_editor();
                    },
                    xcb::KEY_PRESS => {
                        let key_press: &xcb::KeyPressEvent = unsafe {
                            xcb::cast_event(&event)
                        };
                        info!("Key '{}' pressed", key_press.detail());
                    },
                    _ => {}
                }
            }
        }
    }

    fn draw_editor(&mut self) {
        info!("GuiVstEditor::draw_editor()");
        // Draw the polyline
        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        xcb::poly_line(&conn,
                       xcb::COORD_MODE_PREVIOUS as u8,
                       self.window_handle,
                       self.draw_context,
                       &self.polyline
        );

        // Flush the request
        conn.flush();

        std::thread::sleep_ms(1000);
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
        self.create_window();
    }

    fn is_open(&mut self) -> bool {
        info!("Editor::is_open()");
        self.is_open
    }
}



plugin_main!(GuiVst);