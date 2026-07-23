// Will need refactoring later, this code is mostly from https://docs.rs/cpal/latest/cpal/
// as a starting point.
use cpal::{Sample, SampleFormat, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};


fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::EQUILIBRIUM;
    }
}

fn err_fn(err: cpal::Error) {
    eprintln!("an error occurred on the output audio stream: {}", err);
}

fn make_stream() -> cpal::Stream {
    // setup audio output stream
    let host = cpal::default_host();
    let device = host.default_output_device().expect("Failed to get default output device");

    let supported_config = device
        .default_output_config()
        .unwrap();


    let sample_format = supported_config.sample_format();
    let config: StreamConfig = supported_config.into();

    // We are most likely going to convert audio data to f32
    // so we will only implement the f32 case for now.
    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(
            config, 
            write_silence::<f32>, // This is where the audio data is written to output.
            err_fn, 
            None
        ),

        _ => unimplemented!(),
    }.unwrap();

    return stream;
}

fn main() {
    let stream = make_stream();

    // Play the stream for 5 seconds, then pause it.
    println!("Stream playing...");
    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(5));
    stream.pause().unwrap();
    println!("Stream paused...");
}