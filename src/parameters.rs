use crate::atomic_float::AtomicFloat;

pub struct Parameters {
    pub param1: AtomicFloat,
    pub param2: AtomicFloat,
}

impl Parameters {
    pub fn new() -> Self {
        Self {
            param1: AtomicFloat::new(1.0),
            param2: AtomicFloat::new(0.5),
        }
    }
}