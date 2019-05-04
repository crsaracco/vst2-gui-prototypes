use std::sync::Arc;
use std::{mem, ptr};
use vst::buffer::AudioBuffer;

mod square_oscillator;
use self::square_oscillator::SquareOscillator;
use crate::parameters::Parameters;

pub struct AudioEngine {
    oscillators: [SquareOscillator; 128],
    sample_rate: f32,
}

impl AudioEngine {
    pub fn new(params: Arc<Parameters>) -> Self {
        // Create a bank of 128 different SquareOscillators, tuned to all the different MIDI notes.
        // This is a terrible way to actually do it, but it works for now.
        // TODO: make this a little smarter.
        let oscillators = unsafe {
            let mut oscillators: [SquareOscillator; 128] = mem::uninitialized();

            for (i, osc) in oscillators.iter_mut().enumerate() {
                let mut oscillator = SquareOscillator::new(params.clone());
                oscillator.change_frequency(midi_pitch_to_freq(i as u8));
                ptr::write(osc, oscillator);
            }

            oscillators
        };

        Self {
            oscillators,
            sample_rate: 44100.0,
        }
    }

    pub fn process(&mut self, buffer: &mut AudioBuffer<f32>, active_notes: Vec<u8>) {
        let output_channels = buffer.output_count();
        let num_samples = buffer.samples();
        let (_, output_buffer) = buffer.split();

        // Precompute the samples that should go to each channel.
        // Our oscillator will output the same signal to all channels.
        // TODO: put a tiny envelope here (based on sample rate) to prevent clicks

        // TODO: don't allocate inside process() !!!
        let mut samples: Vec<f64> = Vec::new();
        samples.resize(num_samples, 0.0);

        for sample_num in 0..num_samples {
            for note in &active_notes {
                samples[sample_num] +=
                    self.oscillators[*note as usize].next_sample(self.sample_rate);
            }
        }

        // Write the output to each channel.
        for channel in 0..output_channels {
            let output_channel = output_buffer.get_mut(channel);
            let mut sample_counter = 0;
            for output_sample in output_channel {
                let sample_value = samples[sample_counter] as f32;
                *output_sample = sample_value;
                sample_counter += 1;
            }
        }
    }
}

fn midi_pitch_to_freq(pitch: u8) -> f64 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f64 = 440.0;

    (((pitch as i8 - A4_PITCH) as f64) / 12.).exp2() * A4_FREQ
}
