mod atomic_float;
use self::atomic_float::AtomicFloat;

pub struct Parameters {
    pub amplitude: AtomicFloat,
    pub pulse_width: AtomicFloat,
}

impl Parameters {
    pub fn new() -> Self {
        Self {
            amplitude: AtomicFloat::new(0.3),
            pulse_width: AtomicFloat::new(0.5),
        }
    }
}
