use symphonia::core::errors::Error;

/// The mode the audio source is loaded with.
pub enum LoadMode {
    Static, // might rename later
    Streaming,
    Auto,
}

/// A trait for audio sources that provide PCM sample streams.
//  (Basically an interface)
pub trait SampleSource: Send + 'static {
    fn sample_rate(&self) -> u32;
    fn channels(&self) -> u16;
    fn read(&mut self, out_buf: &mut [f32]) -> Result<usize, Error>;
}

