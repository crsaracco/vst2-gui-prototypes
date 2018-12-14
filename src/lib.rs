#![feature(uniform_paths)]

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


mod editor;
use editor::Editor;

struct GuiVst {
    editor: Editor,
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
            editor: Editor::new(),
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

    // TODO: return None if the editor couldn't be created
    // (for example, if the connection to the X server couldn't be established)
    fn get_editor(&mut self) -> Option<&mut vst::editor::Editor> {
        info!("get_editor()");
        self.editor.draw_editor();
        Some(&mut self.editor)
    }
}

plugin_main!(GuiVst);