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
        CanDo,
        Category,
        HostCallback,
    },
    event::Event,
    api::{Supported, Events},
    buffer::AudioBuffer,
};
use log::*;
use std::sync::Arc;
use std::sync::Mutex;

mod x_handle;
mod editor;
mod atomic_float;
mod parameters;
mod square_oscillator;

use x_handle::XHandle;
use editor::Editor;
use parameters::Parameters;
use square_oscillator::SquareOscillator;

struct GuiVst {
    host: HostCallback,
    editor: Editor,
    parameters: Arc<Parameters>,
    square_oscillator: SquareOscillator,
    sample_rate: f64,
    note_duration: f64,
    note: Option<u8>,
}

fn midi_pitch_to_freq(pitch: u8) -> f64 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f64 = 440.0;

    (((pitch as i8 - A4_PITCH) as f64) / 12.).exp2() * A4_FREQ
}

impl GuiVst {
    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _ => ()
        }
    }

    fn note_on(&mut self, note: u8) {
        self.note_duration = 0.0;
        self.note = Some(note);
        self.square_oscillator.change_frequency(midi_pitch_to_freq(note));
    }

    fn note_off(&mut self, note: u8) {
        if self.note == Some(note) {
            self.note = None;
        }
    }
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
        let editor_parameters = parameters.clone();
        let square_oscillator_parameters = parameters.clone();
        Self {
            host,
            editor: Editor::new(x_handle, editor_parameters.clone(), host_callback),
            parameters,
            square_oscillator: SquareOscillator::new(square_oscillator_parameters.clone()),
            sample_rate: 44000.0,
            note_duration: 0.0,
            note: None,
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "gui-vst".to_string(),
            vendor: "crsaracco".to_string(),
            unique_id: 1147000001, // Make sure this is a unique number across all of your VSTs!
            category: Category::Synth,
            inputs: 0,
            midi_inputs: 1,
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

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.process_midi_event(ev.data),
                // More events can be handled here.
                _ => ()
            }
        }
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let output_channels = buffer.output_count();
        let num_samples = buffer.samples();
        let (_, output_buffer) = buffer.split();

        // Precompute the samples that should go to each channel.
        // Our oscillator will output the same signal to all channels.
        let mut samples: Vec<f64> = Vec::new(); // NOTE: don't actually use Vec in a real synth!
        if let Some(_) = self.note {
            for _ in 0..(num_samples) {
                samples.push(self.square_oscillator.next_sample(self.sample_rate));
            }
        }
        else {
            for _ in 0..(num_samples) {
                // NOTE: You want to use some sort of envelope for real music use, otherwise you
                // will get clicks at the start and end of playback.
                samples.push(0.0);
            }
        }

        // Write the output to each channel.
        for channel in 0..output_channels {
            let output_channel = output_buffer.get_mut(channel);
            let mut sample_counter = 0;
            for output_sample in output_channel {
                *output_sample = samples[sample_counter] as f32;
                sample_counter += 1;
            }
        }
    }
}

plugin_main!(GuiVst);