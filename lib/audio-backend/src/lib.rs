use cpal::{SampleFormat, StreamConfig, Stream};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[derive(Debug, Clone, Copy)] // derive Debug, Clone, and Copy for easy printing and copying of AudioFormat instances
pub struct AudioFormat {
    pub channels: u16,
    pub sample_rate: u32,
}

pub struct AudioBackend {
    stream: Stream,
    format: AudioFormat,
}

//*
// Will eventually handle errors more gracefully, but for now we will just panic on errors.
// */


// EXPLANATION of AudioBackend::new() in the context of Rust closures and traits:
// https://doc.rust-lang.org/book/ch13-01-closures.html
// 
// This is kind of like cpp templates. Similar to
// template<typename F>
// requires std::invocable<F&, float*, uint16_t, uint32_t> in cpp.
//
// From what I have read, passing a closure to a function is the best way to handle callbacks in Rust,
// particularly for audio, so we can keep track of the state of the audio stream in the closure. 
// And so the audio can be generated in real time, which I think we will need.

impl AudioBackend {
    pub fn new<F>(mut render: F) -> Self
    where
        F: FnMut(&mut [f32], u16, u32) + Send + 'static, 
        // FnMut is a trait for closures that can be called multiple times and can mutate their environment.
        // the Send bound and the 'static lifetime bound are required for the closure to be used in a separate thread.
    {
        let host = cpal::default_host();

        let device = host.default_output_device()
                    .expect("Failed to get default output device");

        let supported_config = device
            .default_output_config()
            .expect("Failed to get default output config");

        let sample_format = supported_config.sample_format();
        let config: StreamConfig = supported_config.into();

        let format = AudioFormat {
            channels: config.channels,
            sample_rate: config.sample_rate,
        };

        let callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            render(data, format.channels, format.sample_rate);
        };


        // We are most likely going to convert audio data to f32
        // so we will only implement the f32 case for now.
       let stream = match sample_format {
            SampleFormat::F32 => device.build_output_stream(
                config, 
                callback,
                Self::err_fn, 
                None
            ).expect("Failed to build output stream"),

            _ => unimplemented!(),
        };

        Self { stream, format }
    }

    fn err_fn(err: cpal::Error) {
        eprintln!("an error occurred on the output audio stream: {}", err);
    }

    pub fn format(&self) -> AudioFormat {
        self.format
    }

    pub fn play(&self) -> Result<(), cpal::Error> { self.stream.play() }
    pub fn pause(&self) -> Result<(), cpal::Error> { self.stream.pause() }
}

//////////////////////////////////
// TESTS
//////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_format() {
        let format = AudioFormat {
            channels: 2,
            sample_rate: 44100,
        };
        assert_eq!(format.channels, 2);
        assert_eq!(format.sample_rate, 44100);
    }

    #[test]
    fn test_audio_backend_creation() {
        // |data, _channels, _sample_rate| is like [](data, _channels, _sample_rate) in cpp.
        let backend = AudioBackend::new(|data, _channels, _sample_rate| { 
            for sample in data.iter_mut() {
                *sample = 0.0;
            }
        });

        // Don't assert specific numbers - they depend on whatever's
        // actually plugged into the machine running this test.
        assert!(backend.format.channels > 0);
        assert!(backend.format.sample_rate > 0);
    }

    #[test]
    fn test_audio_backend_play_pause() {
        let backend = AudioBackend::new(|data, _channels, _sample_rate| {
            for sample in data.iter_mut() {
                *sample = 0.0;
            }
        });

        assert!(backend.play().is_ok());
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert!(backend.pause().is_ok());
    }
}
