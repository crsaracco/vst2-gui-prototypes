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
        HostCallback,
    },
    host::Host,
};
use log::*;
use std::sync::Arc;
use std::sync::Mutex;

mod x_handle;
mod editor;
mod atomic_float;
mod parameters;

use x_handle::XHandle;
use editor::Editor;
use parameters::Parameters;

struct GuiVst {
    host: HostCallback,
    editor: Editor,
    parameters: Arc<Parameters>,
}

impl Default for GuiVst {
    fn default() -> Self {
        GuiVst::new(HostCallback::default())
    }
}

impl Plugin for GuiVst {
    fn new(host: HostCallback) -> Self {
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

        let x_handle = Box::new(XHandle::new());
        let parameters = Arc::new(Parameters::new());
        let host_callback = Arc::new(Mutex::new(host));

        // Set up an Editor that uses this connection.
        Self {
            host,
            editor: Editor::new(x_handle, parameters.clone(), host_callback),
            parameters,
        }
    }

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
        info!("host VST version: {}", self.host.vst_version());
    }

    // TODO: return None if the editor couldn't be created
    // (for example, if the connection to the X server couldn't be established)
    fn get_editor(&mut self) -> Option<&mut vst::editor::Editor> {
        //info!("get_editor()");
        self.editor.draw_editor();
        Some(&mut self.editor)
    }

    fn get_parameter(&self, index: i32) -> f32 {
        info!("get_parameter");
        match index {
            0 => self.parameters.param1.get(),
            1 => self.parameters.param2.get(),
            _ => 0.0,
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.1}%", self.parameters.param1.get() * 100.0),
            1 => format!("{:.1}%", self.parameters.param2.get() * 100.0),
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
        info!("set_parameter");
        match index {
            0 => {
                self.parameters.param1.set(val);
            },
            1 => {
                self.parameters.param2.set(val);
            },
            _ => (),
        }
    }
}

plugin_main!(GuiVst);