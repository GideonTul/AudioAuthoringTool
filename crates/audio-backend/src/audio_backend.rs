use cpal::{SampleFormat, StreamConfig, Stream};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

/// AudioFormat struct represents the audio format with channels and sample rate.
#[derive(Debug, Clone, Copy)] // derive Debug, Clone, and Copy for easy printing and copying of AudioFormat instances
pub struct AudioFormat {
    pub channels: u16,
    pub sample_rate: u32,
}

/// AudioBackend struct represents the audio backend with a stream and format.
pub struct AudioBackend {
    stream: Stream,
    format: AudioFormat,
}

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

/// AudioBackend implementaion
impl AudioBackend {
    /// Creates a new AudioBackend instance with the given render callback.
    // The render callback is a closure that takes a mutable slice of f32 samples, the number of channels, and the sample rate.
    // may remove the format parameter in the future. (May remove AudioFormat entirely.)
    pub fn new<F>(mut callback: F) -> Result<Self, Box<dyn std::error::Error>>
    where
        // FnMut is a trait for closures that can be called multiple times and can mutate their environment.
        F: FnMut(&mut [f32], AudioFormat) + Send + 'static,
    {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .ok_or("No output audio device found")?;

        let supported_config = device.default_output_config()?;

        let sample_format = supported_config.sample_format();
        let config: StreamConfig = supported_config.into();

        let format = AudioFormat {
            channels: config.channels,
            sample_rate: config.sample_rate,
        };

        let callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            callback(data, format);
        };

        // Create the output stream based on the sample format.
        let stream = match sample_format {
            // For now, we only support f32 samples. We can add support for other formats later if needed.
            SampleFormat::F32 => {
                // The build_output_stream method creates a new output stream with the given configuration and callback.
                device.build_output_stream(
                    // The configuration for the output stream.
                    config,
                    // The callback that will be called when the stream needs more data.
                    callback,
                    Self::err_fn,
                    None,
                )?
            }

            format => {
                return Err(
                    format!("Unsupported sample format: {:?}", format).into()
                );
            }
        };

        Ok(Self {
            stream,
            format,
        })
    }

    pub fn sample_rate(&self) -> u32 {
        self.format.sample_rate
    }

    pub fn channels(&self) -> u16 {
        self.format.channels
    }

    fn err_fn(err: cpal::Error) {
        eprintln!("an error occurred on the output audio stream: {}", err);
    }

    pub fn format(&self) -> AudioFormat {
        self.format
    }

    pub fn play(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.stream.play()?;
        Ok(())
    }

    pub fn pause(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.stream.pause()?;
        Ok(())
    }
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
        let backend = AudioBackend::new(|data, _format| { 
            for sample in data.iter_mut() {
                *sample = 0.0;
            }
        }).expect("Failed to initialize backend in test");

        // Don't assert specific numbers - they depend on whatever's
        // actually plugged into the machine running this test.
        assert!(backend.channels() > 0);
        assert!(backend.sample_rate() > 0);
    }

    #[test]
    fn test_audio_backend_play_pause() {
        let backend = AudioBackend::new(|data, _format| {
            for sample in data.iter_mut() {
                *sample = 0.0;
            }
        }).expect("Failed to initialize backend in test");

        assert!(backend.play().is_ok());
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert!(backend.pause().is_ok());
    }
}
