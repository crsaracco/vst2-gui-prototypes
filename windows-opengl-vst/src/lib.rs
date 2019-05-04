extern crate log;
extern crate simplelog;
extern crate vst;
extern crate gl;
extern crate kernel32;
extern crate opengl32;
extern crate user32;
extern crate winapi;

use vst::plugin_main;

mod audio_engine;
mod editor;
mod gvw_plugin;
mod midi_input_processor;
mod parameters;

plugin_main!(gvw_plugin::GvlPlugin);
