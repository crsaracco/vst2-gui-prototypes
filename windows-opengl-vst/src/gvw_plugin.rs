use std::fs::File;
use std::sync::{Arc, Mutex};

use log::*;
use vst::editor::Editor as VstEditor;
use vst::{
    api::{Events, Supported},
    buffer::AudioBuffer,
    event::Event,
    plugin::{CanDo, Category, HostCallback, Info, Plugin},
};

use crate::audio_engine::AudioEngine;
use crate::editor::Editor;
use crate::midi_input_processor::MidiInputProcessor;
use crate::parameters::Parameters;

pub struct GvlPlugin {
    host: HostCallback,
    audio_engine: AudioEngine,
    midi_input_processor: MidiInputProcessor,
    params: Arc<Parameters>,
    editor: Box<dyn VstEditor>,
}

impl vst::plugin::Plugin for GvlPlugin {
    fn new(host: HostCallback) -> Self {
        // Set up a logger so we can see what's going on in the VST
        let mut logger_config = simplelog::Config::default();
        logger_config.time_format = Some("%H:%M:%S%.6f");
        simplelog::CombinedLogger::init(vec![simplelog::WriteLogger::new(
            simplelog::LevelFilter::max(),
            logger_config,
            File::create("C://users/crs/Documents/plugin.log").unwrap(),
        )])
            .unwrap();
        info!("====================================================================");
        info!("Plugin::new()");

        // Create the plugin itself
        let host_callback = Arc::new(Mutex::new(host));
        let params = Arc::new(Parameters::new());
        Self {
            host,
            audio_engine: AudioEngine::new(params.clone()),
            midi_input_processor: MidiInputProcessor::new(),
            params: params.clone(),
            editor: Box::new(Editor::new(host_callback, params.clone())),
        }
    }

    fn get_info(&self) -> Info {
        info!("get_info()");
        Info {
            name: "gvl".to_string(),
            vendor: "crsaracco".to_string(),
            unique_id: 1147000002, // Make sure this is a unique number across all of your VSTs!
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

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        self.audio_engine
            .process(buffer, self.midi_input_processor.get_active_notes());
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe,
        }
    }

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.midi_input_processor.process_midi_event(ev.data),
                // More events can be handled here.
                _ => (),
            }
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        info!("get_parameter({})", index);
        match index {
            0 => self.params.amplitude.get(),
            1 => self.params.pulse_width.get(),
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        info!("set_parameter()");
        match index {
            0 => self.params.amplitude.set(value),
            1 => self.params.pulse_width.set(value),
            _ => (),
        }
    }

    // "Amplitude", "Pulse width", etc.
    fn get_parameter_name(&self, index: i32) -> String {
        info!("get_parameter_name({})", index);
        match index {
            0 => format!("Amplitude"),
            1 => format!("Pulse width"),
            _ => format!(""),
        }
    }

    // Ignored by Bitwig, so I just put this in `get_parameter_text` instead.
    // "db", "sec", "ms", etc.
    fn get_parameter_label(&self, index: i32) -> String {
        info!("get_parameter_label({})", index);
        format!("asdf")
    }

    // "1.0", "150", "Plate", etc.
    fn get_parameter_text(&self, index: i32) -> String {
        info!("get_parameter_text({})", index);
        match index {
            0 => format!("{:0.2} %", self.params.amplitude.get() * 100.0), // Amplitude
            1 => format!("{:0.2} %", self.params.pulse_width.get() * 100.0), // Pulse width
            _ => format!(""),
        }
    }

    fn can_be_automated(&self, index: i32) -> bool {
        info!("can_be_automated({})", index);
        match index {
            0 => true, // Amplitude
            1 => true, // Pulse width
            _ => false,
        }
    }

    fn get_editor(&mut self) -> Option<&mut vst::editor::Editor> {
        //info!("Plugin::get_editor()");
        Some(self.editor.as_mut())
    }
}

impl Default for GvlPlugin {
    fn default() -> Self {
        Plugin::new(HostCallback::default())
    }
}
