use std::sync::Arc;

use crate::parameters::Parameters;

pub struct SquareOscillator {
    parameters: Arc<Parameters>,
    frequency: f64,
    phase: f64,
}

impl SquareOscillator {
    /// Creates a new Sine wave signal generator.
    pub fn new(parameters: Arc<Parameters>) -> Self {
        Self {
            parameters,
            frequency: 0.0,
            phase: 0.0,
        }
    }

    pub fn change_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
    }

    pub fn next_sample(&mut self, sample_rate: f64) -> f64 {
        let mut output = 1.0;

        if self.phase <= self.parameters.param2.get() as f64 {
            output = -1.0;
        }

        self.phase = (self.phase + self.frequency / sample_rate).fract();

        output * self.parameters.param1.get() as f64
    }
}