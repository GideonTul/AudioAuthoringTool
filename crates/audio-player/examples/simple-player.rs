use std::env;
use std::sync::{
    atomic::Ordering,
};
use std::time::Duration;

use audio_backend::AudioBackend;
use audio_decode::{SampleSource, SymphoniaLoader};
use audio_engine::VoiceMixer;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut path: &str = "";

    let mut mixer = VoiceMixer::new();

    if args.len() > 1 {
        for arg in &args[1..] {
            path = arg;
            mixer.add_voice(SymphoniaLoader::auto(path).unwrap(), 1.0);
        }
    }
    else if path.is_empty() {
        println!("Usage: cargo run -p audio-player --example simple-player <path_to_audio_file>");
        return;
    }
    
    println!("Decoding audio...");

    // let mut renderer = AudioRenderer::new();
    let finished = mixer.finished();
    let mut backend = match AudioBackend::new() {
        Ok(backend) => backend,
        Err(e) => {
            eprintln!("Failed to create audio backend: {e}");
            return;
        }
    };

    // start() is what actually builds the stream, wiring in this callback.
    if let Err(e) = backend.start(move |buffer, _| {
        // renderer.render(buffer, &mut sources);
        mixer.render(buffer);
    }) {
        eprintln!("Failed to start audio backend: {e}");
        return;
    }

    if let Err(e) = backend.play() {
        eprintln!("Failed to play audio backend: {e}");
        return;
    }

    // Wait until the audio stream has finished playing.
    while !finished.load(Ordering::Relaxed) {
        std::thread::sleep(Duration::from_millis(50));
    }

    println!("Stream finished...");
}
