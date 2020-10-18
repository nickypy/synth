use std::f64::consts::PI;

pub const A440: f64 = 440.0;
pub const BASE: f64 = 1.059_463_094_359;
pub const SAMPLE_RATE: f64 = 44_100.0;

const AMPLITUDE: f32 = 0.25;

pub struct Note {
    wavetable: Vec<f32>,
    enabled: bool,
}

pub struct Synth {
    notes: Vec<Note>,
}

impl Synth {
    pub fn new() -> Synth {
        let mut notes = Vec::new();
        // middle C is C4, so we need to compute all the keys from C0-C8
        for i in -44..=44 {
            notes.push(Note {
                wavetable: wavetable(i),
                enabled: false,
            });
        }

        Synth { notes }
    }

    pub fn wavejoin(&self, phase: i64) -> f32 {
        let mut total = 0.0;
        for note in self.notes.iter() {
            if note.enabled {
                let table = &note.wavetable;
                let index = phase % table.len() as i64;
                total += table[index as usize];
            }
        }
        total
    }

    pub fn set_note(&mut self, note_idx: usize, enabled: bool) -> Option<bool> {
        match self.notes.get_mut(note_idx) {
            Some(n) => {
                n.enabled = enabled;
                Some(enabled)
            }
            None => None,
        }
    }
}

pub fn wavetable(half_steps: i32) -> Vec<f32> {
    let pitch = hertz(half_steps);
    let table_size = (SAMPLE_RATE / pitch).floor() as usize;

    let mut table = Vec::new();

    for i in 0..table_size {
        let time = i as f64 / table_size as f64;
        table.push(AMPLITUDE * (time * 2.0 * PI).sin() as f32);
    }

    table
}

pub fn hertz(half_steps: i32) -> f64 {
    BASE.powi(half_steps) * A440
}
