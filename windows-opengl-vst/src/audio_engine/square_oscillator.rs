use std::sync::Arc;
use std::f64;

use crate::parameters::Parameters;

pub struct SquareOscillator {
    params: Arc<Parameters>,
    frequency: f64,
    phase: f64,
}

impl SquareOscillator {
    pub fn new(params: Arc<Parameters>) -> Self {
        Self {
            params,
            frequency: 0.0,
            phase: 0.0,
        }
    }

    pub fn change_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
    }

    pub fn next_sample(&mut self, sample_rate: f32) -> f64 {
        let mut output: f64 = 1.0;

        if self.phase <= self.params.pulse_width.get() as f64 {
            output = -1.0;
        }

        self.phase = (self.phase + self.frequency / sample_rate as f64).fract();

        output * (self.params.amplitude.get() as f64)
    }
}
