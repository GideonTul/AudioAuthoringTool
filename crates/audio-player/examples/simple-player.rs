use std::env;
use std::sync::{
    atomic::Ordering,
};
use std::time::Duration;

use audio_backend::AudioBackend;
use audio_decode::{SampleSource, SymphoniaLoader};
use audio_engine::AudioRenderer;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut path: &str = "";

    let mut sources: Vec<Box<dyn SampleSource>> = Vec::new();

    if args.len() > 1 {
        for arg in &args[1..] {
            path = arg;
            sources.push(SymphoniaLoader::auto(path).unwrap());
        }
    }
    else if path.is_empty() {
        println!("Usage: cargo run -p audio-player --example simple-player <path_to_audio_file>");
        return;
    }
    
    println!("Decoding audio...");

    println!(
        "Sample rate: {}, Channels: {}",
        sources[0].sample_rate(),
        sources[0].channels()
    );

    let mut renderer = AudioRenderer::new();
    let finished = renderer.finished();

    // Create an audio stream using the AudioBackend and provide a callback for rendering audio.
    let stream = AudioBackend::new(move |buffer, _, _| {
        renderer.render(buffer, &mut sources);
    });

    println!("Stream playing...");
    stream.play().unwrap();

    // Wait until the audio stream has finished playing.
    while !finished.load(Ordering::Relaxed) {
        std::thread::sleep(Duration::from_millis(50));
    }

    println!("Stream finished...");
}
