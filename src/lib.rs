#![feature(uniform_paths)]
#![feature(test)]

extern crate vst;
extern crate log;
extern crate simplelog;
extern crate xcb;

mod x_handle;
mod editor;
mod atomic_float;
mod parameters;
mod square_oscillator;
mod gui_vst;

use vst::plugin_main;

plugin_main!(gui_vst::GuiVst);