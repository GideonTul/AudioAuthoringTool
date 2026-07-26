// Will need refactoring later, this code is mostly from https://docs.rs/cpal/latest/cpal/
// as a starting point.
use audio_backend::AudioBackend;

fn main() {
    let stream = AudioBackend::new(|data, _channels, _sample_rate| {
        for sample in data.iter_mut() {
            *sample = 0.0;
        }
    });

    // Play the stream for 5 seconds, then pause it.
    println!("Stream playing...");
    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(5));
    stream.pause().unwrap();
    println!("Stream paused...");
}
