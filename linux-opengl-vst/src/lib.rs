extern crate log;
extern crate simplelog;
extern crate vst;
extern crate rand;

use vst::plugin_main;

mod audio_engine;
mod editor;
mod gvl_plugin;
mod midi_input_processor;
mod parameters;

plugin_main!(gvl_plugin::GvlPlugin);
