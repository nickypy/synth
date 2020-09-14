use std::f64::consts::PI;

pub const A440: f64 = 440.0;
pub const BASE: f64 = 1.059_463_094_359;
pub const SAMPLE_RATE: f64 = 44_100.0;

const AMPLITUDE: f32 = 0.25;

pub fn wavejoin(tables: &[Vec<f32>], phase: i64) -> f32 {
    let mut total = 0.0;
    for table in tables.iter() {
        let index = phase % table.len() as i64;
        total += table[index as usize];
    }
    total
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
