extern crate portaudio;

use portaudio as pa;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod note;

const CHANNELS: i32 = 1;
const FRAMES_PER_BUFFER: u32 = 64;

fn main() {
    match run() {
        Ok(_) => {}
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn run() -> Result<(), pa::Error> {
    println!(
        "PortAudio Test: output sine wave. SR = {}, BufSize = {}",
        note::SAMPLE_RATE,
        FRAMES_PER_BUFFER
    );

    let pa = pa::PortAudio::new()?;
    let mut settings =
        pa.default_output_stream_settings(CHANNELS, note::SAMPLE_RATE, FRAMES_PER_BUFFER)?;
    settings.flags = pa::stream_flags::CLIP_OFF;

    // initialize a synth, and enable C4 and C5 to play an octave
    let mut synth = note::Synth::new();
    if let Some(_) = synth.set_note(44, true) {}
    if let Some(_) = synth.set_note(56, true) {}

    let mut phase = 0;
    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        for i in 0..frames {
            buffer[i] = synth.wavejoin(phase);
            phase += 1;
        }
        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    println!("Press 'q' or CTRL+C to exit.");
    stream.start()?;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Ctrl('c') => break,
            _ => {}
        }
        stdout.flush().unwrap();
    }

    stream.stop()?;
    stream.close()?;

    write!(stdout, "{}", termion::cursor::Show).unwrap();

    Ok(())
}
