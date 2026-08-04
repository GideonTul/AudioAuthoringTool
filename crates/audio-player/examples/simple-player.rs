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

    println!("Decoding audio...");
    let mut path: &str = "";

    if args.len() > 1 {
        path = &args[1];
    }
    else if path.is_empty() {
        println!("Usage: cargo run --example audio-player -- <path_to_audio_file>");
        return;
    }

    let mut sources: Vec<Box<dyn SampleSource>> = Vec::new();

    sources.push(SymphoniaLoader::auto(path).unwrap());

    // Add another sound to demonstrate multiple sources.
    // sources.push(Box::new(
    //     SymphoniaLoader::decode_static(
    //         "path/to/another/audio/file.wav",
    //     )
    //     .unwrap(),
    // ));

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
