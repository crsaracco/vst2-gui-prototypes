#![feature(uniform_paths)]
#![feature(test)]

extern crate vst;
extern crate log;
extern crate simplelog;
extern crate xcb;

use std::fs::File;
use vst::{
    plugin_main,
    plugin::{
        Plugin,
        Info,
        Category,
    },
};
use log::*;
use std::thread;
use std::sync::Arc;

mod x_handle;
use x_handle::XHandle;

mod editor;
use editor::Editor;

struct GuiVst {
    editor: Editor,
    param1: f32,
    param2: f32,
}

impl GuiVst {
    fn handle_events(conn: Arc<xcb::Connection>) {
        loop {
            let event = conn.wait_for_event();
            match event {
                None => (),
                Some(event) => {
                    let r = event.response_type();
                    match r {
                        xcb::BUTTON_PRESS => {
                            info!("Button pressed...");
                        },
                        xcb::BUTTON_RELEASE => {
                            info!("Button released...");
                        },
                        _ => {
                            info!("Some sort of event...?");
                        }
                    }
                }
            }
        }
    }
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

        // Set up the X connection.
        let x_handle = Box::new(XHandle::new());

        // Start handling events on this connection.
        let thread_conn = x_handle.conn();
        thread::spawn(move || {
            GuiVst::handle_events(thread_conn);
        });

        // Set up an Editor that uses this connection.
        Self {
            editor: Editor::new(x_handle),
            param1: 0.0,
            param2: 0.0,
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
            parameters: 2,
            initial_delay: 0,
            ..Info::default()
        }
    }

    fn init(&mut self) {
        info!("init()");


    }

    // TODO: return None if the editor couldn't be created
    // (for example, if the connection to the X server couldn't be established)
    fn get_editor(&mut self) -> Option<&mut vst::editor::Editor> {
        //info!("get_editor()");
        self.editor.draw_editor();
        Some(&mut self.editor)
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.param1,
            1 => self.param2,
            _ => 0.0,
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.1}%", self.param1 * 100.0),
            1 => format!("{:.1}%", self.param2 * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Parameter 1",
            1 => "Parameter 2",
            _ => "",
        }.to_string()
    }

    fn set_parameter(&mut self, index: i32, val: f32) {
        match index {
            0 => {
                self.param1 = val;
                self.editor.change_param1_value(val);
            },
            1 => {
                self.param2 = val;
                self.editor.change_param2_value(val);
            },
            _ => (),
        }
    }
}

plugin_main!(GuiVst);