use std::collections::BTreeSet;

pub struct MidiInputProcessor {
    notes: BTreeSet<u8>,
}

impl MidiInputProcessor {
    pub fn new() -> Self {
        Self {
            notes: BTreeSet::new(),
        }
    }

    pub fn process_midi_event(&mut self, event_data: [u8; 3]) {
        match event_data[0] {
            128 => self.note_off(event_data[1]),
            144 => self.note_on(event_data[1]),
            _ => (),
        }
    }

    fn note_on(&mut self, index: u8) {
        self.notes.insert(index);
    }

    fn note_off(&mut self, index: u8) {
        self.notes.remove(&index);
    }

    pub fn get_active_notes(&self) -> Vec<u8> {
        let mut active_notes = Vec::new(); // TODO: Might not want to use Vec
        for note in &self.notes {
            active_notes.push(*note);
        }
        active_notes
    }
}
